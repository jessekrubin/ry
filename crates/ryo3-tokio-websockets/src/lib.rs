#![doc = include_str!("../README.md")]
use pyo3::prelude::*;
use pyo3::types::PyModule;
use pyo3::{PyResult, wrap_pyfunction};

#[pyfunction]
#[must_use]
pub fn ws() -> i32 {
    0
}

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(ws, m)?)?;
    Ok(())
}
