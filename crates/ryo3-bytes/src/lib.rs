#![doc = include_str!("../README.md")]

mod pyo3_bytes;
mod ry_bytes;

use pyo3::prelude::*;
pub use pyo3_bytes::Pyo3Bytes;
pub mod bytes;
pub use ry_bytes::RyBytes;

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<RyBytes>()?;
    Ok(())
}
