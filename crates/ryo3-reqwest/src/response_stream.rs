//! Response stream impls for ryo3-reqwest
//!
//!
//! ---
//!
//! ## OLD NOTES (pre-great-refactor of 2026-10-14)
//!
//! This whole response iterator was a difficult thing to figure out.
//!
//! NOTE: I (jesse) am pretty proud of this. I was struggling to get the
//!       async-iterator thingy to work bc rust + async is quite hard, but
//!       after lots and lots and lots of trial and error this works!
//!
//! clippy says this is too long and complicated to just sit in the struct def
//!
//! ---
//!
//! # UPDATE [2025-06-06] ~ Switch to using BoxStream:
//!    In the obstore library, Kyle Barron (et al) use `BoxStream` and fuse the
//!    stream which seems very smart.
//!    REF: https://github.com/developmentseed/obstore/blob/50782ed782a15185a936d435d13ca0a7969154ae/obstore/src/get.rs#L219
//!
//! The inner stream type changes from:
//! ```rust
//! type OldStreamType = `Arc<Mutex<Pin<Box<dyn Stream<Item = Result<Bytes, reqwest::Error>> + Send>>>>`
//! ```
//! to:
//! ```rust
//! type NewStreamType = `Arc<Mutex<Fuse<BoxStream<'static, Result<Bytes, reqwest::Error>>>>>`
//! ```
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use bytes::{Bytes, BytesMut};
use futures_util::StreamExt;
use pyo3::IntoPyObjectExt;
use pyo3::exceptions::{PyStopAsyncIteration, PyStopIteration};
use pyo3::prelude::*;
use reqwest::StatusCode;
use ryo3_bytes::RyBytes;
use ryo3_macro_rules::py_runtime_error;
#[cfg(feature = "experimental-async")]
use ryo3_tokio_rt::on_tokio_py;
use ryo3_tokio_rt::{future_into_py, get_tokio_runtime};
use tokio::sync::mpsc::error::TryRecvError;
use tokio::sync::{Mutex, Notify, mpsc};
use tokio::task::AbortHandle;

use crate::errors::map_reqwest_err;

// TODO: cap is 8 *items* not 8 *bytes* ~ w/ `min_read_size`
//       fix so that we dont eat all the ram if each item is large
const RESP_BODY_CHANNEL_CAP: usize = 8;

type ResponseItem = Result<Bytes, reqwest::Error>;

/// the running read-ahead task ~ aborts it on drop
struct RunningPump {
    rx: mpsc::Receiver<ResponseItem>,
    task: AbortHandle,
}

impl Drop for RunningPump {
    fn drop(&mut self) {
        self.task.abort();
    }
}

/// lives outside the rx lock so `close()` can wake a read that is parked in
/// `recv()` while it holds the lock
#[derive(Default)]
struct CloseSignal {
    closed: AtomicBool,
    notify: Notify,
}

impl CloseSignal {
    #[inline]
    fn close(&self) {
        self.closed.store(true, Ordering::Release);
        self.notify.notify_waiters();
    }

    #[inline]
    fn is_closed(&self) -> bool {
        self.closed.load(Ordering::Acquire)
    }
}

// IMPORTANT: dont `Drop` on this guy; so that the body can be moved out of
//            the idle state
enum ResponseRx {
    Idle {
        body: reqwest::Body,
        min_read_size: usize,
    },
    Running(RunningPump),
    Closed,
}

impl ResponseRx {
    /// start/spawn the pump on first read (not when the stream is made)
    #[inline]
    fn start(&mut self) {
        if !matches!(self, Self::Idle { .. }) {
            return;
        }
        let Self::Idle {
            body,
            min_read_size,
        } = std::mem::replace(self, Self::Closed)
        else {
            unreachable!()
        };
        let (tx, rx) = mpsc::channel(RESP_BODY_CHANNEL_CAP);
        let task = get_tokio_runtime()
            .spawn(pump_response_body(body, min_read_size, tx))
            .abort_handle();
        *self = Self::Running(RunningPump { rx, task });
    }

