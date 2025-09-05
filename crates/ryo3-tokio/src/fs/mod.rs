//! python `tokio::fs` module
pub use crate::fs::file::PyAsyncFile;
use crate::fs::py_open_mode::PyOpenMode;
use crate::fs::read_dir::PyReadDirAsync;
use pyo3::IntoPyObjectExt;
use pyo3::prelude::*;
use pyo3::pybacked::PyBackedStr;
use pyo3::types::PyDict;
use ryo3_bytes::PyBytes;
use ryo3_std::fs::PyMetadata;
use std::path::PathBuf;
use tracing::warn;

pub mod file;
mod py_open_mode;
mod read_dir;

#[pyfunction]
pub fn canonicalize_async(py: Python<'_>, pth: PathBuf) -> PyResult<Bound<'_, PyAny>> {
    pyo3_async_runtimes::tokio::future_into_py(py, async move {
        tokio::fs::canonicalize(pth)
            .await
            .map(|p| p.to_string_lossy().to_string())
            .map_err(PyErr::from)
    })
}

#[pyfunction]
pub fn copy_async(py: Python<'_>, from: PathBuf, to: PathBuf) -> PyResult<Bound<'_, PyAny>> {
    pyo3_async_runtimes::tokio::future_into_py(py, async move {
        tokio::fs::copy(from, to).await.map_err(PyErr::from)
    })
}

#[pyfunction]
pub fn create_dir_async(py: Python<'_>, pth: PathBuf) -> PyResult<Bound<'_, PyAny>> {
    pyo3_async_runtimes::tokio::future_into_py(py, async move {
        tokio::fs::create_dir(pth).await.map_err(PyErr::from)
    })
}

#[pyfunction]
pub fn create_dir_all_async(py: Python<'_>, pth: PathBuf) -> PyResult<Bound<'_, PyAny>> {
    pyo3_async_runtimes::tokio::future_into_py(py, async move {
        tokio::fs::create_dir_all(pth).await.map_err(PyErr::from)
    })
}

#[pyfunction]
pub fn hard_link_async(py: Python<'_>, from: PathBuf, to: PathBuf) -> PyResult<Bound<'_, PyAny>> {
    pyo3_async_runtimes::tokio::future_into_py(py, async move {
        tokio::fs::hard_link(from, to).await.map_err(PyErr::from)
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
pub fn read_async(py: Python<'_>, pth: PathBuf) -> PyResult<Bound<'_, PyAny>> {
    pyo3_async_runtimes::tokio::future_into_py(py, async move {
        tokio::fs::read(pth)
            .await
            .map(ryo3_bytes::PyBytes::from)
            .map_err(PyErr::from)
    })
}

#[pyfunction]
pub fn read_dir_async(py: Python<'_>, pth: PathBuf) -> PyResult<Bound<'_, PyAny>> {
    pyo3_async_runtimes::tokio::future_into_py(py, async move {
        let readdir = tokio::fs::read_dir(pth).await.map_err(PyErr::from)?;
        Ok(PyReadDirAsync::from(readdir))
    })
}

#[pyfunction]
pub fn read_link_async(py: Python<'_>, pth: PathBuf) -> PyResult<Bound<'_, PyAny>> {
    pyo3_async_runtimes::tokio::future_into_py(py, async move {
        tokio::fs::read_link(pth)
            .await
            .map(|p| p.to_string_lossy().to_string())
            .map_err(PyErr::from)
    })
}

#[pyfunction]
pub fn read_to_string_async(py: Python<'_>, pth: PathBuf) -> PyResult<Bound<'_, PyAny>> {
    pyo3_async_runtimes::tokio::future_into_py(py, async move {
        tokio::fs::read_to_string(pth).await.map_err(PyErr::from)
    })
}

#[pyfunction]
pub fn remove_dir_async(py: Python<'_>, pth: PathBuf) -> PyResult<Bound<'_, PyAny>> {
    pyo3_async_runtimes::tokio::future_into_py(py, async move {
        tokio::fs::remove_dir(pth).await.map_err(PyErr::from)
    })
}

#[pyfunction]
pub fn remove_dir_all_async(py: Python<'_>, pth: PathBuf) -> PyResult<Bound<'_, PyAny>> {
    pyo3_async_runtimes::tokio::future_into_py(py, async move {
        tokio::fs::remove_dir_all(pth).await.map_err(PyErr::from)
    })
}

#[pyfunction]
pub fn remove_file_async(py: Python<'_>, pth: PathBuf) -> PyResult<Bound<'_, PyAny>> {
    pyo3_async_runtimes::tokio::future_into_py(py, async move {
        tokio::fs::remove_file(pth).await.map_err(PyErr::from)
    })
}

#[pyfunction]
pub fn rename_async(py: Python<'_>, from: PathBuf, to: PathBuf) -> PyResult<Bound<'_, PyAny>> {
    pyo3_async_runtimes::tokio::future_into_py(py, async move {
        tokio::fs::rename(from, to).await.map_err(PyErr::from)
    })
}

#[pyfunction]
pub fn try_exists_async(py: Python<'_>, pth: PathBuf) -> PyResult<Bound<'_, PyAny>> {
    pyo3_async_runtimes::tokio::future_into_py(py, async move {
        tokio::fs::try_exists(pth)
            .await
            .map(|b| b.to_string())
            .map_err(PyErr::from)
    })
}

#[pyfunction]
pub fn exists_async(py: Python<'_>, pth: PathBuf) -> PyResult<Bound<'_, PyAny>> {
    pyo3_async_runtimes::tokio::future_into_py(py, async move {
        tokio::fs::try_exists(pth)
            .await
            .map(|b| b.to_string())
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
pub fn write_string_async(
    py: Python<'_>,
    fspath: PathBuf,
    s: PyBackedStr,
) -> PyResult<Bound<'_, PyAny>> {
    pyo3_async_runtimes::tokio::future_into_py(py, async move {
        let nbytes = s.len();

        tokio::fs::write(fspath, s).await.map_err(PyErr::from)?;
        Ok(nbytes)
    })
}

#[pyfunction(
    signature = (path, mode = PyOpenMode::default(), **kwargs),
)]
pub fn aiopen<'py>(
    py: Python<'py>,
    path: PathBuf,
    mode: PyOpenMode,
    kwargs: Option<&Bound<'py, PyDict>>,
) -> PyResult<Bound<'py, PyAny>> {
    if let Some(kwargs) = kwargs {
        warn!("aiopen kwargs not impl: {kwargs:?}");
    }
    if !mode.is_binary() {
        return Err(pyo3::exceptions::PyNotImplementedError::new_err(
            "aiopen text mode not implemented",
        ));
    }
    PyAsyncFile::new(path, mode.into()).into_bound_py_any(py)
}

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // file
    m.add_class::<PyAsyncFile>()?;
    //fns
    m.add_function(wrap_pyfunction!(aiopen, m)?)?;
    m.add_function(wrap_pyfunction!(canonicalize_async, m)?)?;
    m.add_function(wrap_pyfunction!(copy_async, m)?)?;
    m.add_function(wrap_pyfunction!(create_dir_async, m)?)?;
    m.add_function(wrap_pyfunction!(create_dir_all_async, m)?)?;
    m.add_function(wrap_pyfunction!(hard_link_async, m)?)?;
    m.add_function(wrap_pyfunction!(metadata_async, m)?)?;
    m.add_function(wrap_pyfunction!(read_async, m)?)?;
    m.add_function(wrap_pyfunction!(read_dir_async, m)?)?;
    m.add_function(wrap_pyfunction!(read_link_async, m)?)?;
    m.add_function(wrap_pyfunction!(read_to_string_async, m)?)?;
    m.add_function(wrap_pyfunction!(remove_dir_async, m)?)?;
    m.add_function(wrap_pyfunction!(remove_dir_all_async, m)?)?;
    m.add_function(wrap_pyfunction!(remove_file_async, m)?)?;
    m.add_function(wrap_pyfunction!(rename_async, m)?)?;
    // m.add_function(wrap_pyfunction!(set_permissions_async, m)?)?;
    // m.add_function(wrap_pyfunction!(symlink_dir_async, m)?)?;
    // m.add_function(wrap_pyfunction!(symlink_file_async, m)?)?;
    // m.add_function(wrap_pyfunction!(symlink_metadata_async, m)?)?;
    m.add_function(wrap_pyfunction!(try_exists_async, m)?)?;
    m.add_function(wrap_pyfunction!(exists_async, m)?)?;
    m.add_function(wrap_pyfunction!(write_async, m)?)?;

    Ok(())
}
