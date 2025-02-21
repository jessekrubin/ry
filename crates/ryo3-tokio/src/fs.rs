//! python `tokio::fs` module

use pyo3::prelude::*;
use ryo3_bytes::PyBytes;
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
