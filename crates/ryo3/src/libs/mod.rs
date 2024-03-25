use pyo3::types::PyModule;
use pyo3::{PyResult, Python};

#[cfg(feature = "jiter")]
mod jiter_ry;

#[cfg(feature = "brotli")]
mod brotli;

pub fn madd(_py: Python, _m: &PyModule) -> PyResult<()> {
    #[cfg(feature = "jiter")]
    jiter_ry::madd(_py, _m)?;

    #[cfg(feature = "brotli")]
    brotli::madd(_py, _m)?;

    Ok(())
}
