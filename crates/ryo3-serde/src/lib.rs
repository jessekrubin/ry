#![doc = include_str!("../README.md")]

pub mod ser;
mod type_cache;
mod py_datetime;
mod scalars;
mod errors;
mod sequence;
mod mapping;
mod any_repr;
mod macro_rules;
mod py_uuid;

use pyo3::prelude::*;
use pyo3::types::PyModule;

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    Ok(())
}
