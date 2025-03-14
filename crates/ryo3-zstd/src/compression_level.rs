use pyo3::prelude::*;
use zstd::zstd_safe::CompressionLevel;

#[derive(Copy, Clone, Debug)]
pub struct PyCompressionLevel(pub CompressionLevel);

impl Default for PyCompressionLevel {
    fn default() -> Self {
        PyCompressionLevel(::zstd::DEFAULT_COMPRESSION_LEVEL)
    }
}

impl TryFrom<i32> for PyCompressionLevel {
    type Error = ();
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value < 1 || value > 22 {
            Err(())
        } else {
            Ok(PyCompressionLevel(value))
        }
    }
}

impl Into<i32> for PyCompressionLevel {
    fn into(self) -> i32 {
        self.0
    }
}

impl FromPyObject<'_> for PyCompressionLevel {
    fn extract_bound(ob: &Bound<'_, PyAny>) -> PyResult<Self> {
        if let Ok(level) = ob.extract::<i32>() {
            if level < 1 || level > 22 {
                return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                    "Invalid compression level: {level}. Must be between 1 and 22."
                )));
            }
            Ok(PyCompressionLevel(level))
        } else {
            Ok(PyCompressionLevel(::zstd::DEFAULT_COMPRESSION_LEVEL))
        }
    }
}
