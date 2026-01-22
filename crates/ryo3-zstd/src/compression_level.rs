use pyo3::prelude::*;
use zstd::zstd_safe::CompressionLevel;

#[derive(Copy, Clone, Debug)]
pub struct PyCompressionLevel(pub CompressionLevel);

impl Default for PyCompressionLevel {
    fn default() -> Self {
        Self(::zstd::DEFAULT_COMPRESSION_LEVEL)
    }
}

impl TryFrom<i32> for PyCompressionLevel {
    type Error = ();
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if (1..=22).contains(&value) {
            Ok(Self(value))
        } else {
            Err(())
        }
    }
}

impl From<PyCompressionLevel> for i32 {
    fn from(val: PyCompressionLevel) -> Self {
        val.0
    }
}

const ZSTD_COMPRESSION_LEVEL_ERROR: &str =
    "zstd-compression-level must be an integer between 1 and 22";
impl<'py> FromPyObject<'_, 'py> for PyCompressionLevel {
    type Error = pyo3::PyErr;

    fn extract(obj: Borrowed<'_, 'py, PyAny>) -> Result<Self, Self::Error> {
        if let Ok(level) = obj.extract::<i32>() {
            Self::try_from(level).map_err(|()| {
                PyErr::new::<pyo3::exceptions::PyValueError, _>(ZSTD_COMPRESSION_LEVEL_ERROR)
            })
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                ZSTD_COMPRESSION_LEVEL_ERROR,
            ))
        }
    }
}
