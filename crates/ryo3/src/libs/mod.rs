use pyo3::prelude::*;
use pyo3::PyResult;

#[cfg(feature = "brotli")]
mod brotli;
#[cfg(feature = "bzip2")]
mod bzip2;
#[cfg(feature = "flate2")]
mod flate2;
#[cfg(feature = "fnv")]
mod fnv;
#[cfg(feature = "jiter")]
mod jiter_ry;
#[cfg(feature = "shlex")]
mod shlex;
#[cfg(feature = "walkdir")]
mod walkdir;
#[cfg(feature = "which")]
mod which;
#[cfg(feature = "xxhash")]
mod xxhash;
#[cfg(feature = "zstd")]
mod zstd;

pub fn madd(m: &Bound<'_, PyModule>) -> PyResult<()> {
    #[cfg(feature = "brotli")]
    brotli::madd(m)?;

    #[cfg(feature = "bzip2")]
    bzip2::madd(m)?;

    #[cfg(feature = "fnv")]
    fnv::madd(m)?;

    #[cfg(feature = "flate2")]
    flate2::madd(m)?;

    #[cfg(feature = "jiter")]
    jiter_ry::madd(m)?;

    #[cfg(feature = "shlex")]
    shlex::madd(m)?;

    #[cfg(feature = "walkdir")]
    walkdir::madd(m)?;

    #[cfg(feature = "which")]
    which::madd(m)?;

    #[cfg(feature = "xxhash")]
    xxhash::madd(m)?;

    #[cfg(feature = "zstd")]
    zstd::madd(m)?;

    Ok(())
}
