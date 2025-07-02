#![doc = include_str!("../README.md")]

mod py_regex;
mod py_regex_options;

pub use crate::py_regex::PyRegex;
use pyo3::PyResult;
use pyo3::prelude::*;
use pyo3::types::PyModule;

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyRegex>()?;
    Ok(())
}
