use std::path::Path;

use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyModule};
use pyo3::{pyfunction, wrap_pyfunction, PyResult};

pub mod fspath;

#[pyfunction]
pub fn read_text(s: &str) -> String {
    let p = Path::new(s);
    std::fs::read_to_string(p).unwrap()
}

#[pyfunction]
pub fn read_bytes(py: Python<'_>, s: &str) -> PyResult<PyObject> {
    let p = Path::new(s);
    let b = std::fs::read(p).unwrap();
    Ok(PyBytes::new(py, &b).into())
}

pub fn pymod(m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(read_text, m)?)?;
    m.add_function(wrap_pyfunction!(read_bytes, m)?)?;
    m.add_class::<fspath::PyPath>()?;

    Ok(())
}
