use std::num::NonZeroUsize;
use std::sync::Arc;
use std::time::Duration;

use futures_util::{SinkExt, StreamExt};
use pyo3::exceptions::{PyRuntimeError, PyStopAsyncIteration};
use pyo3::prelude::*;
use ryo3_core::sync::RyMutex;
use ryo3_http::{PyHeaders, PyHeadersLike, PyHttpStatus};
use ryo3_macro_rules::py_runtime_err;
use ryo3_std::time::{PyDuration, PyTimeout};
use ryo3_tokio_rt::{RyCoroutine, get_tokio_runtime, pyloop_coro, tokio_coro};
use tokio::sync::Mutex;
use tokio_websockets::{ClientBuilder, Config, Limits, Message};

use crate::constants::{
    DEFAULT_CLOSE_TIMEOUT, DEFAULT_FLUSH_THRESHOLD, DEFAULT_FRAME_SIZE, DEFAULT_MAX_PAYLOAD_LEN,
};
use crate::errors::map_ws_err;
use crate::types::{PyWsCloseCode, PyWsCloseReason, TokioWsRead, TokioWsWrite, UriLike};
use crate::{PyMessageLike, PyPingPayload, PyPongPayload, PyWsMessage};

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

/// mdn "readyState"
///
/// REF: <https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/readyState>
#[derive(Debug)]
enum WebSocketState {
    /// CONNECTING ~ created but not awaited yet
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

impl<'py> IntoPyObject<'py> for WebSocketConfig {
    type Target = pyo3::types::PyDict;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    #[inline]
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        let d = pyo3::types::PyDict::new(py);
        // py-str-keys
        let headers_pystr = pyo3::intern!(py, "headers");
        let close_timeout_pystr = pyo3::intern!(py, "close_timeout");
        let recv_timeout_pystr = pyo3::intern!(py, "recv_timeout");

        d.set_item(pyo3::intern!(py, "uri"), self.uri.to_string())?;
        if let Some(headers) = self.headers {
            let py_headers = PyHeaders::from(headers);
            d.set_item(headers_pystr, py_headers)?;
        } else {
            d.set_item(headers_pystr, py.None())?;
        }
        d.set_item(
            pyo3::intern!(py, "max_payload_len"),
            self.max_payload_len.get(),
        )?;
        d.set_item(pyo3::intern!(py, "frame_size"), self.frame_size.get())?;
        d.set_item(
            pyo3::intern!(py, "flush_threshold"),
            self.flush_threshold.get(),
        )?;
        if let Some(close_timeout) = self.close_timeout {
            d.set_item(close_timeout_pystr, PyDuration::from(close_timeout))?;
        } else {
            d.set_item(close_timeout_pystr, py.None())?;
        }
        if let Some(recv_timeout) = self.recv_timeout {
            d.set_item(recv_timeout_pystr, PyDuration::from(recv_timeout))?;
        } else {
            d.set_item(recv_timeout_pystr, py.None())?;
        }
        Ok(d)
    }
}

#[derive(Debug)]
struct WebSocketInner {
    cfg: WebSocketConfig,
    state: RyMutex<WebSocketState, false>,
}

#[derive(Debug, Clone)]
#[pyclass(name = "WebSocket", frozen, immutable_type)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
pub struct PyWebSocket(Arc<WebSocketInner>);

impl From<WebSocketConfig> for PyWebSocket {
    fn from(cfg: WebSocketConfig) -> Self {
        Self::new(cfg)
    }
}

impl PyWebSocket {
    fn new(cfg: WebSocketConfig) -> Self {
        let inner = WebSocketInner {
            cfg,
            state: RyMutex::new(WebSocketState::Connecting),
        };
        Self(Arc::new(inner))
    }

    async fn connect(&self) -> PyResult<()> {
        self.try_connectable()?;
        match Self::handshake(self.0.cfg.clone()).await {
            Ok(connected) => self.set_open(connected),
            Err(err) => {
                // "couldn't be opened"
                let mut state = self.0.state.py_lock();
                if matches!(&*state, WebSocketState::Connecting) {
                    *state = WebSocketState::Closed(None);
                }
                Err(err)
            }
        }
    }

