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
//! # UPDATE [2025-06-06] ~ Switch to using `BoxStream`:
//!    In the obstore library, Kyle Barron (et al) use `BoxStream` and fuse the
//!    stream which seems very smart.
//!    REF: <https://github.com/developmentseed/obstore/blob/50782ed782a15185a936d435d13ca0a7969154ae/obstore/src/get.rs#L219>
//!
//! The inner stream type changes from:
//! ```text
//! type OldStreamType = `Arc<Mutex<Pin<Box<dyn Stream<Item = Result<Bytes, reqwest::Error>> + Send>>>>`
//! ```
//! to:
//! ```text
//! type NewStreamType = `Arc<Mutex<Fuse<BoxStream<'static, Result<Bytes, reqwest::Error>>>>>`
//! ```
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::task::Poll;

use bytes::{Bytes, BytesMut};
use futures_util::StreamExt;
use pyo3::IntoPyObjectExt;
use pyo3::exceptions::{PyStopAsyncIteration, PyStopIteration};
use pyo3::prelude::*;
use reqwest::StatusCode;
use ryo3_bytes::RyBytes;
use ryo3_tokio_rt::{
    RyCoroutine, done_coro, failed_coro, get_tokio_runtime, pyloop_coro, tokio_coro,
};
use tokio::sync::mpsc::error::TryRecvError;
use tokio::sync::{Mutex, Notify, mpsc};
use tokio::task::AbortHandle;

use crate::errors::map_reqwest_err;

// TODO: cap is 8 *items* not 8 *bytes* ~ w/ `min_read_size`
//       fix so that we dont eat all the ram if each item is large
const RESP_BODY_CHANNEL_CAP: usize = 8;

type ResponseItem = Result<Bytes, reqwest::Error>;
type BodyStream = http_body_util::BodyDataStream<reqwest::Body>;

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
        body: BodyStream,
        min_read_size: usize,
        /// read ready frames w/o the pump
        inline_reads: bool,
    },
    Running(RunningPump),
    /// small body that was read along w/ the head ~ no pump needed
    Buffered(Option<Bytes>),
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
            ..
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
            Self::Buffered(bytes) => bytes.take().map(Ok),
            _ => None,
        }
    }

    #[inline]
    fn try_recv(&mut self, close_sig: &CloseSignal) -> Result<ResponseItem, TryRecvError> {
        if close_sig.is_closed() {
            return Err(TryRecvError::Disconnected);
        }
        if let Poll::Ready(next) = self.poll_inline() {
            return next.ok_or(TryRecvError::Disconnected);
        }
        self.start();
        match self {
            Self::Running(pump) => pump.rx.try_recv(),
            Self::Buffered(bytes) => bytes.take().map(Ok).ok_or(TryRecvError::Disconnected),
            _ => Err(TryRecvError::Disconnected),
        }
    }

    /// ready frame w/o spawning the pump
    ///
    /// enter the rt b/c reqwest's read timeout makes a
    /// tokio timer (panics outside a rt) causing confusing af errors
    ///
    /// (this was tricky to figure out)
    #[inline]
    fn poll_inline(&mut self) -> Poll<Option<ResponseItem>> {
        use futures_util::FutureExt;

        let Self::Idle {
            body,
            inline_reads: true,
            ..
        } = self
        else {
            return Poll::Pending;
        };
        let _rt = get_tokio_runtime().enter();
        let Some(next) = body.next().now_or_never() else {
            return Poll::Pending;
        };
        // err/end -> closed (same as the pump)
        if !matches!(next, Some(Ok(_))) {
            *self = Self::Closed;
        }
        Poll::Ready(next)
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
        use http_body::Body;

        // exact len == not-being-decompressed (ie not really doing any cpu-ing)
        let inline_reads = min_read_size == 0 && body.size_hint().exact().is_some();
        let rx = ResponseRx::Idle {
            body: BodyStream::new(body),
            min_read_size,
            inline_reads,
        };
        Self {
            status,
            shared: Arc::new(ResponseShared {
                rx: Mutex::new(rx),
                close_sig: CloseSignal::default(),
            }),
        }
    }

    fn from_bytes(status: StatusCode, bytes: Bytes) -> Self {
        // empty body == nothing to yield (same as the pump)
        let rx = ResponseRx::Buffered((!bytes.is_empty()).then_some(bytes));
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
    mut stream: BodyStream,
    min_read_size: usize,
    tx: mpsc::Sender<ResponseItem>,
) {
    // no collection/copying w/ 0
    if min_read_size == 0 {
        while let Some(item) = stream.next().await {
            let is_err = item.is_err();
            if tx.send(item).await.is_err() || is_err {
                return;
            }
        }
        return;
    }
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

#[pyclass(name = "ResponseStream", frozen, immutable_type)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
pub struct RyResponseStream(ResponseStreamInner);

impl RyResponseStream {
    pub(crate) fn from_body(status: StatusCode, body: reqwest::Body, min_read_size: usize) -> Self {
        let inner = ResponseStreamInner::from_body(status, body, min_read_size);
        Self(inner)
    }

    pub(crate) fn from_bytes(status: StatusCode, bytes: Bytes) -> Self {
        Self(ResponseStreamInner::from_bytes(status, bytes))
    }
}

impl std::fmt::Display for RyResponseStream {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ResponseStream<{}>", self.0.status)
    }
}

