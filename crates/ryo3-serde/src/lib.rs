#![doc = include_str!("../README.md")]

mod any_repr;
mod errors;
mod macro_rules;
pub mod pyser;
mod pytypes;
mod rytypes;
mod type_cache;

use pyo3::prelude::*;
use pyo3::types::PyModule;

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    Ok(())
}
