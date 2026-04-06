use std::{ops::Deref, os::raw::c_int};

use bytes::Bytes;
use pyo3::{
    IntoPyObjectExt,
    prelude::*,
    pybacked::PyBackedStr,
    types::{PyDict, PyTuple},
};
use ryo3_bytes::{PyBytes as RyBytes, ReadableBuffer};
use ryo3_core::{PyTryFrom, py_type_err, py_value_err, py_value_error};
use tokio_websockets::Message;

use crate::{
    PyWebSocketMessageKind,
    constants::WS_MSG_PINGPONG_PAYLOAD_MAX_LEN,
    types::{PyWsCloseCode, PyWsCloseReason},
};

#[derive(Debug, Clone)]
#[pyclass(name = "WsMessage", frozen, immutable_type, skip_from_py_object)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
pub struct PyWsMessage(pub(crate) Message);

impl PyWsMessage {
    fn payload_bytes(&self) -> Bytes {
        self.0.clone().into_payload().into()
    }
}

impl PartialEq for PyWsMessage {
    fn eq(&self, other: &Self) -> bool {
        self.kind() == other.kind() && self.0.as_payload().deref() == other.0.as_payload().deref()
    }
}

#[pymethods]
impl PyWsMessage {
    #[new]
    #[pyo3(signature = (kind, data = None, *, code = None, reason = None))]
    fn py_new(
        kind: PyWebSocketMessageKind,
        data: Option<Bound<'_, PyAny>>,
        code: Option<u16>,
        reason: Option<&str>,
    ) -> PyResult<Self> {
        if matches!(kind, PyWebSocketMessageKind::Close) && data.is_some() {
            return py_value_err!("data not valid for close message")?;
        }
        if !matches!(kind, PyWebSocketMessageKind::Close) && (code.is_some() || reason.is_some()) {
            return py_value_err!("code/reason not valid for non-close message")?;
        }
        match kind {
            PyWebSocketMessageKind::Text => {
                let data = data
                    .ok_or_else(|| py_value_error!("data required for message w/ kind='text'"))?;
                let s = data.extract::<PyBackedStr>()?;
                Ok(Self::from(Message::text(s.to_string())))
            }
            PyWebSocketMessageKind::Binary => {
                let data = data
                    .ok_or_else(|| py_value_error!("data required for message w/ kind='binary'"))?;
                let bytes = data.extract::<RyBytes>()?;
                Ok(Self::from(Message::binary(bytes.into_inner())))
            }
            PyWebSocketMessageKind::Ping => {
                let pl = data
                    .map(|d| d.extract::<RyBytes>().and_then(PyPingPayload::py_try_from))
                    .transpose()?
                    .unwrap_or_default();
                Ok(Self::from(pl.into_inner()))
            }
            PyWebSocketMessageKind::Pong => {
                let pl = data
                    .map(|d| d.extract::<RyBytes>().and_then(PyPongPayload::py_try_from))
                    .transpose()?
                    .unwrap_or_default();
                Ok(Self::from(pl.into_inner()))
            }
            PyWebSocketMessageKind::Close => {
                let code = code
                    .map(|c| {
                        tokio_websockets::CloseCode::try_from(c)
                            .map(PyWsCloseCode::from)
                            .map_err(|_| py_value_error!("invalid close code: {c}"))
                    })
                    .transpose()?
                    .unwrap_or(PyWsCloseCode::NORMAL_CLOSURE);
                let reason = reason.map(|r| PyWsCloseReason(r.to_owned()));
                Self::close(code, reason)
            }
        }
    }

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
    #[expect(clippy::needless_pass_by_value)]
    #[staticmethod]
    #[pyo3(
        signature = (code = PyWsCloseCode::NORMAL_CLOSURE, reason = None),
        text_signature = "(code=1_000, reason=None)"
    )]
    pub(crate) fn close(code: PyWsCloseCode, reason: Option<PyWsCloseReason>) -> PyResult<Self> {
        // check for reserved codes
        if code.is_reserved() {
            return py_value_err!(
                "close code {} is reserved and cannot be sent",
                u16::from(code.0)
            )?;
        }
        // check that if reason is present/non-empty, code is also present
        let reason = reason.map(|r| r.0).unwrap_or_default();
        Ok(Self::from(Message::close(Some(code.0), reason.as_ref())))
    }

    fn __repr__(&self) -> String {
        format!("{self}")
    }

    fn __eq__(&self, other: &Self) -> bool {
        self == other
    }

    fn __len__(&self) -> usize {
        self.payload_bytes().len()
    }

    fn __getnewargs_ex__<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyTuple>> {
        let kwargs = PyDict::new(py);
        let args = if let Some(text) = self.0.as_text() {
            PyTuple::new(
                py,
                [
                    PyWebSocketMessageKind::Text.into_bound_py_any(py)?,
                    text.into_bound_py_any(py)?,
                ],
            )?
        } else if let Some((code, reason)) = self.0.as_close() {
            kwargs.set_item(pyo3::intern!(py, "code"), u16::from(code))?;
            kwargs.set_item(pyo3::intern!(py, "reason"), reason)?;
            PyTuple::new(py, [PyWebSocketMessageKind::Close.into_bound_py_any(py)?])?
        } else {
            let kind = self.kind();
            PyTuple::new(
                py,
                [
                    kind.into_bound_py_any(py)?,
                    self.payload_bytes().as_ref().into_bound_py_any(py)?,
                ],
            )?
        };
        PyTuple::new(
            py,
            [args.into_bound_py_any(py)?, kwargs.into_bound_py_any(py)?],
        )
    }

    #[expect(unsafe_code, clippy::needless_pass_by_value, clippy::ptr_as_ptr)]
    unsafe fn __getbuffer__(
        slf: PyRef<Self>,
        view: *mut pyo3::ffi::Py_buffer,
        flags: c_int,
    ) -> PyResult<()> {
        unsafe {
            let bytes = slf.0.as_payload().deref();
            let ret = pyo3::ffi::PyBuffer_FillInfo(
                view,
                slf.as_ptr() as *mut _,
                bytes.as_ptr() as *mut _,
                bytes.len().try_into()?,
                1, // read only
                flags,
            );
            if ret == -1 {
                return Err(PyErr::fetch(slf.py()));
            }
            Ok(())
        }
    }

    #[expect(unsafe_code, clippy::unused_self)]
    unsafe fn __releasebuffer__(&self, _view: *mut pyo3::ffi::Py_buffer) {}

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

    // TODO: possibly add the other kwargs to this that jiter provides
    /// Parse the message payload as JSON
    #[cfg(feature = "json")]
    fn json<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        ryo3_jiter::JiterParseOptions::default().parse(py, self.0.as_payload())
    }

    fn __bytes__<'py>(&'py self, py: Python<'py>) -> Bound<'py, pyo3::types::PyBytes> {
        pyo3::types::PyBytes::new(py, self.0.as_payload())
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
    fn code(&self) -> Option<u16> {
        self.0.as_close().map(|(code, _)| u16::from(code))
    }

    #[getter]
    fn reason(&self) -> Option<String> {
        self.0.as_close().map(|(_, reason)| reason.to_owned())
    }
}

