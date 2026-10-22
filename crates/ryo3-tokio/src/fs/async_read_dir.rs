use std::ffi::OsString;
use std::format;
use std::path::PathBuf;
use std::pin::Pin;
use std::sync::Arc;

use pyo3::prelude::*;
use ryo3_macro_rules::py_stop_async_iteration_err;
use ryo3_std::fs::PyMetadata;
use ryo3_tokio_rt::{RyCoroutine, tokio_coro};
use tokio::fs::ReadDir;
use tokio::sync::Mutex;

type AsyncResponseStreamInner = Arc<Mutex<Pin<Box<ReadDir>>>>;

#[pyclass(name = "AsyncReadDir", frozen, immutable_type)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
pub struct PyAsyncReadDir {
    stream: AsyncResponseStreamInner,
}

impl From<ReadDir> for PyAsyncReadDir {
    fn from(readdir: ReadDir) -> Self {
        Self {
            stream: Arc::new(Mutex::new(Box::pin(readdir))),
        }
    }
}

#[pymethods]
impl PyAsyncReadDir {
    fn __aiter__(this: PyRef<Self>) -> PyRef<Self> {
        this
    }

    fn __anext__<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, RyCoroutine>> {
        let stream = Arc::clone(&self.stream);
        tokio_coro(py, "AsyncReadDir.__anext__", async move {
            let mut guard = stream.lock().await;
            match guard.as_mut().next_entry().await {
                Ok(Some(entry)) => Ok(PyAsyncDirEntry::from(entry)),
                Ok(None) => py_stop_async_iteration_err!("fin"),
                Err(e) => Err(PyErr::from(e)),
            }
        })
    }

    fn collect<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, RyCoroutine>> {
        let stream = Arc::clone(&self.stream);
        tokio_coro(py, "AsyncReadDir.collect", async move {
            let mut guard = stream.lock().await;
            let mut entries = Vec::new();
            while let Some(entry) = guard.as_mut().next_entry().await? {
                entries.push(PyAsyncDirEntry::from(entry));
            }
            Ok(entries)
        })
    }

    #[pyo3(signature = (n = 1))]
    fn take<'py>(&self, py: Python<'py>, n: u32) -> PyResult<Bound<'py, RyCoroutine>> {
        let stream = Arc::clone(&self.stream);
        tokio_coro(py, "AsyncReadDir.take", async move {
            let mut guard = stream.lock().await;
            let mut entries = Vec::new();
            for _ in 0..n {
                match guard.as_mut().next_entry().await? {
                    Some(entry) => entries.push(PyAsyncDirEntry::from(entry)),
                    None => break,
                }
            }
            Ok(entries)
        })
    }
}

#[pyclass(name = "AsyncDirEntry", frozen, immutable_type)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
struct PyAsyncDirEntry(pub Arc<tokio::fs::DirEntry>);

impl From<tokio::fs::DirEntry> for PyAsyncDirEntry {
    fn from(entry: tokio::fs::DirEntry) -> Self {
        Self(Arc::new(entry))
    }
}

#[pymethods]
impl PyAsyncDirEntry {
    fn __repr__(&self) -> String {
        format!("{self}")
    }

    #[must_use]
    fn __fspath__(&self) -> OsString {
        self.0.path().into_os_string()
    }

    #[getter]
    fn path(&self) -> PathBuf {
        self.0.path()
    }

    #[getter]
    fn filename(&self) -> OsString {
        self.0.file_name()
    }

    fn file_type<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, RyCoroutine>> {
        let inner = Arc::clone(&self.0);
        tokio_coro(py, "AsyncDirEntry.file_type", async move {
            let file_type = inner.file_type().await?;
            Ok(ryo3_std::fs::PyFileType::from(file_type))
        })
    }

    fn metadata<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, RyCoroutine>> {
        let inner = Arc::clone(&self.0);
        tokio_coro(py, "AsyncDirEntry.metadata", async move {
            let metadata = inner.metadata().await?;
            Ok(PyMetadata::from(metadata))
        })
    }
}

impl std::fmt::Display for PyAsyncDirEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AsyncDirEntry<'{:?}'>", self.0.path().display())
    }
}