    #[inline]
    async fn recv(&mut self, close_sig: &CloseSignal) -> Option<ResponseItem> {
        // made before the flag check so a racing `close()` cant slip by
        let closed = close_sig.notify.notified();
        if close_sig.is_closed() {
            return None;
        }
        self.start();
        match self {
            Self::Running(pump) => tokio::select! {
                biased;
                item = pump.rx.recv() => item,
                () = closed => None,
            },
            _ => None,
        }
    }

    #[inline]
    fn try_recv(&mut self, close_sig: &CloseSignal) -> Result<ResponseItem, TryRecvError> {
        if close_sig.is_closed() {
            return Err(TryRecvError::Disconnected);
        }
        self.start();
        match self {
            Self::Running(pump) => pump.rx.try_recv(),
            _ => Err(TryRecvError::Disconnected),
        }
    }
}

struct ResponseShared {
    rx: Mutex<ResponseRx>,
    close_sig: CloseSignal,
}

#[derive(Clone)]
struct ResponseStreamInner {
    status: StatusCode,
    shared: Arc<ResponseShared>,
}

impl ResponseStreamInner {
    fn from_body(status: StatusCode, body: reqwest::Body, min_read_size: usize) -> Self {
        let rx = ResponseRx::Idle {
            body,
            min_read_size,
        };
        Self {
            status,
            shared: Arc::new(ResponseShared {
                rx: Mutex::new(rx),
                close_sig: CloseSignal::default(),
            }),
        }
    }

    #[inline]
    fn try_next(&self) -> Option<Result<Option<Bytes>, reqwest::Error>> {
        let shared = &*self.shared;
        let Ok(mut rx) = shared.rx.try_lock() else {
            return None;
        };
        match rx.try_recv(&shared.close_sig) {
            Ok(item) => Some(Some(item).transpose()),
            Err(TryRecvError::Empty) => None,
            Err(TryRecvError::Disconnected) => Some(Ok(None)),
        }
    }

    #[inline]
    async fn next(&self) -> Result<Option<Bytes>, reqwest::Error> {
        let shared = &*self.shared;
        shared
            .rx
            .lock()
            .await
            .recv(&shared.close_sig)
            .await
            .transpose()
    }

    /// kick off the read-ahead early (eg on enter); never waits on the lock ~
    /// if somebody holds it they're reading so the pump is already going
    #[inline]
    fn start(&self) {
        let shared = &*self.shared;
        if shared.close_sig.is_closed() {
            return;
        }
        if let Ok(mut rx) = shared.rx.try_lock() {
            rx.start();
        }
    }

    /// drop the body + pump (aborts the read-ahead task if it got going)
    #[inline]
    async fn close(&self) {
        // wake any read parked in `recv()` first so it lets go of the lock
        self.shared.close_sig.close();
        *self.shared.rx.lock().await = ResponseRx::Closed;
    }
}

#[inline]
fn py_anext_result(res: Result<Option<Bytes>, reqwest::Error>) -> PyResult<RyBytes> {
    match res {
        Ok(Some(bytes)) => Ok(RyBytes::from(bytes)),
        Ok(None) => Err(PyStopAsyncIteration::new_err("response-stream-end")),
        Err(e) => Err(map_reqwest_err(e)),
    }
}

// TODO: lines stream ~ allow spliting lines or chunks prolly using
//       memchr + split_to and maybe `keepends: bool= False` kw
async fn pump_response_body(
    body: reqwest::Body,
    min_read_size: usize,
    tx: mpsc::Sender<ResponseItem>,
) {
    let mut stream = http_body_util::BodyDataStream::new(body);
    let mut chunks = Vec::with_capacity(8);
    let mut total_size = 0;
    while let Some(item) = stream.next().await {
        match item {
            Ok(bytes) => {
                total_size += bytes.len();
                chunks.push(bytes);
            }
            Err(e) => {
                let _ = tx.send(Err(e)).await;
                return;
            }
        }
        if total_size >= min_read_size {
            let bytes = join_bytes_chunks(&mut chunks, total_size);
            total_size = 0;
            if tx.send(Ok(bytes)).await.is_err() {
                return;
            }
        }
    }
    if !chunks.is_empty() {
        let _ = tx
            .send(Ok(join_bytes_chunks(&mut chunks, total_size)))
            .await;
    }
}

