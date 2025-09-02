#![doc = include_str!("../README.md")]
use pyo3::types::{PyModule, PyModuleMethods};
use pyo3::{Bound, PyResult, pyfunction, wrap_pyfunction};

#[pyfunction(signature = (string), text_signature = "(string: str) -> list[str] | None")]
#[must_use]
pub fn shplit(string: &str) -> Option<Vec<String>> {
    shlex::split(string)
}

#[pyfunction(signature = (string), text_signature = "(string: str) -> list[str] | None")]
#[must_use]
pub fn split(string: &str) -> Option<Vec<String>> {
    shlex::split(string)
}

#[pyfunction(signature = (string, *, allow_nul = false), text_signature = "(string: str, *, allow_nul=False) -> str")]
pub fn quote(string: &str, allow_nul: bool) -> PyResult<std::borrow::Cow<'_, str>> {
    let q = shlex::Quoter::new().allow_nul(allow_nul);

    let a = q
        .quote(string)
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
    Ok(a)
}

pub fn pysubmod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(quote, m)?)?;
    m.add_function(wrap_pyfunction!(shplit, m)?)?;
    m.add_function(wrap_pyfunction!(split, m)?)?;
    Ok(())
}

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(shplit, m)?)?;
    Ok(())
}
