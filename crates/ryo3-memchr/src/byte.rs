use pyo3::{
    exceptions::{PyTypeError, PyValueError},
    prelude::*,
    types::{PyBytes, PyInt},
};

pub struct Byte(u8);

impl Byte {
    pub fn new(value: u8) -> Self {
        Byte(value)
    }
}

impl From<u8> for Byte {
    fn from(value: u8) -> Self {
        Byte::new(value)
    }
}

impl FromPyObject<'_> for Byte {
    fn extract_bound(ob: &Bound<'_, PyAny>) -> PyResult<Self> {
        if let Ok(i) = ob.downcast::<PyInt>() {
            if let Ok(b) = i.extract::<u8>() {
                return Ok(Byte(b));
            }
        }

        if let Ok(i) = ob.downcast::<PyBytes>() {
            let l = i.len()?;
            if l == 1 {
                let b = i.extract::<[u8; 1]>()?;
                return Ok(Byte(b[0]));
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
