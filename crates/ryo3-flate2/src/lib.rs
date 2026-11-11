#![doc = include_str!("../README.md")]
mod gz;
pub use gz::{gzip_compress, gzip_decompress, is_gzip};
use pyo3::types::{PyModule, PyModuleMethods};
use pyo3::{Bound, PyResult, wrap_pyfunction};
mod compression;

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(gzip_compress, m)?)?;
    m.add_function(wrap_pyfunction!(gzip_decompress, m)?)?;
    m.add_function(wrap_pyfunction!(is_gzip, m)?)?;
    Ok(())
}
