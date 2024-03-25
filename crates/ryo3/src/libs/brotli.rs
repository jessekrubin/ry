use ::brotli as br;
use pyo3::prelude::*;
use pyo3::types::PyBytes;
use std::io::{Read, Write};

#[pyfunction]
pub fn brotli_encode(py: Python<'_>, data: &[u8], quality: Option<u32>) -> PyResult<PyObject> {
    let quality = if let Some(param) = quality { param } else { 11 };
    let mut encoder = br::CompressorWriter::new(Vec::new(), 4 * 1024, quality, 22);
    encoder
        .write_all(data)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Error: {:?}", e)))?;
    let t = encoder.into_inner();
    Ok(PyBytes::new(py, &t).into())
}

#[pyfunction]
pub fn brotli_decode(py: Python<'_>, data: &[u8]) -> PyResult<PyObject> {
    let mut decompressed = Vec::new();
    br::Decompressor::new(data, 4 * 1024)
        .read_to_end(&mut decompressed)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Error: {:?}", e)))?;
    Ok(PyBytes::new(py, &decompressed).into())
}

pub fn madd(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(brotli_decode, m)?)?;
    m.add_function(wrap_pyfunction!(brotli_encode, m)?)?;
    Ok(())
}
