use std::{num::NonZeroUsize, sync::Arc, time::Duration};

use futures_util::{SinkExt, StreamExt};
use pyo3::{exceptions::PyStopAsyncIteration, prelude::*};
use ryo3_http::{PyHeaders, PyHeadersLike, PyHttpStatus};
use ryo3_macro_rules::py_runtime_err;
use ryo3_std::time::PyTimeout;
use ryo3_tokio_rt::future_into_py;
use tokio::sync::Mutex;
use tokio_websockets::{ClientBuilder, Config, Limits, Message};

use crate::{
    PyMessageLike, PyPingPayload, PyPongPayload, PyWsMessage,
    constants::{
        DEFAULT_CLOSE_TIMEOUT, DEFAULT_FLUSH_THRESHOLD, DEFAULT_FRAME_SIZE, DEFAULT_MAX_PAYLOAD_LEN,
    },
    errors::map_ws_err,
    types::{PyWsCloseCode, PyWsCloseReason, TokioWsRead, TokioWsWrite, UriLike},
};

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
        WebSocketState::Closed(Some(self.handshake))
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum WebSocketReadyState {
    Connecting = 0,
    Open = 1,
    Closing = 2,
    Closed = 3,
}

#[derive(Debug)]
enum WebSocketState {
    Connecting,
    Open(WebSocketConnected),
    Closing(WebSocketConnected),
    Closed(Option<WebSocketHandshake>),
}

impl WebSocketState {
    #[inline]
    fn handshake(&self) -> Option<&WebSocketHandshake> {
        match self {
            Self::Open(conn) | Self::Closing(conn) => Some(&conn.handshake),
            Self::Closed(handshake) => handshake.as_ref(),
            Self::Connecting => None,
        }
    }

    #[inline]
    fn is_open(&self) -> bool {
        matches!(self, Self::Open(_))
    }

    #[inline]
    fn ready_state(&self) -> WebSocketReadyState {
        match self {
            Self::Connecting => WebSocketReadyState::Connecting,
            Self::Open(_) => WebSocketReadyState::Open,
            Self::Closing(_) => WebSocketReadyState::Closing,
            Self::Closed(_) => WebSocketReadyState::Closed,
        }
    }
}

#[derive(Debug, Clone)]
struct WebSocketConfig {
    uri: http::Uri,
    headers: Option<http::HeaderMap>,
    max_payload_len: NonZeroUsize,
    frame_size: NonZeroUsize,
    flush_threshold: NonZeroUsize,
    close_timeout: Option<Duration>,
    recv_timeout: Option<Duration>,
}

#[derive(Debug)]
struct WebSocketInner {
    cfg: WebSocketConfig,
    state: Mutex<WebSocketState>,
}

#[derive(Debug, Clone)]
#[pyclass(name = "WebSocket", frozen, immutable_type, skip_from_py_object)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
pub struct PyWebSocket(Arc<WebSocketInner>);

impl From<WebSocketConfig> for PyWebSocket {
    fn from(cfg: WebSocketConfig) -> Self {
        let inner = WebSocketInner {
            cfg,
            state: Mutex::new(WebSocketState::Closed(None)),
        };
        Self(Arc::new(inner))
    }
}

