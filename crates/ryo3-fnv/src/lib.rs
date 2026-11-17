#![doc = include_str!("../README.md")]
pub use fnv1a::RyFnv1a;
use pyo3::prelude::*;
mod fnv1a;

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<RyFnv1a>()?;
    Ok(())
}
