use flate2::Compression;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
pub struct PyCompression(Compression);

impl PyCompression {
    pub const fn new(level: u32) -> Self {
        Self(Compression::new(level))
    }

    pub const fn fast() -> Self {
        Self(Compression::fast())
    }

    pub const fn best() -> Self {
        Self(Compression::best())
    }

    pub const fn from_inner(inner: Compression) -> Self {
        Self(inner)
    }

    pub const fn into_inner(self) -> Compression {
        self.0
    }
}

impl From<Compression> for PyCompression {
    fn from(compression: Compression) -> Self {
        Self::from_inner(compression)
    }
}

impl From<PyCompression> for Compression {
    fn from(py_compression: PyCompression) -> Self {
        py_compression.into_inner()
    }
}

impl<'py> FromPyObject<'_, 'py> for PyCompression {
    type Error = pyo3::PyErr;

    fn extract(obj: Borrowed<'_, 'py, PyAny>) -> Result<Self, Self::Error> {
        if let Ok(level) = obj.extract::<u32>() {
            if level < 10 {
                return Ok(Self::new(level));
            }
        } else if let Ok(pystr) = obj.extract::<&str>() {
            match pystr {
                "fast" => return Ok(Self::fast()),
                "best" => return Ok(Self::best()),
                _ => {}
            }
        }
        Err(PyValueError::new_err(
            "Invalid compression level; valid levels are int 0-9 or string 'fast' or 'best'",
        ))
    }
}
