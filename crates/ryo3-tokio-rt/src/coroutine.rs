//! python coroutine for rust futures + a gil-free loop wake
//!
//! tokio side of a wake: flip an atomic, push onto the loop's queue and (only
//! for the first wake of a burst) write one byte to a socketpair that the loop
//! watches w/ `add_reader`.
//!
//! loop side: readall from socket, run whole queue,
//! call the task's done-cbs right there. no gil on tokio threads + one loop
//! wakeup per burst
//!
//! loops w/o `add_reader` (windows proactor, ...) get `call_soon_threadsafe`
//! once per burst instead ~ that one does need the gil (bummer)

use std::cell::RefCell;
use std::future::Future;
#[cfg(unix)]
use std::io::{Read, Write};
#[cfg(unix)]
use std::os::fd::AsRawFd;
#[cfg(unix)]
use std::os::unix::net::UnixStream;
use std::pin::Pin;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, MutexGuard, OnceLock, PoisonError, TryLockError, Weak};
use std::task::{Context, Poll, Wake, Waker};

use pyo3::exceptions::asyncio::{CancelledError, InvalidStateError};
use pyo3::exceptions::{
    PyBaseException, PyKeyboardInterrupt, PyRuntimeError, PyStopIteration, PySystemExit,
};
use pyo3::prelude::*;
use pyo3::sync::PyOnceLock;
use pyo3::types::PyDict;
use pyo3::{IntoPyObjectExt, PyTraverseError, PyVisit};

use crate::get_ry_tokio_runtime;

const REUSED: &str = "cannot reuse already awaited coroutine";

#[inline]
fn lock<T>(mutex: &Mutex<T>) -> MutexGuard<'_, T> {
    mutex.lock().unwrap_or_else(PoisonError::into_inner)
}

fn empty_context(py: Python<'_>) -> PyResult<Bound<'_, PyAny>> {
    static CONTEXT: PyOnceLock<Py<PyAny>> = PyOnceLock::new();
    CONTEXT.import(py, "contextvars", "Context")?.call0()
}

fn call_soon<'py>(
    event_loop: &Bound<'py, PyAny>,
    callback: &Bound<'py, PyAny>,
    argument: &Bound<'py, PyAny>,
    context: Option<&Bound<'py, PyAny>>,
) -> PyResult<()> {
    let kwargs = context
        .map(|context| {
            let kwargs = PyDict::new(event_loop.py());
            kwargs.set_item(pyo3::intern!(event_loop.py(), "context"), context)?;
            Ok::<_, PyErr>(kwargs)
        })
        .transpose()?;
    event_loop.call_method(
        pyo3::intern!(event_loop.py(), "call_soon"),
        (callback, argument),
        kwargs.as_ref(),
    )?;
    Ok(())
}

