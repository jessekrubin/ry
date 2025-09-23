use std::ops::Deref;

use pyo3::{
    exceptions::PyTypeError,
    prelude::*,
    types::{PyBytes, PyInt},
};

#[derive(Clone, Copy)]
pub struct Byte(u8);

impl std::fmt::Debug for Byte {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            // https://doc.rust-lang.org/reference/tokens.html#byte-escapes
            b'\\' => f.write_str("b'\\\\'"),
            b'"' => f.write_str("b'\"'"),
            b'\'' => f.write_str("b\"'\""),
            b'\n' => f.write_str(r"b'\n'"),
            b'\r' => f.write_str(r"b'\r'"),
            b'\t' => f.write_str(r"b'\t'"),
            // printable ASCII
            0x20..=0x7E => {
                let c = self.0 as char;
                write!(f, "b'{c}'")
            }
            _ => write!(f, "b'\\x{:02x}'", self.0),
        }
    }
}

impl Deref for Byte {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<u8> for Byte {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

impl FromPyObject<'_> for Byte {
    fn extract_bound(ob: &Bound<'_, PyAny>) -> PyResult<Self> {
        if let Ok(i) = ob.cast::<PyInt>() {
            if let Ok(b) = i.extract::<u8>() {
                Ok(Self(b))
            } else {
                Err(PyTypeError::new_err("Integer out of range for a byte"))
            }
        } else if let Ok(i) = ob.cast::<PyBytes>() {
            let l = i.len()?;
            if l == 1 {
                let b = i.extract::<[u8; 1]>()?;
                Ok(Self::from(b[0]))
            } else {
                Err(PyTypeError::new_err(format!(
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

impl<'py> IntoPyObject<'py> for &Byte {
    type Target = PyBytes;
    type Output = Bound<'py, Self::Target>;
    type Error = std::convert::Infallible;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        Ok(PyBytes::new(py, &[self.0]))
    }
}

impl<'py> IntoPyObject<'py> for Byte {
    type Target = PyBytes;
    type Output = Bound<'py, Self::Target>;
    type Error = std::convert::Infallible;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        Ok(PyBytes::new(py, &[self.0]))
    }
}