impl PyWebSocket {
    #[inline]
    async fn connect(&self) -> PyResult<()> {
        let previous_state = {
            let mut state = self.0.state.lock().await;
            match &*state {
                WebSocketState::Closed(_) => {
                    std::mem::replace(&mut *state, WebSocketState::Connecting)
                }
                WebSocketState::Connecting => {
                    return py_runtime_err!("WebSocket connection already in progress");
                }
                WebSocketState::Open(_) => {
                    return py_runtime_err!("WebSocket already connected");
                }
                WebSocketState::Closing(_) => {
                    return py_runtime_err!("WebSocket is closing");
                }
            }
        };

        let cfg = self.0.cfg.clone();
        let connect_result = async move {
            let mut builder = ClientBuilder::from_uri(cfg.uri.clone());

            if let Some(headers) = &cfg.headers {
                for (name, value) in headers {
                    builder = builder
                        .add_header(name.clone(), value.clone())
                        .map_err(map_ws_err)?;
                }
            }

            // if cfg.frame_size == 0 {
            //     return py_value_err!("frame_size must be non-zero");
            // }

            let config = Config::default()
                .frame_size(cfg.frame_size.get())
                .flush_threshold(cfg.flush_threshold.get());
            builder = builder.config(config);

            let limits = Limits::default().max_payload_len(Some(cfg.max_payload_len.get()));
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

        let mut state = self.0.state.lock().await;
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
        let state = self.0.state.lock().await;
        match &*state {
            WebSocketState::Open(conn) => Ok(conn.clone()),
            WebSocketState::Closing(_) => py_runtime_err!("WebSocket is closing"),
            WebSocketState::Closed(_) => {
                py_runtime_err!(
                    "WebSocket not connected; use `await ws` or `async with ws:` to connect"
                )
            }
            WebSocketState::Connecting => {
                py_runtime_err!("WebSocket connection is still in progress")
            }
        }
    }

    #[inline]
    async fn close_current_connection(&self, current: &WebSocketConnected) {
        let mut state = self.0.state.lock().await;
        match &*state {
            WebSocketState::Open(open_conn) | WebSocketState::Closing(open_conn)
                if Arc::ptr_eq(&open_conn.writer, &current.writer)
                    && Arc::ptr_eq(&open_conn.reader, &current.reader) =>
            {
                *state = current.clone().into_closed();
            }
            _ => {}
        }
    }

    #[inline]
    async fn finalize_close_conn(&self, conn: &WebSocketConnected) -> PyResult<()> {
        let close_future = async {
            let mut reader = conn.reader.lock().await;
            loop {
                match reader.next().await {
                    Some(Ok(msg)) if msg.is_close() => break,
                    Some(Ok(_)) => {}
                    Some(Err(err)) => return Err(map_ws_err(err)),
                    None => break,
                }
            }
            Ok(())
        };

        let result = match self.0.cfg.close_timeout {
            Some(close_timeout) => {
                if let Ok(result) = tokio::time::timeout(close_timeout, close_future).await {
                    result
                } else {
                    self.close_current_connection(conn).await;
                    return py_runtime_err!(
                        "websocket close timed out after {} seconds",
                        close_timeout.as_secs_f64()
                    );
                }
            }
            None => close_future.await,
        };

        self.close_current_connection(conn).await;
        result
    }

    #[inline]
    async fn recv_msg(&self, timeout: Option<Duration>) -> PyResult<Option<PyWsMessage>> {
        let conn = async {
            let state = self.0.state.lock().await;
            match &*state {
                WebSocketState::Open(conn) | WebSocketState::Closing(conn) => Ok(conn.clone()),
                WebSocketState::Connecting => {
                    py_runtime_err!("WebSocket connection is still in progress")
                }
                WebSocketState::Closed(_) => {
                    py_runtime_err!(
                        "WebSocket not connected; use `await ws` or `async with ws:` to connect"
                    )
                }
            }
        }
        .await?;

        let timeout = timeout.or(self.0.cfg.recv_timeout);
        let msg_result = if let Some(dur) = timeout {
            match tokio::time::timeout(dur, conn.recv_next()).await {
                Ok(result) => result,
                Err(_) => return py_runtime_err!("recv timeout"),
            }
        } else {
            conn.recv_next().await
        };

        if let Some(msg) = msg_result? {
            if msg.is_close() {
                self.close_current_connection(&conn).await;
            }
            Ok(Some(PyWsMessage::from(msg)))
        } else {
            self.close_current_connection(&conn).await;
            Ok(None)
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
        let close_state = {
            let mut state = self.0.state.lock().await;
            match &*state {
                WebSocketState::Open(conn) => {
                    let conn = conn.clone();
                    *state = WebSocketState::Closing(conn.clone());
                    Some((conn, true))
                }
                WebSocketState::Closing(conn) => Some((conn.clone(), false)),
                WebSocketState::Closed(_) => None,
                WebSocketState::Connecting => {
                    return py_runtime_err!("WebSocket connection is still in progress");
                }
            }
        };

        let Some((conn, should_send_close)) = close_state else {
            return Ok(());
        };

        if should_send_close {
            let mut writer = conn.writer.lock().await;
            match writer.send(message).await {
                Ok(()) | Err(tokio_websockets::Error::AlreadyClosed) => {}
                Err(err) => {
                    drop(writer);
                    self.close_current_connection(&conn).await;
                    return Err(map_ws_err(err));
                }
            }
        }

        self.finalize_close_conn(&conn).await
    }

    #[inline]
    async fn disconnect(&self) -> PyResult<()> {
        self.close_with(Message::close(None, "")).await
    }
}

#[pymethods]
impl PyWebSocket {
    #[new]
    #[pyo3(
        signature = (
            uri,
            *,
            headers = None,
            max_payload_len = DEFAULT_MAX_PAYLOAD_LEN,
            frame_size = DEFAULT_FRAME_SIZE,
            flush_threshold = DEFAULT_FLUSH_THRESHOLD,
            close_timeout = Some(DEFAULT_CLOSE_TIMEOUT),
            recv_timeout = None,
        ),
        text_signature = "(uri, *, headers=None, max_payload_len=67_108_864, frame_size=4_194_304, flush_threshold=8_192, close_timeout=10, recv_timeout=None)"
    )]
    fn py_new(
        uri: UriLike,
        headers: Option<PyHeadersLike>,
        max_payload_len: NonZeroUsize,
        frame_size: NonZeroUsize,
        flush_threshold: NonZeroUsize,
        close_timeout: Option<PyTimeout>,
        recv_timeout: Option<PyTimeout>,
    ) -> Self {
        let uri = uri.into();
        let headers = headers.map(http::HeaderMap::from);
        let cfg = WebSocketConfig {
            uri,
            headers,
            max_payload_len,
            frame_size,
            flush_threshold,
            close_timeout: close_timeout.map(Into::into),
            recv_timeout: recv_timeout.map(Into::into),
        };
        Self::from(cfg)
    }

