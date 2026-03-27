use crate::constants::{DEFAULT_FLUSH_THRESHOLD, DEFAULT_FRAME_SIZE, DEFAULT_MAX_PAYLOAD_LEN};
use crate::py_message::{PyMessageLike, PyPingPayload, PyPongPayload, PyWebSocketMessage};
use crate::util::{map_ws_err, parse_uri, validate_close_reason};
use futures_util::{
    SinkExt, StreamExt,
    stream::{SplitSink, SplitStream},
};
use parking_lot::Mutex as ParkingLotMutex;
use pyo3::exceptions::{PyEOFError, PyRuntimeError, PyStopAsyncIteration};
use pyo3::{prelude::*, pybacked::PyBackedStr};
use ryo3_bytes::PyBytes as RyBytes;
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
struct WebSocketConnected {
    status: http::StatusCode,
    response_headers: http::HeaderMap,
    writer: Arc<Mutex<TokioWsWrite>>,
    reader: Arc<Mutex<TokioWsRead>>,
}

impl WebSocketConnected {
    #[inline]
    async fn recv_next(&self) -> PyResult<Option<PyWebSocketMessage>> {
        let mut reader = self.reader.lock().await;
        match reader.next().await {
            Some(Ok(msg)) => Ok(Some(PyWebSocketMessage::from(msg))),
            Some(Err(err)) => Err(map_ws_err(err)),
            None => Ok(None),
        }
    }
}

#[derive(Debug)]
struct WebSocketInner {
    // config stuff
    uri: http::Uri,
    headers: Option<http::HeaderMap>,
    max_payload_len: usize,
    frame_size: usize,
    flush_threshold: usize,

    // state
    connection: Mutex<Option<WebSocketConnected>>,

    // response data
    cached_status: ParkingLotMutex<Option<http::StatusCode>>,
    cached_headers: ParkingLotMutex<Option<http::HeaderMap>>,
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
        let inner = WebSocketInner {
            uri,
            headers,
            max_payload_len,
            frame_size,
            flush_threshold,
            connection: Mutex::new(None),
            cached_status: ParkingLotMutex::new(None),
            cached_headers: ParkingLotMutex::new(None),
        };
        Self {
            uri_string,
            inner: Arc::new(inner),
        }
    }

    #[inline]
    async fn connect(&self) -> PyResult<()> {
        // scoped already conn chezch
        {
            let conn = self.inner.connection.lock().await;
            if conn.is_some() {
                return Err(PyRuntimeError::new_err("WebSocket already connected"));
            }
        }

        // actually do the thing here
        let mut builder = ClientBuilder::from_uri(self.inner.uri.clone());

        if let Some(headers) = &self.inner.headers {
            for (name, value) in headers {
                builder = builder
                    .add_header(name.clone(), value.clone())
                    .map_err(map_ws_err)?;
            }
        }

        let mut config = Config::default();
        if self.inner.frame_size == 0 {
            return py_value_err!("frame_size must be non-zero");
        }
        config = config.frame_size(self.inner.frame_size);
        config = config.flush_threshold(self.inner.flush_threshold);
        builder = builder.config(config);

        let limits = Limits::default().max_payload_len(Some(self.inner.max_payload_len));
        builder = builder.limits(limits);

        let (ws, response) = builder.connect().await.map_err(map_ws_err)?;
        let (writer, reader) = ws.split();

        let status = response.status();
        let response_headers = response.headers().clone();

        let connected = WebSocketConnected {
            status,
            response_headers: response_headers.clone(),
            writer: Arc::new(Mutex::new(writer)),
            reader: Arc::new(Mutex::new(reader)),
        };

        // response shit saved here
        *self.inner.cached_status.lock() = Some(status);
        *self.inner.cached_headers.lock() = Some(response_headers);

        // and bing bang boom this is the connnnn
        *self.inner.connection.lock().await = Some(connected);

        Ok(())
    }

    #[inline]
    async fn get_connected(&self) -> PyResult<WebSocketConnected> {
        let conn = self.inner.connection.lock().await;
        conn.clone().ok_or_else(|| {
            PyRuntimeError::new_err("WebSocket not connected — use `async with ws:` to connect")
        })
    }

    #[inline]
    async fn recv_next_msg(&self) -> PyResult<PyWebSocketMessage> {
        let conn = self.get_connected().await?;
        conn.recv_next()
            .await?
            .ok_or_else(|| PyEOFError::new_err("websocket closed"))
    }

    #[inline]
    async fn iter_next_msg(&self) -> PyResult<PyWebSocketMessage> {
        let conn = self.get_connected().await?;
        match conn.recv_next().await? {
            Some(msg) => Ok(msg),
            None => Err(PyStopAsyncIteration::new_err("websocket closed")),
        }
    }

    #[inline]
    async fn send_message(&self, message: Message) -> PyResult<()> {
        let conn = self.get_connected().await?;
        let mut writer = conn.writer.lock().await;
        writer.send(message).await.map_err(map_ws_err)?;
        Ok(())
    }

    #[inline]
    async fn disconnect(&self) -> PyResult<()> {
        let mut conn_guard = self.inner.connection.lock().await;
        let conn = match conn_guard.take() {
            Some(c) => c,
            None => return Ok(()),
        };

        let mut writer = conn.writer.lock().await;
        writer
            .send(Message::close(None, ""))
            .await
            .map_err(map_ws_err)?;

        Ok(())
    }

    #[inline]
    async fn send_close(
        &self,
        code: Option<tokio_websockets::CloseCode>,
        reason: &str,
    ) -> PyResult<()> {
        let mut conn_guard = self.inner.connection.lock().await;
        let conn = match conn_guard.take() {
            Some(c) => c,
            None => return Ok(()),
        };

        let mut writer = conn.writer.lock().await;
        writer
            .send(Message::close(code, reason))
            .await
            .map_err(map_ws_err)?;
        Ok(())
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
        self.inner.cached_status.lock().is_some()
    }

    #[getter]
    fn uri(&self) -> &str {
        &self.uri_string
    }

    #[getter]
    fn status(&self, py: Python<'_>) -> PyResult<Py<PyHttpStatus>> {
        let status = self.inner.cached_status.lock().clone();
        match status {
            Some(s) => PyHttpStatus::from_status_code_cached(py, s),
            None => Err(PyRuntimeError::new_err("WebSocket not connected")),
        }
    }

    #[getter]
    fn headers(&self) -> PyResult<PyHeaders> {
        let headers = self.inner.cached_headers.lock().clone();
        match headers {
            Some(h) => Ok(PyHeaders::from(h)),
            None => Err(PyRuntimeError::new_err("WebSocket not connected")),
        }
    }

    #[getter]
    fn closed(&self) -> bool {
        self.inner.cached_status.lock().is_none()
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
