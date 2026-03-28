use crate::PyWebSocketMessageKind;
use crate::types::{PyWsCloseCode, PyWsCloseReason};
use bytes::Bytes;
use pyo3::{IntoPyObjectExt, prelude::*, pybacked::PyBackedStr};
use ryo3_bytes::PyBytes as RyBytes;
use ryo3_core::{py_type_err, py_value_err};
use tokio_websockets::Message;

#[derive(Debug, Clone)]
#[pyclass(name = "WsMessage", frozen, immutable_type, skip_from_py_object)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
pub struct PyWsMessage(pub(crate) Message);

impl PyWsMessage {
    fn payload_bytes(&self) -> Bytes {
        self.0.clone().into_payload().into()
    }
}

#[pymethods]
impl PyWsMessage {
    #[expect(clippy::needless_pass_by_value)]
    #[staticmethod]
    fn text(text: PyBackedStr) -> Self {
        Self::from(Message::text(text.to_string()))
    }

    #[staticmethod]
    fn binary(data: RyBytes) -> Self {
        Self::from(Message::binary(data.into_inner()))
    }

    #[staticmethod]
    #[pyo3(signature = (payload = None))]
    fn ping(payload: Option<PyPingPayload>) -> Self {
        let payload = payload.unwrap_or_else(|| PyPingPayload(Message::ping(Bytes::new())));
        payload.into()
    }

    #[staticmethod]
    #[pyo3(signature = (payload = None))]
    fn pong(payload: Option<PyPongPayload>) -> Self {
        let payload = payload.unwrap_or_else(|| PyPongPayload(Message::pong(Bytes::new())));
        payload.into()
    }
    /// # Panics
    /// - If the `code` is reserved so it cannot be sent.
    /// - If `code` is present and the `reason` exceeds 123 bytes, the
    ///   protocol-imposed limit.
    #[staticmethod]
    #[pyo3(signature = (*, code = None, reason = None))]
    pub(crate) fn close(
        code: Option<PyWsCloseCode>,
        reason: Option<PyWsCloseReason>,
    ) -> PyResult<Self> {
        // check for reserved codes
        if let Some(code) = &code
            && code.is_reserved()
        {
            return py_value_err!(
                "close code {} is reserved and cannot be sent",
                u16::from(code.0)
            )?;
        }
        // check that if reason is present/non-empty, code is also present
        let code = code.map(|c| c.0);
        let reason = reason.map(|r| r.0).unwrap_or_default();
        if !reason.is_empty() && code.is_none() {
            return py_value_err!("a close reason requires a close code");
        }
        Ok(Self::from(Message::close(code, reason.as_ref())))
    }

    fn __repr__(&self) -> String {
        format!("{self}")
    }

    #[getter]
    fn kind(&self) -> PyWebSocketMessageKind {
        self.into()
    }

    #[getter]
    fn is_text(&self) -> bool {
        self.0.is_text()
    }

    #[getter]
    fn is_binary(&self) -> bool {
        self.0.is_binary()
    }

    #[getter]
    fn is_close(&self) -> bool {
        self.0.is_close()
    }

    #[getter]
    fn is_ping(&self) -> bool {
        self.0.is_ping()
    }

    #[getter]
    fn is_pong(&self) -> bool {
        self.0.is_pong()
    }

    #[getter]
    fn text_data(&self) -> Option<String> {
        self.0.as_text().map(ToOwned::to_owned)
    }

    #[getter]
    fn data<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        if let Some(text) = self.0.as_text() {
            text.into_bound_py_any(py)
        } else {
            RyBytes::from(self.payload_bytes()).into_bound_py_any(py)
        }
    }

    #[getter]
    fn payload(&self) -> RyBytes {
        RyBytes::from(self.payload_bytes())
    }

    #[getter]
    fn close_code(&self) -> Option<u16> {
        self.0.as_close().map(|(code, _)| u16::from(code))
    }

    #[getter]
    fn close_reason(&self) -> Option<String> {
        self.0.as_close().map(|(_, reason)| reason.to_owned())
    }
}

impl std::fmt::Display for PyWsMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(text) = self.0.as_text() {
            write!(f, "Message(type='text', data={text:?})")
        } else if let Some((code, reason)) = self.0.as_close() {
            write!(
                f,
                "Message(type=\"close\", code={}, reason={reason:?})",
                u16::from(code)
            )
        } else {
            write!(
                f,
                "Message(type='{}', data={:?})",
                self.kind().as_str(),
                self.payload_bytes()
            )
        }
    }
}

pub(crate) enum PyMessageLike {
    Message(PyWsMessage),
    Text(PyBackedStr),
    Bytes(RyBytes),
}

impl<'py> FromPyObject<'_, 'py> for PyMessageLike {
    type Error = PyErr;

    fn extract(obj: Borrowed<'_, 'py, PyAny>) -> PyResult<Self> {
        if let Ok(msg) = obj.cast::<PyWsMessage>() {
            Ok(Self::Message(msg.get().clone()))
        } else if let Ok(text) = obj.extract::<PyBackedStr>() {
            Ok(Self::Text(text))
        } else if let Ok(bytes) = obj.extract::<RyBytes>() {
            Ok(Self::Bytes(bytes))
        } else {
            py_type_err!("expected Message, str, or bytes-like object")
        }
    }
}

#[derive(Debug)]
pub(crate) struct PyPingPayload(pub(crate) Message);

impl PyPingPayload {
    pub(crate) fn into_inner(self) -> Message {
        self.0
    }
}
impl std::default::Default for PyPingPayload {
    fn default() -> Self {
        Self(Message::ping(Bytes::new()))
    }
}

impl<'py> FromPyObject<'_, 'py> for PyPingPayload {
    type Error = PyErr;

    fn extract(obj: Borrowed<'_, 'py, PyAny>) -> PyResult<Self> {
        let bytes = obj.extract::<RyBytes>()?;
        if bytes.as_slice().len() > 125 {
            py_value_err!("ping-payload exceeds the websocket limit of 125 bytes")
        } else {
            Ok(Self(Message::ping(bytes.into_inner())))
        }
    }
}

#[derive(Debug)]
pub(crate) struct PyPongPayload(pub(crate) Message);

impl PyPongPayload {
    pub(crate) fn into_inner(self) -> Message {
        self.0
    }
}
impl std::default::Default for PyPongPayload {
    fn default() -> Self {
        Self(Message::pong(Bytes::new()))
    }
}

impl<'py> FromPyObject<'_, 'py> for PyPongPayload {
    type Error = PyErr;

    fn extract(obj: Borrowed<'_, 'py, PyAny>) -> PyResult<Self> {
        let bytes = obj.extract::<RyBytes>()?;
        if bytes.as_slice().len() > 125 {
            py_value_err!("pong-payload exceeds the websocket limit of 125 bytes")
        } else {
            Ok(Self(Message::pong(bytes.into_inner())))
        }
    }
}
