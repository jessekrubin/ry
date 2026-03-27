use crate::constants::{DEFAULT_FLUSH_THRESHOLD, DEFAULT_FRAME_SIZE, DEFAULT_MAX_PAYLOAD_LEN};
use crate::py_message::{PyMessageLike, PyPingPayload, PyPongPayload, PyWebSocketMessage};
use crate::util::{map_ws_err, parse_uri, validate_close_reason};
use bytes::Bytes;
use futures_util::{
    SinkExt, StreamExt,
    stream::{SplitSink, SplitStream},
};
use pyo3::exceptions::{PyEOFError, PyRuntimeError, PyStopAsyncIteration};
use pyo3::{prelude::*, pybacked::PyBackedStr};
use ryo3_bytes::PyBytes as RyBytes;
use ryo3_core::py_value_err;
use ryo3_http::{PyHeaders, PyHeadersLike, PyHttpStatus};
use ryo3_url::UrlLike;
use std::sync::{
    Arc, Mutex as StdMutex,
    atomic::{AtomicBool, Ordering},
};
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use tokio_websockets::{ClientBuilder, Config, Limits, MaybeTlsStream, Message, WebSocketStream};

type TokioWs = WebSocketStream<MaybeTlsStream<TcpStream>>;
type TokioWsWrite = SplitSink<TokioWs, Message>;
type TokioWsRead = SplitStream<TokioWs>;

#[derive(Debug, Clone)]
struct WebSocketConfig {
    uri: http::Uri,
    uri_string: String,
    headers: Option<http::HeaderMap>,
    max_payload_len: usize,
    frame_size: usize,
    flush_threshold: usize,
}

impl WebSocketConfig {
    async fn connect(self) -> PyResult<(http::StatusCode, http::HeaderMap, TokioWs)> {
        let mut builder = ClientBuilder::from_uri(self.uri.clone());

        if let Some(headers) = self.headers {
            for (name, value) in &headers {
                builder = builder
                    .add_header(name.clone(), value.clone())
                    .map_err(map_ws_err)?;
            }
        }
        let mut config = Config::default();
        if self.frame_size == 0 {
            return py_value_err!("frame_size must be non-zero");
        }
        config = config.frame_size(self.frame_size);
        config = config.flush_threshold(self.flush_threshold);
        builder = builder.config(config);

        let limits = Limits::default().max_payload_len(Some(self.max_payload_len));
        builder = builder.limits(limits);

        let (ws, response) = builder.connect().await.map_err(map_ws_err)?;
        Ok((response.status(), response.headers().clone(), ws))
    }

    async fn ensure_open(&self) -> PyResult<()> {
        Ok(())
    }
}

#[derive(Debug, Clone)]
struct WebSocketConnected {
    status: http::StatusCode,
    response_headers: http::HeaderMap,
    closed: Arc<AtomicBool>,
    writer: Arc<Mutex<TokioWsWrite>>,
    reader: Arc<Mutex<TokioWsRead>>,
}

impl WebSocketConnected {
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

#[derive(Debug)]
enum WebSocketState {
    Idle(WebSocketConfig),
    Connected(WebSocketConnected),
    Closed,
}

#[derive(Debug, Clone)]
#[pyclass(name = "WebSocket", frozen, immutable_type, skip_from_py_object)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
pub struct PyWebSocket {
    uri_string: String,
    state: Arc<Mutex<WebSocketState>>,
    // Cached for quick sync access after connecting
    cached_status: Arc<StdMutex<Option<http::StatusCode>>>,
    cached_headers: Arc<StdMutex<Option<http::HeaderMap>>>,
    closed: Arc<AtomicBool>,
}

impl PyWebSocket {
    fn new_idle(
        uri: http::Uri,
        uri_string: String,
        headers: Option<http::HeaderMap>,
        max_payload_len: usize,
        frame_size: usize,
        flush_threshold: usize,
    ) -> Self {
        Self {
            uri_string: uri_string.clone(),
            state: Arc::new(Mutex::new(WebSocketState::Idle(WebSocketConfig {
                uri,
                uri_string,
                headers,
                max_payload_len,
                frame_size,
                flush_threshold,
            }))),
            cached_status: Arc::new(StdMutex::new(None)),
            cached_headers: Arc::new(StdMutex::new(None)),
            closed: Arc::new(AtomicBool::new(false)),
        }
    }

