#![doc = include_str!("../README.md")]
use pyo3::prelude::*;
use pyo3::types::PyModule;
use pyo3::{PyResult, wrap_pyfunction};
use ryo3_core::types::PathLike;

/// Return `True` if pathlike points to same file
#[pyfunction]
pub fn is_same_file(py: Python<'_>, left: PathLike, right: PathLike) -> PyResult<bool> {
    py.detach(|| same_file::is_same_file(left, right))
        .map_err(|e| pyo3::exceptions::PyOSError::new_err(format!("is_same_file error: {e}")))
}

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(is_same_file, m)?)?;
    Ok(())
}
