use crate::PyBytes;
use pyo3::prelude::*;

/// Custom zero-copy bytes-like for extracting `&[u8]`
#[derive(Debug)]
pub enum ReadableBuffer<'a, 'py> {
    /// python `builtins.bytes`
    PyBytes(Borrowed<'a, 'py, pyo3::types::PyBytes>),
    /// Reference to a `ryo3-bytes::PyBytes` object
    RyBytes(Borrowed<'a, 'py, PyBytes>),
    /// Any object that supports the buffer protocol
    Buffer(PyBytes),
}

impl AsRef<[u8]> for ReadableBuffer<'_, '_> {
    fn as_ref(&self) -> &[u8] {
        match self {
            ReadableBuffer::PyBytes(pybytes) => pybytes.as_bytes(),
            ReadableBuffer::RyBytes(rybytes) => rybytes.get().as_slice(),
            ReadableBuffer::Buffer(buffer) => buffer.as_slice(),
        }
    }
}

impl<'a, 'py> FromPyObject<'a, 'py> for ReadableBuffer<'a, 'py> {
    type Error = PyErr;

    fn extract(ob: Borrowed<'a, 'py, PyAny>) -> Result<Self, Self::Error> {
        if let Ok(pybytes) = ob.cast_exact::<pyo3::types::PyBytes>() {
            Ok(Self::PyBytes(pybytes))
        } else if let Ok(rybytes) = ob.cast_exact::<PyBytes>() {
            Ok(Self::RyBytes(rybytes))
        } else if let Ok(buffer) = ob.extract::<PyBytes>() {
            // TODO: possibly short circut here and dont extracct via thingy
            // because it does redundant checks...
            Ok(Self::Buffer(buffer))
        } else {
            Err(pyo3::exceptions::PyTypeError::new_err(
                "Expected bytes, bytearray, or buffer-protocol object",
            ))
        }
    }
}
