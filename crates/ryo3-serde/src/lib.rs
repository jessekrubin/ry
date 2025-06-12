#![doc = include_str!("../README.md")]

pub mod ser;

use pyo3::prelude::*;
use pyo3::types::PyModule;

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    Ok(())
}