    async fn connect(&self) -> PyResult<()> {
        let config = {
            let state_guard = self.state.lock().await;
            match &*state_guard {
                WebSocketState::Idle(cfg) => cfg.clone(),
                WebSocketState::Connected(_) => {
                    return Err(PyRuntimeError::new_err("WebSocket already connected"));
                }
                WebSocketState::Closed => {
                    return Err(PyRuntimeError::new_err("WebSocket is closed"));
                }
            }
        };

        let (status, response_headers, ws) = config.connect().await?;
        let (writer, reader) = ws.split();

        let connected = WebSocketConnected {
            status,
            response_headers: response_headers.clone(),
            closed: Arc::new(AtomicBool::new(false)),
            writer: Arc::new(Mutex::new(writer)),
            reader: Arc::new(Mutex::new(reader)),
        };

        // Cache the response metadata
        *self.cached_status.lock().unwrap() = Some(status);
        *self.cached_headers.lock().unwrap() = Some(response_headers);
        self.closed.store(false, Ordering::SeqCst);

        // Update state
        *self.state.lock().await = WebSocketState::Connected(connected);

        Ok(())
    }

    async fn get_connected(&self) -> PyResult<WebSocketConnected> {
        let state = self.state.lock().await;
        match &*state {
            WebSocketState::Connected(conn) => Ok(conn.clone()),
            WebSocketState::Idle(_) => Err(PyRuntimeError::new_err(
                "WebSocket not connected — use `async with ws:` to connect",
            )),
            WebSocketState::Closed => Err(PyRuntimeError::new_err("WebSocket is closed")),
        }
    }

    async fn recv_next_msg(&self) -> PyResult<PyWebSocketMessage> {
        let conn = self.get_connected().await?;
        conn.recv_next()
            .await?
            .ok_or_else(|| PyEOFError::new_err("websocket closed"))
    }

    async fn iter_next_msg(&self) -> PyResult<PyWebSocketMessage> {
        let conn = self.get_connected().await?;
        match conn.recv_next().await? {
            Some(msg) => Ok(msg),
            None => Err(PyStopAsyncIteration::new_err("websocket closed")),
        }
    }

    async fn send_message(&self, message: Message) -> PyResult<()> {
        let conn = self.get_connected().await?;
        let mut writer = conn.writer.lock().await;
        writer.send(message).await.map_err(map_ws_err)?;
        Ok(())
    }

    async fn disconnect(&self) -> PyResult<()> {
        let conn = match &*self.state.lock().await {
            WebSocketState::Connected(c) => c.clone(),
            WebSocketState::Closed => return Ok(()),
            _ => return Ok(()),
        };

        if conn.closed.load(Ordering::SeqCst) {
            return Ok(());
        }

        let mut writer = conn.writer.lock().await;
        writer
            .send(Message::close(None, ""))
            .await
            .map_err(map_ws_err)?;
        conn.closed.store(true, Ordering::SeqCst);
        self.closed.store(true, Ordering::SeqCst);

        let mut state_guard = self.state.lock().await;
        *state_guard = WebSocketState::Closed;

        Ok(())
    }

    async fn send_close(
        &self,
        code: Option<tokio_websockets::CloseCode>,
        reason: &str,
    ) -> PyResult<()> {
        let conn = match &*self.state.lock().await {
            WebSocketState::Connected(c) => c.clone(),
            _ => return Ok(()),
        };
        let mut writer = conn.writer.lock().await;
        writer
            .send(Message::close(code, reason))
            .await
            .map_err(map_ws_err)?;
        conn.closed.store(true, Ordering::SeqCst);
        self.closed.store(true, Ordering::SeqCst);
        Ok(())
    }

