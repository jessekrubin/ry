#![doc = include_str!("../README.md")]
use std::io::{Read, Write};

use ::bzip2::Compression;
use ::bzip2::read::BzDecoder;
use ::bzip2::write::BzEncoder;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::PyModule;
use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyInt, PyString};

fn rs_bzip2_encode(py: Python<'_>, data: &[u8], quality: Compression) -> PyResult<Py<PyAny>> {
    let mut bzip2_encoder = BzEncoder::new(Vec::new(), quality);
    bzip2_encoder.write_all(data.as_ref())?;
    let encoded = bzip2_encoder.finish()?;
    Ok(PyBytes::new(py, &encoded).into())
}

#[pyfunction]
#[pyo3(signature = (data, quality=None))]
#[expect(clippy::needless_pass_by_value)]
pub fn bzip2_encode(
    py: Python<'_>,
    data: ryo3_bytes::PyBytes,
    quality: Option<PyCompression>,
) -> PyResult<Py<PyAny>> {
    let data = data.as_ref();
    rs_bzip2_encode(py, data, quality.unwrap_or_default().0)
}

#[pyfunction]
#[pyo3(signature = (data, quality=None))]
#[expect(clippy::needless_pass_by_value)]
pub fn bzip2(
    py: Python<'_>,
    data: ryo3_bytes::PyBytes,
    quality: Option<PyCompression>,
) -> PyResult<Py<PyAny>> {
    let data = data.as_ref();
    rs_bzip2_encode(py, data, quality.unwrap_or_default().0)
}

#[pyfunction]
#[expect(clippy::needless_pass_by_value)]
pub fn bzip2_decode(py: Python<'_>, data: ryo3_bytes::PyBytes) -> PyResult<Py<PyAny>> {
    let mut decompressed = Vec::new();
    let data: &[u8] = data.as_ref();
    BzDecoder::new(data).read_to_end(&mut decompressed)?;

    Ok(PyBytes::new(py, &decompressed).into())
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
pub struct PyCompression(pub(crate) Compression);

impl FromPyObject<'_> for PyCompression {
    fn extract_bound(ob: &Bound<'_, PyAny>) -> PyResult<Self> {
        if let Ok(pyint) = ob.cast::<PyInt>() {
            let level = pyint.extract::<u32>()?;
            if level < 10 {
                return Ok(Self(Compression::new(level)));
            }
        } else if let Ok(pystr) = ob.cast::<PyString>() {
            let s = pystr.to_str()?;
            let c = match s {
                "fast" => Some(Self(Compression::fast())),
                "default" => Some(Self(Compression::default())),
                "best" => Some(Self(Compression::best())),
                _ => None,
            };
            if let Some(c) = c {
                return Ok(c);
            }
        }
        Err(PyValueError::new_err(
            "Invalid compression level; valid levels are int 0-9 or string 'fast', 'default', 'best'",
        ))
    }
}

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(bzip2_decode, m)?)?;
    m.add_function(wrap_pyfunction!(bzip2_encode, m)?)?;
    m.add_function(wrap_pyfunction!(self::bzip2, m)?)?;
    Ok(())
}
