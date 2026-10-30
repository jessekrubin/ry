use pyo3::prelude::*;
use ryo3_bytes::{ReadableBuffer, RyBytes};

use crate::compression_level::PyCompressionLevel;

fn zstd_compress_impl(data: &[u8], level: i32) -> PyResult<Vec<u8>> {
    ::zstd::bulk::compress(data, level)
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(format!("zstd-encode-error: {e:?}")))
}

fn zstd_decompress_impl(data: &[u8]) -> PyResult<Vec<u8>> {
    ::zstd::stream::decode_all(data)
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(format!("zstd-decode-error: {e:?}")))
}

#[expect(clippy::needless_pass_by_value)]
#[pyfunction]
#[pyo3(
    signature = (data, /, level = PyCompressionLevel::default()),
    text_signature = "(data, /, level=3)"
)]
pub fn zstd_compress(
    py: Python<'_>,
    data: ReadableBuffer,
    level: PyCompressionLevel,
) -> PyResult<RyBytes> {
    let bin = data.as_slice();
    py.detach(|| zstd_compress_impl(bin, level.into_inner()))
        .map(RyBytes::from)
}

#[expect(clippy::needless_pass_by_value)]
#[pyfunction]
#[pyo3(signature = (data, /))]
pub fn zstd_decompress(py: Python<'_>, data: ReadableBuffer) -> PyResult<RyBytes> {
    let bin = data.as_slice();
    py.detach(|| zstd_decompress_impl(bin)).map(RyBytes::from)
}

#[expect(clippy::needless_pass_by_value)]
#[pyfunction]
#[pyo3(signature = (data, /))]
#[must_use]
pub fn is_zstd(data: ReadableBuffer) -> bool {
    data.as_slice().starts_with(b"\x28\xB5\x2F\xFD")
}

// ============================================================================
// DEPRECATED ~ remove in v0.0.109
// ============================================================================

#[pyfunction]
#[pyo3(
    signature = (data, level = PyCompressionLevel::default()),
    text_signature = "(data, level=3)",
    warn(
        message = "`zstd_encode` is deprecated; use `zstd_compress` instead [removal: v0.0.109]",
        category = pyo3::exceptions::PyDeprecationWarning
    )
)]
pub fn zstd_encode(
    py: Python<'_>,
    data: ReadableBuffer,
    level: PyCompressionLevel,
) -> PyResult<RyBytes> {
    zstd_compress(py, data, level)
}

#[pyfunction]
#[pyo3(warn(
    message = "`zstd_decode` is deprecated; use `zstd_decompress` instead [removal: v0.0.109]",
    category = pyo3::exceptions::PyDeprecationWarning
))]
pub fn zstd_decode(py: Python<'_>, data: ReadableBuffer) -> PyResult<RyBytes> {
    zstd_decompress(py, data)
}
