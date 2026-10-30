#![doc = include_str!("../README.md")]
mod gz;
pub use gz::{
    gunzip, gzip, gzip_compress, gzip_decode, gzip_decompress, gzip_encode, is_gzip, is_gzipped,
};
use pyo3::types::{PyModule, PyModuleMethods};
use pyo3::{Bound, PyResult, wrap_pyfunction};
mod compression;

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(gzip_compress, m)?)?;
    m.add_function(wrap_pyfunction!(gzip_decompress, m)?)?;
    m.add_function(wrap_pyfunction!(is_gzip, m)?)?;
    // deprecated ~ remove in v0.0.109
    m.add_function(wrap_pyfunction!(gzip_encode, m)?)?;
    m.add_function(wrap_pyfunction!(gzip_decode, m)?)?;
    m.add_function(wrap_pyfunction!(gzip, m)?)?;
    m.add_function(wrap_pyfunction!(gunzip, m)?)?;
    m.add_function(wrap_pyfunction!(is_gzipped, m)?)?;
    Ok(())
}
