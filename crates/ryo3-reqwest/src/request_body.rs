//! reqwest Body implementations for Python body types
//!
//! This was a huge pain in the ass to figure out but I think I got it.
//!
//! python iterables are pulled from off of the tokio workers (sync iterables
//! on the blocking pool, async iterables on the python event loop) and
//! handed to reqwest via channel
//!
//! async ones: the client primes the body on the loop thread before the
//! request goes to tokio ~ the pump is its own (eager `Py_3_12`) task so it
//! runs right there until the iterator actually has to wait on something
//! (generators over in-memory chunks never do, so the whole body is queued
//! before the request even goes out). a pump task left running when the body
//! is dropped gets cancelled via `LoopHandle::call_soon` ~ no gil grabbing on
//! tokio threads
use std::pin::Pin;
use std::sync::{Arc, Mutex, MutexGuard, PoisonError};
use std::task::{Context, Poll};

use bytes::Bytes;
use pyo3::exceptions::{PyAttributeError, PyStopAsyncIteration, PyStopIteration};
use pyo3::prelude::*;
use pyo3::types::PyIterator;
use ryo3_bytes::{ReadableBuffer, RyBytes};
use ryo3_macro_rules::{py_runtime_error, py_type_err, py_type_error};
use ryo3_tokio_rt::{LoopHandle, get_tokio_runtime, pyloop_coro};
use tokio::sync::mpsc;
use tokio::sync::mpsc::error::TrySendError;

/// capacity for the synchronous request body channel
const REQ_BODY_CHANNEL_CAP_SYNC: usize = 32;
/// capacity for the asynchronous request body channel (64 is sweet spot)
const REQ_BODY_CHANNEL_CAP_ASYNC: usize = 64;

type BodyItem = PyResult<Bytes>;

/// sync iterable body ~ pumped on the blocking pool once reqwest first polls it
pub(crate) enum PyBodySyncStream {
    Idle(Py<PyAny>),
    Running(mpsc::Receiver<BodyItem>),
}

/// async iterable body
pub(crate) struct PyBodyAsyncStream {
    rx: mpsc::Receiver<BodyItem>,
    src: Arc<AsyncBodySrc>,
    /// first poll checks that the body was primed
    polled: bool,
}
pub(crate) enum PyBodyStream {
    Sync(PyBodySyncStream),
    Async(PyBodyAsyncStream),
}

impl PyBodyStream {
    #[inline]
    pub(crate) fn is_async(&self) -> bool {
        matches!(self, Self::Async(_))
    }

    #[inline]
    pub(crate) fn primer(&self) -> Option<RequestBodyPrimer> {
        match self {
            Self::Async(s) => Some(RequestBodyPrimer(Arc::clone(&s.src))),
            Self::Sync(_) => None,
        }
    }
}

#[derive(Debug)]
pub(crate) enum PyBody {
    Stream(PyBodyStream),
    Bytes(RyBytes),
}

impl std::fmt::Debug for PyBodyStream {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Sync(_) => f.debug_struct("PyBodyStream::Sync").finish(),
            Self::Async(_) => f.debug_struct("PyBodyStream::Async").finish(),
        }
    }
}

#[inline]
fn extract_chunk(obj: &Bound<'_, PyAny>) -> BodyItem {
    obj.extract::<ReadableBuffer>()
        .map(ReadableBuffer::into_bytes)
}

fn pump_request_body_sync(iter: &Py<PyAny>, tx: &mpsc::Sender<BodyItem>) {
    loop {
        let full = Python::attach(|py| {
            loop {
                let item = match iter.call_method0(py, pyo3::intern!(py, "__next__")) {
                    Ok(obj) => extract_chunk(obj.bind(py)),
                    Err(e) if e.is_instance_of::<PyStopIteration>(py) => return None,
                    Err(e) => Err(e),
                };
                let is_err = item.is_err();
                match tx.try_send(item) {
                    Ok(()) if !is_err => {}
                    Err(TrySendError::Full(item)) => return Some(item),
                    _ => return None,
                }
            }
        });
        let Some(item) = full else { break };
        let is_err = item.is_err();
        if tx.blocking_send(item).is_err() || is_err {
            break;
        }
    }
}

/// The thing the pump pulls from
struct AsyncBodySrc {
    /// the python `__anext__` coroutine for the async iterator
    anext: Py<PyAny>,
    /// the running pump task + its loop (loop that awaited the req, not
    /// the one the body was made on)
    aio_task: Mutex<Option<(LoopHandle, Py<PyAny>)>>,
    /// the pump's end of the channel ~ taken by [`BodyPrimer::prime`]
    tx: Mutex<Option<mpsc::Sender<BodyItem>>>,
}

