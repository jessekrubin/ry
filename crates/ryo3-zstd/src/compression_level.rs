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

impl<'py> FromPyObject<'_, 'py> for PyCompressionLevel {
    type Error = pyo3::PyErr;

    fn extract(obj: Borrowed<'_, 'py, PyAny>) -> Result<Self, Self::Error> {
        if let Ok(level) = obj.extract::<i32>() {
            if !(1..=22).contains(&level) {
                return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                    "Invalid compression level: {level}. Must be between 1 and 22."
                )));
            }
            Ok(Self(level))
        } else {
            Ok(Self(::zstd::DEFAULT_COMPRESSION_LEVEL))
        }
    }
}
