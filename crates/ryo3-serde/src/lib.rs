#![doc = include_str!("../README.md")]

// mod pyany_serialize;

use pyo3::prelude::*;
use pyo3::types::PyModule;

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    Ok(())
}
