#![doc = include_str!("../README.md")]
use std::io::Read;

use ::brotli as br;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use ryo3_bytes::{ReadableBuffer, RyBytes};

fn brotli_decompress_impl(data: &[u8]) -> PyResult<Vec<u8>> {
    let mut decompressed = Vec::new();
    br::Decompressor::new(data, 4 * 1024)
        .read_to_end(&mut decompressed)
        .map_err(|e| PyValueError::new_err(format!("Brotli decode error: {e:?}")))?;
    Ok(decompressed)
}

#[expect(clippy::needless_pass_by_value)]
#[pyfunction]
#[pyo3(signature = (data, /))]
pub fn brotli_decompress(py: Python<'_>, data: ReadableBuffer) -> PyResult<RyBytes> {
    let bin: &[u8] = data.as_ref();
    py.detach(|| brotli_decompress_impl(bin)).map(RyBytes::from)
}

// ============================================================================
// DEPRECATED ~ remove in v0.0.109
// ============================================================================

#[pyfunction]
#[pyo3(warn(
    message = "`brotli_decode` is deprecated; use `brotli_decompress` instead [removal: v0.0.109]",
    category = pyo3::exceptions::PyDeprecationWarning
))]
pub fn brotli_decode(py: Python<'_>, data: ReadableBuffer) -> PyResult<RyBytes> {
    brotli_decompress(py, data)
}

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(brotli_decompress, m)?)?;
    // deprecated ~ remove in v0.0.109
    m.add_function(wrap_pyfunction!(brotli_decode, m)?)?;
    Ok(())
}
