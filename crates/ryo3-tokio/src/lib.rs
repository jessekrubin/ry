#![doc = include_str!("../README.md")]

pub mod fs;
pub mod time;

use pyo3::prelude::*;
use pyo3::types::PyModule;
use pyo3::{PyResult, wrap_pyfunction};

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(time::asleep, m)?)?;
    m.add_function(wrap_pyfunction!(time::sleep_async, m)?)?;

    // fs
    fs::pymod_add(m)?;

    Ok(())
}
