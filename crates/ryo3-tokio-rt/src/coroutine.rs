//! python coroutine for rust futures + a GIL-free loop wake
//!
//! Rust finishes some work and python needs to hear about it. Instead of
//! grabbing the GIL on a tokio worker, tokio queues the wake and writes a byte
//! to a socketpair the loop watches w/ `add_reader`. The loop runs the whole
//! queue in one go.
//!
//! This replaces `pyo3-async-runtimes` (py-future + done-cb + 2 spawns +
//! blocking pool hop + GIL + `call_soon_threadsafe` per await). pyo3's
//! `experimental-async` is way cheaper but still takes the GIL on every wake.
//!
//! # threads
//!
//! tokio threads only ever run `Wake::wake` -> `post_event` -> `poke`, no
//! python. everything else is on the loop thread. the one exception is
//! `poke_fallback`.
//!
//! # How it wakes?
//!
//! 1. [`tokio_coro`] -> `Unstarted`, nothing spawned yet
//! 2. Task calls `send(None)`. `poll_once` gets the loop's hub and polls once
//!    w/ a noop waker, which is what spawns. already `Ready`? done. otherwise
//!    make the waker + py-future and poll again w/ the real one
//! 3. `Pending` -> yield the `RyCoroutineFuture`, Task adds `__wakeup` to it
//! 4. tokio calls the waker on a worker: set `queued`, push onto `ready`, and
//!    on the first wake of a burst write 1 byte to the socketpair
//! 5. loop sees the byte -> `run_ready` drains the socket, takes the queue and
//!    runs the done-cbs inline
//! 6. `__wakeup` -> `send` -> `Ready(v)` -> `StopIteration(v)`
//!
//! N wakes = 1 byte = 1 flush.
//!
//! # MUST MUST MUST
//!
//! - no lost wakes: if something is in `ready`, a flush is coming (see `run_ready`)
//! - one queued event per waker
//! - retire the waker before dropping the future, so a late wake is a noop
//! - one hub per loop
//!
//! # cancel == drop
//!
//! `throw` drops the rust future. for `tokio_coro` that drops the
//! `RyJoinHandle`, which requests an abort. there's no async cleanup, so
//! cleanup goes in `Drop`, and `Drop` can't await or `block_on`.
//!
//! # which one
//!
//! - [`tokio_coro`]: io/timers/spawn, the default
//! - [`pyloop_coro`]: only needs a waker (tokio channels/notify/mutex), polled
//!   on the loop thread
//! - [`blocking_coro`]: a sync closure, run on tokio's blocking thread pool
//! - [`done_coro`] / [`failed_coro`]: already have the answer
//! - [`yield_coro`]: yield once, like `asyncio.sleep(0)`
//!
//! # meh
//!
//! - loops w/o `add_reader` (windows proactor, ...) get one
//!   `call_soon_threadsafe` per burst, which needs the GIL (bummer)
//! - free-threaded python isn't measured; the `ready` mutex might matter there
//!
//! # prior art
//!
//! the self-pipe trick, same as asyncio's `_write_to_self` and grpc.aio's
//! poller, but w/o the GIL and w/ one byte per burst. PR #897.
//!
//! REFS:
//! - djb self-pipe trick <https://cr.yp.to/docs/selfpipe.html>
//! - asyncio self-pipe <https://github.com/python/cpython/blob/v3.13.0/Lib/asyncio/selector_events.py#L118-L147>
//! - asyncio `call_soon_threadsafe` <https://github.com/python/cpython/blob/v3.13.0/Lib/asyncio/base_events.py#L872-L882>
//! - grpc.aio poller <https://github.com/grpc/grpc/blob/v1.83.2/src/python/grpcio/grpc/_cython/_cygrpc/aio/completion_queue.pyx.pxi#L91-L116> + fallback <https://github.com/grpc/grpc/blob/v1.83.2/src/python/grpcio/grpc/_cython/_cygrpc/aio/completion_queue.pyx.pxi#L18-L57>
//! - uvloop `call_soon_threadsafe` <https://github.com/MagicStack/uvloop/blob/v0.23.0/uvloop/loop.pyx#L1277-L1286>
//! - libuv `uv_async_send` <https://docs.libuv.org/en/v1.x/async.html>
//! - trio `WakeupSocketpair` <https://github.com/python-trio/trio/blob/v0.35.0/src/trio/_core/_wakeup_socketpair.py#L12-L50>
//! - pyo3 waker (GIL per wake) <https://github.com/PyO3/pyo3/blob/v0.30.1/src/coroutine/waker.rs#L43-L51>, `experimental-async` <https://github.com/PyO3/pyo3/pull/3540>
//! - pyo3-async-runtimes <https://github.com/PyO3/pyo3-async-runtimes/blob/v0.30.0/src/generic.rs#L608-L680>
//! - ry PR <https://github.com/jessekrubin/ry/pull/897>

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

