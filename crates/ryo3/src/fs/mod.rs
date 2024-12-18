use pyo3::prelude::*;
use pyo3::types::PyModule;
use pyo3::PyResult;

pub mod fspath;

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // fileio::pymod_add(m)?;
    fspath::pymod_add(m)?;
    Ok(())
}
