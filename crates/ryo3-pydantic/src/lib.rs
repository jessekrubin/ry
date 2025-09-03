#![doc = include_str!("../README.md")]
use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyModule};

#[pyfunction]
#[must_use]
pub fn unindent(input: &str) -> String {
    ::unindent::unindent(input)
}

#[pyfunction]
#[must_use]
pub fn unindent_bytes<'py>(py: Python<'py>, input: &[u8]) -> Bound<'py, PyBytes> {
    let b = ::unindent::unindent_bytes(input);
    PyBytes::new(py, &b)
}

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(crate::unindent, m)?)?;
    m.add_function(wrap_pyfunction!(crate::unindent_bytes, m)?)?;
    Ok(())
}
