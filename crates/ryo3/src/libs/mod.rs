use pyo3::prelude::*;
use pyo3::PyResult;

#[cfg(feature = "brotli")]
mod brotli;

#[cfg(feature = "fnv")]
mod fnv;

#[cfg(feature = "jiter")]
mod jiter_ry;

#[cfg(feature = "which")]
mod which;
#[cfg(feature = "xxhash")]
mod xxhash;

pub fn madd(m: &Bound<'_, PyModule>) -> PyResult<()> {
    #[cfg(feature = "brotli")]
    brotli::madd(m)?;

    #[cfg(feature = "fnv")]
    fnv::madd(m)?;

    #[cfg(feature = "jiter")]
    jiter_ry::madd(m)?;

    #[cfg(feature = "which")]
    which::madd(m)?;

    #[cfg(feature = "xxhash")]
    xxhash::madd(m)?;

    Ok(())
}
