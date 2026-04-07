#![doc = include_str!("../README.md")]

mod py_regex;
mod py_regex_options;

use pyo3::prelude::*;

pub use crate::py_regex::PyRegex;

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyRegex>()?;
    Ok(())
}
