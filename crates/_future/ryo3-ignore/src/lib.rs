#![doc = include_str!("../README.md")]
use pyo3::prelude::*;
use pyo3::types::PyModule;
use pyo3::{PyResult, wrap_pyfunction};

#[pyfunction]
pub fn _ignore<'py>() -> bool {
    false
}

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(_ignore, m)?)?;
    Ok(())
}