#[inline]
fn join_bytes_chunks(chunks: &mut Vec<Bytes>, total_size: usize) -> Bytes {
    if chunks.len() == 1 {
        return chunks.pop().expect("wenodis: just checked len == 1");
    }
    let mut bytes_mut = BytesMut::with_capacity(total_size);
    for chunk in chunks.drain(..) {
        bytes_mut.extend_from_slice(&chunk);
    }
    bytes_mut.freeze()
}

/// already-done awaitable
///
/// skips making an asyncio future when the value is ready (a chunk the pump
/// already has, `self` from `__aenter__`, ...)
#[pyclass(name = "Ready", immutable_type)]
struct RyReady(Option<PyResult<Py<PyAny>>>);

impl RyReady {
    #[inline]
    fn new_bound(py: Python<'_>, res: PyResult<Py<PyAny>>) -> PyResult<Bound<'_, PyAny>> {
        Ok(Bound::new(py, Self(Some(res)))?.into_any())
    }
}

#[pymethods]
impl RyReady {
    fn __await__(slf: Py<Self>) -> Py<Self> {
        slf
    }

    fn __iter__(slf: Py<Self>) -> Py<Self> {
        slf
    }

    fn __next__(&mut self) -> PyResult<()> {
        match self.0.take() {
            // an awaitable returns its value via StopIteration
            Some(Ok(value)) => Err(PyStopIteration::new_err((value,))),
            Some(Err(e)) => Err(e),
            None => Err(py_runtime_error!("cannot reuse already awaited coroutine")),
        }
    }
}

#[pyclass(name = "ResponseStream", frozen, immutable_type)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
pub struct RyResponseStream {
    inner: ResponseStreamInner,
}

impl RyResponseStream {
    pub(crate) fn from_body(status: StatusCode, body: reqwest::Body, min_read_size: usize) -> Self {
        let inner = ResponseStreamInner::from_body(status, body, min_read_size);
        Self { inner }
    }

    fn py_anext<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        if let Some(res) = self.inner.try_next() {
            let res = py_anext_result(res).and_then(|bytes| bytes.into_py_any(py));
            return RyReady::new_bound(py, res);
        }
        let inner = self.inner.clone();
        future_into_py(py, async move { py_anext_result(inner.next().await) })
    }

    fn py_aenter(slf: Py<Self>, py: Python<'_>) -> PyResult<Bound<'_, PyAny>> {
        slf.get().inner.start();
        RyReady::new_bound(py, Ok(slf.into_any()))
    }

    fn py_close<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let inner = self.inner.clone();
        future_into_py(py, async move {
            inner.close().await;
            Ok(())
        })
    }
}

impl std::fmt::Display for RyResponseStream {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ResponseStream<{}>", self.inner.status)
    }
}

