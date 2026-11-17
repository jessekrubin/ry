#![doc = include_str!("../README.md")]
use pyo3::prelude::*;
pub use ry_crc32::RyCrc32Fast;
mod ry_crc32;

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<RyCrc32Fast>()?;
    Ok(())
}
