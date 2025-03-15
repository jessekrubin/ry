use crate::{constants, oneshot};
use pyo3::prelude::PyModule;
use pyo3::prelude::*;
use pyo3::{Bound, PyResult};

pub fn pysubmod_register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    constants::pymod_add(m)?;
    m.add_function(wrap_pyfunction!(oneshot::is_zstd, m)?)?;
    m.add_function(wrap_pyfunction!(oneshot::compress, m)?)?;
    m.add_function(wrap_pyfunction!(oneshot::decode, m)?)?;
    m.add_function(wrap_pyfunction!(oneshot::decompress, m)?)?;
    m.add_function(wrap_pyfunction!(oneshot::unzstd, m)?)?;
    Ok(())
}

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(oneshot::is_zstd, m)?)?;
    m.add_function(wrap_pyfunction!(oneshot::zstd_decode, m)?)?;
    m.add_function(wrap_pyfunction!(oneshot::zstd_encode, m)?)?;
    m.add_function(wrap_pyfunction!(oneshot::zstd_decompress, m)?)?;
    m.add_function(wrap_pyfunction!(oneshot::zstd_compress, m)?)?;
    Ok(())
}
