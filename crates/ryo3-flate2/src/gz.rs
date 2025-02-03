use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;
use pyo3::prelude::*;
use pyo3::types::PyBytes;
use ryo3_macros::py_value_error;
use std::io::{Read, Write};

#[pyfunction]
#[pyo3(signature = (data, quality=None))]
pub fn gzip_encode(py: Python<'_>, data: &[u8], quality: Option<u32>) -> PyResult<PyObject> {
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

#[pyfunction]
pub fn gzip_decode(py: Python<'_>, data: &[u8]) -> PyResult<PyObject> {
    let mut decompressed = Vec::new();
    GzDecoder::new(data)
        .read_to_end(&mut decompressed)
        .map_err(|e| py_value_error!("gzip-decode-error: {e:?}"))?;
    Ok(PyBytes::new(py, &decompressed).into())
}

// aliases...
#[pyfunction]
#[pyo3(signature = (data, quality=None))]
pub fn gzip(py: Python<'_>, data: &[u8], quality: Option<u32>) -> PyResult<PyObject> {
    gzip_encode(py, data, quality)
}

#[pyfunction]
pub fn gunzip(py: Python<'_>, data: &[u8]) -> PyResult<PyObject> {
    gzip_decode(py, data)
}