impl AsyncBodySrc {
    fn aio_task(&self) -> MutexGuard<'_, Option<(LoopHandle, Py<PyAny>)>> {
        self.aio_task.lock().unwrap_or_else(PoisonError::into_inner)
    }

    fn tx(&self) -> MutexGuard<'_, Option<mpsc::Sender<BodyItem>>> {
        self.tx.lock().unwrap_or_else(PoisonError::into_inner)
    }
}

/// starts an async body's pump (on the loop thread)
pub(crate) struct RequestBodyPrimer(Arc<AsyncBodySrc>);

impl RequestBodyPrimer {
    /// loop thread ~ call before the request is handed to tokio
    pub(crate) fn prime(self, py: Python<'_>) {
        let src = self.0;
        let Some(tx) = src.tx().take() else {
            return;
        };
        // loop is picked here (on first await) not when the body was made
        let event_loop = match LoopHandle::current(py) {
            Ok(event_loop) => event_loop,
            Err(e) => {
                let _ = tx.try_send(Err(e));
                return;
            }
        };
        let pump = RyRequestBodyPump {
            src: Arc::clone(&src),
            tx: Some(tx.clone()),
            awaiting: None,
        };
        let pump = match Bound::new(py, pump) {
            Ok(pump) => pump,
            Err(e) => {
                let _ = tx.try_send(Err(e));
                return;
            }
        };
        match spawn_pump(event_loop.event_loop(py), &pump) {
            // pump may have finished already (eager) ~ dont keep if done/dead
            Ok(task) => {
                if pump.borrow().tx.is_some() {
                    *src.aio_task() = Some((event_loop, task.unbind()));
                }
            }
            Err(e) => pump.borrow_mut().ofuk(py, &e),
        }
    }
}

/// the pump gets its own task (+ context) from its very first step so
/// contextvar tokens, `current_task()`, cancel scopes etc all line up; eager on
/// 3.12+ so a body that never waits is fully queued right here
fn spawn_pump<'py>(
    event_loop: &Bound<'py, PyAny>,
    pump: &Bound<'py, RyRequestBodyPump>,
) -> PyResult<Bound<'py, PyAny>> {
    cfg_select! {
        Py_3_12 => {
            use pyo3::sync::PyOnceLock;
            use pyo3::types::PyDict;
            static TASK: PyOnceLock<Py<PyAny>> = PyOnceLock::new();
            let py = event_loop.py();
            let kwargs = PyDict::new(py);
            kwargs.set_item(pyo3::intern!(py, "loop"), event_loop)?;
            kwargs.set_item(pyo3::intern!(py, "eager_start"), true)?;
            TASK.import(py, "asyncio", "Task")?
                .call((pump,), Some(&kwargs))
        }
        _ => event_loop.call_method1(pyo3::intern!(event_loop.py(), "create_task"), (pump,)),
    }
}

enum Awaiting {
    Pulling(Py<PyIterator>),
    Paused { iter: Py<PyIterator>, last: bool },
}

impl Awaiting {
    #[inline]
    fn await_iter(&self) -> &Py<PyIterator> {
        match self {
            Self::Pulling(iter) | Self::Paused { iter, .. } => iter,
        }
    }
}

#[inline]
fn py_type_name(ob: &Bound<'_, PyAny>) -> String {
    ob.get_type()
        .name()
        .map_or_else(|_| "<UNKNOWN>".to_string(), |n| n.to_string())
}

/// `awaitable.__await__()` w/ the same type errs python raises
fn await_iter_for<'py>(awaitable: &Bound<'py, PyAny>) -> PyResult<Bound<'py, PyIterator>> {
    let py = awaitable.py();
    let await_method = awaitable
        .getattr(pyo3::intern!(py, "__await__"))
        .map_err(|e| {
            if e.is_instance_of::<PyAttributeError>(py) {
                let type_name = py_type_name(awaitable);
                py_type_error!("__anext__ returned non-awaitable of type '{}'", type_name)
            } else {
                e
            }
        })?;
    // same check as python would do ~ iterable aint iterator
    await_method
        .call0()?
        .cast_into::<PyIterator>()
        .map_err(|e| {
            let inner_pyob = e.into_inner();
            let type_name = py_type_name(&inner_pyob);
            py_type_error!("__await__() returned non-iterator of type '{}'", type_name)
        })
}

