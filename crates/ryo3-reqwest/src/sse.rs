//! server-sent events (sse) via the response stream aparatus
//!
//! parsing is done by an adapted sse-decoder which is vendored from `sse-core` (see `decode.rs`);
//! this is just the glue to the response stream + python
//!
//! parse only ~ no auto-reconnect; the stream exposes `last_event_id` and
//! `retry` so the caller can reconnect w/ `Last-Event-ID` themselves
//!
//! REF: <https://html.spec.whatwg.org/multipage/server-sent-events.html#event-stream-interpretation>
use std::num::NonZeroUsize;
use std::sync::Arc;
use std::task::Poll;

use bytes::Bytes;
use decode::{MessageEvent, SseDecoder, SseEvent};
use pyo3::exceptions::{PyStopAsyncIteration, PyStopIteration, PyValueError};
use pyo3::prelude::*;
use reqwest::StatusCode;
use ryo3_bytes::RyBytes;
use ryo3_core::sync::RyMutex;
use ryo3_tokio_rt::{RyCoroutine, done_coro, failed_coro, get_tokio_runtime, pyloop_coro};

use crate::errors::map_reqwest_err;
use crate::response_stream::ResponseStreamInner;

mod decode;

/// default max event size 512KiB (same as sse-core)
pub(crate) const SSE_MAX_EVENT_SIZE: NonZeroUsize = NonZeroUsize::new(512 * 1024).expect("wenodis");

#[pyclass(name = "ServerSentEvent", frozen, immutable_type)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RyServerSentEvent(MessageEvent);

#[pymethods]
impl RyServerSentEvent {
    #[getter]
    fn event(&self) -> &str {
        &self.0.event
    }

    #[getter]
    fn data(&self) -> &str {
        self.0.data.as_str()
    }

    /// payload as `ry.Bytes` (zero-copy)
    #[getter]
    fn bytes(&self) -> RyBytes {
        RyBytes::from(self.0.data.as_bytes().clone())
    }

    #[getter]
    fn id(&self) -> Option<&str> {
        self.0.last_event_id.as_deref()
    }

    /// Parse the payload as JSON
    #[pyo3(
        signature = (
            *,
            allow_inf_nan = false,
            cache_mode = ryo3_jiter::StringCacheMode::All,
            partial_mode = ryo3_jiter::PartialMode::Off,
            catch_duplicate_keys = false,
        ),
        text_signature = "(self, *, allow_inf_nan=False, cache_mode=\"all\", partial_mode=False, catch_duplicate_keys=False)"
    )]
    fn json<'py>(
        &self,
        py: Python<'py>,
        allow_inf_nan: bool,
        cache_mode: ryo3_jiter::StringCacheMode,
        partial_mode: ryo3_jiter::PartialMode,
        catch_duplicate_keys: bool,
    ) -> Result<Bound<'py, PyAny>, ryo3_jiter::RyJSONDecodeError> {
        let options = ryo3_jiter::JiterParseOptions::new()
            .with_allow_inf_nan(allow_inf_nan)
            .with_cache_mode(cache_mode)
            .with_partial_mode(partial_mode)
            .with_catch_duplicate_keys(catch_duplicate_keys);
        options.parse(py, self.0.data.as_bytes())
    }

    fn __eq__(&self, other: &Self) -> bool {
        self == other
    }

    fn __repr__(&self) -> String {
        let MessageEvent {
            event,
            data,
            last_event_id,
        } = &self.0;
        let id = last_event_id
            .as_deref()
            .map_or_else(|| "None".to_string(), |id| format!("{id:?}"));
        let data = data.as_str();
        format!("ServerSentEvent(event={event:?}, data={data:?}, id={id})")
    }
}

enum SseError {
    Reqwest(reqwest::Error),
    TooLarge,
}

impl From<SseError> for PyErr {
    fn from(e: SseError) -> Self {
        match e {
            SseError::Reqwest(e) => map_reqwest_err(e),
            SseError::TooLarge => PyValueError::new_err("sse event exceeded max_event_size"),
        }
    }
}

type SseItem = Result<Option<RyServerSentEvent>, SseError>;

struct SseParser {
    decoder: SseDecoder,
    /// not-yet-decoded rest of the current chunk
    chunk: Bytes,
    retry: Option<u32>,
    eof: bool,
}

impl SseParser {
    fn new(max_event_size: NonZeroUsize) -> Self {
        Self {
            decoder: SseDecoder::with_limit(max_event_size),
            chunk: Bytes::new(),
            retry: None,
            eof: false,
        }
    }

