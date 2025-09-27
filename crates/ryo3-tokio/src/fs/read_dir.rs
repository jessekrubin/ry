use pyo3::exceptions::{PyStopAsyncIteration, PyValueError};
use pyo3::prelude::*;
use ryo3_std::fs::PyMetadata;
use std::ffi::OsString;
use std::format;
use std::path::PathBuf;
use std::pin::Pin;
use std::sync::Arc;
use tokio::fs::ReadDir;
use tokio::sync::Mutex;

type AsyncResponseStreamInner = Arc<Mutex<Pin<Box<ReadDir>>>>;

#[pyclass(name = "AsyncReadDir", frozen)]
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

    fn __anext__<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let stream = self.stream.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let mut guard = stream.lock().await;
            let nextentry = guard.as_mut().next_entry().await;
            match nextentry {
                Ok(Some(entry)) => {
                    let pde = PyAsyncDirEntry::from(entry);
                    Ok(pde)
                }
                Ok(None) => Err(PyStopAsyncIteration::new_err("fin")),
                Err(e) => Err(PyErr::from(e)),
            }
        })
    }

    fn collect<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let stream = self.stream.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let mut guard = stream.lock().await;
            let mut entries = Vec::new();
            while let Some(entry) = guard.as_mut().next_entry().await? {
                let pde = PyAsyncDirEntry::from(entry);
                entries.push(pde);
            }
            Ok(entries)
        })
    }

    fn take<'py>(&self, py: Python<'py>, n: u32) -> PyResult<Bound<'py, PyAny>> {
        let stream = self.stream.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let mut guard = stream.lock().await;
            let mut entries = Vec::new();
            for _ in 0..n {
                match guard.as_mut().next_entry().await {
                    Ok(Some(entry)) => {
                        let pde = PyAsyncDirEntry::from(entry);
                        entries.push(pde);
                    }
                    Ok(None) => break,
                    Err(e) => return Err(PyErr::from(e)),
                }
            }
            Ok(entries)
        })
    }
}

#[pyclass(name = "AsyncDirEntry", frozen)]
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
        let path = self.0.path();
        let pathstr = path.to_string_lossy();
        format!("AsyncDirEntry<'{pathstr}'>")
    }

    #[must_use]
    fn __fspath__(&self) -> OsString {
        let p = self.0.path();
        p.into_os_string()
    }

    #[getter]
    fn path(&self) -> PathBuf {
        self.0.path()
    }

    #[getter]
    fn filename(&self) -> OsString {
        self.0.file_name()
    }

    #[getter]
    fn file_type<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let inner = self.0.clone();

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let file_type = inner.file_type().await.map_err(PyErr::from)?;
            Ok(ryo3_std::fs::PyFileType::new(file_type))
        })
    }

    #[getter]
    fn metadata<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let inner = self.0.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let metadata = inner.metadata().await.map_err(PyErr::from)?;
            Ok(PyMetadata::from(metadata))
        })
    }

    #[getter]
    fn basename(&self) -> PyResult<OsString> {
        let path = self.0.path();
        let name = path.file_name().ok_or_else(|| {
            PyValueError::new_err(format!(
                "basename - path: {} - no file name",
                path.to_string_lossy()
            ))
        })?;
        Ok(name.to_os_string())
    }
}

impl std::fmt::Debug for PyAsyncDirEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AsyncDirEntry<'{:?}'>", self.0.path())
    }
}
