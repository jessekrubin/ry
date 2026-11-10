use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use ryo3_bytes::{ReadableBuffer, RyBytes};

fn brotli_compress_impl(data: &[u8], level: PyBrotliLevel) -> PyResult<Vec<u8>> {
    burli_encode::compress(data, level.0)
        .map_err(|e| PyValueError::new_err(format!("Brotli encode error: {e}")))
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
