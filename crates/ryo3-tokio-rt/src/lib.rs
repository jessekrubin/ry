//! tokio-runtime + python
mod coroutine;

use std::future::Future;
use std::pin::Pin;
use std::sync::OnceLock;
use std::task::{Context, Poll, ready};

use pyo3::exceptions::PyRuntimeError;
use tokio::runtime::Runtime;

pub use crate::coroutine::{
    LoopHandle, RyCoroutine, done_coro, failed_coro, pyloop_coro, tokio_coro, yield_coro,
};

pub struct RyRuntime<'r>(pub &'r Runtime);

/// join handle that maps join errors to `PyErr` and aborts the task on drop
///
/// abort-on-drop is the cancel story ~ if whoever is awaiting this goes away
/// (cancelled task, dropped coroutine, ...) the tokio task gets nuked instead
/// of running detached in the background
pub struct RyJoinHandle<T>(pub tokio::task::JoinHandle<T>);

impl<T> Drop for RyJoinHandle<T> {
    fn drop(&mut self) {
        // noop if the task already finished
        self.0.abort();
    }
}

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

/// the shared multi-thread runtime (all drivers on), built on first use
///
/// adapted from `pyo3-async-runtimes`
///
/// # Panics
///
/// if the runtime can't be built (eg the os won't give us threads)
#[inline]
#[must_use]
pub fn get_tokio_runtime<'r>() -> &'r Runtime {
    static TOKIO_RUNTIME: OnceLock<Runtime> = OnceLock::new();
    TOKIO_RUNTIME.get_or_init(|| {
        tokio::runtime::Builder::new_multi_thread()
            .name("ry-tokio")
            .enable_all()
            .build()
            .expect("failed to build the tokio runtime")
    })
}

#[inline]
#[must_use]
pub fn get_ry_tokio_runtime<'r>() -> RyRuntime<'r> {
    RyRuntime(get_tokio_runtime())
}

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
// NOTE: no `RyJoinHandle -> JoinHandle` bc `RyJoinHandle` aborts on drop
impl<T> From<tokio::task::JoinHandle<T>> for RyJoinHandle<T> {
    fn from(handle: tokio::task::JoinHandle<T>) -> Self {
        Self(handle)
    }
}