    fn try_connectable(&self) -> PyResult<()> {
        match &*self.0.state.py_lock() {
            WebSocketState::Connecting | WebSocketState::Closed(_) => Ok(()),
            WebSocketState::Open(_) => py_runtime_err!("WebSocket already connected"),
            WebSocketState::Closing(_) => py_runtime_err!("WebSocket is closing"),
        }
    }

    /// errs if a concurrent connect won
    fn set_open(&self, connected: WebSocketConnected) -> PyResult<()> {
        let mut state = self.0.state.py_lock();
        match &*state {
            WebSocketState::Connecting | WebSocketState::Closed(_) => {
                *state = WebSocketState::Open(connected);
                Ok(())
            }
            WebSocketState::Open(_) => py_runtime_err!("WebSocket already connected"),
            WebSocketState::Closing(_) => py_runtime_err!("WebSocket is closing"),
        }
    }

    async fn handshake(cfg: WebSocketConfig) -> PyResult<WebSocketConnected> {
        let mut builder = ClientBuilder::from_uri(cfg.uri.clone());

        if let Some(headers) = &cfg.headers {
            for (name, value) in headers {
                builder = builder
                    .add_header(name.clone(), value.clone())
                    .map_err(map_ws_err)?;
            }
        }

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
        Ok(WebSocketConnected {
            handshake,
            writer: Arc::new(Mutex::new(writer)),
            reader: Arc::new(Mutex::new(reader)),
        })
    }

    #[inline]
    fn get_connected(&self) -> PyResult<WebSocketConnected> {
        let state = self.0.state.py_lock();
        match &*state {
            WebSocketState::Open(conn) => Ok(conn.clone()),
            WebSocketState::Closing(_) => py_runtime_err!("WebSocket is closing"),
            WebSocketState::Connecting | WebSocketState::Closed(_) => {
                py_runtime_err!(
                    "WebSocket not connected; use `await ws` or `async with ws:` to connect"
                )
            }
        }
    }

    #[inline]
    fn close_current_connection(&self, current: &WebSocketConnected) {
        let mut state = self.0.state.py_lock();
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
    fn close_current_connection_with_err(
        &self,
        current: &WebSocketConnected,
        err: tokio_websockets::Error,
    ) -> PyErr {
        self.close_current_connection(current);
        map_ws_err(err)
    }

    #[inline]
    fn close_current_connection_with_pyerr(
        &self,
        current: &WebSocketConnected,
        err: PyErr,
    ) -> PyErr {
        self.close_current_connection(current);
        err
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
                    self.close_current_connection(conn);
                    return py_runtime_err!(
                        "websocket close timed out after {} seconds",
                        close_timeout.as_secs_f64()
                    );
                }
            }
            None => close_future.await,
        };

        self.close_current_connection(conn);
        result
    }

    #[inline]
    async fn recv_msg(&self, timeout: Option<Duration>) -> PyResult<Option<PyWsMessage>> {
        let conn = match &*self.0.state.py_lock() {
            WebSocketState::Open(conn) | WebSocketState::Closing(conn) => conn.clone(),
            WebSocketState::Connecting | WebSocketState::Closed(_) => {
                return py_runtime_err!(
                    "WebSocket not connected; use `await ws` or `async with ws:` to connect"
                );
            }
        };

        let timeout = timeout.or(self.0.cfg.recv_timeout);
        let msg_result = if let Some(dur) = timeout {
            match tokio::time::timeout(dur, conn.recv_next()).await {
                Ok(result) => result,
                Err(_) => return py_runtime_err!("recv timeout"),
            }
        } else {
            conn.recv_next().await
        };

        match msg_result {
            Ok(Some(msg)) => {
                if msg.is_close() {
                    self.close_current_connection(&conn);
                }
                Ok(Some(PyWsMessage::from(msg)))
            }
            Ok(None) => {
                // peer gone... ws is `Closed` and mark before raising/returning
                self.close_current_connection(&conn);
                Ok(None)
            }
            Err(err) => {
                // abrupt ws-protocol failure, raise the err
                Err(self.close_current_connection_with_pyerr(&conn, err))
            }
        }
    }

    #[inline]
    async fn recv_required(&self, timeout: Option<Duration>) -> PyResult<PyWsMessage> {
        if let Some(msg) = self.recv_msg(timeout).await? {
            Ok(msg)
        } else {
            py_runtime_err!("websocket closed")
        }
    }

    #[inline]
    async fn send(&self, message: Message) -> PyResult<()> {
        let conn = self.get_connected()?;
        let mut writer = conn.writer.lock().await;
        writer
            .send(message)
            .await
            // if the peer disappeared, mark the socket closed
            // this was happening for me when working on a server that i was
            // restarting over and over and over
            .map_err(|err| self.close_current_connection_with_err(&conn, err))?;
        Ok(())
    }

    async fn close_with(&self, message: Message) -> PyResult<()> {
        let (conn, should_send_close) = {
            let mut state = self.0.state.py_lock();
            match &*state {
                WebSocketState::Open(conn) => {
                    let conn = conn.clone();
                    *state = WebSocketState::Closing(conn.clone());
                    (conn, true)
                }
                // already closing ~ wait for it
                WebSocketState::Closing(conn) => (conn.clone(), false),
                WebSocketState::Closed(_) => return Ok(()),
                WebSocketState::Connecting => {
                    *state = WebSocketState::Closed(None);
                    return Ok(());
                }
            }
        };

        let result = async {
            if should_send_close {
                let mut writer = conn.writer.lock().await;
                match writer.send(message).await {
                    Ok(()) | Err(tokio_websockets::Error::AlreadyClosed) => {}
                    Err(err) => return Err(map_ws_err(err)),
                }
            }

            self.finalize_close_conn(&conn).await
        }
        .await;
        self.close_current_connection(&conn);
        result
    }
}

