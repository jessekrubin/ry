use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyString};
use pyo3::{IntoPyObject, Python};

use crate::pystring::pystring_fast_new_ascii;

const HEX_CHARS_LOWER: &[u8; 16] = b"0123456789abcdef";

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

macro_rules! impl_into_py_object_py_digest_uint {
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

impl_into_py_object_py_digest_uint!(u32, 4);
impl_into_py_object_py_digest_uint!(u64, 8);
impl_into_py_object_py_digest_uint!(u128, 16);

#[inline]
fn encode_hex<const N: usize, const S: usize>(bytes: [u8; N]) -> [u8; S] {
    debug_assert!(S == N * 2, "S != N * 2");
    let mut out = [0u8; S];
    for i in 0..N {
        let b = bytes[i];
        out[i * 2] = HEX_CHARS_LOWER[(b >> 4) as usize];
        out[i * 2 + 1] = HEX_CHARS_LOWER[(b & 0x0f) as usize];
    }
    out
}

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
        let bytes = encode_hex::<4, 8>(self.0.to_be_bytes());
        #[expect(unsafe_code)]
        let s = unsafe { std::str::from_utf8_unchecked(&bytes) };
        Ok(pystring_fast_new_ascii(py, s))
    }
}

impl<'py> IntoPyObject<'py> for PyHexDigest<u64> {
    type Target = PyString;
    type Output = Bound<'py, Self::Target>;
    type Error = std::convert::Infallible;

    #[inline]
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        let bytes = encode_hex::<8, 16>(self.0.to_be_bytes());
        #[expect(unsafe_code)]
        let s = unsafe { std::str::from_utf8_unchecked(&bytes) };
        Ok(pystring_fast_new_ascii(py, s))
    }
}

impl<'py> IntoPyObject<'py> for PyHexDigest<u128> {
    type Target = PyString;
    type Output = Bound<'py, Self::Target>;
    type Error = std::convert::Infallible;

    #[inline]
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        let bytes = encode_hex::<16, 32>(self.0.to_be_bytes());
        #[expect(unsafe_code)]
        let s = unsafe { std::str::from_utf8_unchecked(&bytes) };
        Ok(pystring_fast_new_ascii(py, s))
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
