use crate::constants::{DEFAULT_FLUSH_THRESHOLD, DEFAULT_FRAME_SIZE, DEFAULT_MAX_PAYLOAD_LEN};
use crate::errors::map_ws_err;
use crate::py_message::{
    PyMessageLike, PyPingPayload, PyPongPayload, PyWsCloseCode, PyWsCloseReason, PyWsMessage,
};
use crate::util::parse_uri;
use futures_util::{
    SinkExt, StreamExt,
    stream::{SplitSink, SplitStream},
};
use pyo3::exceptions::{PyEOFError, PyRuntimeError, PyStopAsyncIteration};
use pyo3::prelude::*;
use ryo3_core::py_value_err;
use ryo3_http::{PyHeaders, PyHeadersLike, PyHttpStatus};
use ryo3_url::UrlLike;
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use tokio_websockets::{ClientBuilder, Config, Limits, MaybeTlsStream, Message, WebSocketStream};

type TokioWs = WebSocketStream<MaybeTlsStream<TcpStream>>;
type TokioWsWrite = SplitSink<TokioWs, Message>;
type TokioWsRead = SplitStream<TokioWs>;

#[derive(Debug, Clone)]
struct WebSocketHandshake {
    status: http::StatusCode,
    response_headers: http::HeaderMap,
}

#[derive(Debug, Clone)]
struct WebSocketConnected {
    handshake: WebSocketHandshake,
    writer: Arc<Mutex<TokioWsWrite>>,
    reader: Arc<Mutex<TokioWsRead>>,
}

impl WebSocketConnected {
    #[inline]
    fn into_closed(self) -> WebSocketState {
        WebSocketState::Closed(self.handshake)
    }

    #[inline]
    async fn recv_next(&self) -> PyResult<Option<Message>> {
        let mut reader = self.reader.lock().await;
        match reader.next().await {
            Some(Ok(msg)) => Ok(Some(msg)),
            Some(Err(err)) => Err(map_ws_err(err)),
            None => Ok(None),
        }
    }
}

#[derive(Debug)]
enum WebSocketState {
    Idle,
    Connecting,
    Open(WebSocketConnected),
    Closed(WebSocketHandshake),
}

impl WebSocketState {
    #[inline]
    fn handshake(&self) -> Option<&WebSocketHandshake> {
        match self {
            Self::Open(conn) => Some(&conn.handshake),
            Self::Closed(handshake) => Some(handshake),
            Self::Idle | Self::Connecting => None,
        }
    }

    #[inline]
    fn is_open(&self) -> bool {
        matches!(self, Self::Open(_))
    }
}

#[derive(Debug, Clone)]
struct WebSocketConfig {
    uri: http::Uri,
    headers: Option<http::HeaderMap>,
    max_payload_len: usize,
    frame_size: usize,
    flush_threshold: usize,
}

#[derive(Debug)]
struct WebSocketInner {
    cfg: WebSocketConfig,
    state: Mutex<WebSocketState>,
}

