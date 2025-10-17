#![doc = include_str!("../README.md")]
use pyo3::prelude::*;
use pyo3::types::PyModule;

#[pyfunction]
pub fn _tokio_ws<'py>() -> bool {
    false
}

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(_tokio_ws, m)?)?;
    Ok(())
}
