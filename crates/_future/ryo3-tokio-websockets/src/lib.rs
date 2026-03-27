#![doc = include_str!("../README.md")]
mod py_message;
use bytes::Bytes;
use futures_util::{
    SinkExt, StreamExt,
    stream::{SplitSink, SplitStream},
};
mod from;
mod util;
use crate::py_message::PyWebSocketMessage;
use crate::util::{map_ws_err, parse_uri, validate_close_reason, validate_control_payload_len};
use http::Uri;
use pyo3::exceptions::{PyEOFError, PyStopAsyncIteration, PyValueError};
use pyo3::{IntoPyObjectExt, prelude::*, pybacked::PyBackedStr, types::PyModule};
use ryo3_bytes::PyBytes as RyBytes;
use ryo3_http::{PyHeaders, PyHeadersLike, PyHttpStatus};
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
type TokioWs = WebSocketStream<MaybeTlsStream<TcpStream>>;
type TokioWsWrite = SplitSink<TokioWs, Message>;
type TokioWsRead = SplitStream<TokioWs>;

enum PySendMessage {
    Message(PyWebSocketMessage),
    Text(PyBackedStr),
    Bytes(RyBytes),
}

impl<'py> FromPyObject<'_, 'py> for PySendMessage {
    type Error = PyErr;

    fn extract(obj: Borrowed<'_, 'py, PyAny>) -> PyResult<Self> {
        if let Ok(msg) = obj.cast::<PyWebSocketMessage>() {
            Ok(Self::Message(msg.get().clone()))
        } else if let Ok(text) = obj.extract::<PyBackedStr>() {
            Ok(Self::Text(text))
        } else if let Ok(bytes) = obj.extract::<RyBytes>() {
            Ok(Self::Bytes(bytes))
        } else {
            Err(pyo3::exceptions::PyTypeError::new_err(
                "expected Message, str, or bytes-like object",
            ))
        }
    }
}

impl From<PySendMessage> for Message {
    fn from(value: PySendMessage) -> Self {
        match value {
            PySendMessage::Message(msg) => msg.inner,
            PySendMessage::Text(text) => Message::text(text.to_string()),
            PySendMessage::Bytes(bytes) => Message::binary(bytes.into_inner()),
        }
    }
}

// #[pymethods]
// impl PyWebSocketMessage {
//     #[staticmethod]
//     fn text(text: PyBackedStr) -> Self {
//         Self::from(Message::text(text.to_string()))
//     }

//     #[staticmethod]
//     fn binary(data: RyBytes) -> Self {
//         Self::from(Message::binary(data.into_inner()))
//     }

//     #[staticmethod]
//     #[pyo3(signature = (payload = RyBytes::new(Bytes::new())))]
//     fn ping(payload: RyBytes) -> PyResult<Self> {
//         validate_control_payload_len(&payload, "ping")?;
//         Ok(Self::from(Message::ping(payload.into_inner())))
//     }

//     #[staticmethod]
//     #[pyo3(signature = (payload = RyBytes::new(Bytes::new())))]
//     fn pong(payload: RyBytes) -> PyResult<Self> {
//         validate_control_payload_len(&payload, "pong")?;
//         Ok(Self::from(Message::pong(payload.into_inner())))
//     }

//     #[staticmethod]
//     #[pyo3(signature = (*, code = None, reason = ""))]
//     fn close(code: Option<u16>, reason: &str) -> PyResult<Self> {
//         let code = validate_close_reason(code, reason)?;
//         Ok(Self::from(Message::close(code, reason)))
//     }

//     fn __repr__(&self) -> String {
//         if let Some(text) = self.inner.as_text() {
//             format!("Message(type='text', data={text:?})")
//         } else if let Some((code, reason)) = self.inner.as_close() {
//             format!(
//                 "Message(type='close', code={}, reason={reason:?})",
//                 u16::from(code)
//             )
//         } else {
//             format!(
//                 "Message(type='{}', data={:?})",
//                 self.type_name(),
//                 self.payload_bytes()
//             )
//         }
//     }

//     #[getter]
//     fn kind(&self) -> &'static str {
//         self.type_name()
//     }

//     #[getter]
//     fn message_type(&self) -> &'static str {
//         self.type_name()
//     }

//     #[getter]
//     fn is_text(&self) -> bool {
//         self.inner.is_text()
//     }

//     #[getter]
//     fn is_binary(&self) -> bool {
//         self.inner.is_binary()
//     }

//     #[getter]
//     fn is_close(&self) -> bool {
//         self.inner.is_close()
//     }

