#![doc = include_str!("../README.md")]
use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyModule};

/// Unindent a string removing the maximum common leading whitespace
#[pyfunction(signature = (s, /))]
#[must_use]
pub fn unindent(s: &str) -> String {
    ::unindent::unindent(s)
}

/// Unindent a python bytes removing the maximum common leading whitespace
#[pyfunction(signature = (b, /))]
#[must_use]
pub fn unindent_bytes<'py>(py: Python<'py>, b: &[u8]) -> Bound<'py, PyBytes> {
    let b = ::unindent::unindent_bytes(b);
    PyBytes::new(py, &b)
}

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(crate::unindent, m)?)?;
    m.add_function(wrap_pyfunction!(crate::unindent_bytes, m)?)?;
    Ok(())
}
