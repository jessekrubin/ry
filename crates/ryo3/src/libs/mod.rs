use pyo3::prelude::*;
use pyo3::PyResult;

#[cfg(feature = "jiter")]
mod jiter_ry;

#[cfg(feature = "brotli")]
mod brotli;

pub fn madd(m: &Bound<'_, PyModule>) -> PyResult<()> {
    #[cfg(feature = "jiter")]
    jiter_ry::madd(m)?;

    #[cfg(feature = "brotli")]
    brotli::madd(m)?;

    Ok(())
}
