use bytes::Bytes;
use pyo3::prelude::*;
use pyo3::types::PyBytes;

pub(crate) struct Pyo3Bytes(pub Bytes);

impl Pyo3Bytes {
    pub fn new(buf: Bytes) -> Self {
        Self(buf)
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
