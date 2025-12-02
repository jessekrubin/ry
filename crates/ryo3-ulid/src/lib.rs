#![doc = include_str!("../README.md")]
mod py_ulid;
pub use py_ulid::PyUlid;
use pyo3::prelude::*;

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyUlid>()?;
    Ok(())
}