type LoopCall = Box<dyn FnOnce(Python<'_>) + Send>;

enum LoopEvent {
    Wake(Arc<CoroutineWaker>),
    Call(LoopCall),
}

struct LoopHub {
    event_loop: Py<PyAny>,
    context: Py<PyAny>,
    flush: Py<FlushCallback>,
    #[cfg(unix)]
    wake_pair: Option<(UnixStream, UnixStream)>,
    ready: Mutex<Vec<LoopEvent>>,
    flush_scheduled: AtomicBool,
    closed: AtomicBool,
}

impl LoopHub {
    fn new(event_loop: &Bound<'_, PyAny>) -> PyResult<Arc<Self>> {
        let py = event_loop.py();
        let context = empty_context(py)?;
        let flush = Py::new(py, FlushCallback(OnceLock::new()))?;
        #[cfg(unix)]
        let wake_pair = register_reader(event_loop, &context, flush.bind(py));
        let hub = Arc::new(Self {
            event_loop: event_loop.clone().unbind(),
            context: context.unbind(),
            flush,
            #[cfg(unix)]
            wake_pair,
            ready: Mutex::new(Vec::new()),
            flush_scheduled: AtomicBool::new(false),
            closed: AtomicBool::new(false),
        });
        let _ = hub.flush.get().0.set(Arc::downgrade(&hub));
        Ok(hub)
    }

    #[inline]
    fn post_event(&self, event: LoopEvent) {
        if self.closed.load(Ordering::Acquire) {
            return;
        }
        {
            let mut ready = lock(&self.ready);
            // re-check under the lock so a race on `close()` dont strand event
            if self.closed.load(Ordering::Acquire) {
                return;
            }
            ready.push(event);
        }
        if !self.flush_scheduled.swap(true, Ordering::AcqRel) {
            self.poke();
        }
    }

    #[inline]
    fn poke(&self) {
        #[cfg(unix)]
        if let Some((_, tx)) = &self.wake_pair {
            // WouldBlock => a byte is already there so finna flush soon anyway
            let _ = (&*tx).write(&[1]);
            return;
        }
        self.poke_fallback();
    }

    #[cold]
    fn poke_fallback(&self) {
        Python::try_attach(|py| {
            let kwargs = PyDict::new(py);
            let result = kwargs
                .set_item(pyo3::intern!(py, "context"), self.context.bind(py))
                .and_then(|()| {
                    self.event_loop.bind(py).call_method(
                        pyo3::intern!(py, "call_soon_threadsafe"),
                        (self.flush.bind(py),),
                        Some(&kwargs),
                    )
                });
            if result.is_err() {
                self.close();
            }
        });
    }

    /// runs the whole batch
    ///
    /// `KeyboardInterrupt`/`SystemExit` oot gets handed back (after the rest
    /// of the batch) so it stops das loop how a plain asyncio handle would
    fn run_ready(&self, py: Python<'_>) -> PyResult<()> {
        #[cfg(unix)]
        if let Some((rx, _)) = &self.wake_pair {
            let mut bytes = [0; 256];
            loop {
                match (&*rx).read(&mut bytes) {
                    Ok(0) => break,
                    Ok(_) => {}
                    Err(error) if error.kind() == std::io::ErrorKind::Interrupted => {}
                    Err(_) => break,
                }
            }
        }

        // chill before taking the batch ~ a racing flush either lands in this
        // batch or pokes again (no lost wakes)
        self.flush_scheduled.store(false, Ordering::SeqCst);
        let batch = std::mem::take(&mut *lock(&self.ready));
        let mut fatal = None;
        for event in batch {
            match event {
                LoopEvent::Wake(waker) => {
                    if let Err(e) = waker.wake_task(py) {
                        fatal.get_or_insert(e);
                    }
                }
                LoopEvent::Call(call) => call(py),
            }
        }
        fatal.map_or(Ok(()), Err)
    }

    fn close(&self) {
        self.closed.store(true, Ordering::Release);
        lock(&self.ready).clear();
    }

    fn is_closed(&self, py: Python<'_>) -> bool {
        self.event_loop
            .bind(py)
            .call_method0(pyo3::intern!(py, "is_closed"))
            .and_then(|value| value.is_truthy())
            .unwrap_or(true)
    }
}

#[cfg(unix)]
fn register_reader(
    event_loop: &Bound<'_, PyAny>,
    context: &Bound<'_, PyAny>,
    flush: &Bound<'_, FlushCallback>,
) -> Option<(UnixStream, UnixStream)> {
    let py = event_loop.py();
    let (rx, tx) = UnixStream::pair().ok()?;
    rx.set_nonblocking(true).ok()?;
    tx.set_nonblocking(true).ok()?;
    let add_reader = event_loop.getattr(pyo3::intern!(py, "add_reader")).ok()?;
    context
        .call_method1(
            pyo3::intern!(py, "run"),
            (add_reader, rx.as_raw_fd(), flush),
        )
        .ok()?;
    Some((rx, tx))
}

#[pyclass(name = "CoroutineFlush", frozen, immutable_type)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
struct FlushCallback(OnceLock<Weak<LoopHub>>);

#[pymethods]
impl FlushCallback {
    fn __call__(&self, py: Python<'_>) -> PyResult<()> {
        match self.0.get().and_then(Weak::upgrade) {
            Some(hub) => hub.run_ready(py),
            None => Ok(()),
        }
    }
}

thread_local! {
    static LOOP_HUBS: RefCell<Vec<Arc<LoopHub>>> = const { RefCell::new(Vec::new()) };
}

/// handle to the running event loop on this thread (+ its wake hub)
#[derive(Clone)]
pub struct LoopHandle(Arc<LoopHub>);

impl LoopHandle {
    /// errs if there is no running loop
    pub fn current(py: Python<'_>) -> PyResult<Self> {
        static GET_RUNNING_LOOP: PyOnceLock<Py<PyAny>> = PyOnceLock::new();
        let event_loop = GET_RUNNING_LOOP
            .import(py, "asyncio", "get_running_loop")?
            .call0()?;
        if let Some(hub) = LOOP_HUBS.with(|hubs| {
            hubs.borrow()
                .iter()
                .find(|hub| hub.event_loop.as_ptr() == event_loop.as_ptr())
                .cloned()
        }) {
            return Ok(Self(hub));
        }

        // RefCell stays unborrowed while python runs (`is_closed`/`new` can
        // end up back in here)
        let mut hubs = LOOP_HUBS.with(RefCell::take);
        hubs.retain(|hub| {
            let open = !hub.is_closed(py);
            if !open {
                hub.close();
            }
            open
        });
        let hub = LoopHub::new(&event_loop);
        if let Ok(hub) = &hub {
            hubs.push(Arc::clone(hub));
        }
        // put em back even on err ~ wakers only hold a `Weak` to their hub
        LOOP_HUBS.with(|slot| {
            hubs.append(&mut slot.borrow_mut());
            *slot.borrow_mut() = hubs;
        });
        hub.map(Self)
    }

    /// the python event loop
    #[must_use]
    pub fn event_loop<'py>(&self, py: Python<'py>) -> &Bound<'py, PyAny> {
        self.0.event_loop.bind(py)
    }

    /// run `call` w/ the gil on the loop thread soon ~ callable from any thread
    /// (no gil needed); if the loop is gone `call` just gets dropped
    #[inline]
    pub fn call_soon<F>(&self, call: F)
    where
        F: FnOnce(Python<'_>) + Send + 'static,
    {
        self.0.post_event(LoopEvent::Call(Box::new(call)));
    }
}

