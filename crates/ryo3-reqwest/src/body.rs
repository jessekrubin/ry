//! reqwest Body implementations for Python body types
//!
//! This was a huge pain in the ass to figure out but I think I got it.
//!
//! python iterables are pulled from off of the tokio workers (sync iterables
//! on the blocking pool, async iterables on the python event loop) and
//! handed to reqwest via channel
use std::pin::Pin;
use std::sync::{Arc, Mutex, MutexGuard, PoisonError};
use std::task::{Context, Poll};

use bytes::Bytes;
use pyo3::exceptions::{PyAttributeError, PyStopAsyncIteration, PyStopIteration};
use pyo3::prelude::*;
use pyo3::sync::PyOnceLock;
use pyo3::types::iterator::PySendResult;
use pyo3::types::{PyDict, PyIterator};
use pyo3_async_runtimes::TaskLocals;
use ryo3_bytes::{ReadableBuffer, RyBytes};
use ryo3_macro_rules::{py_runtime_error, py_type_err, py_type_error};
use ryo3_tokio_rt::get_tokio_runtime;
use tokio::sync::mpsc;
use tokio::sync::mpsc::error::TrySendError;

/// capacity for the synchronous request body channel
const REQ_BODY_CHANNEL_CAP_SYNC: usize = 32;
/// capacity for the asynchronous request body channel (64 is sweet spot)
const REQ_BODY_CHANNEL_CAP_ASYNC: usize = 64;

type BodyItem = PyResult<Bytes>;

enum RequestBodyRx<Src> {
    Idle(Src),
    Running(mpsc::Receiver<BodyItem>),
}

impl<Src> RequestBodyRx<Src> {
    #[inline]
    fn poll_recv(
        &mut self,
        cx: &mut Context<'_>,
        capacity: usize,
        spawn: impl FnOnce(Src, mpsc::Sender<BodyItem>),
    ) -> Poll<Option<BodyItem>> {
        if let Self::Idle(_) = self {
            let (tx, rx) = mpsc::channel(capacity);
            let Self::Idle(src) = std::mem::replace(self, Self::Running(rx)) else {
                unreachable!()
            };
            spawn(src, tx);
        }
        let Self::Running(rx) = self else {
            // can t get here bc we just replaced self with Self::Running(rx) above
            unreachable!()
        };
        rx.poll_recv(cx)
    }
}