    /// next event from what we have; pending == need another chunk
    fn poll_event(&mut self) -> Poll<SseItem> {
        loop {
            if self.eof {
                // an unfinished event at the end is dropped (per spec)
                return Poll::Ready(Ok(None));
            }
            match self.decoder.next(&mut self.chunk) {
                Some(Ok(SseEvent::Message(msg))) => {
                    return Poll::Ready(Ok(Some(RyServerSentEvent(msg))));
                }
                Some(Ok(SseEvent::Retry(ms))) => self.retry = Some(ms),
                Some(Err(_)) => return Poll::Ready(Err(SseError::TooLarge)),
                None => return Poll::Pending,
            }
        }
    }

    fn feed(&mut self, chunk: Option<Bytes>) {
        match chunk {
            Some(chunk) => self.chunk = chunk,
            None => self.eof = true,
        }
    }
}

struct SseShared {
    /// held across a whole read so chunks get fed in order
    read: tokio::sync::Mutex<()>,
    parser: RyMutex<SseParser, false>,
}

#[derive(Clone)]
struct EventStreamInner {
    status: StatusCode,
    body: ResponseStreamInner,
    shared: Arc<SseShared>,
}

impl EventStreamInner {
    fn new(status: StatusCode, body: ResponseStreamInner, max_event_size: NonZeroUsize) -> Self {
        Self {
            status,
            body,
            shared: Arc::new(SseShared {
                read: tokio::sync::Mutex::new(()),
                parser: RyMutex::new(SseParser::new(max_event_size)),
            }),
        }
    }

    #[inline]
    fn poll_event(&self) -> Poll<SseItem> {
        self.shared.parser.py_lock().poll_event()
    }

    #[inline]
    fn feed(&self, chunk: Option<Bytes>) {
        self.shared.parser.py_lock().feed(chunk);
    }

    fn try_next(&self) -> Option<SseItem> {
        let _read = self.shared.read.try_lock().ok()?;
        loop {
            if let Poll::Ready(item) = self.poll_event() {
                return Some(item);
            }
            match self.body.try_next()? {
                Ok(chunk) => self.feed(chunk),
                Err(e) => return Some(Err(SseError::Reqwest(e))),
            }
        }
    }

    async fn next(&self) -> SseItem {
        let _read = self.shared.read.lock().await;
        loop {
            if let Poll::Ready(item) = self.poll_event() {
                return item;
            }
            let chunk = self.body.next().await.map_err(SseError::Reqwest)?;
            self.feed(chunk);
        }
    }

    async fn close(&self) {
        self.body.close().await;
        self.feed(None);
    }

    fn last_event_id(&self) -> Option<String> {
        let parser = self.shared.parser.py_lock();
        parser.decoder.last_event_id().map(ToString::to_string)
    }

    fn retry(&self) -> Option<u32> {
        self.shared.parser.py_lock().retry
    }
}

#[pyclass(name = "EventStream", frozen, immutable_type)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
pub struct RyEventStream(EventStreamInner);

impl std::fmt::Display for RyEventStream {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "EventStream<{}>", self.0.status)
    }
}

impl RyEventStream {
    pub(crate) fn new(
        status: StatusCode,
        body: ResponseStreamInner,
        max_event_size: NonZeroUsize,
    ) -> Self {
        Self(EventStreamInner::new(status, body, max_event_size))
    }
}

#[inline]
fn py_anext_result(res: SseItem) -> PyResult<RyServerSentEvent> {
    match res {
        Ok(Some(event)) => Ok(event),
        Ok(None) => Err(PyStopAsyncIteration::new_err("event-stream-end")),
        Err(e) => Err(e.into()),
    }
}

#[pymethods]
impl RyEventStream {
    fn __repr__(&self) -> String {
        format!("{self}")
    }

    #[getter]
    fn last_event_id(&self) -> Option<String> {
        self.0.last_event_id()
    }

    /// last `retry` (ms) the server sent
    #[getter]
    fn retry(&self) -> Option<u32> {
        self.0.retry()
    }

    fn close<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, RyCoroutine>> {
        let inner = self.0.clone();
        pyloop_coro(py, "EventStream.close", async move {
            inner.close().await;
            Ok(())
        })
    }

