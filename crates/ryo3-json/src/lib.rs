#![doc = include_str!("../README.md")]
use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyModule};

pub fn pymod_add(_m: &Bound<'_, PyModule>) -> PyResult<()> {
    Ok(())
}
