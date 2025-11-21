#![doc = include_str!("../README.md")]
use std::io::{Read, Write};

use ::brotli as br;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyBytes;
use ryo3_bytes::PyBytes as RyBytes;

fn encode(data: &[u8], quality: PyBrQuality, magic_number: bool) -> PyResult<Vec<u8>> {
    let encoded = if magic_number {
        let params = br::enc::BrotliEncoderParams {
            quality: quality.0.into(),
            magic_number: true,
            lgwin: 22,
            ..Default::default()
        };
        let mut encoder = br::CompressorWriter::with_params(Vec::new(), 4 * 1024, &params);
        encoder.write_all(data)?;
        encoder.into_inner()
    } else {
        let mut encoder = br::CompressorWriter::new(Vec::new(), 4 * 1024, quality.0.into(), 22);
        encoder.write_all(data)?;
        encoder.into_inner()
    };
    Ok(encoded)
}

#[pyfunction]
#[pyo3(signature = (data, quality=PyBrQuality::default(), *, magic_number=false))]
#[expect(clippy::needless_pass_by_value)]
pub fn brotli_encode<'py>(
    py: Python<'py>,
    data: RyBytes,
    quality: PyBrQuality,
    magic_number: bool,
) -> PyResult<Bound<'py, PyBytes>> {
    let bin: &[u8] = data.as_ref();
    let encoded = py.detach(|| encode(bin, quality, magic_number))?;
    Ok(PyBytes::new(py, &encoded))
}

#[pyfunction]
#[pyo3(signature = (data, quality=PyBrQuality::default(), *, magic_number=false))]
#[expect(clippy::needless_pass_by_value)]
pub fn brotli<'py>(
    py: Python<'py>,
    data: RyBytes,
    quality: PyBrQuality,
    magic_number: bool,
) -> PyResult<Bound<'py, PyBytes>> {
    let bin: &[u8] = data.as_ref();
    let encoded = py.detach(|| encode(bin, quality, magic_number))?;
    Ok(PyBytes::new(py, &encoded))
}

#[pyfunction]
#[expect(clippy::needless_pass_by_value)]
pub fn brotli_decode<'py>(py: Python<'py>, data: RyBytes) -> PyResult<Bound<'py, PyBytes>> {
    let decompressed = py.detach(|| {
        let mut decompressed = Vec::new();
        let bin: &[u8] = data.as_ref();
        let res = br::Decompressor::new(bin, 4 * 1024).read_to_end(&mut decompressed);
        res.map(|_| decompressed).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Brotli decode error: {e:?}"))
        })
    })?;
    Ok(PyBytes::new(py, &decompressed))
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct PyBrQuality(u8);

impl<'py> FromPyObject<'_, 'py> for PyBrQuality {
    type Error = pyo3::PyErr;
    fn extract(ob: Borrowed<'_, 'py, PyAny>) -> Result<Self, Self::Error> {
        if let Ok(pyint) = ob.extract::<u8>() {
            if pyint <= 11 {
                return Ok(PyBrQuality(pyint));
            }
        }
        Err(PyErr::new::<PyValueError, _>(
            "Compression level must be an integer 0-11",
        ))
    }
}

impl Default for PyBrQuality {
    fn default() -> Self {
        PyBrQuality(11)
    }
}

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(brotli_decode, m)?)?;
    m.add_function(wrap_pyfunction!(brotli_encode, m)?)?;
    m.add_function(wrap_pyfunction!(self::brotli, m)?)?;
    Ok(())
}
