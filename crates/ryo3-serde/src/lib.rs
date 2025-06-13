#![doc = include_str!("../README.md")]

mod any_repr;
mod errors;
mod macro_rules;
mod pytypes;
mod rytypes;
pub mod ser;
mod type_cache;

use pyo3::prelude::*;
use pyo3::types::PyModule;

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    Ok(())
}
