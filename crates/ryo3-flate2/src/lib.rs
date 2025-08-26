#![doc = include_str!("../README.md")]
mod gz;
pub use gz::*;
use pyo3::prelude::PyModule;
use pyo3::prelude::*;
mod compression;

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(gzip_encode, m)?)?;
    m.add_function(wrap_pyfunction!(gzip_decode, m)?)?;
    m.add_function(wrap_pyfunction!(gzip, m)?)?;
    m.add_function(wrap_pyfunction!(gunzip, m)?)?;
    m.add_function(wrap_pyfunction!(is_gzipped, m)?)?;
    Ok(())
}
