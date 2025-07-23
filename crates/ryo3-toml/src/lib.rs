#![doc = include_str!("../README.md")]
use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyModule};

mod serialize;

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(crate::serialize::dumps_toml, m)?)?;
    Ok(())
}
