//! python `tokio::fs` module
use pyo3::prelude::*;
use ryo3_bytes::PyBytes;
use ryo3_std::PyMetadata;
use std::path::PathBuf;

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
pub fn read_dir_async(_py: Python<'_>, _pth: PathBuf) -> PyResult<Bound<'_, PyAny>> {
    Err(PyErr::new::<pyo3::exceptions::PyNotImplementedError, _>(
        "read_dir_async not implemented",
    ))
}

#[pyfunction]
pub fn copy_async(py: Python<'_>, from: PathBuf, to: PathBuf) -> PyResult<Bound<'_, PyAny>> {
    pyo3_async_runtimes::tokio::future_into_py(py, async move {
        tokio::fs::copy(from, to).await.map_err(PyErr::from)
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
