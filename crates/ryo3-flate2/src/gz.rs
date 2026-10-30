use std::io::{Read, Write};

use flate2::bufread::GzDecoder;
use flate2::write::GzEncoder;
use pyo3::prelude::*;
use ryo3_bytes::{ReadableBuffer, RyBytes};

use crate::compression::PyCompression;

fn gzip_compress_impl(data: &[u8], level: PyCompression) -> PyResult<Vec<u8>> {
    let mut gzip_encoder = GzEncoder::new(Vec::new(), level.into_inner());
    gzip_encoder.write_all(data)?;
    let encoded = gzip_encoder.finish()?;
    Ok(encoded)
}

fn gzip_decompress_impl(data: &[u8]) -> PyResult<Vec<u8>> {
    let mut decompressed = Vec::new();
    GzDecoder::new(data).read_to_end(&mut decompressed)?;
    Ok(decompressed)
}

#[expect(clippy::needless_pass_by_value)]
#[pyfunction]
#[pyo3(
    signature = (data, /, level = PyCompression::default()),
    text_signature = "(data, /, level=6)"
)]
pub fn gzip_compress(
    py: Python<'_>,
    data: ReadableBuffer,
    level: PyCompression,
) -> PyResult<RyBytes> {
    let bin: &[u8] = data.as_ref();
    py.detach(|| gzip_compress_impl(bin, level))
        .map(RyBytes::from)
}

#[expect(clippy::needless_pass_by_value)]
#[pyfunction]
#[pyo3(signature = (data, /))]
pub fn gzip_decompress(py: Python<'_>, data: ReadableBuffer) -> PyResult<RyBytes> {
    let bin: &[u8] = data.as_ref();
    py.detach(|| gzip_decompress_impl(bin)).map(RyBytes::from)
}

#[expect(clippy::needless_pass_by_value)]
#[pyfunction]
#[pyo3(signature = (data, /))]
#[must_use]
pub fn is_gzip(data: ReadableBuffer) -> bool {
    let bin: &[u8] = data.as_ref();
    bin.starts_with(b"\x1F\x8B")
}

// ============================================================================
// DEPRECATED ~ remove in v0.0.109
// ============================================================================

#[pyfunction]
#[pyo3(
    signature = (data, quality = PyCompression::default()),
    text_signature = "(data, quality=6)",
    warn(
        message = "`gzip_encode` is deprecated; use `gzip_compress` instead [removal: v0.0.109]",
        category = pyo3::exceptions::PyDeprecationWarning
    )
)]
pub fn gzip_encode(
    py: Python<'_>,
    data: ReadableBuffer,
    quality: PyCompression,
) -> PyResult<RyBytes> {
    gzip_compress(py, data, quality)
}

#[pyfunction]
#[pyo3(warn(
    message = "`gzip_decode` is deprecated; use `gzip_decompress` instead [removal: v0.0.109]",
    category = pyo3::exceptions::PyDeprecationWarning
))]
pub fn gzip_decode(py: Python<'_>, data: ReadableBuffer) -> PyResult<RyBytes> {
    gzip_decompress(py, data)
}

#[pyfunction]
#[pyo3(
    signature = (data, quality = PyCompression::default()),
    text_signature = "(data, quality=6)",
    warn(
        message = "`gzip` is deprecated; use `gzip_compress` instead [removal: v0.0.109]",
        category = pyo3::exceptions::PyDeprecationWarning
    )
)]
pub fn gzip(py: Python<'_>, data: ReadableBuffer, quality: PyCompression) -> PyResult<RyBytes> {
    gzip_compress(py, data, quality)
}

#[pyfunction]
#[pyo3(warn(
    message = "`gunzip` is deprecated; use `gzip_decompress` instead [removal: v0.0.109]",
    category = pyo3::exceptions::PyDeprecationWarning
))]
pub fn gunzip(py: Python<'_>, data: ReadableBuffer) -> PyResult<RyBytes> {
    gzip_decompress(py, data)
}

#[pyfunction]
#[pyo3(warn(
    message = "`is_gzipped` is deprecated; use `is_gzip` instead [removal: v0.0.109]",
    category = pyo3::exceptions::PyDeprecationWarning
))]
#[must_use]
pub fn is_gzipped(data: ReadableBuffer) -> bool {
    is_gzip(data)
}
