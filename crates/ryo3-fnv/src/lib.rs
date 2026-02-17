#![doc = include_str!("../README.md")]
pub use fnv1a::PyFnv1a;
use pyo3::prelude::*;
mod fnv1a;

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyFnv1a>()?;
    Ok(())
}