    async fn send_control(&self, message: Message) -> PyResult<()> {
        let conn = self.get_connected().await?;
        let mut writer = conn.writer.lock().await;
        writer.send(message).await.map_err(map_ws_err)?;
        Ok(())
    }
}

#[pymethods]
impl PyWebSocket {
    #[new]
    #[pyo3(signature = (
        uri,
        *,
        headers = None,
        max_payload_len = DEFAULT_MAX_PAYLOAD_LEN,
        frame_size = DEFAULT_FRAME_SIZE,
        flush_threshold = DEFAULT_FLUSH_THRESHOLD
    ))]
    fn py_new(
        uri: UrlLike,
        headers: Option<PyHeadersLike>,
        max_payload_len: usize,
        frame_size: usize,
        flush_threshold: usize,
    ) -> PyResult<Self> {
        let uri = parse_uri(uri)?;
        let uri_string = uri.to_string();
        let headers = headers.map(|h| http::HeaderMap::from(h));
        Ok(Self::new_idle(
            uri,
            uri_string,
            headers,
            max_payload_len,
            frame_size,
            flush_threshold,
        ))
    }

    fn __repr__(&self) -> String {
        format!("WebSocket(uri={:?})", self.uri_string)
    }

    fn __bool__(&self) -> bool {
        !self.closed.load(Ordering::SeqCst)
    }

    #[getter]
    fn uri(&self) -> &str {
        &self.uri_string
    }

    #[getter]
    fn status(&self, py: Python<'_>) -> PyResult<Py<PyHttpStatus>> {
        let status = self.cached_status.lock().unwrap().clone();
        match status {
            Some(s) => PyHttpStatus::from_status_code_cached(py, s),
            None => Err(PyRuntimeError::new_err("WebSocket not connected")),
        }
    }

    #[getter]
    fn headers(&self) -> PyResult<PyHeaders> {
        let headers = self.cached_headers.lock().unwrap().clone();
        match headers {
            Some(h) => Ok(PyHeaders::from(h)),
            None => Err(PyRuntimeError::new_err("WebSocket not connected")),
        }
    }

    #[getter]
    fn closed(&self) -> bool {
        self.closed.load(Ordering::SeqCst)
    }

    #[getter]
    fn open(&self) -> bool {
        !self.closed()
    }

    fn __aiter__(this: PyRef<Self>) -> PyRef<Self> {
        this
    }

    fn __anext__<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let this = self.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move { this.iter_next_msg().await })
    }

    fn __await__(slf: Py<Self>, py: Python<'_>) -> PyResult<Bound<'_, PyAny>> {
        let coro = pyo3_async_runtimes::tokio::future_into_py(py, async move {
            slf.get().connect().await?;
            Ok(slf)
        })?;
        coro.getattr(pyo3::intern!(py, "__await__"))?.call0()
    }

    fn __aenter__(slf: Py<Self>, py: Python<'_>) -> PyResult<Bound<'_, PyAny>> {
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            slf.get().connect().await?;
            Ok(slf)
        })
    }

    #[pyo3(name = "__aexit__")]
    fn __aexit__<'py>(
        &self,
        py: Python<'py>,
        _exc_type: Py<PyAny>,
        _exc_value: Py<PyAny>,
        _traceback: Py<PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let this = self.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move { this.disconnect().await })
    }

    fn recv<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let this = self.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move { this.recv_next_msg().await })
    }

    fn receive<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        self.recv(py)
    }

    fn send<'py>(&self, py: Python<'py>, message: PyMessageLike) -> PyResult<Bound<'py, PyAny>> {
        let this = self.clone();
        let message = Message::from(message);
        pyo3_async_runtimes::tokio::future_into_py(
            py,
            async move { this.send_message(message).await },
        )
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
        let code = validate_close_reason(code, reason)?;
        let reason = reason.to_owned();
        let this = self.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            this.send_close(code, &reason).await
        })
    }

    #[pyo3(signature = (payload = None))]
    fn ping<'py>(
        &self,
        py: Python<'py>,
        payload: Option<PyPingPayload>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let this = self.clone();
        let payload = payload.unwrap_or_default().into();
        pyo3_async_runtimes::tokio::future_into_py(
            py,
            async move { this.send_control(payload).await },
        )
    }

    #[pyo3(signature = (payload = None))]
    fn pong<'py>(
        &self,
        py: Python<'py>,
        payload: Option<PyPongPayload>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let this = self.clone();
        let payload = payload.unwrap_or_default().into();
        pyo3_async_runtimes::tokio::future_into_py(
            py,
            async move { this.send_control(payload).await },
        )
    }
}

#[pyfunction]
#[pyo3(signature = (
    uri,
    *,
    headers = None,
    flush_threshold = DEFAULT_FLUSH_THRESHOLD,
    frame_size = DEFAULT_FRAME_SIZE,
    max_payload_len = DEFAULT_MAX_PAYLOAD_LEN,
))]
pub(crate) fn websocket(
    uri: UrlLike,
    headers: Option<PyHeadersLike>,
    flush_threshold: usize,
    frame_size: usize,
    max_payload_len: usize,
) -> PyResult<PyWebSocket> {
    let uri = parse_uri(uri)?;
    let uri_string = uri.to_string();
    let headers = headers.map(|h| http::HeaderMap::from(h));
    Ok(PyWebSocket::new_idle(
        uri,
        uri_string,
        headers,
        max_payload_len,
        frame_size,
        flush_threshold,
    ))
}