#[derive(Clone, Copy, Default, PartialEq, Eq)]
enum WakeStatus {
    #[default]
    Pending,
    /// woken by tokio, or the future is done/dropped
    Woken,
    Cancelled,
}

type DoneCallback = (Py<PyAny>, Option<Py<PyAny>>);

#[derive(Default)]
struct WakeState {
    status: WakeStatus,
    py_future: Option<Py<RyCoroutineFuture>>,
    callbacks: Vec<DoneCallback>,
    cancel_message: Option<Py<PyAny>>,
}

struct CoroutineWaker {
    hub: Weak<LoopHub>,
    queued: AtomicBool,
    state: Mutex<WakeState>,
}

impl CoroutineWaker {
    fn set_py_future(&self, py_future: Py<RyCoroutineFuture>) {
        let mut state = lock(&self.state);
        state.py_future = Some(py_future);
    }

    fn rearm(&self) {
        let mut state = lock(&self.state);
        state.status = WakeStatus::Pending;
        state.callbacks.clear();
        state.cancel_message = None;
    }

    /// done for good ~ drops the `py_future` + cbs so a late wake is a noop
    fn retire(&self) {
        *lock(&self.state) = WakeState {
            status: WakeStatus::Woken,
            ..WakeState::default()
        };
    }

    /// errs only w/ a `KeyboardInterrupt`/`SystemExit` (see [`LoopHub::run_ready`])
    fn wake_task(&self, py: Python<'_>) -> PyResult<()> {
        self.queued.store(false, Ordering::Release);
        let (py_future, callbacks) = {
            let mut state = lock(&self.state);
            if state.status != WakeStatus::Pending {
                return Ok(());
            }
            state.status = WakeStatus::Woken;
            let Some(py_future) = state
                .py_future
                .as_ref()
                .map(|py_future| py_future.clone_ref(py))
            else {
                return Ok(());
            };
            (py_future, std::mem::take(&mut state.callbacks))
        };
        let py_future = py_future.into_bound(py).into_any();
        let mut fatal_err = None;
        for (callback, context) in callbacks {
            let result = match context {
                Some(context) => context
                    .bind(py)
                    .call_method1(pyo3::intern!(py, "run"), (callback.bind(py), &py_future)),
                None => callback.bind(py).call1((&py_future,)),
            };
            match result {
                Ok(_) => {}
                Err(error)
                    if error.is_instance_of::<PyKeyboardInterrupt>(py)
                        || error.is_instance_of::<PySystemExit>(py) =>
                {
                    fatal_err.get_or_insert(error);
                }
                Err(error) => error.write_unraisable(py, Some(&py_future)),
            }
        }
        fatal_err.map_or(Ok(()), Err)
    }

