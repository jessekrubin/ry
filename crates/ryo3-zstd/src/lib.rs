#![doc = include_str!("../README.md")]
use pyo3::prelude::PyModule;
use pyo3::prelude::*;
use pyo3::types::PyBytes;

#[pyfunction(signature = (data, level=None))]
pub fn zstd_encode(py: Python<'_>, data: &[u8], level: Option<i32>) -> PyResult<PyObject> {
    let level = level.unwrap_or(::zstd::DEFAULT_COMPRESSION_LEVEL);
    let encoded = ::zstd::stream::encode_all(data, level).map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("zstd-encode-error: {e:?}"))
    })?;
    Ok(PyBytes::new(py, &encoded).into())
}

#[pyfunction(signature = (data, level=None))]
pub fn zstd(py: Python<'_>, data: &[u8], level: Option<i32>) -> PyResult<PyObject> {
    zstd_encode(py, data, level)
}

#[pyfunction(signature = (data))]
pub fn zstd_decode(py: Python<'_>, data: &[u8]) -> PyResult<PyObject> {
    let mut decompressed = Vec::new();
    ::zstd::stream::copy_decode(data, &mut decompressed).map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("zstd-decode-error: {e:?}"))
    })?;
    Ok(PyBytes::new(py, &decompressed).into())
}

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(self::zstd, m)?)?;
    m.add_function(wrap_pyfunction!(zstd_decode, m)?)?;
    m.add_function(wrap_pyfunction!(zstd_encode, m)?)?;
    Ok(())
}
