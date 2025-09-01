use flate2::bufread::GzDecoder;
use flate2::write::GzEncoder;
use pyo3::prelude::*;
use pyo3::types::PyBytes;
use std::io::{Read, Write};

use crate::compression::PyCompression;

fn rs_gzip_encode(py: Python<'_>, data: &[u8], quality: PyCompression) -> PyResult<Py<PyAny>> {
    let mut gzip_encoder = GzEncoder::new(Vec::new(), quality.0);
    gzip_encoder.write_all(data)?;
    let encoded = gzip_encoder.finish()?;
    Ok(PyBytes::new(py, &encoded).into())
}

pub fn rs_gzip_decode(data: &[u8]) -> PyResult<ryo3_bytes::PyBytes> {
    let mut decompressed = Vec::new();
    GzDecoder::new(data).read_to_end(&mut decompressed)?;
    Ok(ryo3_bytes::PyBytes::from(decompressed))
}

#[pyfunction]
#[pyo3(signature = (data, quality=None))]
#[expect(clippy::needless_pass_by_value)]
pub fn gzip_encode(
    py: Python<'_>,
    data: ryo3_bytes::PyBytes,
    quality: Option<PyCompression>,
) -> PyResult<Py<PyAny>> {
    let bin: &[u8] = data.as_ref();
    rs_gzip_encode(py, bin, quality.unwrap_or_default())
}

#[pyfunction]
#[expect(clippy::needless_pass_by_value)]
pub fn gzip_decode(data: ryo3_bytes::PyBytes) -> PyResult<ryo3_bytes::PyBytes> {
    let bin: &[u8] = data.as_ref();
    rs_gzip_decode(bin)
}

// aliases...
#[pyfunction]
#[pyo3(signature = (data, quality=None))]
#[expect(clippy::needless_pass_by_value)]
pub fn gzip(
    py: Python<'_>,
    data: ryo3_bytes::PyBytes,
    quality: Option<PyCompression>,
) -> PyResult<Py<PyAny>> {
    let bin: &[u8] = data.as_ref();
    rs_gzip_encode(py, bin, quality.unwrap_or_default())
}

#[pyfunction]
#[expect(clippy::needless_pass_by_value)]
pub fn gunzip(data: ryo3_bytes::PyBytes) -> PyResult<ryo3_bytes::PyBytes> {
    rs_gzip_decode(data.as_ref())
}

#[pyfunction]
#[expect(clippy::needless_pass_by_value)]
pub fn is_gzipped(data: ryo3_bytes::PyBytes) -> bool {
    let bin: &[u8] = data.as_ref();
    bin.starts_with(b"\x1F\x8B")
}