    fn traverse(&self, visit: &PyVisit<'_>) -> Result<(), PyTraverseError> {
        // busy ~ in use rn skip for now
        let Ok(state) = self.state.try_lock() else {
            return Ok(());
        };
        visit.call(&state.py_future)?;
        visit.call(&state.cancel_message)?;
        for (callback, context) in &state.callbacks {
            visit.call(callback)?;
            visit.call(context)?;
        }
        Ok(())
    }
}

/// The Future-shaped object yielded to `asyncio.Task` while Rust is pending.
///
/// This aint a python `asyncio.Future` subclass; tokio waker resolves it and
/// runs the tasks done-cbs on the loop thread to avoid a `call_soon` py
#[pyclass(name = "CoroutineFuture", frozen, immutable_type)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
struct RyCoroutineFuture {
    waker: Arc<CoroutineWaker>,
    event_loop: Py<PyAny>,
    asyncio_blocking: AtomicBool,
}

impl RyCoroutineFuture {
    fn status(&self) -> WakeStatus {
        lock(&self.waker.state).status
    }

    fn cancelled_error(&self, py: Python<'_>) -> PyErr {
        match lock(&self.waker.state).cancel_message.as_ref() {
            Some(message) => CancelledError::new_err((message.clone_ref(py),)),
            None => CancelledError::new_err(()),
        }
    }
}

#[pymethods]
impl RyCoroutineFuture {
    #[getter(_asyncio_future_blocking)]
    fn asyncio_blocking(&self) -> bool {
        self.asyncio_blocking.load(Ordering::Relaxed)
    }

    #[setter(_asyncio_future_blocking)]
    fn set_asyncio_blocking(&self, value: bool) {
        self.asyncio_blocking.store(value, Ordering::Relaxed);
    }

    fn get_loop(&self, py: Python<'_>) -> Py<PyAny> {
        self.event_loop.clone_ref(py)
    }

    #[pyo3(signature = (callback, *, context = None))]
    fn add_done_callback(
        slf: &Bound<'_, Self>,
        callback: Bound<'_, PyAny>,
        context: Option<Bound<'_, PyAny>>,
    ) -> PyResult<()> {
        let this = slf.get();
        {
            let mut state = lock(&this.waker.state);
            if state.status == WakeStatus::Pending {
                state
                    .callbacks
                    .push((callback.unbind(), context.map(Bound::unbind)));
                return Ok(());
            }
        }
        call_soon(
            this.event_loop.bind(slf.py()),
            &callback,
            slf.as_any(),
            context.as_ref(),
        )
    }