/// pypy friendly version of [`pyo3::types::iterator::PySendResult`]
enum RySendResult<'py> {
    Next(Bound<'py, PyAny>),
    Return(Bound<'py, PyAny>),
}

/// hacky work around to [`pyo3::types::iterator::PySendResult`] not being pypy friendly
fn send_to<'py>(
    iter: &Bound<'py, PyIterator>,
    value: &Bound<'py, PyAny>,
) -> PyResult<RySendResult<'py>> {
    cfg_select! {
        // same cfg as pyop3's `PySendResult`
        all(not(PyPy), Py_3_10) => {
            use pyo3::types::iterator::PySendResult;
            Ok(match iter.send(value)? {
                PySendResult::Next(yielded) => RySendResult::Next(yielded),
                PySendResult::Return(returned) => RySendResult::Return(returned),
            })
        }
        _ => {
            let py = iter.py();
            // `None` is `__next__` for some reason on plain iters
            let res = if value.is_none() {
                iter.call_method0(pyo3::intern!(py, "__next__"))
            } else {
                iter.call_method1(pyo3::intern!(py, "send"), (value,))
            };
            match res {
                Ok(yielded) => Ok(RySendResult::Next(yielded)),
                Err(e) if e.is_instance_of::<PyStopIteration>(py) => Ok(RySendResult::Return(
                    e.value(py).getattr(pyo3::intern!(py, "value"))?,
                )),
                Err(e) => Err(e),
            }
        }
    }
}

/// a python "coroutine" for the whole body
///
/// this took a while to figure out, it is a weird imitation async iterator
/// coroutine based kinda on the coroutine that is exposed via the
/// `pyo3/experimental-async` feature
///
/// it forwards the inner iterator's awaitables instead of making a new task
/// for every chunk consumed
#[pyclass(name = "RequestBodyPump", immutable_type)]
struct RyRequestBodyPump {
    src: Arc<AsyncBodySrc>,
    tx: Option<mpsc::Sender<BodyItem>>,
    awaiting: Option<Awaiting>,
}

impl RyRequestBodyPump {
    #[inline]
    fn is_closed(&self) -> bool {
        self.tx.as_ref().is_none_or(mpsc::Sender::is_closed)
    }

    #[inline]
    fn close(&mut self) {
        self.awaiting.take();
        self.tx.take();
        self.src.aio_task().take();
    }

    #[cold]
    fn done(&mut self) -> PyResult<Py<PyAny>> {
        self.close();
        Err(PyStopIteration::new_err(()))
    }

    // forward any errs to the actual reqwest impl, so that dropping tx dont
    // look like a clean stream end
    #[cold]
    fn ofuk(&mut self, py: Python<'_>, e: &PyErr) {
        if let Some(tx) = self.tx.take()
            && let Err(TrySendError::Full(item)) = tx.try_send(Err(e.clone_ref(py)))
        {
            get_tokio_runtime().spawn(async move {
                let _ = tx.send(item).await;
            });
        }
        self.close();
    }

    #[inline]
    fn fwd_pyerr(&mut self, py: Python<'_>, res: PyResult<Py<PyAny>>) -> PyResult<Py<PyAny>> {
        if let Err(e) = &res
            && !e.is_instance_of::<PyStopIteration>(py)
        {
            self.ofuk(py, e);
        }
        res
    }

    fn wait_for_capacity(
        &mut self,
        py: Python<'_>,
        tx: mpsc::Sender<BodyItem>,
        item: BodyItem,
        last: bool,
    ) -> PyResult<()> {
        // false => the rx is gone/dropped
        let coro = pyloop_coro(py, "RequestBodyPump.wait_for_capacity", async move {
            Ok(tx.send(item).await.is_ok())
        })?;
        self.awaiting = Some(Awaiting::Paused {
            // our own coroutine is an iterator so use that
            iter: coro.into_any().cast_into::<PyIterator>()?.unbind(),
            last,
        });
        Ok(())
    }

    // true => pull pull pull w/in same py task thingy
    fn queue(&mut self, py: Python<'_>, item: BodyItem) -> PyResult<bool> {
        let last = item.is_err();
        let Some(tx) = self.tx.clone() else {
            return Ok(false);
        };
        match tx.try_send(item) {
            Ok(()) if last => Ok(false),
            Ok(()) => Ok(true),
            Err(TrySendError::Full(item)) => {
                self.wait_for_capacity(py, tx, item, last)?;
                Ok(true)
            }
            Err(TrySendError::Closed(_)) => Ok(false),
        }
    }