    fn __repr__(&self) -> String {
        format!("{self}")
    }

    fn __bool__(&self) -> bool {
        self.0.state.blocking_lock().is_open()
    }

    #[getter]
    fn uri(&self) -> String {
        self.0.cfg.uri.to_string()
    }

    #[getter]
    fn status(&self, py: Python<'_>) -> PyResult<Option<Py<PyHttpStatus>>> {
        self.0
            .state
            .blocking_lock()
            .handshake()
            .map(|handshake| PyHttpStatus::from_status_code_cached(py, handshake.status))
            .transpose()
    }

    #[getter]
    fn headers(&self) -> Option<PyHeaders> {
        self.0
            .state
            .blocking_lock()
            .handshake()
            .map(|handshake| PyHeaders::from(handshake.response_headers.clone()))
    }

    #[getter]
    fn closed(&self) -> bool {
        matches!(&*self.0.state.blocking_lock(), WebSocketState::Closed(_))
    }

    #[getter]
    fn open(&self) -> bool {
        self.0.state.blocking_lock().is_open()
    }

    /// Return the ready state of the `WebSocket` (`0`=CONNECTING, `1`=OPEN, `2`=CLOSING, `3`=CLOSED).
    ///
    /// Based on the `WebSocket.readyState` property from the Web API:
    /// <https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/readyState>
    #[getter]
    fn ready_state(&self) -> u8 {
        self.0.state.blocking_lock().ready_state().into()
    }

    fn __aiter__(this: PyRef<Self>) -> PyRef<Self> {
        this
    }

