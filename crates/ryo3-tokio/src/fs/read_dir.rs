use pyo3::prelude::*;
use ryo3_macro_rules::{py_stop_async_iteration_err, py_value_error};
use ryo3_std::fs::PyMetadata;
use std::ffi::OsString;
use std::format;
use std::path::PathBuf;
use std::pin::Pin;
use std::sync::Arc;
use tokio::fs::ReadDir;
use tokio::sync::Mutex;

#[cfg(feature = "experimental-async")]
use crate::rt::get_ry_tokio_runtime;

type AsyncResponseStreamInner = Arc<Mutex<Pin<Box<ReadDir>>>>;

#[pyclass(name = "AsyncReadDir", frozen, immutable_type, skip_from_py_object)]
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

#[cfg(not(feature = "experimental-async"))]
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
                Ok(None) => py_stop_async_iteration_err!("fin"),
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

#[cfg(feature = "experimental-async")]
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
                Ok(None) => py_stop_async_iteration_err!("fin"),
                Err(e) => Err(PyErr::from(e)),
            }
        })
    }

    async fn collect(&self) -> PyResult<Vec<PyAsyncDirEntry>> {
        let stream = self.stream.clone();
        get_ry_tokio_runtime()
            .py_spawn(async move {
                let mut guard = stream.lock().await;
                let mut entries = Vec::new();
                while let Some(entry) = guard.as_mut().next_entry().await? {
                    let pde = PyAsyncDirEntry::from(entry);
                    entries.push(pde);
                }
                Ok(entries)
            })
            .await?
    }

    async fn take(&self, n: u32) -> PyResult<Vec<PyAsyncDirEntry>> {
        let stream = self.stream.clone();
        get_ry_tokio_runtime()
            .py_spawn(async move {
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
            .await?
    }
}
#[pyclass(name = "AsyncDirEntry", frozen, immutable_type, skip_from_py_object)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
struct PyAsyncDirEntry(pub Arc<tokio::fs::DirEntry>);

impl From<tokio::fs::DirEntry> for PyAsyncDirEntry {
    fn from(entry: tokio::fs::DirEntry) -> Self {
        Self(Arc::new(entry))
    }
}

#[cfg(not(feature = "experimental-async"))]
#[pymethods]
impl PyAsyncDirEntry {
    fn __repr__(&self) -> String {
        format!("{self:?}")
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

    fn file_type<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let inner = self.0.clone();

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let file_type = inner.file_type().await.map_err(PyErr::from)?;
            Ok(ryo3_std::fs::PyFileType::new(file_type))
        })
    }

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
            py_value_error!("basename - path: {} - no file name", path.to_string_lossy())
        })?;
        Ok(name.to_os_string())
    }
}

#[cfg(feature = "experimental-async")]
#[pymethods]
impl PyAsyncDirEntry {
    fn __repr__(&self) -> String {
        format!("{self:?}")
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

    async fn file_type(&self) -> PyResult<ryo3_std::fs::PyFileType> {
        let inner = self.0.clone();
        get_ry_tokio_runtime()
            .py_spawn(async move {
                let file_type = inner.file_type().await?;
                Ok(ryo3_std::fs::PyFileType::new(file_type))
            })
            .await?
    }

    async fn metadata(&self) -> PyResult<PyMetadata> {
        let inner = self.0.clone();
        get_ry_tokio_runtime()
            .py_spawn(async move {
                let metadata = inner.metadata().await?;
                Ok(PyMetadata::from(metadata))
            })
            .await?
    }

    #[getter]
    fn basename(&self) -> PyResult<OsString> {
        let path = self.0.path();
        let name = path.file_name().ok_or_else(|| {
            py_value_error!("basename - path: {} - no file name", path.to_string_lossy())
        })?;
        Ok(name.to_os_string())
    }
}

impl std::fmt::Debug for PyAsyncDirEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AsyncDirEntry<'{:?}'>", self.0.path())
    }
}
