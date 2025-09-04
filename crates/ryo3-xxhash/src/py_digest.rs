use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyString};
use pyo3::{IntoPyObject, Python};
use ryo3_core::pystring::pystring_fast_new;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PyDigest<T>(pub T);
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PyHexDigest<T>(pub T);

impl<T> From<T> for PyDigest<T> {
    fn from(t: T) -> Self {
        Self(t)
    }
}

impl<T> From<T> for PyHexDigest<T> {
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
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        let s = format!("{:08x}", self.0);
        let pystr = pystring_fast_new(py, &s, true);
        Ok(pystr)
    }
}

impl<'py> IntoPyObject<'py> for PyHexDigest<u64> {
    type Target = PyString;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        let s = format!("{:016x}", self.0);
        let pystr = pystring_fast_new(py, &s, true);
        Ok(pystr)
    }
}

impl<'py> IntoPyObject<'py> for PyHexDigest<u128> {
    type Target = PyString;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        let s = format!("{:032x}", self.0);
        let pystr = pystring_fast_new(py, &s, true);
        Ok(pystr)
    }
}
// #[pyo3(signature = (data, *, seed = None))]
// pub fn xxh3_64_hexdigest(data: ryo3_bytes::PyBytes, seed: Option<u64>) -> PyResult<String> {
//     Ok(format!(
//         "{:016x}",
//         xxh3_64_with_seed(data.as_ref(), seed.unwrap_or(0))
//     ))
