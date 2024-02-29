use pyo3::prelude::*;
use pyo3::types::PyModule;
use pyo3::PyResult;

pub mod fileio;
pub mod fspath;

pub fn madd(_py: Python, m: &PyModule) -> PyResult<()> {
    fileio::madd(_py, m)?;
    fspath::madd(_py, m)?;
    Ok(())
}
