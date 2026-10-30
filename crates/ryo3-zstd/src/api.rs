use pyo3::prelude::*;
use zstd::zstd_safe;

use crate::oneshot;

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    let version = zstd_safe::version_number();
    m.add("ZSTD_VERSION", zstd_safe::version_string())?;
    m.add(
        "ZSTD_VERSION_INFO",
        (version / 10_000, (version / 100) % 100, version % 100),
    )?;
    m.add_function(wrap_pyfunction!(oneshot::zstd_compress, m)?)?;
    m.add_function(wrap_pyfunction!(oneshot::zstd_decompress, m)?)?;
    m.add_function(wrap_pyfunction!(oneshot::is_zstd, m)?)?;
    // deprecated ~ remove in v0.0.109
    m.add_function(wrap_pyfunction!(oneshot::zstd_decode, m)?)?;
    m.add_function(wrap_pyfunction!(oneshot::zstd_encode, m)?)?;
    Ok(())
}
