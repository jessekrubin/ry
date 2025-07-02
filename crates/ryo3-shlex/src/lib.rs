#![doc = include_str!("../README.md")]
use pyo3::types::{PyModule, PyModuleMethods};
use pyo3::{Bound, PyResult, pyfunction, wrap_pyfunction};

#[pyfunction(signature = (string), text_signature = "(string: str) -> list[str] | None")]
#[must_use]
pub fn shplit(string: &str) -> Option<Vec<String>> {
    shlex::split(string)
}

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(shplit, m)?)?;
    Ok(())
}
