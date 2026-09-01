#![doc = include_str!("../README.md")]
pub use py_crc32::PyCrc32Fast;
use pyo3::prelude::*;
mod py_crc32;

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyCrc32Fast>()?;
    Ok(())
}
