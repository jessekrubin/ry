use crate::py_message::{PyMessageLike, PyWebSocketMessage};
use crate::util::{map_ws_err, parse_uri, validate_close_reason, validate_control_payload_len};
use bytes::Bytes;
use futures_util::{
    SinkExt, StreamExt,
    stream::{SplitSink, SplitStream},
};
use pyo3::exceptions::{PyEOFError, PyStopAsyncIteration, PyValueError};
use pyo3::{prelude::*, pybacked::PyBackedStr};
use ryo3_bytes::PyBytes as RyBytes;
use ryo3_http::{PyHeaders, PyHeadersLike, PyHttpStatus};
use ryo3_url::UrlLike;
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use tokio_websockets::{ClientBuilder, Config, Limits, MaybeTlsStream, Message, WebSocketStream};
type TokioWs = WebSocketStream<MaybeTlsStream<TcpStream>>;
type TokioWsWrite = SplitSink<TokioWs, Message>;
type TokioWsRead = SplitStream<TokioWs>;

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

    #[new]
    fn py_new() -> PyResult<Self> {
        Err(PyValueError::new_err(
            "WebSocket cannot be instantiated directly, use the websocket() function instead",
        ))
    }

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

    fn send<'py>(&self, py: Python<'py>, message: PyMessageLike) -> PyResult<Bound<'py, PyAny>> {
        let inner = self.inner.clone();
        let message = Message::from(message);
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let mut writer = inner.writer.lock().await;
            writer.send(message).await.map_err(map_ws_err)?;
            Ok(())
        })
    }

    fn send_text<'py>(&self, py: Python<'py>, text: PyBackedStr) -> PyResult<Bound<'py, PyAny>> {
        self.send(py, PyMessageLike::Text(text))
    }

    fn send_bytes<'py>(&self, py: Python<'py>, data: RyBytes) -> PyResult<Bound<'py, PyAny>> {
        self.send(py, PyMessageLike::Bytes(data))
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
    fn ping<'py>(&self, py: Python<'py>, payload: PyPingPongPayload) -> PyResult<Bound<'py, PyAny>> {
        let inner = self.inner.clone();
        let payload = payload.into_ping_message();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let mut writer = inner.writer.lock().await;
            writer
                .send(payload)
                .await
                .map_err(map_ws_err)?;
            Ok(())
        })
    }

    #[pyo3(signature = (payload = RyBytes::new(Bytes::new())))]
    fn pong<'py>(&self, py: Python<'py>, payload: PyPingPongPayload) -> PyResult<Bound<'py, PyAny>> {
        let inner = self.inner.clone();
        let payload = payload.into_pong_message();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let mut writer = inner.writer.lock().await;
            writer
                .send(payload)
                .await
                .map_err(map_ws_err)?;
            Ok(())
        })
    }
}

#[pyfunction]
#[pyo3(signature = (
    uri,
    *,
    headers = None,
    max_payload_len = None,
    frame_size = None,
    flush_threshold = None
))]
pub(crate) fn websocket<'_>(
    py: Python<'_>,
    uri: UrlLike,
    headers: Option<PyHeadersLike>,
    max_payload_len: Option<usize>,
    frame_size: Option<usize>,
    flush_threshold: Option<usize>,
) -> PyResult<Bound<'_, PyAny>> {
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
