use pyo3::prelude::*;

// #[cfg(feature = "jiff")]
// pub fn pymod_add_jiff(m: &Bound<'_, PyModule>) -> PyResult<()> {
//     // classes
//     m.add_class::<ryo3_jiff::RyDate>()?;
//     m.add_class::<ryo3_jiff::RyDateTime>()?;
//     m.add_class::<ryo3_jiff::RySignedDuration>()?;
//     m.add_class::<ryo3_jiff::RySpan>()?;
//     m.add_class::<ryo3_jiff::RyTime>()?;
//     m.add_class::<ryo3_jiff::RyTimeZone>()?;
//     m.add_class::<ryo3_jiff::RyTimestamp>()?;
//     m.add_class::<ryo3_jiff::RyZoned>()?;
//     m.add_class::<ryo3_jiff::RyOffset>()?;
//
//     // difference
//     m.add_class::<ryo3_jiff::RyDateDifference>()?;
//     m.add_class::<ryo3_jiff::RyDateTimeDifference>()?;
//     m.add_class::<ryo3_jiff::RyTimeDifference>()?;
//     m.add_class::<ryo3_jiff::RyTimestampDifference>()?;
//
//     // round
//     m.add_class::<ryo3_jiff::RyDateTimeRound>()?;
//
//     // functions
//     m.add_function(wrap_pyfunction!(ryo3_jiff::date, m)?)?;
//     m.add_function(wrap_pyfunction!(ryo3_jiff::time, m)?)?;
//     m.add_function(wrap_pyfunction!(ryo3_jiff::datetime, m)?)?;
//     m.add_function(wrap_pyfunction!(ryo3_jiff::offset, m)?)?;
//     m.add_function(wrap_pyfunction!(ryo3_jiff::timespan, m)?)?;
//
//
//     ryo3_jiff::pymod_add(m)?;
//     Ok(())
// }
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
    #[cfg(feature = "regex")]
    ryo3_regex::pymod_add(m)?;
    #[cfg(feature = "shlex")]
    ryo3_shlex::pymod_add(m)?;
    #[cfg(feature = "sqlformat")]
    ryo3_sqlformat::pymod_add(m)?;
    #[cfg(feature = "unindent")]
    ryo3_unindent::pymod_add(m)?;
    #[cfg(feature = "url")]
    ryo3_url::pymod_add(m)?;
    #[cfg(feature = "walkdir")]
    ryo3_walkdir::pymod_add(m)?;
    #[cfg(feature = "which")]
    ryo3_which::pymod_add(m)?;
    // #[cfg(feature = "xxhash")]
    // ryo3_xxhash::pymod_add(m)?;
    #[cfg(feature = "zstd")]
    ryo3_zstd::pymod_add(m)?;
    Ok(())
}
