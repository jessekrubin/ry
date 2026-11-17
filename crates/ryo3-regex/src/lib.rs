#![doc = include_str!("../README.md")]

mod py_regex_options;
mod ry_regex;

use pyo3::prelude::*;
pub use regex::Regex;

pub use crate::ry_regex::RyRegex;

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<RyRegex>()?;
    Ok(())
}
