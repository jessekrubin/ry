use pyo3::prelude::*;
use pyo3::PyResult;
#[cfg(feature = "bzip2")]
mod bzip2;
#[cfg(feature = "flate2")]
mod flate2;
#[cfg(feature = "fnv")]
mod fnv;
#[cfg(feature = "globset")]
mod globset;
#[cfg(feature = "walkdir")]
mod walkdir;
#[cfg(feature = "xxhash")]
mod xxhash;
#[cfg(feature = "zstd")]
mod zstd;

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    #[cfg(feature = "brotli")]
    ryo3_brotli::pymod_add(m)?;
    #[cfg(feature = "jiff")]
    ryo3_jiff::pymod_add(m)?;
    #[cfg(feature = "jiter")]
    ryo3_jiter::pymod_add(m)?;
    #[cfg(feature = "shlex")]
    ryo3_shlex::pymod_add(m)?;
    #[cfg(feature = "sqlformat")]
    ryo3_sqlformat::pymod_add(m)?;
    #[cfg(feature = "which")]
    ryo3_which::pymod_add(m)?;

    // TODO: move the below libs to their own crates
    #[cfg(feature = "bzip2")]
    bzip2::pymod_add(m)?;

    #[cfg(feature = "fnv")]
    fnv::pymod_add(m)?;

    #[cfg(feature = "flate2")]
    flate2::pymod_add(m)?;

    #[cfg(feature = "globset")]
    globset::pymod_add(m)?;

    #[cfg(feature = "walkdir")]
    walkdir::pymod_add(m)?;

    #[cfg(feature = "xxhash")]
    xxhash::pymod_add(m)?;

    #[cfg(feature = "zstd")]
    zstd::pymod_add(m)?;

    Ok(())
}
