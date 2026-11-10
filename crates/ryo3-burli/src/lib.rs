#![doc = include_str!("../README.md")]
use pyo3::prelude::*;

mod compress;

pub use compress::{PyBrotliLevel, brotli_compress};

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(compress::brotli_compress, m)?)?;
    Ok(())
}
