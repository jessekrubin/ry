use std::ops::Deref;

use pyo3::{
    exceptions::{PyTypeError, PyValueError},
    prelude::*,
    types::{PyBytes, PyInt},
};

pub struct Byte(u8);

impl Deref for Byte {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Byte {
    #[must_use]
    pub fn new(value: u8) -> Self {
        Self(value)
    }
}

impl From<u8> for Byte {
    fn from(value: u8) -> Self {
        Self::new(value)
    }
}

impl FromPyObject<'_> for Byte {
    fn extract_bound(ob: &Bound<'_, PyAny>) -> PyResult<Self> {
        if let Ok(i) = ob.downcast::<PyInt>() {
            if let Ok(b) = i.extract::<u8>() {
                Ok(Self(b))
            } else {
                Err(PyValueError::new_err("Integer out of range for a byte"))
            }
        } else if let Ok(i) = ob.downcast::<PyBytes>() {
            let l = i.len()?;
            if l == 1 {
                let b = i.extract::<[u8; 1]>()?;
                Ok(Self(b[0]))
            } else {
                Err(PyValueError::new_err(format!(
                    "Expected a single byte, got a bytes object of length {l}"
                )))
            }
        } else {
            Err(PyTypeError::new_err(
                "Expected an integer in range(0, 256) or a bytes object of length 1",
            ))
        }
    }
}
