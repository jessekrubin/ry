use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;
use pyo3::prelude::*;
use pyo3::types::PyBytes;
use ryo3_macro_rules::py_value_error;
use std::io::{Read, Write};

pub fn rs_gzip_encode(py: Python<'_>, data: &[u8], quality: Option<u32>) -> PyResult<PyObject> {
    let quality = if let Some(param) = quality {
        if param > 9 {
            return Err(py_value_error!(
                "Quality must be between 0 and 9 - got: {param:?}"
            ));
        }
        Compression::new(param)
    } else {
        Compression::default()
    };
    let mut gzip_encoder = GzEncoder::new(Vec::new(), quality);
    gzip_encoder
        .write_all(data)
        .map_err(|e| py_value_error!("gzip-decode-error: {e:?}"))?;
    let encoded = gzip_encoder
        .finish()
        .map_err(|e| py_value_error!("gzip-decode-error: {e:?}"))?;
    Ok(PyBytes::new(py, &encoded).into())
}

pub fn rs_gzip_decode(data: &[u8]) -> PyResult<ryo3_bytes::PyBytes> {
    let mut decompressed = Vec::new();
    GzDecoder::new(data)
        .read_to_end(&mut decompressed)
        .map_err(|e| py_value_error!("gzip-decode-error: {e:?}"))?;
    Ok(ryo3_bytes::PyBytes::from(decompressed))
}

#[pyfunction]
#[pyo3(signature = (data, quality=None))]
#[expect(clippy::needless_pass_by_value)]
pub fn gzip_encode(
    py: Python<'_>,
    data: ryo3_bytes::PyBytes,
    quality: Option<u32>,
) -> PyResult<PyObject> {
    let bin: &[u8] = data.as_ref();
    rs_gzip_encode(py, bin, quality)
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
pub fn gzip(py: Python<'_>, data: ryo3_bytes::PyBytes, quality: Option<u32>) -> PyResult<PyObject> {
    let data: &[u8] = data.as_ref();
    rs_gzip_encode(py, data, quality)
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
    bin.starts_with(&[0x1F, 0x8B])
}