type PyBodySyncRx = RequestBodyRx<Py<PyAny>>;
type PyBodyAsyncRx = RequestBodyRx<Arc<AsyncBodySrc>>;
pub(crate) struct PyBodySyncStream(PyBodySyncRx);
pub(crate) struct PyBodyAsyncStream {
    rx: PyBodyAsyncRx,
    src: Arc<AsyncBodySrc>,
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
    obj.extract::<ReadableBuffer>().map(|rb| rb.to_bytes())
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

fn ensure_future_pyfn(py: Python<'_>) -> PyResult<&Bound<'_, PyAny>> {
    static ENSURE_FUTURE: PyOnceLock<Py<PyAny>> = PyOnceLock::new();
    ENSURE_FUTURE.import(py, "asyncio", "ensure_future")
}

/// The thing `BodyRx` holds while idle, and the thing the pump pulls from
struct AsyncBodySrc {
    /// the python `__anext__` coroutine for the async iterator
    anext: Py<PyAny>,
    /// the aio task locals for the iterator
    locals: TaskLocals,
    /// the currently running aio task
    task: Mutex<Option<Py<PyAny>>>,
}

impl AsyncBodySrc {
    fn task(&self) -> MutexGuard<'_, Option<Py<PyAny>>> {
        self.task.lock().unwrap_or_else(PoisonError::into_inner)
    }

    fn call_soon<'py>(
        &self,
        py: Python<'py>,
        args: impl pyo3::call::PyCallArgs<'py>,
    ) -> PyResult<()> {
        let kwargs = PyDict::new(py);
        kwargs.set_item(pyo3::intern!(py, "context"), self.locals.context(py))?;
        self.locals.event_loop(py).call_method(
            pyo3::intern!(py, "call_soon_threadsafe"),
            args,
            Some(&kwargs),
        )?;
        Ok(())
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
        self.src.task().take();
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
        let fut = pyo3_async_runtimes::tokio::future_into_py_with_locals(
            py,
            self.src.locals.clone(),
            // the rx is gone/dropped
            async move { Ok(tx.send(item).await.is_ok()) },
        )?;
        self.awaiting = Some(Awaiting::Paused {
            iter: await_iter_for(&fut)?.unbind(),
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

    fn step<'py>(&mut self, py: Python<'py>, value: &Bound<'py, PyAny>) -> PyResult<Py<PyAny>> {
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
            let result = awaiting.await_iter().bind(py).send(&sent);
            match result {
                Ok(PySendResult::Next(yielded)) => {
                    self.awaiting = Some(awaiting);
                    return Ok(yielded.unbind());
                }
                Ok(PySendResult::Return(value)) => {
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
                    self.step(py, py.None().bind(py))
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
    fn __call__(slf: &Bound<'_, Self>) -> PyResult<()> {
        let py = slf.py();
        let src = {
            let pump = slf.borrow();
            if pump.is_closed() {
                return Ok(());
            }
            Arc::clone(&pump.src)
        };
        let task = match ensure_future_pyfn(py).and_then(|f| f.call1((slf,))) {
            Ok(task) => task,
            Err(e) => {
                slf.borrow_mut().ofuk(py, &e);
                return Err(e);
            }
        };
        // pump may have finished ~ dont keep if done/dead
        if slf.borrow().tx.is_some() {
            *src.task() = Some(task.unbind());
        }
        Ok(())
    }

    fn __await__(slf: Py<Self>) -> Py<Self> {
        slf
    }

    fn __next__(&mut self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        let res = self.step(py, py.None().bind(py));
        self.fwd_pyerr(py, res)
    }

    #[pyo3(name = "send")]
    fn py_send(&mut self, py: Python<'_>, value: &Bound<'_, PyAny>) -> PyResult<Py<PyAny>> {
        let res = self.step(py, value);
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
    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.0.poll_recv(cx, REQ_BODY_CHANNEL_CAP_SYNC, |iter, tx| {
            get_tokio_runtime().spawn_blocking(move || pump_request_body_sync(&iter, &tx));
        })
    }
}

fn start_request_body_pump(src: &Arc<AsyncBodySrc>, tx: &mpsc::Sender<BodyItem>) {
    Python::attach(|py| {
        let pump = RyRequestBodyPump {
            src: Arc::clone(src),
            tx: Some(tx.clone()),
            awaiting: None,
        };
        if let Err(e) = Bound::new(py, pump).and_then(|pump| src.call_soon(py, (pump,))) {
            let _ = tx.try_send(Err(e));
        }
    });
}

impl futures_core::Stream for PyBodyAsyncStream {
    type Item = BodyItem;

    #[inline]
    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.rx
            .poll_recv(cx, REQ_BODY_CHANNEL_CAP_ASYNC, |src, tx| {
                get_tokio_runtime().spawn_blocking(move || start_request_body_pump(&src, &tx));
            })
    }
}

impl Drop for PyBodyAsyncStream {
    fn drop(&mut self) {
        if let RequestBodyRx::Running(rx) = &mut self.rx {
            rx.close();
        }
        // dropped mid `__anext__` ~ cancel it (not on a tokio bc gil)
        let Some(task) = self.src.task().take() else {
            return;
        };
        let src = Arc::clone(&self.src);
        get_tokio_runtime().spawn_blocking(move || {
            Python::attach(move |py| {
                let _ = task
                    .getattr(py, pyo3::intern!(py, "cancel"))
                    .and_then(|cancel| src.call_soon(py, (cancel,)));
            });
        });
    }
}

impl PyBody {
    #[inline]
    fn sync_stream(iter: Py<PyAny>) -> Self {
        Self::Stream(PyBodyStream::Sync(PyBodySyncStream(PyBodySyncRx::Idle(
            iter,
        ))))
    }

    #[inline]
    fn async_stream(aiter: &Bound<'_, PyAny>) -> PyResult<Self> {
        let py = aiter.py();
        let locals = pyo3_async_runtimes::tokio::get_current_locals(py)?;
        let anext = aiter.getattr(pyo3::intern!(py, "__anext__"))?.unbind();
        let src = Arc::new(AsyncBodySrc {
            anext,
            locals,
            task: Mutex::new(None),
        });
        Ok(Self::Stream(PyBodyStream::Async(PyBodyAsyncStream {
            rx: PyBodyAsyncRx::Idle(Arc::clone(&src)),
            src,
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
            Ok(Self::Bytes(buffer.to_rybytes()))
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
