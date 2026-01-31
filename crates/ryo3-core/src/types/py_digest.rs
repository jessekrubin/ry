use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyString};
use pyo3::{IntoPyObject, Python};

use crate::pystring_fast_new;

const HEX: &[u8; 16] = b"0123456789abcdef";
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

// bytes

impl<'py> IntoPyObject<'py> for PyDigest<&[u8; 32]> {
    type Target = PyBytes;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    #[inline]
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        let i = self.0;
        let bytes: [u8; 32] = *i;
        Ok(PyBytes::new(py, &bytes))
    }
}

impl<'py> IntoPyObject<'py> for PyHexDigest<String> {
    type Target = PyString;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    #[inline]
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        Ok(pystring_fast_new(py, &self.0, true))
    }
}

impl From<&[u8; 32]> for PyHexDigest<String> {
    fn from(b: &[u8; 32]) -> Self {
        let s = {
            let mut s = String::with_capacity(64);
            for byte in b {
                s.push(HEX[(byte >> 4) as usize] as char);
                s.push(HEX[(byte & 0x0f) as usize] as char);
            }
            s
        };
        Self(s)
    }
}

macro_rules! impl_from_bytes_py_hex_digest_string {
    ($size:expr) => {
        impl From<&[u8; $size]> for PyHexDigest<String> {
            fn from(b: &[u8; $size]) -> Self {
                const STR_LEN: usize = $size * 2;
                let mut s = String::with_capacity(STR_LEN);
                for byte in b {
                    s.push(HEX[(byte >> 4) as usize] as char);
                    s.push(HEX[(byte & 0x0f) as usize] as char);
                }
                Self(s)
            }
        }
    };
}

impl_from_bytes_py_hex_digest_string!(64); // 8 * 64 = 512

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

////////////

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PyDigestBytes<'a, const SIZE: usize>(pub &'a [u8; SIZE]);

impl<'py, const SIZE: usize> IntoPyObject<'py> for PyDigestBytes<'_, SIZE> {
    type Target = PyBytes;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    #[inline]
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        let i = self.0;
        // let bytes: [u8; SIZE] = *i;
        Ok(PyBytes::new(py, self.0))
    }
}

impl<'a, const SIZE: usize> From<&'a [u8; SIZE]> for PyDigestBytes<'a, SIZE> {
    #[inline]
    fn from(b: &'a [u8; SIZE]) -> Self {
        Self(b)
    }
}
