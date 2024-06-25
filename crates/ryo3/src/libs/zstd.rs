use ::zstd as zstdrs;
use pyo3::prelude::*;

// use pyo3::{Bound, PyObject, PyResult, wrap_pyfunction};
use pyo3::prelude::PyModule;
use pyo3::types::PyBytes;

#[pyfunction]
#[pyo3(signature = (data, level=None))]
pub fn zstd_encode(py: Python<'_>, data: &[u8], level: Option<i32>) -> PyResult<PyObject> {
    let level = level.unwrap_or(zstdrs::DEFAULT_COMPRESSION_LEVEL);
    let encoded = zstdrs::stream::encode_all(data, level).map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("zstd-encode-error: {e:?}"))
    })?;
    Ok(PyBytes::new_bound(py, &encoded).into())
}
#[pyfunction]
#[pyo3(signature = (data, level=None))]
pub fn zstd(py: Python<'_>, data: &[u8], level: Option<i32>) -> PyResult<PyObject> {
    zstd_encode(py, data, level)
}

#[pyfunction]
#[pyo3(signature = (data))]
pub fn zstd_decode(py: Python<'_>, data: &[u8]) -> PyResult<PyObject> {
    let mut decompressed = Vec::new();
    zstdrs::stream::copy_decode(data, &mut decompressed).map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("zstd-decode-error: {e:?}"))
    })?;
    Ok(PyBytes::new_bound(py, &decompressed).into())
}

pub fn madd(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(self::zstd, m)?)?;
    m.add_function(wrap_pyfunction!(zstd_decode, m)?)?;
    m.add_function(wrap_pyfunction!(zstd_encode, m)?)?;
    Ok(())
}
