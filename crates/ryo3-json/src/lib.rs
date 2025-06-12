#![doc = include_str!("../README.md")]

pub mod orjson;
mod stringify;

use pyo3::prelude::*;
use pyo3::types::PyModule;
pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(stringify::stringify, m)?)?;
    orjson::pymod_add(m)?;
    Ok(())
}