#[cfg(feature = "experimental-async")]
#[pymethods]
impl RyResponseStream {
    fn close<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        self.py_close(py)
    }

    fn __repr__(&self) -> String {
        format!("{self}")
    }

    fn __aenter__(slf: Py<Self>, py: Python<'_>) -> PyResult<Bound<'_, PyAny>> {
        Self::py_aenter(slf, py)
    }

    #[pyo3(name = "__aexit__")]
    fn __aexit__<'py>(
        &self,
        py: Python<'py>,
        _exc_type: &Bound<'py, PyAny>,
        _exc_value: &Bound<'py, PyAny>,
        _traceback: &Bound<'py, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        self.py_close(py)
    }

    fn __aiter__(this: PyRef<Self>) -> PyRef<Self> {
        this
    }

    fn __anext__<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        self.py_anext(py)
    }

    async fn readall(
        &self,
        #[pyo3(cancel_handle)] mut cancel: pyo3::coroutine::CancelHandle,
    ) -> PyResult<RyBytes> {
        let inner = self.inner.clone();
        on_tokio_py(async move {
            tokio::select! {
                res = readall_bytes(&inner) => res.map(RyBytes::from).map_err(map_reqwest_err),
                _ = cancel.cancelled() => Err(pyo3::exceptions::asyncio::CancelledError::new_err("Response stream read was cancelled")),
            }
        })
        .await
    }

    #[pyo3(signature = (n = 1))]
    async fn take(
        &self,
        n: usize,
        #[pyo3(cancel_handle)] mut cancel: pyo3::coroutine::CancelHandle,
    ) -> PyResult<Vec<RyBytes>> {
        let inner = self.inner.clone();
        on_tokio_py(async move {
            tokio::select! {
                res = take_chunks(&inner, n) => res.map_err(map_reqwest_err),
                _ = cancel.cancelled() => Err(pyo3::exceptions::asyncio::CancelledError::new_err("Response stream read was cancelled")),
            }
        })
        .await
    }

    async fn collect(
        &self,
        #[pyo3(cancel_handle)] mut cancel: pyo3::coroutine::CancelHandle,
    ) -> PyResult<Vec<RyBytes>> {
        let inner = self.inner.clone();
        on_tokio_py(async move {
            tokio::select! {
                res = collect_chunks(&inner) => res.map_err(map_reqwest_err),
                _ = cancel.cancelled() => Err(pyo3::exceptions::asyncio::CancelledError::new_err("Response stream read was cancelled")),
            }
        })
        .await
    }
}

#[cfg(not(feature = "experimental-async"))]
#[pymethods]
impl RyResponseStream {
    fn close<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        self.py_close(py)
    }

    fn __repr__(&self) -> String {
        format!("{self}")
    }

    fn __aenter__(slf: Py<Self>, py: Python<'_>) -> PyResult<Bound<'_, PyAny>> {
        Self::py_aenter(slf, py)
    }

    #[pyo3(name = "__aexit__")]
    fn __aexit__<'py>(
        &self,
        py: Python<'py>,
        _exc_type: &Bound<'py, PyAny>,
        _exc_value: &Bound<'py, PyAny>,
        _traceback: &Bound<'py, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        self.py_close(py)
    }

    fn __aiter__(this: PyRef<Self>) -> PyRef<Self> {
        this
    }

    fn __anext__<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        self.py_anext(py)
    }

    #[pyo3(signature = (n = 1))]
    fn take<'py>(&self, py: Python<'py>, n: usize) -> PyResult<Bound<'py, PyAny>> {
        let inner = self.inner.clone();
        future_into_py(py, async move {
            take_chunks(&inner, n).await.map_err(map_reqwest_err)
        })
    }

    fn readall<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let inner = self.inner.clone();
        future_into_py(py, async move {
            readall_bytes(&inner)
                .await
                .map(RyBytes::from)
                .map_err(map_reqwest_err)
        })
    }

    fn collect<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let inner = self.inner.clone();
        future_into_py(py, async move {
            collect_chunks(&inner).await.map_err(map_reqwest_err)
        })
    }
}

#[pyclass(name = "BlockingResponseStream", frozen, immutable_type)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
pub struct RyBlockingResponseStream {
    inner: ResponseStreamInner,
}

impl RyBlockingResponseStream {
    pub(crate) fn from_body(status: StatusCode, body: reqwest::Body, min_read_size: usize) -> Self {
        let inner = ResponseStreamInner::from_body(status, body, min_read_size);
        Self { inner }
    }
}

#[pymethods]
impl RyBlockingResponseStream {
    fn __repr__(&self) -> String {
        format!("BlockingResponseStream<{}>", self.inner.status)
    }

    fn __enter__(slf: Py<Self>) -> Py<Self> {
        slf.get().inner.start();
        slf
    }

    #[pyo3(name = "__exit__")]
    fn __exit__(
        &self,
        py: Python<'_>,
        _exc_type: &Bound<'_, PyAny>,
        _exc_value: &Bound<'_, PyAny>,
        _traceback: &Bound<'_, PyAny>,
    ) {
        self.close(py);
    }

