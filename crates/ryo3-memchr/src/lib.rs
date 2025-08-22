#![doc = include_str!("../README.md")]
use pyo3::prelude::*;
use pyo3::types::PyModule;
pub mod byte;
mod py_memchr;

pub use crate::py_memchr::memchr;

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(crate::py_memchr::memchr, m)?)?;
    Ok(())
}
