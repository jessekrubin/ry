use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;
use pyo3::prelude::PyModule;
use pyo3::prelude::*;
use pyo3::types::PyBytes;
use std::io::{Read, Write};

#[pyfunction]
pub fn gzip_encode(py: Python<'_>, data: &[u8], quality: Option<u32>) -> PyResult<PyObject> {
    let quality = if let Some(param) = quality {
        if param > 9 {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "The optional second argument to gzip() must be between 0 and 9",
            ));
        }
        Compression::new(param)
    } else {
        Compression::default()
    };
    let mut gzip_encoder = GzEncoder::new(Vec::new(), quality);
    gzip_encoder.write_all(data).map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("gzip-encode-error: {e:?}"))
    })?;
    let encoded = gzip_encoder.finish().map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("gzip-encode-error: {e:?}"))
    })?;
    Ok(PyBytes::new_bound(py, &encoded).into())
}

#[pyfunction]
pub fn gzip_decode(py: Python<'_>, data: &[u8]) -> PyResult<PyObject> {
    let mut decompressed = Vec::new();
    GzDecoder::new(data)
        .read_to_end(&mut decompressed)
        .map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("gzip-decode-error: {e:?}"))
        })?;
    Ok(PyBytes::new_bound(py, &decompressed).into())
}

// aliases...
#[pyfunction]
pub fn gzip(py: Python<'_>, data: &[u8], quality: Option<u32>) -> PyResult<PyObject> {
    gzip_encode(py, data, quality)
}

#[pyfunction]
pub fn gunzip(py: Python<'_>, data: &[u8]) -> PyResult<PyObject> {
    gzip_decode(py, data)
}

pub fn madd(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(gzip_encode, m)?)?;
    m.add_function(wrap_pyfunction!(gzip_decode, m)?)?;
    m.add_function(wrap_pyfunction!(gzip, m)?)?;
    m.add_function(wrap_pyfunction!(gunzip, m)?)?;
    Ok(())
}