    fn __anext__<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let this = self.clone();
        future_into_py(py, async move {
            if let Some(msg) = this.recv_msg(None).await? {
                Ok(msg)
            } else {
                Err(PyStopAsyncIteration::new_err("websocket closed"))
            }
        })
    }

    fn __await__(slf: Py<Self>, py: Python<'_>) -> PyResult<Bound<'_, PyAny>> {
        let coro = future_into_py(py, async move {
            slf.get().connect().await?;
            Ok(slf)
        })?;
        coro.getattr(pyo3::intern!(py, "__await__"))?.call0()
    }

    fn __aenter__(slf: Py<Self>, py: Python<'_>) -> PyResult<Bound<'_, PyAny>> {
        future_into_py(py, async move {
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
        future_into_py(py, async move { this.disconnect().await })
    }

    #[pyo3(signature = (timeout = None))]
    fn recv<'py>(
        &self,
        py: Python<'py>,
        timeout: Option<PyTimeout>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let this = self.clone();
        future_into_py(py, async move {
            if let Some(msg) = this.recv_msg(timeout.map(Into::into)).await? {
                Ok(msg)
            } else {
                py_runtime_err!("websocket closed")
            }
        })
    }

    #[pyo3(signature = (timeout = None))]
    fn receive<'py>(
        &self,
        py: Python<'py>,
        timeout: Option<PyTimeout>,
    ) -> PyResult<Bound<'py, PyAny>> {
        self.recv(py, timeout)
    }

    #[pyo3(name = "send")]
    fn py_send<'py>(&self, py: Python<'py>, message: PyMessageLike) -> PyResult<Bound<'py, PyAny>> {
        let this = self.clone();
        let message = Message::from(message);
        future_into_py(py, async move { this.send(message).await })
    }

    #[pyo3(
        signature = (code = PyWsCloseCode::NORMAL_CLOSURE, reason = None),
        text_signature = "(self, code=1_000, reason=None)"
    )]
    fn close<'py>(
        &self,
        py: Python<'py>,
        code: PyWsCloseCode,
        reason: Option<PyWsCloseReason>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let this = self.clone();
        let pymsg = PyWsMessage::close(code, reason)?;
        future_into_py(py, async move { this.close_with(pymsg.into()).await })
    }

    #[pyo3(signature = (payload = None))]
    fn ping<'py>(
        &self,
        py: Python<'py>,
        payload: Option<PyPingPayload>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let this = self.clone();
        let payload = payload.unwrap_or_default().into();
        future_into_py(py, async move { this.send(payload).await })
    }

    #[pyo3(signature = (payload = None))]
    fn pong<'py>(
        &self,
        py: Python<'py>,
        payload: Option<PyPongPayload>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let this = self.clone();
        let payload = payload.unwrap_or_default().into();
        future_into_py(py, async move { this.send(payload).await })
    }
}
impl std::fmt::Display for PyWebSocket {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "WebSocket(uri={:?}, open={})",
            self.0.cfg.uri,
            self.0.state.blocking_lock().is_open()
        )
    }
}

#[pyfunction]
#[pyo3(
    signature = (
        uri,
        *,
        headers = None,
        flush_threshold = DEFAULT_FLUSH_THRESHOLD,
        frame_size = DEFAULT_FRAME_SIZE,
        max_payload_len = DEFAULT_MAX_PAYLOAD_LEN,
        close_timeout = Some(DEFAULT_CLOSE_TIMEOUT),
        recv_timeout = None,
    ),
    text_signature = "(uri, *, headers=None, close_timeout=10, flush_threshold=8_192, frame_size=4_194_304, max_payload_len=67_108_864, recv_timeout=None)"
)]
pub(crate) fn websocket(
    uri: UriLike,
    headers: Option<PyHeadersLike>,
    flush_threshold: NonZeroUsize,
    frame_size: NonZeroUsize,
    max_payload_len: NonZeroUsize,
    close_timeout: Option<PyTimeout>,
    recv_timeout: Option<PyTimeout>,
) -> PyWebSocket {
    PyWebSocket::py_new(
        uri,
        headers,
        max_payload_len,
        frame_size,
        flush_threshold,
        close_timeout,
        recv_timeout,
    )
}
