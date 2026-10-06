//! reqwest Body implementations for Python body types
//!
//! This was a huge pain in the ass to figure out but I think I got it.
//!
//! python iterables are pulled from off of the tokio workers (sync iterables
//! on the blocking pool, async iterables on the python event loop) and
//! handed to reqwest via channel
use std::pin::Pin;
use std::sync::{Mutex, MutexGuard, PoisonError};
use std::task::{Context, Poll};

use bytes::Bytes;
use pyo3::exceptions::{PyStopAsyncIteration, PyStopIteration};
use pyo3::prelude::*;
use pyo3::sync::PyOnceLock;
use pyo3::types::PyDict;
use pyo3_async_runtimes::TaskLocals;
use ryo3_bytes::{ReadableBuffer, RyBytes};
use ryo3_macro_rules::py_type_err;
use ryo3_tokio_rt::get_tokio_runtime;
use tokio::sync::mpsc;
use tokio::sync::mpsc::error::TrySendError;

const BODY_CHANNEL_CAPACITY: usize = 32;

type BodyItem = PyResult<Bytes>;

enum BodyRx<S> {
    Idle(S),
    Running(mpsc::Receiver<BodyItem>),
}

impl<S> BodyRx<S> {
    #[inline]
    fn poll_recv(
        &mut self,
        cx: &mut Context<'_>,
        spawn: impl FnOnce(S, mpsc::Sender<BodyItem>),
    ) -> Poll<Option<BodyItem>> {
        if let Self::Idle(_) = self {
            let (tx, rx) = mpsc::channel(BODY_CHANNEL_CAPACITY);
            if let Self::Idle(src) = std::mem::replace(self, Self::Running(rx)) {
                spawn(src, tx);
            }
        }
        match self {
            Self::Running(rx) => rx.poll_recv(cx),
            Self::Idle(_) => Poll::Ready(None),
        }
    }
}

type PyBodySyncRx = BodyRx<Py<PyAny>>;
type PyBodyAsyncRx = BodyRx<(Py<PyAny>, TaskLocals)>;
pub(crate) struct PyBodySyncStream(PyBodySyncRx);
pub(crate) struct PyBodyAsyncStream(PyBodyAsyncRx);
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

fn produce_sync(iter: &Py<PyAny>, tx: &mpsc::Sender<BodyItem>) {
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

// runs on the python event loop as its own done-callback
#[pyclass(frozen)]
struct AsyncBodyPump {
    anext: Py<PyAny>,
    locals: TaskLocals,
    tx: Mutex<Option<mpsc::Sender<BodyItem>>>,
}

impl AsyncBodyPump {
    fn tx(&self) -> MutexGuard<'_, Option<mpsc::Sender<BodyItem>>> {
        self.tx.lock().unwrap_or_else(PoisonError::into_inner)
    }

    fn close(&self) {
        self.tx().take();
    }

    fn schedule(slf: &Bound<'_, Self>) -> PyResult<()> {
        let py = slf.py();
        let locals = &slf.get().locals;
        let kwargs = PyDict::new(py);
        kwargs.set_item(pyo3::intern!(py, "context"), locals.context(py))?;
        locals.event_loop(py).call_method(
            pyo3::intern!(py, "call_soon_threadsafe"),
            (slf,),
            Some(&kwargs),
        )?;
        Ok(())
    }

    fn pull(slf: &Bound<'_, Self>) -> PyResult<()> {
        let py = slf.py();
        let fut = ensure_future_pyfn(py)?.call1((slf.get().anext.bind(py).call0()?,))?;
        fut.call_method1(pyo3::intern!(py, "add_done_callback"), (slf,))?;
        Ok(())
    }

    // true => pull the next chunk
    fn send(slf: &Bound<'_, Self>, tx: mpsc::Sender<BodyItem>, item: BodyItem) -> bool {
        let is_err = item.is_err();
        match tx.try_send(item) {
            Ok(()) if !is_err => return true,
            Err(TrySendError::Full(item)) => {
                let pump = slf.clone().unbind();
                get_tokio_runtime().spawn(async move {
                    let sent = tx.send(item).await.is_ok();
                    Python::attach(|py| {
                        let pump = pump.into_bound(py);
                        if !sent || is_err || Self::schedule(&pump).is_err() {
                            pump.get().close();
                        }
                    });
                });
            }
            _ => slf.get().close(),
        }
        false
    }
}

#[pymethods]
impl AsyncBodyPump {
    #[pyo3(signature = (fut = None))]
    fn __call__(slf: &Bound<'_, Self>, fut: Option<&Bound<'_, PyAny>>) {
        let py = slf.py();
        let Some(tx) = slf.get().tx().clone() else {
            return;
        };
        if tx.is_closed() {
            return slf.get().close();
        }
        if let Some(fut) = fut {
            let item = match fut.call_method0(pyo3::intern!(py, "result")) {
                Ok(obj) => extract_chunk(&obj),
                Err(e) if e.is_instance_of::<PyStopAsyncIteration>(py) => {
                    return slf.get().close();
                }
                Err(e) => Err(e),
            };
            if !Self::send(slf, tx.clone(), item) {
                return;
            }
        }
        if let Err(e) = Self::pull(slf) {
            Self::send(slf, tx, Err(e));
        }
    }
}

fn start_async_pump(anext: Py<PyAny>, locals: TaskLocals, tx: &mpsc::Sender<BodyItem>) {
    Python::attach(|py| {
        let pump = AsyncBodyPump {
            anext,
            locals,
            tx: Mutex::new(Some(tx.clone())),
        };
        if let Err(e) = Bound::new(py, pump).and_then(|pump| AsyncBodyPump::schedule(&pump)) {
            let _ = tx.try_send(Err(e));
        }
    });
}

impl futures_core::Stream for PyBodySyncStream {
    type Item = BodyItem;

    #[inline]
    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.0.poll_recv(cx, |iter, tx| {
            get_tokio_runtime().spawn_blocking(move || produce_sync(&iter, &tx));
        })
    }
}

impl futures_core::Stream for PyBodyAsyncStream {
    type Item = BodyItem;

    #[inline]
    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.0.poll_recv(cx, |(anext, locals), tx| {
            get_tokio_runtime().spawn_blocking(move || start_async_pump(anext, locals, &tx));
        })
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
        Ok(Self::Stream(PyBodyStream::Async(PyBodyAsyncStream(
            PyBodyAsyncRx::Idle((anext, locals)),
        ))))
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
