#![doc = include_str!("../README.md")]
use std::io::{Read, Write};

use ::bzip2::read::BzDecoder;
use ::bzip2::write::BzEncoder;
use ::bzip2::Compression;
use pyo3::prelude::PyModule;
use pyo3::prelude::*;
use pyo3::types::PyBytes;

fn rs_bzip2_encode(py: Python<'_>, data: &[u8], quality: Option<u32>) -> PyResult<PyObject> {
    let quality = if let Some(param) = quality {
        if param < Compression::fast().level() || param > Compression::best().level() {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "The optional second argument to bzip2() must be between 0 and 9",
            ));
        }
        Compression::new(param)
    } else {
        Compression::default()
    };
    let mut bzip2_encoder = BzEncoder::new(Vec::new(), quality);
    bzip2_encoder.write_all(data.as_ref()).map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("bzip2-encode-error: {e:?}"))
    })?;
    let encoded = bzip2_encoder.finish().map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("bzip2-encode-error: {e:?}"))
    })?;
    Ok(PyBytes::new(py, &encoded).into())
}

#[pyfunction]
#[pyo3(signature = (data, quality=None))]
#[expect(clippy::needless_pass_by_value)]
pub fn bzip2_encode(
    py: Python<'_>,
    data: ryo3_bytes::PyBytes,
    quality: Option<u32>,
) -> PyResult<PyObject> {
    let data = data.as_ref();
    rs_bzip2_encode(py, data, quality)
}

#[pyfunction]
#[pyo3(signature = (data, quality=None))]
#[expect(clippy::needless_pass_by_value)]
pub fn bzip2(
    py: Python<'_>,
    data: ryo3_bytes::PyBytes,
    quality: Option<u32>,
) -> PyResult<PyObject> {
    let data = data.as_ref();
    rs_bzip2_encode(py, data, quality)
}

#[pyfunction]
#[expect(clippy::needless_pass_by_value)]
pub fn bzip2_decode(py: Python<'_>, data: ryo3_bytes::PyBytes) -> PyResult<PyObject> {
    let mut decompressed = Vec::new();
    let data: &[u8] = data.as_ref();
    BzDecoder::new(data)
        .read_to_end(&mut decompressed)
        .map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("bzip2-decode-error: {e:?}"))
        })?;
    Ok(PyBytes::new(py, &decompressed).into())
}

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(bzip2_decode, m)?)?;
    m.add_function(wrap_pyfunction!(bzip2_encode, m)?)?;
    m.add_function(wrap_pyfunction!(self::bzip2, m)?)?;
    Ok(())
}
