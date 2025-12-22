#![doc = include_str!("../README.md")]
use std::io::{Read, Write};

use ::bzip2::Compression;
use ::bzip2::read::BzDecoder;
use ::bzip2::write::BzEncoder;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use ryo3_bytes::PyBytes;

fn rs_bzip2_encode(data: &[u8], quality: Compression) -> PyResult<PyBytes> {
    let mut bzip2_encoder = BzEncoder::new(Vec::new(), quality);
    bzip2_encoder.write_all(data.as_ref())?;
    let encoded = bzip2_encoder.finish()?;
    Ok(encoded.into())
}

fn rs_bzip2_decode(data: &[u8]) -> PyResult<ryo3_bytes::PyBytes> {
    let mut decompressed = Vec::new();
    BzDecoder::new(data).read_to_end(&mut decompressed)?;
    Ok(decompressed.into())
}

#[pyfunction]
#[pyo3(
    signature = (data, quality=PyCompression::default()),
    text_signature = "(data, quality=6)",
)]
#[expect(clippy::needless_pass_by_value)]
pub fn bzip2_encode(py: Python<'_>, data: PyBytes, quality: PyCompression) -> PyResult<PyBytes> {
    let data = data.as_ref();
    py.detach(|| rs_bzip2_encode(data, quality.0))
}

#[pyfunction]
#[pyo3(
    signature = (data, quality=PyCompression::default()),
    text_signature = "(data, quality=6)",
)]
#[expect(clippy::needless_pass_by_value)]
pub fn bzip2(py: Python<'_>, data: PyBytes, quality: PyCompression) -> PyResult<PyBytes> {
    let data = data.as_ref();
    py.detach(|| rs_bzip2_encode(data, quality.0))
}

#[pyfunction]
#[expect(clippy::needless_pass_by_value)]
pub fn bzip2_decode(py: Python<'_>, data: PyBytes) -> PyResult<PyBytes> {
    let data: &[u8] = data.as_ref();
    py.detach(|| rs_bzip2_decode(data))
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
pub struct PyCompression(pub(crate) Compression);

impl<'py> FromPyObject<'_, 'py> for PyCompression {
    type Error = pyo3::PyErr;
    fn extract(ob: Borrowed<'_, 'py, PyAny>) -> Result<Self, Self::Error> {
        if let Ok(level) = ob.extract::<u32>() {
            if level < 10 {
                return Ok(Self(Compression::new(level)));
            }
        } else if let Ok(c) = ob.extract::<&str>() {
            match c {
                "fast" => return Ok(Self(Compression::fast())),
                "best" => return Ok(Self(Compression::best())),
                _ => {}
            }
        }
        Err(PyValueError::new_err(
            "Invalid compression level; valid levels are int 0-9 or string 'fast' or 'best'",
        ))
    }
}

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(bzip2_decode, m)?)?;
    m.add_function(wrap_pyfunction!(bzip2_encode, m)?)?;
    m.add_function(wrap_pyfunction!(self::bzip2, m)?)?;
    Ok(())
}
