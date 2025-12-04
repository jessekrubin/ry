use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyString};
use pyo3::{IntoPyObject, Python};

use crate::pystring_fast_new;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PyDigest<T>(pub T);
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PyHexDigest<T>(pub T);

impl<T> From<T> for PyDigest<T> {
    #[inline]
    fn from(t: T) -> Self {
        Self(t)
    }
}

impl<T> From<T> for PyHexDigest<T> {
    #[inline]
    fn from(t: T) -> Self {
        Self(t)
    }
}

macro_rules! impl_into_py_object_py_digest {
    ($t:ty, $size:expr) => {
        impl<'py> IntoPyObject<'py> for PyDigest<$t> {
            type Target = PyBytes;
            type Output = Bound<'py, Self::Target>;
            type Error = PyErr;

            #[inline]
            fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
                let i = self.0;
                let bytes: [u8; $size] = i.to_be_bytes();
                Ok(PyBytes::new(py, &bytes))
            }
        }
    };
}

impl_into_py_object_py_digest!(u32, 4);
impl_into_py_object_py_digest!(u64, 8);
impl_into_py_object_py_digest!(u128, 16);

impl<'py> IntoPyObject<'py> for PyHexDigest<u32> {
    type Target = PyString;
    type Output = Bound<'py, Self::Target>;
    type Error = std::convert::Infallible;

    #[inline]
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        let s = format!("{:08x}", self.0);
        Ok(pystring_fast_new(py, &s, true))
    }
}

impl<'py> IntoPyObject<'py> for PyHexDigest<u64> {
    type Target = PyString;
    type Output = Bound<'py, Self::Target>;
    type Error = std::convert::Infallible;

    #[inline]
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        let s = format!("{:016x}", self.0);
        Ok(pystring_fast_new(py, &s, true))
    }
}

impl<'py> IntoPyObject<'py> for PyHexDigest<u128> {
    type Target = PyString;
    type Output = Bound<'py, Self::Target>;
    type Error = std::convert::Infallible;

    #[inline]
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        let s = format!("{:032x}", self.0);
        Ok(pystring_fast_new(py, &s, true))
    }
}
