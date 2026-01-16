//! tokio-runtime + python
use tokio::runtime::Runtime;

use std::fmt;
use std::future::Future;
use std::marker::PhantomData;
use std::panic::{RefUnwindSafe, UnwindSafe};
use std::pin::Pin;
use std::task::{Context, Poll, Waker, ready};
pub struct RyRuntime<'r>(pub &'r Runtime);

pub struct RyJoinHandle<T>(pub tokio::task::JoinHandle<T>);

impl RyRuntime<'_> {
    pub fn spawn<F, T>(&self, fut: F) -> tokio::task::JoinHandle<T>
    where
        F: std::future::Future<Output = T> + Send + 'static,
        T: Send + 'static,
    {
        self.0.spawn(fut)
    }

    // version of spwan that returns a wrapped JoinHandle that can be polled
    // and ensures the join error is py-err-able
    pub fn py_spawn<F, T>(&self, fut: F) -> RyJoinHandle<T>
    where
        F: std::future::Future<Output = T> + Send + 'static,
        T: Send + 'static,
    {
        RyJoinHandle(self.spawn(fut))
    }
}
impl<T> Unpin for RyJoinHandle<T> {}

impl<T> Future for RyJoinHandle<T> {
    type Output = pyo3::PyResult<T>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // basically just do what JoinHandle does, but map the error to PyErr
        let this = self.get_mut();
        match ready!(Pin::new(&mut this.0).poll(cx)) {
            Ok(v) => Poll::Ready(Ok(v)),
            Err(e) if e.is_panic() => Poll::Ready(Err(pyo3::exceptions::PyRuntimeError::new_err(
                "Task panicked",
            ))),
            Err(e) => Poll::Ready(Err(pyo3::exceptions::PyRuntimeError::new_err(format!(
                "Task cancelled: {}",
                e
            )))),
        }
    }
}

pub fn get_tokio_runtime<'r>() -> &'r Runtime {
    pyo3_async_runtimes::tokio::get_runtime()
}

pub fn get_ry_tokio_runtime<'r>() -> RyRuntime<'r> {
    RyRuntime(get_tokio_runtime())
}

pub fn with_tokio_runtime<F, R>(f: F) -> R
where
    F: for<'r> FnOnce(&'r Runtime) -> R,
{
    let rt = get_tokio_runtime();
    f(rt)
}

// ==========================================================================
// FROM ~ FROM ~ FROM ~ FROM ~ FROM ~ FROM ~ FROM ~ FROM ~ FROM ~ FROM ~ FROM
// ==========================================================================

impl<T> From<RyJoinHandle<T>> for tokio::task::JoinHandle<T> {
    fn from(handle: RyJoinHandle<T>) -> Self {
        handle.0
    }
}

impl<T> From<tokio::task::JoinHandle<T>> for RyJoinHandle<T> {
    fn from(handle: tokio::task::JoinHandle<T>) -> Self {
        RyJoinHandle(handle)
    }
}
