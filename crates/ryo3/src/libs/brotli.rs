use std::io::{Read, Write};

use ::brotli as br;
use pyo3::prelude::*;
use pyo3::types::PyBytes;

#[pyfunction]
#[pyo3(signature = (data, quality=None, magic_number=None))]
pub fn brotli_encode(
    py: Python<'_>,
    data: &[u8],
    quality: Option<u8>,
    magic_number: Option<bool>,
) -> PyResult<PyObject> {
    // error on invalid quality value if given
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
    let encoded = if let Some(true) = magic_number {
        let params = br::enc::BrotliEncoderParams {
            quality: quality_u8.into(),
            magic_number: true,
            lgwin: 22,
            ..Default::default()
        };
        let mut encoder = br::CompressorWriter::with_params(Vec::new(), 4 * 1024, &params);
        // let mut encoder = br::CompressorWriter::new(Vec::new(), 4 * 1024, 11, 22);
        encoder.write_all(data).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Error: {e:?}"))
        })?;
        encoder.into_inner()
    } else {
        let mut encoder = br::CompressorWriter::new(Vec::new(), 4 * 1024, quality_u8.into(), 22);
        encoder.write_all(data).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Error: {e:?}"))
        })?;
        encoder.into_inner()
    };
    Ok(PyBytes::new_bound(py, &encoded).into())
}

#[pyfunction]
#[pyo3(signature = (data, quality=None, magic_number=None))]
pub fn brotli(
    py: Python<'_>,
    data: &[u8],
    quality: Option<u8>,
    magic_number: Option<bool>,
) -> PyResult<PyObject> {
    brotli_encode(py, data, quality, magic_number)
}

#[pyfunction]
pub fn brotli_decode(py: Python<'_>, data: &[u8]) -> PyResult<PyObject> {
    let mut decompressed = Vec::new();
    br::Decompressor::new(data, 4 * 1024)
        .read_to_end(&mut decompressed)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Error: {e:?}")))?;
    Ok(PyBytes::new_bound(py, &decompressed).into())
}

pub fn madd(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(brotli_decode, m)?)?;
    m.add_function(wrap_pyfunction!(brotli_encode, m)?)?;
    m.add_function(wrap_pyfunction!(self::brotli, m)?)?;
    Ok(())
}
