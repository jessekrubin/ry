use pyo3::prelude::*;
use pyo3::types::PyBytes;

#[pyfunction]
pub fn unindent(input: &str) -> String {
    ::unindent::unindent(input)
}

#[pyfunction]
pub fn unindent_bytes<'py>(py: Python<'py>, input: &[u8]) -> Bound<'py, PyBytes> {
    let b = ::unindent::unindent_bytes(input);
    let pb = PyBytes::new(py, &b);
    pb
}
