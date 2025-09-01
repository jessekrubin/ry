#![doc = include_str!("../README.md")]
use std::io::{Read, Write};

use ::brotli as br;
use pyo3::prelude::*;
use pyo3::types::PyBytes;

fn encode(data: &[u8], quality: Option<u8>, magic_number: Option<bool>) -> PyResult<Vec<u8>> {
    let quality_u8 = match quality {
        Some(q) => {
            if q > 11 {
                return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                    "Quality value must be between 0 and 11",
                ));
            }
            q
        }
        _ => 11,
    };
    let encoded = if magic_number == Some(true) {
        let params = br::enc::BrotliEncoderParams {
            quality: quality_u8.into(),
            magic_number: true,
            lgwin: 22,
            ..Default::default()
        };
        let mut encoder = br::CompressorWriter::with_params(Vec::new(), 4 * 1024, &params);
        encoder.write_all(data)?;
        encoder.into_inner()
    } else {
        let mut encoder = br::CompressorWriter::new(Vec::new(), 4 * 1024, quality_u8.into(), 22);
        encoder.write_all(data)?;
        encoder.into_inner()
    };
    Ok(encoded)
}

#[pyfunction]
#[pyo3(signature = (data, quality=None, magic_number=None))]
#[expect(clippy::needless_pass_by_value)]
pub fn brotli_encode(
    py: Python<'_>,
    data: ryo3_bytes::PyBytes,
    quality: Option<u8>,
    magic_number: Option<bool>,
) -> PyResult<Py<PyAny>> {
    let bin: &[u8] = data.as_ref();
    let encoded = encode(bin, quality, magic_number)?;
    Ok(PyBytes::new(py, &encoded).into())
}

#[pyfunction]
#[pyo3(signature = (data, quality=None, magic_number=None))]
#[expect(clippy::needless_pass_by_value)]
pub fn brotli(
    py: Python<'_>,
    data: ryo3_bytes::PyBytes,
    quality: Option<u8>,
    magic_number: Option<bool>,
) -> PyResult<Py<PyAny>> {
    let bin: &[u8] = data.as_ref();
    let encoded = encode(bin, quality, magic_number)?;
    Ok(PyBytes::new(py, &encoded).into())
}

#[pyfunction]
#[expect(clippy::needless_pass_by_value)]
pub fn brotli_decode(py: Python<'_>, data: ryo3_bytes::PyBytes) -> PyResult<Py<PyAny>> {
    let mut decompressed = Vec::new();
    let bin: &[u8] = data.as_ref();
    br::Decompressor::new(bin, 4 * 1024)
        .read_to_end(&mut decompressed)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Error: {e:?}")))?;
    Ok(PyBytes::new(py, &decompressed).into())
}

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(brotli_decode, m)?)?;
    m.add_function(wrap_pyfunction!(brotli_encode, m)?)?;
    m.add_function(wrap_pyfunction!(self::brotli, m)?)?;
    Ok(())
}
