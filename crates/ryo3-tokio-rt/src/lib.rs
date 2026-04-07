//! tokio-runtime + python

use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll, ready},
};

use pyo3::exceptions::PyRuntimeError;
pub use pyo3_async_runtimes::tokio::future_into_py;
use tokio::runtime::Runtime;

pub struct RyRuntime<'r>(pub &'r Runtime);

pub struct RyJoinHandle<T>(pub tokio::task::JoinHandle<T>);

impl RyRuntime<'_> {
    #[inline]
    pub fn spawn<F, T>(&self, fut: F) -> tokio::task::JoinHandle<T>
    where
        F: Future<Output = T> + Send + 'static,
        T: Send + 'static,
    {
        self.0.spawn(fut)
    }

    // version of spawn that returns a wrapped JoinHandle that can be polled
    // and ensures the join error is py-err-able
    #[inline]
    pub fn py_spawn<F, T>(&self, fut: F) -> RyJoinHandle<T>
    where
        F: Future<Output = T> + Send + 'static,
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
            Err(e) if e.is_panic() => {
                Poll::Ready(Err(PyRuntimeError::new_err(format!("Task panicked: {e}"))))
            }
            Err(e) => Poll::Ready(Err(PyRuntimeError::new_err(format!("Task cancelled: {e}")))),
        }
    }
}

#[inline]
#[must_use]
pub fn get_tokio_runtime<'r>() -> &'r Runtime {
    pyo3_async_runtimes::tokio::get_runtime()
}

#[inline]
#[must_use]
pub fn get_ry_tokio_runtime<'r>() -> RyRuntime<'r> {
    RyRuntime(get_tokio_runtime())
}

/// possible future helper functions? (`on_tokio`/`on_tokio_py`)
///
/// Executes the given future on the tokio runtime
///
/// **Note**: This ONLY maps the tokio join errors to `pyo3::PyErr`
#[inline]
pub async fn on_tokio<F, T>(fut: F) -> pyo3::PyResult<T>
where
    F: Future<Output = T> + Send + 'static,
    T: Send + 'static,
{
    get_ry_tokio_runtime().py_spawn(fut).await
}

/// Executes the given future on the tokio runtime (which returns a
/// `pyo3::PyResult`)
///
/// **Note**: This maps the tokio join errors to `pyo3::PyErr` and also the
///           inner errors to `pyo3::PyErr` as well (afaict (jesse))
#[inline]
pub async fn on_tokio_py<F, T>(fut: F) -> pyo3::PyResult<T>
where
    F: Future<Output = pyo3::PyResult<T>> + Send + 'static,
    T: Send + 'static,
{
    get_ry_tokio_runtime().py_spawn(fut).await?
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
        Self(handle)
    }
}
