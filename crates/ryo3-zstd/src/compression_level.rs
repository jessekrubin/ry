use pyo3::prelude::*;
use zstd::zstd_safe::CompressionLevel;

#[derive(Copy, Clone, Debug)]
pub struct PyCompressionLevel(CompressionLevel);

impl PyCompressionLevel {
    pub const DEFAULT: Self = Self(::zstd::DEFAULT_COMPRESSION_LEVEL);
    pub const MIN: Self = Self(1);
    pub const MAX: Self = Self(22);
    pub const fn new(level: i32) -> Self {
        Self(level)
    }

    pub const fn into_inner(self) -> i32 {
        self.0
    }
}

impl Default for PyCompressionLevel {
    fn default() -> Self {
        Self::DEFAULT
    }
}

impl TryFrom<i32> for PyCompressionLevel {
    type Error = ();
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if (Self::MIN.0..=Self::MAX.0).contains(&value) {
            Ok(Self::new(value))
        } else {
            Err(())
        }
    }
}

impl From<PyCompressionLevel> for CompressionLevel {
    fn from(val: PyCompressionLevel) -> Self {
        val.into_inner()
    }
}

const ZSTD_COMPRESSION_LEVEL_EMSG: &str =
    "zstd-compression-level must be an integer between 1 and 22";

impl<'py> FromPyObject<'_, 'py> for PyCompressionLevel {
    type Error = pyo3::PyErr;

    fn extract(obj: Borrowed<'_, 'py, PyAny>) -> Result<Self, Self::Error> {
        let pyint = obj
            .cast_exact::<pyo3::types::PyInt>()
            .map_err(|_| pyo3::exceptions::PyTypeError::new_err(ZSTD_COMPRESSION_LEVEL_EMSG))?;

        pyint
            .extract::<i32>()
            .ok()
            .filter(|level| (1..=22).contains(level))
            .map(Self::new)
            .ok_or_else(|| pyo3::exceptions::PyValueError::new_err(ZSTD_COMPRESSION_LEVEL_EMSG))
    }
}