    fn __aenter__(slf: Py<Self>, py: Python<'_>) -> PyResult<Bound<'_, RyCoroutine>> {
        slf.get().0.body.start();
        done_coro(py, "EventStream.__aenter__", slf)
    }

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
        const QUALNAME: &str = "EventStream.__anext__";
        if let Some(res) = self.0.try_next() {
            return match py_anext_result(res) {
                Ok(event) => done_coro(py, QUALNAME, event),
                Err(e) => failed_coro(py, QUALNAME, e),
            };
        }
        let inner = self.0.clone();
        pyloop_coro(
            py,
            QUALNAME,
            async move { py_anext_result(inner.next().await) },
        )
    }
}

#[pyclass(name = "BlockingEventStream", frozen, immutable_type)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
pub struct RyBlockingEventStream(EventStreamInner);

impl std::fmt::Display for RyBlockingEventStream {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BlockingEventStream<{}>", self.0.status)
    }
}

impl RyBlockingEventStream {
    pub(crate) fn new(
        status: StatusCode,
        body: ResponseStreamInner,
        max_event_size: NonZeroUsize,
    ) -> Self {
        Self(EventStreamInner::new(status, body, max_event_size))
    }
}

#[pymethods]
impl RyBlockingEventStream {
    fn __repr__(&self) -> String {
        format!("{self}")
    }

    #[getter]
    fn last_event_id(&self) -> Option<String> {
        self.0.last_event_id()
    }

    /// last `retry` (ms) the server sent
    #[getter]
    fn retry(&self) -> Option<u32> {
        self.0.retry()
    }

    fn close(&self, py: Python<'_>) {
        py.detach(|| get_tokio_runtime().block_on(self.0.close()));
    }

    fn __enter__(slf: Py<Self>) -> Py<Self> {
        slf.get().0.body.start();
        slf
    }

    fn __exit__(
        &self,
        py: Python<'_>,
        _exc_type: &Bound<'_, PyAny>,
        _exc_value: &Bound<'_, PyAny>,
        _traceback: &Bound<'_, PyAny>,
    ) {
        self.close(py);
    }

    fn __iter__(this: PyRef<Self>) -> PyRef<Self> {
        this
    }

    fn __next__(&self, py: Python<'_>) -> PyResult<RyServerSentEvent> {
        let res = match self.0.try_next() {
            Some(res) => res,
            None => py.detach(|| get_tokio_runtime().block_on(self.0.next())),
        };
        match res {
            Ok(Some(event)) => Ok(event),
            Ok(None) => Err(PyStopIteration::new_err("event-stream-end")),
            Err(e) => Err(e.into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parser(max: usize) -> SseParser {
        SseParser::new(NonZeroUsize::new(max).expect("non-zero"))
    }

    fn events_per_chunk(p: &mut SseParser, chunks: &[&'static [u8]]) -> Vec<Vec<String>> {
        chunks
            .iter()
            .map(|c| {
                p.feed(Some(Bytes::from_static(c)));
                let mut got = vec![];
                while let Poll::Ready(Ok(Some(ev))) = p.poll_event() {
                    got.push(ev.0.data.to_string());
                }
                got
            })
            .collect()
    }

    #[test]
    fn test_trailing_cr_does_not_stall() {
        let mut p = parser(SSE_MAX_EVENT_SIZE.get());
        let got = events_per_chunk(&mut p, &[b"data: a\r\n\r", b"\n", b"data: b\r\r"]);
        assert_eq!(got, [vec!["a"], vec![], vec!["b"]]);
    }

    #[test]
    fn test_retry_and_id() {
        let mut p = parser(SSE_MAX_EVENT_SIZE.get());
        let got = events_per_chunk(&mut p, &[b"retry: 10\nid: 7\ndata: x\n\n"]);
        assert_eq!(got, [vec!["x"]]);
        assert_eq!(p.retry, Some(10));
        assert_eq!(p.decoder.last_event_id().map(|s| &**s), Some("7"));
    }

    #[test]
    fn test_too_large_then_recovers() {
        let mut p = parser(8);
        p.feed(Some(Bytes::from_static(
            b"data: waaaaaaaay too big\n\ndata: ok\n\n",
        )));
        assert!(matches!(
            p.poll_event(),
            Poll::Ready(Err(SseError::TooLarge))
        ));
        assert!(matches!(
            p.poll_event(),
            Poll::Ready(Ok(Some(ev))) if &*ev.0.data == "ok"
        ));
    }

    #[test]
    fn test_eof_drops_partial() {
        let mut p = parser(SSE_MAX_EVENT_SIZE.get());
        let got = events_per_chunk(&mut p, &[b"data: a\n\ndata: b\n"]);
        assert_eq!(got, [vec!["a"]]);
        p.feed(None);
        assert!(matches!(p.poll_event(), Poll::Ready(Ok(None))));
    }
}