    // true => pull pull pull
    fn on_return(
        &mut self,
        py: Python<'_>,
        awaiting: &Awaiting,
        value: &Bound<'_, PyAny>,
    ) -> PyResult<bool> {
        match *awaiting {
            Awaiting::Pulling(_) => self.queue(py, extract_chunk(value)),
            Awaiting::Paused { last, .. } => Ok(!last && value.is_truthy()?),
        }
    }

    fn start_anext(&mut self, py: Python<'_>) -> PyResult<bool> {
        let anext = self.src.anext.bind(py);
        match anext.call0().and_then(|a| await_iter_for(&a)) {
            Ok(iter) => {
                self.awaiting = Some(Awaiting::Pulling(iter.unbind()));
                Ok(true)
            }
            Err(e) if e.is_instance_of::<PyStopAsyncIteration>(py) => Ok(false),
            Err(e) => self.queue(py, Err(e)),
        }
    }

    fn advance<'py>(&mut self, py: Python<'py>, value: &Bound<'py, PyAny>) -> PyResult<Py<PyAny>> {
        if self.tx.is_none() {
            return Err(py_runtime_error!("cannot reuse already awaited coroutine"));
        }
        let mut sent = value.clone();
        loop {
            // we done?
            if self.is_closed() {
                return self.done();
            }
            let Some(awaiting) = self.awaiting.take() else {
                if !self.start_anext(py)? {
                    return self.done();
                }
                sent = py.None().into_bound(py);
                continue;
            };
            let result = send_to(awaiting.await_iter().bind(py), &sent);
            match result {
                Ok(RySendResult::Next(yielded)) => {
                    self.awaiting = Some(awaiting);
                    return Ok(yielded.unbind());
                }
                Ok(RySendResult::Return(value)) => {
                    if !self.on_return(py, &awaiting, &value)? {
                        return self.done();
                    }
                }
                Err(e) if e.is_instance_of::<PyStopAsyncIteration>(py) => {
                    return self.done();
                }
                Err(e) => {
                    if !self.queue(py, Err(e))? {
                        return self.done();
                    }
                }
            }
            sent = py.None().into_bound(py);
        }
    }

    fn throw(&mut self, py: Python<'_>, exc: Py<PyAny>) -> PyResult<Py<PyAny>> {
        let Some(awaiting) = self.awaiting.take() else {
            return Err(PyErr::from_value(exc.into_bound(py)));
        };
        let result = awaiting
            .await_iter()
            .bind(py)
            .call_method1(pyo3::intern!(py, "throw"), (exc.bind(py),));
        match result {
            Ok(yielded) => {
                self.awaiting = Some(awaiting);
                Ok(yielded.unbind())
            }
            Err(e) if e.is_instance_of::<PyStopIteration>(py) => {
                let value = e.value(py).getattr(pyo3::intern!(py, "value"))?;
                if self.on_return(py, &awaiting, &value)? {
                    self.advance(py, py.None().bind(py))
                } else {
                    self.done()
                }
            }
            Err(e) if e.is_instance_of::<PyStopAsyncIteration>(py) => self.done(),
            Err(e) => Err(e),
        }
    }
}

#[pymethods]
impl RyRequestBodyPump {
    fn __await__(slf: Py<Self>) -> Py<Self> {
        slf
    }

    fn __next__(&mut self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        let res = self.advance(py, py.None().bind(py));
        self.fwd_pyerr(py, res)
    }

    #[pyo3(name = "send")]
    fn py_send(&mut self, py: Python<'_>, value: &Bound<'_, PyAny>) -> PyResult<Py<PyAny>> {
        let res = self.advance(py, value);
        self.fwd_pyerr(py, res)
    }

    #[pyo3(name = "throw")]
    fn py_throw(&mut self, py: Python<'_>, exc: Py<PyAny>) -> PyResult<Py<PyAny>> {
        let res = self.throw(py, exc);
        self.fwd_pyerr(py, res)
    }

    #[pyo3(name = "close")]
    fn py_close(&mut self) {
        self.close();
    }

    /// pyo3 cleanup/gc
    fn __traverse__(&self, visit: pyo3::PyVisit<'_>) -> Result<(), pyo3::PyTraverseError> {
        if let Some(awaiting) = &self.awaiting {
            visit.call(awaiting.await_iter())?;
        }
        Ok(())
    }

    fn __clear__(&mut self) {
        self.close();
    }
}

impl futures_core::Stream for PyBodySyncStream {
    type Item = BodyItem;

