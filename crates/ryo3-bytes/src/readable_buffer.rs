use pyo3::prelude::*;

use crate::PyBytes;

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

impl ReadableBuffer<'_, '_> {
    /// Return buffer as byte slice
    #[inline]
    pub fn as_slice(&self) -> &[u8] {
        self.as_ref()
    }

    /// Return buffer length
    #[inline]
    pub fn len(&self) -> usize {
        self.as_slice().len()
    }

    #[inline]
    /// Return `true` if buffer is empty
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Convert to `bytes::Bytes`, potentially zero-copy (cheap clone and refcount bump)
    #[inline]
    pub fn as_bytes(&self) -> PyResult<bytes::Bytes> {
        match self {
            ReadableBuffer::PyBytes(pb) => pb
                .extract::<pyo3::pybacked::PyBackedBytes>()
                .map(bytes::Bytes::from_owner)
                .map_err(PyErr::from),
            ReadableBuffer::RyBytes(rb) => {
                let rbb: &bytes::Bytes = rb.get().as_ref();
                Ok(rbb.clone())
            }
            ReadableBuffer::Buffer(b) => {
                let rbb: &bytes::Bytes = b.as_ref();
                Ok(rbb.clone())
            }
        }
    }

    /// Convert to `ryo3-bytes::PyBytes`, potentially zero-copy (cheap clone and refcount bump)
    #[inline]
    pub fn as_rybytes(&self) -> PyResult<PyBytes> {
        self.as_bytes().map(PyBytes::from)
    }
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

/// Exact size buffer reader
///
/// Use cases:
/// - hash functions with fixed-size keys/seeds
/// - binary protocol headers with fixed sizes
#[derive(Debug)]
pub struct ExactReadableBuffer<'a, 'py, const N: usize>(ReadableBuffer<'a, 'py>);

impl<'a, 'py, const N: usize> FromPyObject<'a, 'py> for ExactReadableBuffer<'a, 'py, N> {
    type Error = PyErr;

    fn extract(ob: Borrowed<'a, 'py, PyAny>) -> PyResult<Self> {
        let buf = ob.extract::<ReadableBuffer<'a, 'py>>()?;
        let len = buf.as_ref().len();

        if len == N {
            Ok(Self(buf))
        } else {
            Err(pyo3::exceptions::PyValueError::new_err(format!(
                "Expected buffer of exactly {N} bytes, got {len}"
            )))
        }
    }
}

impl<const N: usize> AsRef<[u8]> for ExactReadableBuffer<'_, '_, N> {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl<const N: usize> ExactReadableBuffer<'_, '_, N> {
    /// Return buffer as fixed-size array referenc
    ///
    /// # Panics
    ///
    /// Panics if the buffer length is not exactly `N` bytes, but this should
    /// never happen because the constructor checks the length, but wenodis.
    pub fn as_array(&self) -> &[u8; N] {
        self.0.as_ref().try_into().expect("wenodis: length-checked")
    }
}
