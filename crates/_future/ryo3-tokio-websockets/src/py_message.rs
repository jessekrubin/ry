use crate::util::validate_close_reason;
use bytes::Bytes;
use pyo3::{IntoPyObjectExt, prelude::*, pybacked::PyBackedStr};
use ryo3_bytes::PyBytes as RyBytes;
use ryo3_core::{py_type_err, py_value_err};
use tokio_websockets::Message;

#[derive(Debug, Clone)]
#[pyclass(name = "Message", frozen, immutable_type, skip_from_py_object)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
pub struct PyWebSocketMessage(pub(crate) Message);

impl PyWebSocketMessage {
    fn payload_bytes(&self) -> Bytes {
        self.0.clone().into_payload().into()
    }

    fn inner(&self) -> &Message {
        &self.0
    }

    fn type_name(&self) -> &'static str {
        if self.0.is_text() {
            "text"
        } else if self.0.is_binary() {
            "binary"
        } else if self.0.is_close() {
            "close"
        } else if self.0.is_ping() {
            "ping"
        } else {
            "pong"
        }
    }
}

#[pymethods]
impl PyWebSocketMessage {
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

    #[staticmethod]
    #[pyo3(signature = (*, code = None, reason = ""))]
    fn close(code: Option<u16>, reason: &str) -> PyResult<Self> {
        let code = validate_close_reason(code, reason)?;
        Ok(Self::from(Message::close(code, reason)))
    }

    fn __repr__(&self) -> String {
        if let Some(text) = self.0.as_text() {
            format!("Message(type='text', data={text:?})")
        } else if let Some((code, reason)) = self.0.as_close() {
            format!(
                "Message(type='close', code={}, reason={reason:?})",
                u16::from(code)
            )
        } else {
            format!(
                "Message(type='{}', data={:?})",
                self.type_name(),
                self.payload_bytes()
            )
        }
    }

    #[getter]
    fn kind(&self) -> &'static str {
        self.type_name()
    }

    #[getter]
    fn message_type(&self) -> &'static str {
        self.type_name()
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
pub(crate) enum PyMessageLike {
    Message(PyWebSocketMessage),
    Text(PyBackedStr),
    Bytes(RyBytes),
}

impl<'py> FromPyObject<'_, 'py> for PyMessageLike {
    type Error = PyErr;

    fn extract(obj: Borrowed<'_, 'py, PyAny>) -> PyResult<Self> {
        if let Ok(msg) = obj.cast::<PyWebSocketMessage>() {
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

// impl PyPingPayload {
//     pub(crate) fn into_ping_message(self) -> Message {
//         Message::ping(self.0.into_inner())
//     }

//     pub(crate) fn into_pong_message(self) -> Message {
//         Message::pong(self.0.into_inner())
//     }
// }

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