    /// matches w/ `==` like `asyncio.Future` (bound methods are new objects
    /// every access); compared outside the lock b/c `__eq__` can be gilly
    fn remove_done_callback(&self, callback: &Bound<'_, PyAny>) -> PyResult<usize> {
        let py = callback.py();
        let candidates: Vec<Py<PyAny>> = lock(&self.waker.state)
            .callbacks
            .iter()
            .map(|(candidate, _)| candidate.clone_ref(py))
            .collect();
        let mut matched = Vec::new();
        for candidate in candidates {
            if candidate.is(callback) || candidate.bind(py).eq(callback)? {
                matched.push(candidate);
            }
        }
        if matched.is_empty() {
            return Ok(0);
        }
        let mut state = lock(&self.waker.state);
        let before = state.callbacks.len();
        state
            .callbacks
            .retain(|(candidate, _)| !matched.iter().any(|m| m.is(candidate)));
        Ok(before - state.callbacks.len())
    }

    #[pyo3(signature = (msg = None))]
    fn cancel(slf: &Bound<'_, Self>, msg: Option<Py<PyAny>>) -> PyResult<bool> {
        let this = slf.get();
        let callbacks = {
            let mut state = lock(&this.waker.state);
            if state.status != WakeStatus::Pending {
                return Ok(false);
            }
            state.status = WakeStatus::Cancelled;
            state.cancel_message = msg;
            std::mem::take(&mut state.callbacks)
        };
        for (callback, context) in callbacks {
            call_soon(
                this.event_loop.bind(slf.py()),
                callback.bind(slf.py()),
                slf.as_any(),
                context.as_ref().map(|context| context.bind(slf.py())),
            )?;
        }
        Ok(true)
    }

    fn cancelled(&self) -> bool {
        self.status() == WakeStatus::Cancelled
    }

    fn done(&self) -> bool {
        self.status() != WakeStatus::Pending
    }

    fn result(&self, py: Python<'_>) -> PyResult<()> {
        match self.status() {
            WakeStatus::Pending => Err(InvalidStateError::new_err("Result is not ready.")),
            WakeStatus::Cancelled => Err(self.cancelled_error(py)),
            WakeStatus::Woken => Ok(()),
        }
    }

    fn exception(&self, py: Python<'_>) -> PyResult<Option<Py<PyAny>>> {
        self.result(py).map(|()| None)
    }

    fn __traverse__(&self, visit: PyVisit<'_>) -> Result<(), PyTraverseError> {
        visit.call(&self.event_loop)
    }
}

impl Wake for CoroutineWaker {
    fn wake(self: Arc<Self>) {
        if self.queued.swap(true, Ordering::AcqRel) {
            return;
        }
        if let Some(hub) = self.hub.upgrade() {
            hub.post_event(LoopEvent::Wake(self));
        }
    }

    fn wake_by_ref(self: &Arc<Self>) {
        Arc::clone(self).wake();
    }
}

trait PollPy: Send {
    fn poll_py(
        self: Pin<&mut Self>,
        py: Python<'_>,
        context: &mut Context<'_>,
    ) -> Poll<PyResult<Py<PyAny>>>;
}

impl<F, T> PollPy for F
where
    F: Future<Output = PyResult<T>> + Send,
    T: for<'py> IntoPyObject<'py>,
{
    fn poll_py(
        self: Pin<&mut Self>,
        py: Python<'_>,
        context: &mut Context<'_>,
    ) -> Poll<PyResult<Py<PyAny>>> {
        self.poll(context)
            .map(|result| result.and_then(|value| value.into_py_any(py)))
    }
}

type BoxPyFuture = Pin<Box<dyn PollPy>>;

enum CoroutineState {
    Immediate(PyResult<Py<PyAny>>),
    Unstarted(BoxPyFuture),
    Suspended {
        future: BoxPyFuture,
        waker: Arc<CoroutineWaker>,
    },
    Consumed,
}

/// Python "coroutine" that drives Rust futures
#[pyclass(name = "Coroutine", frozen, immutable_type, weakref)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
pub struct RyCoroutine {
    qualname: &'static str,
    state: Mutex<CoroutineState>,
}

