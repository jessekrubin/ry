use bytes::Bytes;
use pyo3::prelude::*;
use pyo3::types::PyBytes;

/// Wrapper struct around a `bytes::Bytes` object for PyO3 python-serde.
pub struct Pyo3Bytes(pub Bytes);

impl Pyo3Bytes {
    /// Create a new `Pyo3Bytes` object from a `bytes::Bytes` object.
    pub fn new(buf: Bytes) -> Self {
        Self(buf)
    }
}

impl From<Bytes> for Pyo3Bytes {
    fn from(value: Bytes) -> Self {
        Self::new(value)
    }
}

impl<'py> IntoPyObject<'py> for Pyo3Bytes {
    type Target = PyBytes;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        Ok(PyBytes::new(py, &self.0[..]))
    }
}
