//! python `tokio::fs` module
use pyo3::exceptions::{PyStopAsyncIteration, PyValueError};
use pyo3::prelude::*;
use ryo3_bytes::PyBytes;
use ryo3_std::PyMetadata;
use std::ffi::OsString;
use std::path::PathBuf;
use std::pin::Pin;
use std::sync::Arc;
use tokio::sync::Mutex;

#[pyfunction]
pub fn read_async(py: Python<'_>, pth: PathBuf) -> PyResult<Bound<'_, PyAny>> {
    pyo3_async_runtimes::tokio::future_into_py(py, async move {
        tokio::fs::read(pth)
            .await
            .map(ryo3_bytes::PyBytes::from)
            .map_err(PyErr::from)
    })
}

#[pyfunction]
pub fn write_async(py: Python<'_>, fspath: PathBuf, b: PyBytes) -> PyResult<Bound<'_, PyAny>> {
    pyo3_async_runtimes::tokio::future_into_py(py, async move {
        let bref: &[u8] = b.as_ref();
        let len = bref.len();
        tokio::fs::write(fspath, b)
            .await
            .map(|()| len)
            .map_err(PyErr::from)
    })
}

#[pyfunction]
pub fn metadata_async(py: Python<'_>, pth: PathBuf) -> PyResult<Bound<'_, PyAny>> {
    pyo3_async_runtimes::tokio::future_into_py(py, async move {
        tokio::fs::metadata(pth)
            .await
            .map(PyMetadata::from)
            .map_err(PyErr::from)
    })
}

#[pyfunction]
pub fn rename_async(py: Python<'_>, from: PathBuf, to: PathBuf) -> PyResult<Bound<'_, PyAny>> {
    pyo3_async_runtimes::tokio::future_into_py(py, async move {
        tokio::fs::rename(from, to).await.map_err(PyErr::from)
    })
}

#[pyfunction]
pub fn remove_file_async(py: Python<'_>, pth: PathBuf) -> PyResult<Bound<'_, PyAny>> {
    pyo3_async_runtimes::tokio::future_into_py(py, async move {
        tokio::fs::remove_file(pth).await.map_err(PyErr::from)
    })
}

#[pyfunction]
pub fn remove_dir_async(py: Python<'_>, pth: PathBuf) -> PyResult<Bound<'_, PyAny>> {
    pyo3_async_runtimes::tokio::future_into_py(py, async move {
        tokio::fs::remove_dir(pth).await.map_err(PyErr::from)
    })
}

#[pyfunction]
pub fn create_dir_async(py: Python<'_>, pth: PathBuf) -> PyResult<Bound<'_, PyAny>> {
    pyo3_async_runtimes::tokio::future_into_py(py, async move {
        tokio::fs::create_dir(pth).await.map_err(PyErr::from)
    })
}

#[pyfunction]
pub fn copy_async(py: Python<'_>, from: PathBuf, to: PathBuf) -> PyResult<Bound<'_, PyAny>> {
    pyo3_async_runtimes::tokio::future_into_py(py, async move {
        tokio::fs::copy(from, to).await.map_err(PyErr::from)
    })
}

#[pyclass(name = "DirEntryAsync", module = "ry.ryo3")]
pub struct PyDirEntryAsync(pub std::sync::Arc<tokio::fs::DirEntry>);

impl From<tokio::fs::DirEntry> for PyDirEntryAsync {
    fn from(entry: tokio::fs::DirEntry) -> Self {
        PyDirEntryAsync(std::sync::Arc::new(entry))
    }
}

#[pymethods]
impl PyDirEntryAsync {
    pub fn __repr__(&self) -> PyResult<String> {
        let path = self.0.path();
        let pathstr = path.to_string_lossy();
        let s = format!("DirEntryAsync('{pathstr}')");
        Ok(s)
    }

    #[must_use]
    pub fn __fspath__(&self) -> OsString {
        let p = self.0.path();
        p.into_os_string()
    }

    #[getter]
    pub fn path(&self) -> PyResult<PathBuf> {
        let path = self.0.path();
        Ok(path)
    }

    #[getter]
    pub fn file_type<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let inner = self.0.clone();

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let file_type = inner.file_type().await.map_err(PyErr::from)?;
            Ok(ryo3_std::PyFileType::new(file_type))
        })
    }

    #[getter]
    pub fn metadata<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let inner = self.0.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let metadata = inner.metadata().await.map_err(PyErr::from)?;
            Ok(PyMetadata::from(metadata))
        })
    }

    #[getter]
    pub fn basename(&self) -> PyResult<OsString> {
        let path = self.0.path();
        let anme = path.file_name().ok_or_else(|| {
            PyValueError::new_err(format!(
                "basename - path: {} - no file name",
                path.to_string_lossy()
            ))
        })?;
        Ok(anme.to_os_string())
    }
}

type AsyncResponseStreamInner = Arc<Mutex<Pin<Box<tokio::fs::ReadDir>>>>;

#[pyclass]
pub struct RyReadDirAsync {
    stream: AsyncResponseStreamInner,
}

#[pymethods]
impl RyReadDirAsync {
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
                    let pde = PyDirEntryAsync::from(entry);
                    Ok(pde)
                }
                Ok(None) => Err(PyStopAsyncIteration::new_err("response-stream-fin")),
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
                let pde = PyDirEntryAsync::from(entry);
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
                        let pde = PyDirEntryAsync::from(entry);
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

#[pyfunction]
pub fn read_dir_async(py: Python<'_>, pth: PathBuf) -> PyResult<Bound<'_, PyAny>> {
    pyo3_async_runtimes::tokio::future_into_py(py, async move {
        let readdir = tokio::fs::read_dir(pth).await.map_err(PyErr::from)?;
        let ob = RyReadDirAsync {
            stream: Arc::new(Mutex::new(Box::pin(readdir))),
        };
        Ok(ob)
    })
}

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(read_async, m)?)?;
    m.add_function(wrap_pyfunction!(write_async, m)?)?;
    m.add_function(wrap_pyfunction!(metadata_async, m)?)?;
    m.add_function(wrap_pyfunction!(rename_async, m)?)?;
    m.add_function(wrap_pyfunction!(remove_file_async, m)?)?;
    m.add_function(wrap_pyfunction!(remove_dir_async, m)?)?;
    m.add_function(wrap_pyfunction!(create_dir_async, m)?)?;
    m.add_function(wrap_pyfunction!(read_dir_async, m)?)?;
    m.add_function(wrap_pyfunction!(copy_async, m)?)?;
    Ok(())
}
