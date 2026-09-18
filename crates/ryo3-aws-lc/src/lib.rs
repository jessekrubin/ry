#![doc = include_str!("../README.md")]
mod digest;

pub use aws_lc_rs::awslc_version;
use pyo3::prelude::*;

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    let __aws_lc_version__: &str = aws_lc_rs::awslc_version();
    m.add("__aws_lc_version__", __aws_lc_version__)?;
    digest::pymod_add(m)?;
    Ok(())
}