impl RyCoroutine {
    fn new<'py>(
        py: Python<'py>,
        qualname: &'static str,
        state: CoroutineState,
    ) -> PyResult<Bound<'py, Self>> {
        Bound::new(
            py,
            Self {
                qualname,
                state: Mutex::new(state),
            },
        )
    }

    fn state(&self) -> PyResult<MutexGuard<'_, CoroutineState>> {
        match self.state.try_lock() {
            Ok(state) => Ok(state),
            Err(TryLockError::Poisoned(error)) => Ok(error.into_inner()),
            Err(TryLockError::WouldBlock) => {
                Err(PyRuntimeError::new_err("coroutine already executing"))
            }
        }
    }

    fn drop_future(&self) -> PyResult<()> {
        let old = std::mem::replace(&mut *self.state()?, CoroutineState::Consumed);
        if let CoroutineState::Suspended { waker, .. } = &old {
            waker.retire();
        }
        drop(old);
        Ok(())
    }

    fn poll_once(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        let mut state = self.state()?;
        let (mut future, waker) = match std::mem::replace(&mut *state, CoroutineState::Consumed) {
            CoroutineState::Consumed => return Err(PyRuntimeError::new_err(REUSED)),
            CoroutineState::Immediate(result) => {
                return Err(
                    result.map_or_else(|error| error, |value| PyStopIteration::new_err((value,)))
                );
            }
            CoroutineState::Unstarted(future) => {
                let loop_handle = LoopHandle::current(py)?;
                let waker = Arc::new(CoroutineWaker {
                    hub: Arc::downgrade(&loop_handle.0),
                    queued: AtomicBool::new(false),
                    state: Mutex::new(WakeState::default()),
                });
                let py_future = Py::new(
                    py,
                    RyCoroutineFuture {
                        waker: Arc::clone(&waker),
                        event_loop: loop_handle.0.event_loop.clone_ref(py),
                        asyncio_blocking: AtomicBool::new(false),
                    },
                )?;
                waker.set_py_future(py_future);
                (future, waker)
            }
            CoroutineState::Suspended { future, waker } => {
                waker.rearm();
                (future, waker)
            }
        };

        let rust_waker = Waker::from(Arc::clone(&waker));
        let result = future
            .as_mut()
            .poll_py(py, &mut Context::from_waker(&rust_waker));

        match result {
            Poll::Ready(result) => {
                waker.retire();
                Err(result.map_or_else(|error| error, |value| PyStopIteration::new_err((value,))))
            }
            Poll::Pending => {
                let py_future = lock(&waker.state)
                    .py_future
                    .as_ref()
                    .map(|py_future| py_future.clone_ref(py))
                    .expect("py_future is installed before polling");
                py_future
                    .get()
                    .asyncio_blocking
                    .store(true, Ordering::Relaxed);
                *state = CoroutineState::Suspended { future, waker };
                Ok(py_future.into_any())
            }
        }
    }
}

impl Drop for RyCoroutine {
    fn drop(&mut self) {
        let state = self.state.get_mut().unwrap_or_else(PoisonError::into_inner);
        if let CoroutineState::Suspended { waker, .. } = state {
            waker.retire();
        }
    }
}

/// `throw(exc)` or deprecated `throw(type, value, tb)` ~ looks like dep in 312
fn thrown_error(typ: &Bound<'_, PyAny>, value: Option<&Bound<'_, PyAny>>) -> PyErr {
    match value.filter(|value| !value.is_none()) {
        Some(value) if value.is_instance_of::<PyBaseException>() => {
            PyErr::from_value(value.clone())
        }
        Some(value) => typ.call1((value,)).map_or_else(|e| e, PyErr::from_value),
        None => PyErr::from_value(typ.clone()),
    }
}

