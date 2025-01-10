#![doc = include_str!("../README.md")]

pub mod time;

use pyo3::prelude::*;
use pyo3::types::PyModule;
use pyo3::{wrap_pyfunction, PyResult};

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(time::asleep, m)?)?;
    m.add_function(wrap_pyfunction!(time::sleep_async, m)?)?;
    Ok(())
}