//     #[getter]
//     fn is_ping(&self) -> bool {
//         self.inner.is_ping()
//     }

//     #[getter]
//     fn is_pong(&self) -> bool {
//         self.inner.is_pong()
//     }

//     #[getter]
//     fn text_data(&self) -> Option<String> {
//         self.inner.as_text().map(ToOwned::to_owned)
//     }

//     #[getter]
//     fn data<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
//         if let Some(text) = self.inner.as_text() {
//             text.into_bound_py_any(py)
//         } else {
//             RyBytes::from(self.payload_bytes()).into_bound_py_any(py)
//         }
//     }

//     #[getter]
//     fn payload(&self) -> RyBytes {
//         RyBytes::from(self.payload_bytes())
//     }

//     #[getter]
//     fn close_code(&self) -> Option<u16> {
//         self.inner.as_close().map(|(code, _)| u16::from(code))
//     }

//     #[getter]
//     fn close_reason(&self) -> Option<String> {
//         self.inner.as_close().map(|(_, reason)| reason.to_owned())
//     }
// }

#[derive(Debug, Clone)]
struct WebSocketInner {
    uri: String,
    status: http::StatusCode,
    headers: http::HeaderMap,
    closed: Arc<AtomicBool>,
    writer: Arc<Mutex<TokioWsWrite>>,
    reader: Arc<Mutex<TokioWsRead>>,
}

impl WebSocketInner {
    async fn recv_next(&self) -> PyResult<Option<PyWebSocketMessage>> {
        let mut reader = self.reader.lock().await;
        match reader.next().await {
            Some(Ok(msg)) => Ok(Some(PyWebSocketMessage::from(msg))),
            Some(Err(err)) => {
                self.closed.store(true, Ordering::SeqCst);
                Err(map_ws_err(err))
            }
            None => {
                self.closed.store(true, Ordering::SeqCst);
                Ok(None)
            }
        }
    }
}

#[derive(Debug, Clone)]
#[pyclass(name = "WebSocket", frozen, immutable_type, skip_from_py_object)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
pub struct PyWebSocket {
    inner: WebSocketInner,
}

impl PyWebSocket {
    fn from_parts(
        uri: String,
        status: http::StatusCode,
        headers: http::HeaderMap,
        ws: TokioWs,
    ) -> Self {
        let (writer, reader) = ws.split();
        Self {
            inner: WebSocketInner {
                uri,
                status,
                headers,
                closed: Arc::new(AtomicBool::new(false)),
                writer: Arc::new(Mutex::new(writer)),
                reader: Arc::new(Mutex::new(reader)),
            },
        }
    }
}

#[pymethods]
impl PyWebSocket {
    fn __repr__(&self) -> String {
        let state = if self.inner.closed.load(Ordering::SeqCst) {
            "closed"
        } else {
            "open"
        };
        format!("WebSocket(uri={:?}, state='{state}')", self.inner.uri)
    }

    fn __bool__(&self) -> bool {
        !self.inner.closed.load(Ordering::SeqCst)
    }

    #[getter]
    fn uri(&self) -> &str {
        &self.inner.uri
    }

    #[getter]
    fn status(&self, py: Python<'_>) -> PyResult<Py<PyHttpStatus>> {
        PyHttpStatus::from_status_code_cached(py, self.inner.status)
    }

    #[getter]
    fn headers(&self) -> PyHeaders {
        PyHeaders::from(self.inner.headers.clone())
    }

    #[getter]
    fn closed(&self) -> bool {
        self.inner.closed.load(Ordering::SeqCst)
    }

    #[getter]
    fn open(&self) -> bool {
        !self.closed()
    }

    fn __aiter__(this: PyRef<Self>) -> PyRef<Self> {
        this
    }

