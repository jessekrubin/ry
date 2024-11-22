use pyo3::prelude::*;
use pyo3::types::PyModule;
use pyo3::PyResult;

pub mod fileio;
pub mod fspath;

pub fn madd(m: &Bound<'_, PyModule>) -> PyResult<()> {
    fileio::madd(m)?;
    fspath::madd(m)?;
    Ok(())
}