impl std::fmt::Display for PyWsMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(text) = self.0.as_text() {
            write!(f, "WsMessage.text({text:?})")
        } else if let Some((code, reason)) = self.0.as_close() {
            write!(
                f,
                "WsMessage.close(code={}, reason={reason:?})",
                u16::from(code)
            )
        } else if self.0.is_binary() {
            write!(f, "WsMessage.binary({:?})", self.payload_bytes())
        } else if self.0.is_ping() {
            write!(f, "WsMessage.ping({:?})", self.payload_bytes())
        } else if self.0.is_pong() {
            write!(f, "WsMessage.pong({:?})", self.payload_bytes())
        } else {
            write!(f, "WsMessage(unknown)")
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
        // TODO: remove clones and stuff here
        if let Ok(msg) = obj.cast_exact::<PyWsMessage>() {
            Ok(Self::Message(msg.get().clone()))
        } else if let Ok(text) = obj.extract::<PyBackedStr>() {
            Ok(Self::Text(text))
        } else if let Ok(bytes) = obj.extract::<ReadableBuffer>() {
            Ok(Self::Bytes(bytes.as_rybytes()?))
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

impl PyTryFrom<RyBytes> for PyPingPayload {
    fn py_try_from(value: RyBytes) -> PyResult<Self> {
        if value.as_slice().len() > WS_MSG_PINGPONG_PAYLOAD_MAX_LEN {
            py_value_err!("ping-payload exceeds the websocket limit of 125 bytes")
        } else {
            Ok(Self(Message::ping(value.into_inner())))
        }
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
        obj.extract::<RyBytes>().map(Self::py_try_from)?
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

impl PyTryFrom<RyBytes> for PyPongPayload {
    fn py_try_from(value: RyBytes) -> PyResult<Self> {
        if value.as_slice().len() > WS_MSG_PINGPONG_PAYLOAD_MAX_LEN {
            py_value_err!("pong-payload exceeds the websocket limit of 125 bytes")
        } else {
            Ok(Self(Message::pong(value.into_inner())))
        }
    }
}

impl<'py> FromPyObject<'_, 'py> for PyPongPayload {
    type Error = PyErr;

    fn extract(obj: Borrowed<'_, 'py, PyAny>) -> PyResult<Self> {
        obj.extract::<RyBytes>().map(Self::py_try_from)?
    }
}
