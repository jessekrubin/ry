#![doc = include_str!("../README.md")]
use std::io::Read;

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use ryo3_bytes::{ReadableBuffer, RyBytes};

fn brotli_compress_impl(data: &[u8], level: PyBrotliLevel) -> PyResult<Vec<u8>> {
    burli_encode::compress(data, level.0)
        .map_err(|e| PyValueError::new_err(format!("Brotli encode error: {e}")))
}

fn brotli_decompress_impl(data: &[u8]) -> PyResult<Vec<u8>> {
    let mut decompressed = Vec::new();
    burli_decode::streaming::StreamDecoder::new(data)
        .read_to_end(&mut decompressed)
        .map_err(|e| PyValueError::new_err(format!("Brotli decode error: {e:?}")))?;
    Ok(decompressed)
}

#[expect(clippy::needless_pass_by_value)]
#[pyfunction]
#[pyo3(
    signature = (data, /, level = PyBrotliLevel::default()),
    text_signature = "(data, /, level=5)"
)]
pub fn brotli_compress(
    py: Python<'_>,
    data: ReadableBuffer,
    level: PyBrotliLevel,
) -> PyResult<RyBytes> {
    let bin: &[u8] = data.as_ref();
    py.detach(|| brotli_compress_impl(bin, level))
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
    signature = (data, quality = PyBrotliLevel::default()),
    text_signature = "(data, quality=5)",
    warn(
        message = "`brotli_encode` is deprecated; use `brotli_compress` instead [removal: v0.0.109]",
        category = pyo3::exceptions::PyDeprecationWarning
    )
)]
pub fn brotli_encode(
    py: Python<'_>,
    data: ReadableBuffer,
    quality: PyBrotliLevel,
) -> PyResult<RyBytes> {
    brotli_compress(py, data, quality)
}

#[pyfunction]
#[pyo3(
    signature = (data, quality = PyBrotliLevel::default()),
    text_signature = "(data, quality=5)",
    warn(
        message = "`brotli` is deprecated; use `brotli_compress` instead [removal: v0.0.109]",
        category = pyo3::exceptions::PyDeprecationWarning
    )
)]
pub fn brotli(py: Python<'_>, data: ReadableBuffer, quality: PyBrotliLevel) -> PyResult<RyBytes> {
    brotli_compress(py, data, quality)
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
            && pyint <= 5
        {
            return Ok(Self(pyint));
        }
        Err(PyErr::new::<PyValueError, _>(
            "Compression level must be an integer 0-5",
        ))
    }
}

impl Default for PyBrotliLevel {
    fn default() -> Self {
        Self(5)
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
