use std::io::{Read, Write};

use flate2::bufread::GzDecoder;
use flate2::write::GzEncoder;
use pyo3::prelude::*;
use ryo3_bytes::{ReadableBuffer, RyBytes};

use crate::compression::PyCompression;

fn rs_gzip_encode(data: &[u8], quality: PyCompression) -> PyResult<RyBytes> {
    let mut gzip_encoder = GzEncoder::new(Vec::new(), quality.0);
    gzip_encoder.write_all(data)?;
    let encoded = gzip_encoder.finish()?;
    Ok(encoded.into())
}

fn rs_gzip_decode(data: &[u8]) -> PyResult<RyBytes> {
    let mut decompressed = Vec::new();
    GzDecoder::new(data).read_to_end(&mut decompressed)?;
    Ok(RyBytes::from(decompressed))
}

#[expect(clippy::needless_pass_by_value)]
#[pyfunction]
#[pyo3(
    signature = (data, quality=PyCompression::default()),
    text_signature = "(data, quality=6)"
)]
pub fn gzip_encode(
    py: Python<'_>,
    data: ReadableBuffer,
    quality: PyCompression,
) -> PyResult<RyBytes> {
    let bin: &[u8] = data.as_ref();
    py.detach(|| rs_gzip_encode(bin, quality))
}

#[pyfunction]
#[expect(clippy::needless_pass_by_value)]
pub fn gzip_decode(py: Python<'_>, data: ReadableBuffer) -> PyResult<RyBytes> {
    let bin: &[u8] = data.as_ref();
    py.detach(|| rs_gzip_decode(bin))
}

// aliases...
#[expect(clippy::needless_pass_by_value)]
#[pyfunction]
#[pyo3(
    signature = (data, quality=PyCompression::default()),
    text_signature = "(data, quality=6)"
)]
pub fn gzip(py: Python<'_>, data: ReadableBuffer, quality: PyCompression) -> PyResult<RyBytes> {
    let bin: &[u8] = data.as_ref();
    py.detach(|| rs_gzip_encode(bin, quality))
}

#[expect(clippy::needless_pass_by_value)]
#[pyfunction]
pub fn gunzip(data: ReadableBuffer) -> PyResult<RyBytes> {
    let bin: &[u8] = data.as_ref();
    rs_gzip_decode(bin)
}

#[expect(clippy::needless_pass_by_value)]
#[pyfunction]
pub fn is_gzipped(data: ReadableBuffer) -> bool {
    let bin: &[u8] = data.as_ref();
    bin.starts_with(b"\x1F\x8B")
}