    fn __anext__<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let inner = self.inner.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            match inner.recv_next().await? {
                Some(msg) => Ok(msg),
                None => Err(PyStopAsyncIteration::new_err("websocket closed")),
            }
        })
    }

    fn __aenter__(slf: Py<Self>, py: Python<'_>) -> PyResult<Bound<'_, PyAny>> {
        pyo3_async_runtimes::tokio::future_into_py(py, async move { Ok(slf) })
    }

    #[pyo3(name = "__aexit__")]
    fn __aexit__<'py>(
        &self,
        py: Python<'py>,
        _exc_type: Py<PyAny>,
        _exc_value: Py<PyAny>,
        _traceback: Py<PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let inner = self.inner.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            if inner.closed.load(Ordering::SeqCst) {
                return Ok(());
            }
            let mut writer = inner.writer.lock().await;
            writer
                .send(Message::close(None, ""))
                .await
                .map_err(map_ws_err)?;
            inner.closed.store(true, Ordering::SeqCst);
            Ok(())
        })
    }

    fn recv<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let inner = self.inner.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            inner
                .recv_next()
                .await?
                .ok_or_else(|| PyEOFError::new_err("websocket closed"))
        })
    }

    fn receive<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        self.recv(py)
    }

    fn send<'py>(&self, py: Python<'py>, message: PySendMessage) -> PyResult<Bound<'py, PyAny>> {
        let inner = self.inner.clone();
        let message = Message::from(message);
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let mut writer = inner.writer.lock().await;
            writer.send(message).await.map_err(map_ws_err)?;
            Ok(())
        })
    }

    fn send_text<'py>(&self, py: Python<'py>, text: PyBackedStr) -> PyResult<Bound<'py, PyAny>> {
        self.send(py, PySendMessage::Text(text))
    }

    fn send_bytes<'py>(&self, py: Python<'py>, data: RyBytes) -> PyResult<Bound<'py, PyAny>> {
        self.send(py, PySendMessage::Bytes(data))
    }

    #[pyo3(signature = (*, code = None, reason = ""))]
    fn close<'py>(
        &self,
        py: Python<'py>,
        code: Option<u16>,
        reason: &str,
    ) -> PyResult<Bound<'py, PyAny>> {
        let inner = self.inner.clone();
        let code = validate_close_reason(code, reason)?;
        let reason = reason.to_owned();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let mut writer = inner.writer.lock().await;
            writer
                .send(Message::close(code, &reason))
                .await
                .map_err(map_ws_err)?;
            inner.closed.store(true, Ordering::SeqCst);
            Ok(())
        })
    }

    #[pyo3(signature = (payload = RyBytes::new(Bytes::new())))]
    fn ping<'py>(&self, py: Python<'py>, payload: RyBytes) -> PyResult<Bound<'py, PyAny>> {
        validate_control_payload_len(&payload, "ping")?;
        let inner = self.inner.clone();
        let payload = payload.into_inner();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let mut writer = inner.writer.lock().await;
            writer
                .send(Message::ping(payload))
                .await
                .map_err(map_ws_err)?;
            Ok(())
        })
    }

    #[pyo3(signature = (payload = RyBytes::new(Bytes::new())))]
    fn pong<'py>(&self, py: Python<'py>, payload: RyBytes) -> PyResult<Bound<'py, PyAny>> {
        validate_control_payload_len(&payload, "pong")?;
        let inner = self.inner.clone();
        let payload = payload.into_inner();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let mut writer = inner.writer.lock().await;
            writer
                .send(Message::pong(payload))
                .await
                .map_err(map_ws_err)?;
            Ok(())
        })
    }
}

#[pyfunction]
#[pyo3(signature = (uri, *, headers = None, max_payload_len = None, frame_size = None, flush_threshold = None))]
fn connect<'py>(
    py: Python<'py>,
    uri: UrlLike,
    headers: Option<PyHeadersLike>,
    max_payload_len: Option<usize>,
    frame_size: Option<usize>,
    flush_threshold: Option<usize>,
) -> PyResult<Bound<'py, PyAny>> {
    let uri = parse_uri(uri)?;
    let uri_string = uri.to_string();
    let mut builder = ClientBuilder::from_uri(uri.clone());

    if let Some(headers) = headers {
        for (name, value) in &http::HeaderMap::from(headers) {
            builder = builder
                .add_header(name.clone(), value.clone())
                .map_err(map_ws_err)?;
        }
    }

    let mut config = Config::default();
    if let Some(frame_size) = frame_size {
        if frame_size == 0 {
            return Err(PyValueError::new_err("frame_size must be non-zero"));
        }
        config = config.frame_size(frame_size);
    }
    if let Some(flush_threshold) = flush_threshold {
        config = config.flush_threshold(flush_threshold);
    }
    builder = builder.config(config);

    let mut limits = Limits::default();
    if max_payload_len.is_some() {
        limits.set_max_payload_len(max_payload_len);
    }
    builder = builder.limits(limits);

    pyo3_async_runtimes::tokio::future_into_py(py, async move {
        let (ws, response) = builder.connect().await.map_err(map_ws_err)?;
        Ok(PyWebSocket::from_parts(
            uri_string,
            response.status(),
            response.headers().clone(),
            ws,
        ))
    })
}

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyWebSocketMessage>()?;
    m.add_class::<PyWebSocket>()?;
    m.add_function(wrap_pyfunction!(connect, m)?)?;
    Ok(())
}