#[derive(Debug, Clone)]
#[pyclass(name = "WebSocket", frozen, immutable_type, skip_from_py_object)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
pub struct PyWebSocket {
    uri_string: String,
    inner: Arc<WebSocketInner>,
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
        let cfg = WebSocketConfig {
            uri,
            headers,
            max_payload_len,
            frame_size,
            flush_threshold,
        };
        let inner = WebSocketInner {
            cfg,
            state: Mutex::new(WebSocketState::Idle),
        };
        Self {
            uri_string,
            inner: Arc::new(inner),
        }
    }

    #[inline]
    async fn connect(&self) -> PyResult<()> {
        let previous_state = {
            let mut state = self.inner.state.lock().await;
            match &*state {
                WebSocketState::Idle | WebSocketState::Closed(_) => {
                    std::mem::replace(&mut *state, WebSocketState::Connecting)
                }
                WebSocketState::Connecting => {
                    return Err(PyRuntimeError::new_err("WebSocket connection already in progress"));
                }
                WebSocketState::Open(_) => {
                    return Err(PyRuntimeError::new_err("WebSocket already connected"));
                }
            }
        };

        let cfg = self.inner.cfg.clone();
        let connect_result = async move {
            let mut builder = ClientBuilder::from_uri(cfg.uri.clone());

            if let Some(headers) = &cfg.headers {
                for (name, value) in headers {
                    builder = builder
                        .add_header(name.clone(), value.clone())
                        .map_err(map_ws_err)?;
                }
            }

            if cfg.frame_size == 0 {
                return py_value_err!("frame_size must be non-zero");
            }

            let config = Config::default()
                .frame_size(cfg.frame_size)
                .flush_threshold(cfg.flush_threshold);
            builder = builder.config(config);

            let limits = Limits::default().max_payload_len(Some(cfg.max_payload_len));
            builder = builder.limits(limits);

            let (ws, response) = builder.connect().await.map_err(map_ws_err)?;
            let (writer, reader) = ws.split();

            let handshake = WebSocketHandshake {
                status: response.status(),
                response_headers: response.headers().clone(),
            };
            Ok::<_, PyErr>(WebSocketConnected {
                handshake,
                writer: Arc::new(Mutex::new(writer)),
                reader: Arc::new(Mutex::new(reader)),
            })
        }
        .await;

        let mut state = self.inner.state.lock().await;
        match connect_result {
            Ok(connected) => {
                *state = WebSocketState::Open(connected);
                Ok(())
            }
            Err(err) => {
                *state = previous_state;
                Err(err)
            }
        }
    }

    #[inline]
    async fn get_connected(&self) -> PyResult<WebSocketConnected> {
        let state = self.inner.state.lock().await;
        match &*state {
            WebSocketState::Open(conn) => Ok(conn.clone()),
            WebSocketState::Connecting => {
                Err(PyRuntimeError::new_err("WebSocket connection is still in progress"))
            }
            WebSocketState::Idle | WebSocketState::Closed(_) => Err(PyRuntimeError::new_err(
                "WebSocket not connected; use `await ws` or `async with ws:` to connect",
            )),
        }
    }

    #[inline]
    async fn mark_closed_if_current(&self, current: &WebSocketConnected) {
        let mut state = self.inner.state.lock().await;
        if let WebSocketState::Open(open_conn) = &*state
            && Arc::ptr_eq(&open_conn.writer, &current.writer)
            && Arc::ptr_eq(&open_conn.reader, &current.reader)
        {
            *state = current.clone().into_closed();
        }
    }

    #[inline]
    async fn recv_next_msg(&self) -> PyResult<PyWsMessage> {
        let conn = self.get_connected().await?;
        match conn.recv_next().await? {
            Some(msg) => {
                if msg.is_close() {
                    self.mark_closed_if_current(&conn).await;
                }
                Ok(PyWsMessage::from(msg))
            }
            None => {
                self.mark_closed_if_current(&conn).await;
                Err(PyEOFError::new_err("websocket closed"))
            }
        }
    }

    #[inline]
    async fn iter_next_msg(&self) -> PyResult<PyWsMessage> {
        let conn = self.get_connected().await?;
        match conn.recv_next().await? {
            Some(msg) => {
                if msg.is_close() {
                    self.mark_closed_if_current(&conn).await;
                }
                Ok(PyWsMessage::from(msg))
            }
            None => {
                self.mark_closed_if_current(&conn).await;
                Err(PyStopAsyncIteration::new_err("websocket closed"))
            }
        }
    }

    #[inline]
    async fn send(&self, message: Message) -> PyResult<()> {
        let conn = self.get_connected().await?;
        let mut writer = conn.writer.lock().await;
        writer.send(message).await.map_err(map_ws_err)?;
        Ok(())
    }

    #[inline]
    async fn close_with(&self, message: Message) -> PyResult<()> {
        let conn = {
            let state = self.inner.state.lock().await;
            match &*state {
                WebSocketState::Open(conn) => Some(conn.clone()),
                WebSocketState::Idle | WebSocketState::Closed(_) => None,
                WebSocketState::Connecting => {
                    return Err(PyRuntimeError::new_err(
                        "WebSocket connection is still in progress",
                    ));
                }
            }
        };

        let Some(conn) = conn else {
            return Ok(());
        };

        let mut writer = conn.writer.lock().await;
        match writer.send(message).await {
            Ok(()) | Err(tokio_websockets::Error::AlreadyClosed) => {}
            Err(err) => return Err(map_ws_err(err)),
        }
        drop(writer);

        self.mark_closed_if_current(&conn).await;
        Ok(())
    }

    #[inline]
    async fn disconnect(&self) -> PyResult<()> {
        self.close_with(Message::close(None, "")).await
    }

    #[inline]
    async fn send_control(&self, message: Message) -> PyResult<()> {
        let conn = self.get_connected().await?;
        let mut writer = conn.writer.lock().await;
        writer.send(message).await.map_err(map_ws_err)?;
        Ok(())
    }
}

