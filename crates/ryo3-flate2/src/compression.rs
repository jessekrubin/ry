use flate2::Compression;
use pyo3::{exceptions::PyValueError, prelude::*};

#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
pub struct PyCompression(pub(crate) Compression);

impl<'py> FromPyObject<'_, 'py> for PyCompression {
    type Error = pyo3::PyErr;

    fn extract(obj: Borrowed<'_, 'py, PyAny>) -> Result<Self, Self::Error> {
        if let Ok(level) = obj.extract::<u32>() {
            if level < 10 {
                return Ok(Self(Compression::new(level)));
            }
        } else if let Ok(pystr) = obj.extract::<&str>() {
            match pystr {
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
