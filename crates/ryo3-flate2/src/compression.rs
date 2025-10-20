use flate2::Compression;
use pyo3::types::{PyInt, PyString};
use pyo3::{exceptions::PyValueError, prelude::*};

#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
pub struct PyCompression(pub(crate) Compression);

impl<'py> FromPyObject<'_, 'py> for PyCompression {
    type Error = pyo3::PyErr;

    fn extract(obj: Borrowed<'_, 'py, PyAny>) -> Result<Self, Self::Error> {
        if let Ok(pyint) = obj.cast::<PyInt>() {
            let level = pyint.extract::<u32>()?;
            if level < 10 {
                return Ok(Self(Compression::new(level)));
            }
        } else if let Ok(pystr) = obj.cast::<PyString>() {
            let s = pystr.to_str()?;
            let c = match s {
                "fast" => Some(Self(Compression::fast())),
                "best" => Some(Self(Compression::best())),
                _ => None,
            };
            if let Some(c) = c {
                return Ok(c);
            }
        }
        Err(PyValueError::new_err(
            "Invalid compression level; valid levels are int 0-9 or string 'fast' or 'best'",
        ))
    }
}
