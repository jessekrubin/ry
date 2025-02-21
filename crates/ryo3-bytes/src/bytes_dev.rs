//! Dev area for `pyo3-bytes`/`ryo3-bytes` crate

use crate::{extract_bytes_ref, PyBytes};
use pyo3::prelude::*;

/// Sum the bytes in a `&[u8]` slice
#[inline]
fn bytes_sum_impl(data: &[u8]) -> u128 {
    data.iter().fold(0, |acc, &x| acc + u128::from(x))
}

/// Sum the bytes in a `&[u8]` slice (python-bytes)
#[pyfunction]
fn bytes_sum_pybytes(data: &[u8]) -> u128 {
    bytes_sum_impl(data)
}

/// Sum the bytes in a `PyBytes` object (by reference)
#[pyfunction]
fn bytes_sum_rybytes_ref(data: &PyBytes) -> u128 {
    bytes_sum_impl(data.as_ref())
}

/// Sum the bytes in a `PyBytes` object
#[pyfunction]
fn bytes_sum_rybytes(data: PyBytes) -> u128 {
    bytes_sum_impl(data.as_ref())
}

#[pyfunction]
fn bytes_sum_bytes_like(data: &Bound<'_, PyAny>) -> PyResult<u128> {
    let data = extract_bytes_ref(data)?;
    Ok(bytes_sum_impl(data))
}

/// ryo3-bytes python module registration
pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(bytes_sum_pybytes, m)?)?;
    m.add_function(wrap_pyfunction!(bytes_sum_rybytes_ref, m)?)?;
    m.add_function(wrap_pyfunction!(bytes_sum_rybytes, m)?)?;
    m.add_function(wrap_pyfunction!(bytes_sum_bytes_like, m)?)?;
    Ok(())
}