impl std::fmt::Display for PyWebSocket {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "WebSocket(uri={:?}, open={})",
            self.0.cfg.uri,
            self.0.state.py_lock().is_open()
        )
    }
}

impl PyWebSocket {
    fn connect_coro<'py>(
        slf: Py<Self>,
        py: Python<'py>,
        qualname: &'static str,
    ) -> PyResult<Bound<'py, RyCoroutine>> {
        tokio_coro(py, qualname, async move {
            slf.get().connect().await?;
            Ok(slf)
        })
    }

    /// plain `JoinHandle` (not aborted on cancel)
    ///
    /// close always finishes (like browser)
    fn close_coro<'py>(
        &self,
        py: Python<'py>,
        qualname: &'static str,
        message: Message,
    ) -> PyResult<Bound<'py, RyCoroutine>> {
        let this = self.clone();
        pyloop_coro(py, qualname, async move {
            get_tokio_runtime()
                .spawn(async move { this.close_with(message).await })
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("websocket close failed: {e}")))?
        })
    }

    fn recv_coro<'py>(
        &self,
        py: Python<'py>,
        qualname: &'static str,
        timeout: Option<PyTimeout>,
    ) -> PyResult<Bound<'py, RyCoroutine>> {
        let this = self.clone();
        tokio_coro(py, qualname, async move {
            this.recv_required(timeout.map(Into::into)).await
        })
    }

    fn send_coro<'py>(
        &self,
        py: Python<'py>,
        qualname: &'static str,
        message: Message,
    ) -> PyResult<Bound<'py, RyCoroutine>> {
        let this = self.clone();
        tokio_coro(py, qualname, async move { this.send(message).await })
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
        let headers = headers.map(PyHeadersLike::into_header_map);
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

    fn config(&self) -> WebSocketConfig {
        self.0.cfg.clone()
    }

    fn __repr__(&self) -> String {
        format!("{self}")
    }

    fn __bool__(&self) -> bool {
        self.0.state.py_lock().is_open()
    }

    #[getter]
    fn uri(&self) -> String {
        self.0.cfg.uri.to_string()
    }

    #[getter]
    fn status(&self, py: Python<'_>) -> PyResult<Option<Py<PyHttpStatus>>> {
        self.0
            .state
            .py_lock()
            .handshake()
            .map(|handshake| PyHttpStatus::from_status_code_cached(py, handshake.status))
            .transpose()
    }

    #[getter]
    fn headers(&self) -> Option<PyHeaders> {
        self.0
            .state
            .py_lock()
            .handshake()
            .map(|handshake| PyHeaders::from(handshake.response_headers.clone()))
    }

    #[getter]
    fn closed(&self) -> bool {
        matches!(&*self.0.state.py_lock(), WebSocketState::Closed(_))
    }

    #[getter]
    fn open(&self) -> bool {
        self.0.state.py_lock().is_open()
    }

    /// Return the ready state of the `WebSocket` (`0`=CONNECTING, `1`=OPEN, `2`=CLOSING, `3`=CLOSED).
    ///
    /// Based on the `WebSocket.readyState` property from the Web API:
    /// <https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/readyState>
    #[getter]
    fn ready_state(&self) -> u8 {
        self.0.state.py_lock().ready_state().into()
    }

    fn __aiter__(this: PyRef<Self>) -> PyRef<Self> {
        this
    }

    fn __anext__<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, RyCoroutine>> {
        let this = self.clone();
        tokio_coro(py, "WebSocket.__anext__", async move {
            this.recv_msg(None)
                .await?
                .ok_or_else(|| PyStopAsyncIteration::new_err("websocket closed"))
        })
    }

    fn __await__(slf: Py<Self>, py: Python<'_>) -> PyResult<Bound<'_, RyCoroutine>> {
        Self::connect_coro(slf, py, "WebSocket.__await__")
    }

    fn __aenter__(slf: Py<Self>, py: Python<'_>) -> PyResult<Bound<'_, RyCoroutine>> {
        Self::connect_coro(slf, py, "WebSocket.__aenter__")
    }

    fn __aexit__<'py>(
        &self,
        py: Python<'py>,
        _exc_type: &Bound<'py, PyAny>,
        _exc_value: &Bound<'py, PyAny>,
        _traceback: &Bound<'py, PyAny>,
    ) -> PyResult<Bound<'py, RyCoroutine>> {
        self.close_coro(py, "WebSocket.__aexit__", Message::close(None, ""))
    }

    #[pyo3(signature = (timeout = None))]
    fn recv<'py>(
        &self,
        py: Python<'py>,
        timeout: Option<PyTimeout>,
    ) -> PyResult<Bound<'py, RyCoroutine>> {
        self.recv_coro(py, "WebSocket.recv", timeout)
    }

    #[pyo3(signature = (timeout = None))]
    fn receive<'py>(
        &self,
        py: Python<'py>,
        timeout: Option<PyTimeout>,
    ) -> PyResult<Bound<'py, RyCoroutine>> {
        self.recv_coro(py, "WebSocket.receive", timeout)
    }

    #[pyo3(name = "send")]
    fn py_send<'py>(
        &self,
        py: Python<'py>,
        message: PyMessageLike,
    ) -> PyResult<Bound<'py, RyCoroutine>> {
        self.send_coro(py, "WebSocket.send", Message::from(message))
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
    ) -> PyResult<Bound<'py, RyCoroutine>> {
        let pymsg = PyWsMessage::close(code, reason)?;
        self.close_coro(py, "WebSocket.close", pymsg.into())
    }

    #[pyo3(signature = (payload = None))]
    fn ping<'py>(
        &self,
        py: Python<'py>,
        payload: Option<PyPingPayload>,
    ) -> PyResult<Bound<'py, RyCoroutine>> {
        let payload = payload.unwrap_or_default().into();
        self.send_coro(py, "WebSocket.ping", payload)
    }

    #[pyo3(signature = (payload = None))]
    fn pong<'py>(
        &self,
        py: Python<'py>,
        payload: Option<PyPongPayload>,
    ) -> PyResult<Bound<'py, RyCoroutine>> {
        let payload = payload.unwrap_or_default().into();
        self.send_coro(py, "WebSocket.pong", payload)
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