    fn close(&self, py: Python<'_>) {
        py.detach(|| get_tokio_runtime().block_on(self.inner.close()));
    }

    fn __iter__(this: PyRef<Self>) -> PyRef<Self> {
        this
    }

    fn __next__(&self, py: Python<'_>) -> PyResult<RyBytes> {
        let result = match self.inner.try_next() {
            Some(result) => result,
            None => py.detach(|| get_tokio_runtime().block_on(self.inner.next())),
        };
        match result {
            Ok(Some(bytes)) => Ok(RyBytes::from(bytes)),
            Ok(None) => Err(PyStopIteration::new_err("response-stream-end")),
            Err(e) => Err(map_reqwest_err(e)),
        }
    }

    #[pyo3(signature = (n = 1))]
    fn take(&self, py: Python<'_>, n: usize) -> PyResult<Vec<RyBytes>> {
        py.detach(|| get_tokio_runtime().block_on(take_chunks(&self.inner, n)))
            .map_err(map_reqwest_err)
    }

    fn readall<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let bytes = py
            .detach(|| get_tokio_runtime().block_on(readall_bytes(&self.inner)))
            .map(RyBytes::from)
            .map_err(map_reqwest_err)?;
        bytes.into_bound_py_any(py)
    }

    fn collect<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let bytes = py
            .detach(|| get_tokio_runtime().block_on(collect_chunks(&self.inner)))
            .map_err(map_reqwest_err)?;
        bytes.into_bound_py_any(py)
    }
}

#[inline]
async fn take_chunks(
    inner: &ResponseStreamInner,
    n: usize,
) -> Result<Vec<RyBytes>, reqwest::Error> {
    let shared = &*inner.shared;
    let mut rx = shared.rx.lock().await;
    let mut chunks = Vec::with_capacity(n);
    for _ in 0..n {
        match rx.recv(&shared.close_sig).await {
            Some(Ok(bytes)) => chunks.push(RyBytes::from(bytes)),
            Some(Err(e)) => return Err(e),
            None => break,
        }
    }
    Ok(chunks)
}

#[inline]
async fn readall_bytes(inner: &ResponseStreamInner) -> Result<Bytes, reqwest::Error> {
    let shared = &*inner.shared;
    let mut rx = shared.rx.lock().await;
    let mut bytes_mut = BytesMut::new();
    while let Some(item) = rx.recv(&shared.close_sig).await {
        match item {
            Ok(bytes) => bytes_mut.extend_from_slice(&bytes),
            Err(e) => return Err(e),
        }
    }
    Ok(bytes_mut.freeze())
}

#[inline]
async fn collect_chunks(inner: &ResponseStreamInner) -> Result<Vec<RyBytes>, reqwest::Error> {
    let shared = &*inner.shared;
    let mut rx = shared.rx.lock().await;
    let mut items = Vec::new();
    while let Some(item) = rx.recv(&shared.close_sig).await {
        match item {
            Ok(bytes) => items.push(RyBytes::from(bytes)),
            Err(e) => return Err(e),
        }
    }
    Ok(items)
}

#[cfg(test)]
mod tests {
    use bytes::Bytes;

    use super::join_bytes_chunks;

    #[test]
    fn test_join_uno_chunk_zerocopy() {
        let bytes = Bytes::from_static(b"howdy");
        let ptr = bytes.as_ptr();
        let mut chunks = vec![bytes];

        let coalesced = join_bytes_chunks(&mut chunks, 5);

        assert_eq!(coalesced, b"howdy"[..]);
        assert_eq!(coalesced.as_ptr(), ptr);
        assert_eq!(chunks, Vec::<Bytes>::new());
    }

    #[test]
    fn test_join_multiple_chunks() {
        let mut chunks = vec![
            Bytes::from_static(b"howdy "),
            Bytes::from_static(b"partner"),
        ];

        let coalesced = join_bytes_chunks(&mut chunks, 13);

        assert_eq!(coalesced, b"howdy partner"[..]);
        assert_eq!(chunks, Vec::<Bytes>::new());
    }
}
