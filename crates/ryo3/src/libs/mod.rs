use pyo3::prelude::*;

#[cfg(feature = "walkdir")]
mod walkdir;
pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    #[cfg(feature = "brotli")]
    ryo3_brotli::pymod_add(m)?;
    #[cfg(feature = "bzip2")]
    ryo3_bzip2::pymod_add(m)?;
    #[cfg(feature = "flate2")]
    ryo3_flate2::pymod_add(m)?;
    #[cfg(feature = "fnv")]
    ryo3_fnv::pymod_add(m)?;
    #[cfg(feature = "globset")]
    ryo3_globset::pymod_add(m)?;
    #[cfg(feature = "heck")]
    ryo3_heck::pymod_add(m)?;
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
    #[cfg(feature = "xxhash")]
    ryo3_xxhash::pymod_add(m)?;
    #[cfg(feature = "zstd")]
    ryo3_zstd::pymod_add(m)?;


    // TODO: move the below libs to their own crates
    #[cfg(feature = "walkdir")]
    walkdir::pymod_add(m)?;

    Ok(())
}