    #[inline]
    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.get_mut();
        if let Self::Idle(_) = this {
            let (tx, rx) = mpsc::channel(REQ_BODY_CHANNEL_CAP_SYNC);
            if let Self::Idle(iter) = std::mem::replace(this, Self::Running(rx)) {
                get_tokio_runtime().spawn_blocking(move || pump_request_body_sync(&iter, &tx));
            }
        }
        let Self::Running(rx) = this else {
            unreachable!("started right above")
        };
        rx.poll_recv(cx)
    }
}

impl futures_core::Stream for PyBodyAsyncStream {
    type Item = BodyItem;

    #[inline]
    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if !self.polled {
            self.polled = true;
            // never primed => nobody will ever send; err instead of hanging
            if self.src.tx().take().is_some() {
                return Poll::Ready(Some(Err(py_runtime_error!(
                    "async request body was never started"
                ))));
            }
        }
        self.rx.poll_recv(cx)
    }
}

impl Drop for PyBodyAsyncStream {
    fn drop(&mut self) {
        self.rx.close();
        // dropped mid `__anext__` ~ cancel it over on the loop (no gil here)
        let Some((event_loop, task)) = self.src.aio_task().take() else {
            return;
        };
        event_loop.call_soon(move |py| {
            let _ = task.call_method0(py, pyo3::intern!(py, "cancel"));
        });
    }
}

impl PyBody {
    #[inline]
    fn sync_stream(iter: Py<PyAny>) -> Self {
        Self::Stream(PyBodyStream::Sync(PyBodySyncStream::Idle(iter)))
    }

    #[inline]
    fn async_stream(aiter: &Bound<'_, PyAny>) -> PyResult<Self> {
        let py = aiter.py();
        let anext = aiter.getattr(pyo3::intern!(py, "__anext__"))?.unbind();
        let (tx, rx) = mpsc::channel(REQ_BODY_CHANNEL_CAP_ASYNC);
        let src = Arc::new(AsyncBodySrc {
            anext,
            aio_task: Mutex::new(None),
            tx: Mutex::new(Some(tx)),
        });
        Ok(Self::Stream(PyBodyStream::Async(PyBodyAsyncStream {
            rx,
            src,
            polled: false,
        })))
    }
}

impl<'py> FromPyObject<'_, 'py> for PyBody {
    type Error = PyErr;

    #[inline]
    fn extract(obj: Borrowed<'_, 'py, PyAny>) -> Result<Self, Self::Error> {
        let py = obj.py();
        // TODO: dedupe these interned strings
        if let Ok(buffer) = obj.extract::<ReadableBuffer>() {
            Ok(Self::Bytes(buffer.into_rybytes()))
        } else if obj.hasattr(pyo3::intern!(py, "__aiter__"))? {
            let inner_iter = obj.call_method0(pyo3::intern!(py, "__aiter__"))?;
            Self::async_stream(&inner_iter)
        } else if obj.hasattr(pyo3::intern!(py, "__anext__"))? {
            Self::async_stream(&obj)
        } else if obj.hasattr(pyo3::intern!(py, "__iter__"))? {
            let iter_obj = obj.call_method0(pyo3::intern!(py, "__iter__"))?;
            Ok(Self::sync_stream(iter_obj.unbind()))
        } else if obj.hasattr(pyo3::intern!(py, "__next__"))? {
            Ok(Self::sync_stream(obj.to_owned().unbind()))
        } else {
            py_type_err!("Expected bytes-like object or an async or sync iterable for request body")
        }
    }
}

// ----------------------------------------------------------------------------
// INTO-BODY
// ----------------------------------------------------------------------------
impl From<PyBodyAsyncStream> for reqwest::Body {
    #[inline]
    fn from(val: PyBodyAsyncStream) -> Self {
        Self::wrap_stream(val)
    }
}

impl From<PyBodySyncStream> for reqwest::Body {
    #[inline]
    fn from(val: PyBodySyncStream) -> Self {
        Self::wrap_stream(val)
    }
}

impl From<PyBodyStream> for reqwest::Body {
    #[inline]
    fn from(val: PyBodyStream) -> Self {
        match val {
            PyBodyStream::Sync(s) => s.into(),
            PyBodyStream::Async(s) => s.into(),
        }
    }
}

impl From<PyBody> for reqwest::Body {
    #[inline]
    fn from(val: PyBody) -> Self {
        match val {
            PyBody::Bytes(b) => Self::from(b.into_inner()),
            PyBody::Stream(s) => s.into(),
        }
    }
}