#[pymethods]
impl PyWebSocket {
    #[expect(clippy::needless_pass_by_value)]
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
        let uri = parse_uri(&uri)?;
        let uri_string = uri.to_string();
        let headers = headers.map(http::HeaderMap::from);
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
        format!(
            "WebSocket(uri={:?}, open={})",
            self.uri_string,
            self.inner.state.blocking_lock().is_open()
        )
    }

    fn __bool__(&self) -> bool {
        self.inner.state.blocking_lock().is_open()
    }

    #[getter]
    fn uri(&self) -> &str {
        &self.uri_string
    }

    #[getter]
    fn status(&self, py: Python<'_>) -> PyResult<Py<PyHttpStatus>> {
        let state = self.inner.state.blocking_lock();
        match state.handshake() {
            Some(handshake) => PyHttpStatus::from_status_code_cached(py, handshake.status),
            None => Err(PyRuntimeError::new_err("WebSocket has not connected yet")),
        }
    }

    #[getter]
    fn headers(&self) -> PyResult<PyHeaders> {
        let state = self.inner.state.blocking_lock();
        match state.handshake() {
            Some(handshake) => Ok(PyHeaders::from(handshake.response_headers.clone())),
            None => Err(PyRuntimeError::new_err("WebSocket has not connected yet")),
        }
    }

    #[getter]
    fn closed(&self) -> bool {
        !self.open()
    }

    #[getter]
    fn open(&self) -> bool {
        self.inner.state.blocking_lock().is_open()
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

    #[pyo3(name = "send")]
    fn py_send<'py>(&self, py: Python<'py>, message: PyMessageLike) -> PyResult<Bound<'py, PyAny>> {
        let this = self.clone();
        let message = Message::from(message);
        pyo3_async_runtimes::tokio::future_into_py(py, async move { this.send(message).await })
    }

    #[pyo3(signature = (*, code = None, reason = None))]
    fn close<'py>(
        &self,
        py: Python<'py>,
        code: Option<PyWsCloseCode>,
        reason: Option<PyWsCloseReason>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let this = self.clone();
        let pymsg = PyWsMessage::close(code, reason)?;
        pyo3_async_runtimes::tokio::future_into_py(
            py,
            async move { this.close_with(pymsg.into()).await },
        )
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

#[expect(clippy::needless_pass_by_value)]
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
    let uri = parse_uri(&uri)?;
    let uri_string = uri.to_string();
    let headers = headers.map(http::HeaderMap::from);
    Ok(PyWebSocket::new_idle(
        uri,
        uri_string,
        headers,
        max_payload_len,
        frame_size,
        flush_threshold,
    ))
}
