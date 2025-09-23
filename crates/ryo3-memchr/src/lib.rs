#![doc = include_str!("../README.md")]
use pyo3::prelude::*;
use pyo3::types::PyModule;
mod py_memchr;

pub use crate::py_memchr::{memchr, memchr2, memchr3, memrchr, memrchr2, memrchr3};

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // memchr
    m.add_function(wrap_pyfunction!(crate::py_memchr::memchr, m)?)?;
    m.add_function(wrap_pyfunction!(crate::py_memchr::memchr2, m)?)?;
    m.add_function(wrap_pyfunction!(crate::py_memchr::memchr3, m)?)?;
    m.add_function(wrap_pyfunction!(crate::py_memchr::memrchr, m)?)?;
    m.add_function(wrap_pyfunction!(crate::py_memchr::memrchr2, m)?)?;
    m.add_function(wrap_pyfunction!(crate::py_memchr::memrchr3, m)?)?;
    // memmem ~ TODO
    Ok(())
}
