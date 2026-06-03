use pyo3::IntoPyObjectExt;
use pyo3::exceptions::PyUnicodeDecodeError;
use pyo3::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PyUtf8Bytes(Vec<u8>);

impl PyUtf8Bytes {
    #[inline]
    #[must_use]
    pub fn new(bytes: Vec<u8>) -> Self {
        Self(bytes)
    }

    #[inline]
    #[must_use]
    pub fn into_inner(self) -> Vec<u8> {
        self.0
    }
}

impl From<Vec<u8>> for PyUtf8Bytes {
    #[inline]
    fn from(bytes: Vec<u8>) -> Self {
        Self::new(bytes)
    }
}

impl<'py> IntoPyObject<'py> for PyUtf8Bytes {
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    #[inline]
    fn into_pyobject(self, py: Python<'py>) -> PyResult<Self::Output> {
        match std::str::from_utf8(&self.0) {
            Ok(s) => s.into_bound_py_any(py),
            Err(e) => Err(PyUnicodeDecodeError::new_utf8(py, &self.0, e)?.into()),
        }
    }
}
