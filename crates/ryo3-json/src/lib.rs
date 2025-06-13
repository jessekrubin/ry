#![doc = include_str!("../README.md")]

pub mod orjson;
mod stringify;

use pyo3::prelude::*;
use pyo3::types::PyModule;

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(stringify::stringify, m)?)?;
    // m.add_wrapped(wrap_pymodule!(orjson::oj))?;
    Ok(())
}
