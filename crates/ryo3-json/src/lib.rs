#![doc = include_str!("../README.md")]

pub mod orjson;

use pyo3::prelude::*;
use pyo3::types::PyModule;
pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    orjson::pymod_add(m)?;
    Ok(())
}
