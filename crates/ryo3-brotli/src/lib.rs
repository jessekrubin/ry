#![doc = include_str!("../README.md")]
use std::io::{Read, Write};

use ::brotli as br;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use ryo3_bytes::{ReadableBuffer, RyBytes};

fn brotli_compress_impl(
    data: &[u8],
    level: PyBrotliLevel,
    magic_number: bool,
) -> PyResult<Vec<u8>> {
    let encoded = if magic_number {
        let params = br::enc::BrotliEncoderParams {
            quality: level.0.into(),
            magic_number: true,
            lgwin: 22,
            ..Default::default()
        };
        let mut encoder = br::CompressorWriter::with_params(Vec::new(), 4 * 1024, &params);
        encoder.write_all(data)?;
        encoder.into_inner()
    } else {
        let mut encoder = br::CompressorWriter::new(Vec::new(), 4 * 1024, level.0.into(), 22);
        encoder.write_all(data)?;
        encoder.into_inner()
    };
    Ok(encoded)
}

fn brotli_decompress_impl(data: &[u8]) -> PyResult<Vec<u8>> {
    let mut decompressed = Vec::new();
    br::Decompressor::new(data, 4 * 1024)
        .read_to_end(&mut decompressed)
        .map_err(|e|
            PyValueError::new_err(format!("Brotli decode error: {e:?}"))
        )?;
    Ok(decompressed)
}

#[expect(clippy::needless_pass_by_value)]
#[pyfunction]
#[pyo3(
    signature = (data, /, level = PyBrotliLevel::default(), *, magic_number = false),
    text_signature = "(data, /, level=11, *, magic_number=False)"
)]
pub fn brotli_compress(
    py: Python<'_>,
    data: ReadableBuffer,
    level: PyBrotliLevel,
    magic_number: bool,
) -> PyResult<RyBytes> {
    let bin: &[u8] = data.as_ref();
    py.detach(|| brotli_compress_impl(bin, level, magic_number))
        .map(RyBytes::from)
}

#[expect(clippy::needless_pass_by_value)]
#[pyfunction]
#[pyo3(signature = (data, /))]
pub fn brotli_decompress(py: Python<'_>, data: ReadableBuffer) -> PyResult<RyBytes> {
    let bin: &[u8] = data.as_ref();
    py.detach(|| brotli_decompress_impl(bin)).map(RyBytes::from)
}

// ============================================================================
// DEPRECATED ~ remove in v0.0.109
// ============================================================================

#[pyfunction]
#[pyo3(
    signature = (data, quality = PyBrotliLevel::default(), *, magic_number = false),
    text_signature = "(data, quality=11, *, magic_number=False)",
    warn(
        message = "`brotli_encode` is deprecated; use `brotli_compress` instead [removal: v0.0.109]",
        category = pyo3::exceptions::PyDeprecationWarning
    )
)]
pub fn brotli_encode(
    py: Python<'_>,
    data: ReadableBuffer,
    quality: PyBrotliLevel,
    magic_number: bool,
) -> PyResult<RyBytes> {
    brotli_compress(py, data, quality, magic_number)
}

#[pyfunction]
#[pyo3(
    signature = (data, quality = PyBrotliLevel::default(), *, magic_number = false),
    text_signature = "(data, quality=11, *, magic_number=False)",
    warn(
        message = "`brotli` is deprecated; use `brotli_compress` instead [removal: v0.0.109]",
        category = pyo3::exceptions::PyDeprecationWarning
    )
)]
pub fn brotli(
    py: Python<'_>,
    data: ReadableBuffer,
    quality: PyBrotliLevel,
    magic_number: bool,
) -> PyResult<RyBytes> {
    brotli_compress(py, data, quality, magic_number)
}

#[pyfunction]
#[pyo3(warn(
    message = "`brotli_decode` is deprecated; use `brotli_decompress` instead [removal: v0.0.109]",
    category = pyo3::exceptions::PyDeprecationWarning
))]
pub fn brotli_decode(py: Python<'_>, data: ReadableBuffer) -> PyResult<RyBytes> {
    brotli_decompress(py, data)
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct PyBrotliLevel(u8);

impl<'py> FromPyObject<'_, 'py> for PyBrotliLevel {
    type Error = pyo3::PyErr;
    fn extract(ob: Borrowed<'_, 'py, PyAny>) -> Result<Self, Self::Error> {
        if let Ok(pyint) = ob.extract::<u8>()
            && pyint <= 11
        {
            return Ok(Self(pyint));
        }
        Err(PyErr::new::<PyValueError, _>(
            "Compression level must be an integer 0-11",
        ))
    }
}

impl Default for PyBrotliLevel {
    fn default() -> Self {
        Self(11)
    }
}

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(brotli_compress, m)?)?;
    m.add_function(wrap_pyfunction!(brotli_decompress, m)?)?;
    // deprecated ~ remove in v0.0.109
    m.add_function(wrap_pyfunction!(brotli_decode, m)?)?;
    m.add_function(wrap_pyfunction!(brotli_encode, m)?)?;
    m.add_function(wrap_pyfunction!(self::brotli, m)?)?;
    Ok(())
}
