use bytes::Bytes;
use futures_util::{
    stream::{SplitSink, SplitStream},
};
use http::Uri;
use pyo3::exceptions::{PyEOFError, PyStopAsyncIteration, PyValueError};
use pyo3::{IntoPyObjectExt, prelude::*, pybacked::PyBackedStr, types::PyModule};
use ryo3_bytes::PyBytes as RyBytes;
use ryo3_http::{PyHeaders, PyHeadersLike};
use ryo3_url::UrlLike;
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use tokio_websockets::{
    ClientBuilder, CloseCode, Config, Error, Limits, MaybeTlsStream, Message, WebSocketStream,
};
use crate::util::{map_ws_err, parse_uri, validate_close_reason, validate_control_payload_len};

type TokioWs = WebSocketStream<MaybeTlsStream<TcpStream>>;
type TokioWsWrite = SplitSink<TokioWs, Message>;
type TokioWsRead = SplitStream<TokioWs>;

#[derive(Debug, Clone)]
#[pyclass(name = "Message", frozen, immutable_type, skip_from_py_object)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
pub struct PyWebSocketMessage {
    pub(crate) inner: Message,
}

impl PyWebSocketMessage {
    fn payload_bytes(&self) -> Bytes {
        self.inner.clone().into_payload().into()
    }

    fn type_name(&self) -> &'static str {
        if self.inner.is_text() {
            "text"
        } else if self.inner.is_binary() {
            "binary"
        } else if self.inner.is_close() {
            "close"
        } else if self.inner.is_ping() {
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
    #[pyo3(signature = (payload = RyBytes::new(Bytes::new())))]
    fn ping(payload: RyBytes) -> PyResult<Self> {
        validate_control_payload_len(&payload, "ping")?;
        Ok(Self::from(Message::ping(payload.into_inner())))
    }

    #[staticmethod]
    #[pyo3(signature = (payload = RyBytes::new(Bytes::new())))]
    fn pong(payload: RyBytes) -> PyResult<Self> {
        validate_control_payload_len(&payload, "pong")?;
        Ok(Self::from(Message::pong(payload.into_inner())))
    }

    #[staticmethod]
    #[pyo3(signature = (*, code = None, reason = ""))]
    fn close(code: Option<u16>, reason: &str) -> PyResult<Self> {
        let code = validate_close_reason(code, reason)?;
        Ok(Self::from(Message::close(code, reason)))
    }

    fn __repr__(&self) -> String {
        if let Some(text) = self.inner.as_text() {
            format!("Message(type='text', data={text:?})")
        } else if let Some((code, reason)) = self.inner.as_close() {
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
        self.inner.is_text()
    }

    #[getter]
    fn is_binary(&self) -> bool {
        self.inner.is_binary()
    }

    #[getter]
    fn is_close(&self) -> bool {
        self.inner.is_close()
    }

    #[getter]
    fn is_ping(&self) -> bool {
        self.inner.is_ping()
    }

    #[getter]
    fn is_pong(&self) -> bool {
        self.inner.is_pong()
    }

    #[getter]
    fn text_data(&self) -> Option<String> {
        self.inner.as_text().map(ToOwned::to_owned)
    }

    #[getter]
    fn data<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        if let Some(text) = self.inner.as_text() {
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
        self.inner.as_close().map(|(code, _)| u16::from(code))
    }

    #[getter]
    fn close_reason(&self) -> Option<String> {
        self.inner.as_close().map(|(_, reason)| reason.to_owned())
    }
}