const COROUTINE_REUSED_EMSG: &str = "cannot reuse already awaited coroutine";
const COROUTINE_EXECUTING_EMSG: &str = "coroutine already executing";

#[inline]
fn lock<T>(mutex: &Mutex<T>) -> MutexGuard<'_, T> {
    mutex.lock().unwrap_or_else(PoisonError::into_inner)
}

/// Get the currently running asyncio event loop via `asyncio.get_running_loop()`
fn asyncio_get_running_loop(py: Python<'_>) -> PyResult<Bound<'_, PyAny>> {
    static GET_RUNNING_LOOP: PyOnceLock<Py<PyAny>> = PyOnceLock::new();
    GET_RUNNING_LOOP
        .import(py, "asyncio", "get_running_loop")?
        .call0()
}

/// empty `contextvars.Context` ~ see `register_reader`
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

/// One per running loop, shared by every coro (wakers hold a `Weak`)
///
/// closed loops:
///
/// - closed loops arent noticed right away.
/// - wakes pile into `ready` till the hub gets cleaned up, which happens when
///   a new loop is created on this thread.
/// - `asyncio.run` cancels everything before closing so its usually ~nothing
struct LoopHub {
    /// Event loop ref ~ also keeps the loop addr from getting reused
    event_loop: Py<PyAny>,
    /// Empty ctx for running flush
    context: Py<PyAny>,
    /// Reader/`call_soon_threadsafe` cb ~ `run_ready`
    flush: Py<RyCoroutineFlush>,
    /// (rx, tx)
    ///
    /// NOTE: `None` => no `add_reader` => `poke_fallback`
    #[cfg(unix)]
    wake_pair: Option<(UnixStream, UnixStream)>,
    /// `Wake`s from tokio + `Call`s from `LoopHandle::call_soon`
    ///
    /// This is how ws/request bodies get stuff onto the loop sans-GIL)
    ready: Mutex<Vec<LoopEvent>>,
    /// `true` => a flush is imminent (byte in the pipe or fallback queued) and
    /// hasnt taken the batch yet, so new events dont need to poke again
    flush_scheduled: AtomicBool,
    /// Loop 100% known dead (pruned or `call_soon_threadsafe` failed) ~ new
    /// events get dropped
    closed: AtomicBool,
}

impl LoopHub {
    #[cold]
    fn new(event_loop: &Bound<'_, PyAny>) -> PyResult<Arc<Self>> {
        let py = event_loop.py();
        let context = empty_context(py)?;
        let flush = Py::new(py, RyCoroutineFlush(OnceLock::new()))?;
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
        // the `ready` mutex orders this: a push after `run_ready` took the batch
        // sees its `store(false)` and pokes. AcqRel/SeqCst is overkill (but yolo)
        if !self.flush_scheduled.swap(true, Ordering::AcqRel) {
            self.poke();
        }
    }

    #[inline]
    fn poke(&self) {
        #[cfg(unix)]
        if let Some((_, tx)) = &self.wake_pair {
            // WouldBlock => a byte is already there so finna flush soon anyway.
            // other errs shouldnt happen (rx lives as long as the hub). if one did,
            // `flush_scheduled` stays true and the hub goes quiet. look here if wakes
            // ever just stop
            let _ = (&*tx).write(&[1]);
            return;
        }
        self.poke_fallback();
    }

    /// the only place a tokio thread takes the GIL. noop during interpreter
    /// shutdown. if `call_soon_threadsafe` raises the loop is closed, so close
    /// the hub
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
            // level-triggered, so a leftover byte fires again
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
        // only the first KeyboardInterrupt/SystemExit sticks, the rest get tossed
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
/// socketpair + `add_reader`. `None` on any failure => fallback
///
/// `add_reader` runs in an empty context b/c `Handle` copies the current one.
/// Otherwise the flush handle keeps the first awaiting task's contextvars
/// alive forever
fn register_reader(
    event_loop: &Bound<'_, PyAny>,
    context: &Bound<'_, PyAny>,
    flush: &Bound<'_, RyCoroutineFlush>,
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

#[pyclass(name = "CoroutineFlush", frozen)]
#[cfg_attr(any(not(Py_LIMITED_API), Py_3_14), pyo3(immutable_type))]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
struct RyCoroutineFlush(OnceLock<Weak<LoopHub>>);

#[pymethods]
impl RyCoroutineFlush {
    fn __call__(&self, py: Python<'_>) -> PyResult<()> {
        match self.0.get().and_then(Weak::upgrade) {
            Some(hub) => hub.run_ready(py),
            None => Ok(()),
        }
    }
}

// Vec not one slot b/c one thread can run a bunch of loops over time
// (`asyncio.run` in a loop, pytest loop-per-test). stale hubs get pruned
// lazily when the next new loop shows up
thread_local! {
    static LOOP_HUBS: RefCell<Vec<Arc<LoopHub>>> = const { RefCell::new(Vec::new()) };
}

/// handle to the running event loop on this thread (+ its wake hub)
#[derive(Clone)]
pub struct LoopHandle(Arc<LoopHub>);

impl LoopHandle {
    /// errs if there is no running loop
    pub fn current(py: Python<'_>) -> PyResult<Self> {
        let event_loop = asyncio_get_running_loop(py)?;
        if let Some(hub) = LOOP_HUBS.with(|hubs| {
            hubs.borrow()
                .iter()
                // ptr compare is fine, the hub's strong ref keeps the address from
                // being reused
                .find(|hub| hub.event_loop.as_ptr() == event_loop.as_ptr())
                .cloned()
        }) {
            return Ok(Self(hub));
        }

        Self::register_loop(&event_loop)
    }

    #[cold]
    fn register_loop(event_loop: &Bound<'_, PyAny>) -> PyResult<Self> {
        let py = event_loop.py();
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
        let hub = LoopHub::new(event_loop);
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

    /// run `call` w/ the GIL on the loop thread soon ~ callable from any thread
    /// (no GIL needed); if the loop is gone `call` just gets dropped
    #[inline]
    pub fn call_soon<F>(&self, call: F)
    where
        F: FnOnce(Python<'_>) + Send + 'static,
    {
        self.0.post_event(LoopEvent::Call(Box::new(call)));
    }
}

/// Pending -> Woken (handled or retired) | Pending -> Cancelled (py `cancel()`)
/// `rearm()` -> back to Pending every re-poll
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

/// tokio waker for one suspended coro + its py-future state
///
/// `queued` is its own atomic so tokio threads can dedupe w/o the lock
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

    /// back to Pending. clearing cbs is fine, Task re-adds `__wakeup` every yield
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
        // inline, not `call_soon` like `asyncio.Future`. that's the hop we skip,
        // but it means the task steps before handles already queued on the loop.
        // `queued` was reset above so a wake during the cbs re-queues
        let mut fatal_err = None;
        for (callback, context) in callbacks {
            let result = match context {
                Some(context) => context
                    .bind(py)
                    .call_method1(pyo3::intern!(py, "run"), (callback.bind(py), &py_future)),
                None => callback.bind(py).call1((&py_future,)),
            };
            if let Err(error) = result {
                Self::on_callback_pyerr(py, error, &py_future, &mut fatal_err);
            }
        }
        fatal_err.map_or(Ok(()), Err)
    }

    fn on_callback_pyerr(
        py: Python<'_>,
        error: PyErr,
        py_future: &Bound<'_, PyAny>,
        fatal_err: &mut Option<PyErr>,
    ) {
        if error.is_instance_of::<PyKeyboardInterrupt>(py)
            || error.is_instance_of::<PySystemExit>(py)
        {
            fatal_err.get_or_insert(error);
        } else {
            error.write_unraisable(py, Some(py_future));
        }
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
/// Not an `asyncio.Future` subclass, and it holds no value. It only wakes the
/// task; the result comes from `StopIteration(value)` on the next `send`.
/// `result()` is `None` once woken, `InvalidStateError` while pending,
/// `CancelledError` if cancelled
#[pyclass(name = "CoroutineFuture", frozen)]
#[cfg_attr(any(not(Py_LIMITED_API), Py_3_14), pyo3(immutable_type))]
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

    // `call_soon` here (not inline like `wake_task`),
    // This is the same as `asyncio.Future.cancel` in that it runs inside of
    // `Task.cancel` from whoever is cancelling,
    //
    // NOTE: This means that the cancellation is processed in the context of
    // the caller who invoked `Task.cancel()`, rather than inline within the
    // coroutine itself. I forget what problem this caused exactly but it was
    // wrong scoping I think.
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
        // hub gone & loop gone & nobody to wake? drop it
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

/// Unstarted -> Suspended -> Consumed. `Immediate`/`YieldOnce` never touch tokio
///
/// `poll_once` swaps in `Consumed` while polling, so an error or panic mid-poll
/// leaves it spent and the next `send` gets "cannot reuse already awaited
/// coroutine" (like cpython)
enum CoroutineState {
    Immediate(PyResult<Py<PyAny>>),
    /// `yield` (eg `asyncio.sleep(0)`)
    YieldOnce(Py<PyAny>),
    Unstarted(BoxPyFuture),
    Suspended {
        future: BoxPyFuture,
        waker: Arc<CoroutineWaker>,
    },
    Consumed,
}

/// Python "coroutine" that drives Rust futures
#[pyclass(name = "Coroutine", frozen, weakref)]
#[cfg_attr(any(not(Py_LIMITED_API), Py_3_14), pyo3(immutable_type))]
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
            Err(TryLockError::WouldBlock) => Err(PyRuntimeError::new_err(COROUTINE_EXECUTING_EMSG)),
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

    /// `send`/`__next__` ~ THE ACTUAL REAL DEAL COROUTINE
    ///
    /// - holds the state lock for the whole poll, so re-entry gets "coroutine
    ///   already executing" like a real coroutine
    /// - Unstarted: needs a running loop. polls once w/ a noop waker (the spawn,
    ///   for `tokio_coro`). ready => return, no waker/py-future made. pending =>
    ///   make them and poll again w/ the real waker
    /// - Pending: yields the same py-future every time, which is why `rearm` exists
    fn poll_once(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        let mut state = self.state()?;
        let (mut future, waker) = match std::mem::replace(&mut *state, CoroutineState::Consumed) {
            CoroutineState::Consumed => {
                return Err(PyRuntimeError::new_err(COROUTINE_REUSED_EMSG));
            }
            CoroutineState::Immediate(result) => {
                return Err(
                    result.map_or_else(|error| error, |value| PyStopIteration::new_err((value,)))
                );
            }
            CoroutineState::YieldOnce(value) => {
                *state = CoroutineState::Immediate(Ok(value));
                return Ok(py.None());
            }
            CoroutineState::Unstarted(mut future) => {
                let loop_handle = LoopHandle::current(py)?;
                // Ready operations need no Python future or wakeup allocation.
                // On Pending, poll again below with the real waker so a wake
                // racing this initial poll cannot be lost.
                if let Poll::Ready(result) = future
                    .as_mut()
                    .poll_py(py, &mut Context::from_waker(Waker::noop()))
                {
                    return Err(result
                        .map_or_else(|error| error, |value| PyStopIteration::new_err((value,))));
                }
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

// retire first, the future + join handle drop w/ the fields after
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

    // the future dies here w/o async cleanup. `Drop` cant `block_on` either, it
    // can run on a tokio worker. see <https://github.com/jessekrubin/ry/pull/899>
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
            CoroutineState::Immediate(Ok(value)) | CoroutineState::YieldOnce(value) => {
                visit.call(value)
            }
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
    // spawn inside the lazy block: never awaited => never spawned.
    // `RyJoinHandle` requests an abort on drop
    let lazy = async move { get_ry_tokio_runtime().py_spawn(future).await? };
    RyCoroutine::new(py, qualname, CoroutineState::Unstarted(Box::pin(lazy)))
}

/// Runs sync `f` on tokio's blocking thread pool. Nothing starts until the
/// coroutine is first awaited.
pub fn blocking_coro<'py, F, T>(
    py: Python<'py>,
    qualname: &'static str,
    f: F,
) -> PyResult<Bound<'py, RyCoroutine>>
where
    F: FnOnce() -> PyResult<T> + Send + 'static,
    T: for<'a> IntoPyObject<'a> + Send + 'static,
{
    pyloop_coro(py, qualname, async move {
        get_ry_tokio_runtime().py_spawn_blocking(f).await?
    })
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

/// coroutine that yields to the loop once then returns `value` ~ same as
/// `await asyncio.sleep(0)`
pub fn yield_coro<'py, T>(
    py: Python<'py>,
    qualname: &'static str,
    value: T,
) -> PyResult<Bound<'py, RyCoroutine>>
where
    T: IntoPyObject<'py>,
{
    let value = value.into_py_any(py)?;
    RyCoroutine::new(py, qualname, CoroutineState::YieldOnce(value))
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
///
/// spawning through the runtime handle (`get_ry_tokio_runtime().py_spawn*`)
/// and awaiting the `JoinHandle` works here; bare `tokio::spawn` panics
///
/// tokio can panic here (no reactor here) -> `PanicException`
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
