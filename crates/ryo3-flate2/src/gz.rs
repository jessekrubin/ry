use flate2::bufread::GzDecoder;
use flate2::write::GzEncoder;
use pyo3::prelude::*;
use ryo3_bytes::PyBytes;
use std::io::{Read, Write};

use crate::compression::PyCompression;

fn rs_gzip_encode(data: &[u8], quality: PyCompression) -> PyResult<PyBytes> {
    let mut gzip_encoder = GzEncoder::new(Vec::new(), quality.0);
    gzip_encoder.write_all(data)?;
    let encoded = gzip_encoder.finish()?;
    Ok(encoded.into())
}

fn rs_gzip_decode(data: &[u8]) -> PyResult<PyBytes> {
    let mut decompressed = Vec::new();
    GzDecoder::new(data).read_to_end(&mut decompressed)?;
    Ok(PyBytes::from(decompressed))
}

#[pyfunction]
#[pyo3(signature = (data, quality=PyCompression::default()))]
#[expect(clippy::needless_pass_by_value)]
pub fn gzip_encode(py: Python<'_>, data: PyBytes, quality: PyCompression) -> PyResult<PyBytes> {
    let bin: &[u8] = data.as_ref();
    py.detach(|| rs_gzip_encode(bin, quality))
}

#[pyfunction]
#[expect(clippy::needless_pass_by_value)]
pub fn gzip_decode(py: Python<'_>, data: PyBytes) -> PyResult<PyBytes> {
    let bin: &[u8] = data.as_ref();
    py.detach(|| rs_gzip_decode(bin))
}

// aliases...
#[pyfunction]
#[pyo3(signature = (data, quality=PyCompression::default()))]
#[expect(clippy::needless_pass_by_value)]
pub fn gzip(py: Python<'_>, data: PyBytes, quality: PyCompression) -> PyResult<PyBytes> {
    let bin: &[u8] = data.as_ref();
    py.detach(|| rs_gzip_encode(bin, quality))
}

#[pyfunction]
#[expect(clippy::needless_pass_by_value)]
pub fn gunzip(data: PyBytes) -> PyResult<PyBytes> {
    rs_gzip_decode(data.as_ref())
}

#[pyfunction]
#[expect(clippy::needless_pass_by_value)]
pub fn is_gzipped(data: PyBytes) -> bool {
    let bin: &[u8] = data.as_ref();
    bin.starts_with(b"\x1F\x8B")
}