#[pymethods]
impl RyResponseStream {
    fn close<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, RyCoroutine>> {
        let inner = self.0.clone();
        pyloop_coro(py, "ResponseStream.close", async move {
            inner.close().await;
            Ok(())
        })
    }

    fn __repr__(&self) -> String {
        format!("{self}")
    }

    fn __aenter__(slf: Py<Self>, py: Python<'_>) -> PyResult<Bound<'_, RyCoroutine>> {
        slf.get().0.start();
        done_coro(py, "ResponseStream.__aenter__", slf)
    }

    #[pyo3(name = "__aexit__")]
    fn __aexit__<'py>(
        &self,
        py: Python<'py>,
        _exc_type: &Bound<'py, PyAny>,
        _exc_value: &Bound<'py, PyAny>,
        _traceback: &Bound<'py, PyAny>,
    ) -> PyResult<Bound<'py, RyCoroutine>> {
        self.close(py)
    }

    fn __aiter__(this: PyRef<Self>) -> PyRef<Self> {
        this
    }

    fn __anext__<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, RyCoroutine>> {
        const QUALNAME: &str = "ResponseStream.__anext__";
        // chunk (or the end) is already here ~ no pause/suspend
        if let Some(res) = self.0.try_next() {
            return match py_anext_result(res) {
                Ok(bytes) => done_coro(py, QUALNAME, bytes),
                Err(e) => failed_coro(py, QUALNAME, e),
            };
        }
        let inner = self.0.clone();
        // the pump task does the io; this is just a channel recv
        pyloop_coro(
            py,
            QUALNAME,
            async move { py_anext_result(inner.next().await) },
        )
    }

    // readall/take/collect run on tokio (not the loop) ~ they copy/collect
    // the whole rest of the body
    fn readall<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, RyCoroutine>> {
        let inner = self.0.clone();
        tokio_coro(py, "ResponseStream.readall", async move {
            readall_bytes(&inner)
                .await
                .map(RyBytes::from)
                .map_err(map_reqwest_err)
        })
    }

    #[pyo3(signature = (n = 1))]
    fn take<'py>(&self, py: Python<'py>, n: usize) -> PyResult<Bound<'py, RyCoroutine>> {
        let inner = self.0.clone();
        tokio_coro(py, "ResponseStream.take", async move {
            take_chunks(&inner, n).await.map_err(map_reqwest_err)
        })
    }

    fn collect<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, RyCoroutine>> {
        let inner = self.0.clone();
        tokio_coro(py, "ResponseStream.collect", async move {
            collect_chunks(&inner).await.map_err(map_reqwest_err)
        })
    }
}

#[pyclass(name = "BlockingResponseStream", frozen, immutable_type)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
pub struct RyBlockingResponseStream(ResponseStreamInner);

impl RyBlockingResponseStream {
    pub(crate) fn from_body(status: StatusCode, body: reqwest::Body, min_read_size: usize) -> Self {
        Self(ResponseStreamInner::from_body(status, body, min_read_size))
    }

    pub(crate) fn from_bytes(status: StatusCode, bytes: Bytes) -> Self {
        Self(ResponseStreamInner::from_bytes(status, bytes))
    }
}

#[pymethods]
impl RyBlockingResponseStream {
    fn __repr__(&self) -> String {
        format!("BlockingResponseStream<{}>", self.0.status)
    }

    fn __enter__(slf: Py<Self>) -> Py<Self> {
        slf.get().0.start();
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
        py.detach(|| get_tokio_runtime().block_on(self.0.close()));
    }

    fn __iter__(this: PyRef<Self>) -> PyRef<Self> {
        this
    }

    fn __next__(&self, py: Python<'_>) -> PyResult<RyBytes> {
        let result = match self.0.try_next() {
            Some(result) => result,
            None => py.detach(|| get_tokio_runtime().block_on(self.0.next())),
        };
        match result {
            Ok(Some(bytes)) => Ok(RyBytes::from(bytes)),
            Ok(None) => Err(PyStopIteration::new_err("response-stream-end")),
            Err(e) => Err(map_reqwest_err(e)),
        }
    }

    #[pyo3(signature = (n = 1))]
    fn take(&self, py: Python<'_>, n: usize) -> PyResult<Vec<RyBytes>> {
        py.detach(|| get_tokio_runtime().block_on(take_chunks(&self.0, n)))
            .map_err(map_reqwest_err)
    }

    fn readall<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        py.detach(|| get_tokio_runtime().block_on(readall_bytes(&self.0)))
            .map(RyBytes::from)
            .map_err(map_reqwest_err)?
            .into_bound_py_any(py)
    }

    fn collect<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        py.detach(|| get_tokio_runtime().block_on(collect_chunks(&self.0)))
            .map_err(map_reqwest_err)?
            .into_bound_py_any(py)
    }
}

#[inline]
async fn take_chunks(
    inner: &ResponseStreamInner,
    n: usize,
) -> Result<Vec<RyBytes>, reqwest::Error> {
    let shared = &*inner.shared;
    let mut rx = shared.rx.lock().await;
    let mut chunks = Vec::new();
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
    let mut chunks = Vec::new();
    let mut total_size = 0;
    while let Some(item) = rx.recv(&shared.close_sig).await {
        let bytes = item?;
        total_size += bytes.len();
        chunks.push(bytes);
    }
    Ok(join_bytes_chunks(&mut chunks, total_size))
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