#[pymethods]
impl RyCoroutine {
    #[getter]
    fn __name__(&self) -> &'static str {
        self.qualname
            .rsplit_once('.')
            .map_or(self.qualname, |(_, name)| name)
    }

    #[getter]
    fn __qualname__(&self) -> &'static str {
        self.qualname
    }

    #[cfg(not(any(PyPy, GraalPy)))]
    fn __repr__(slf: &Bound<'_, Self>) -> String {
        format!(
            "<ry.Coroutine {} at {:p}>",
            slf.get().qualname,
            slf.as_ptr()
        )
    }

    #[cfg(any(PyPy, GraalPy))]
    fn __repr__(slf: &Bound<'_, Self>) -> PyResult<String> {
        // address from `id()` like python's own reprs ~ on pypy the object
        // pointer we get aint what `id()` says
        static ID: PyOnceLock<Py<PyAny>> = PyOnceLock::new();
        let id: usize = ID
            .import(slf.py(), "builtins", "id")?
            .call1((slf,))?
            .extract()?;
        Ok(format!("<ry.Coroutine {} at {id:#x}>", slf.get().qualname))
    }

    fn __await__(slf: Py<Self>) -> Py<Self> {
        slf
    }

    /// `__await__()` returns self so it can be a awaited like a py coroutine
    ///
    /// `yield from aw.__await__()` (generator-y adapters) needs `iter()` afaict
    fn __iter__(slf: Py<Self>) -> Py<Self> {
        slf
    }

    fn __next__(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        self.poll_once(py)
    }

    fn send(&self, py: Python<'_>, _value: &Bound<'_, PyAny>) -> PyResult<Py<PyAny>> {
        self.poll_once(py)
    }

    #[pyo3(signature = (typ, value = None, _traceback = None, /))]
    fn throw(
        &self,
        typ: &Bound<'_, PyAny>,
        value: Option<&Bound<'_, PyAny>>,
        _traceback: Option<&Bound<'_, PyAny>>,
    ) -> PyResult<Py<PyAny>> {
        self.drop_future()?;
        Err(thrown_error(typ, value))
    }

    fn close(&self) -> PyResult<()> {
        self.drop_future()
    }

    fn __traverse__(&self, visit: PyVisit<'_>) -> Result<(), PyTraverseError> {
        let Ok(state) = self.state.try_lock() else {
            return Ok(());
        };
        match &*state {
            CoroutineState::Immediate(Ok(value)) => visit.call(value),
            CoroutineState::Suspended { waker, .. } => waker.traverse(&visit),
            _ => Ok(()),
        }
    }

    fn __clear__(&self) {
        let _ = self.drop_future();
    }
}

/// run `future` on the tokio rt ~ spawned on first poll (not on call)
pub fn tokio_coro<'py, F, T>(
    py: Python<'py>,
    qualname: &'static str,
    future: F,
) -> PyResult<Bound<'py, RyCoroutine>>
where
    F: Future<Output = PyResult<T>> + Send + 'static,
    T: for<'a> IntoPyObject<'a> + Send + 'static,
{
    let lazy = async move { get_ry_tokio_runtime().py_spawn(future).await? };
    RyCoroutine::new(py, qualname, CoroutineState::Unstarted(Box::pin(lazy)))
}

/// already-finished coroutine ~ no tokio, no poll
pub fn done_coro<'py, T>(
    py: Python<'py>,
    qualname: &'static str,
    value: T,
) -> PyResult<Bound<'py, RyCoroutine>>
where
    T: IntoPyObject<'py>,
{
    RyCoroutine::new(
        py,
        qualname,
        CoroutineState::Immediate(value.into_py_any(py)),
    )
}

/// coroutine that raises `err` when awaited (same as an `async def` would)
pub fn failed_coro<'py>(
    py: Python<'py>,
    qualname: &'static str,
    err: PyErr,
) -> PyResult<Bound<'py, RyCoroutine>> {
    RyCoroutine::new(py, qualname, CoroutineState::Immediate(Err(err)))
}

/// poll `future` right on the loop thread ~ ONLY for futures that dont need
/// the tokio rt context (tokio channels/notify/mutex ok; io/timers/spawn no)
pub fn pyloop_coro<'py, F, T>(
    py: Python<'py>,
    qualname: &'static str,
    future: F,
) -> PyResult<Bound<'py, RyCoroutine>>
where
    F: Future<Output = PyResult<T>> + Send + 'static,
    T: for<'a> IntoPyObject<'a> + Send + 'static,
{
    RyCoroutine::new(py, qualname, CoroutineState::Unstarted(Box::pin(future)))
}
