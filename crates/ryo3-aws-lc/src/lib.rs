#![doc = include_str!("../README.md")]
mod digest;
use pyo3::prelude::*;

#[must_use]
#[pyfunction]
pub fn awslc_version() -> &'static str {
    aws_lc_rs::awslc_version()
}

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    digest::pymod_add(m)?;
    m.add_function(wrap_pyfunction!(awslc_version, m)?)?;
    Ok(())
}
