use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyString};

use crate::py_str::pystring_fast_new_ascii;

const HEX_CHARS_LOWER: &[u8; 16] = b"0123456789abcdef";

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PyDigest<T>(T);
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PyHexDigest<T>(T);

impl<T> PyDigest<T> {
    #[inline]
    #[must_use]
    pub fn new(t: T) -> Self {
        Self(t)
    }

    #[inline]
    #[must_use]
    pub fn inner(&self) -> &T {
        &self.0
    }

    #[inline]
    #[must_use]
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T> PyHexDigest<T> {
    #[inline]
    #[must_use]
    pub fn new(t: T) -> Self {
        Self(t)
    }

    #[inline]
    #[must_use]
    pub fn inner(&self) -> &T {
        &self.0
    }

    #[inline]
    #[must_use]
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T> From<T> for PyDigest<T> {
    #[inline]
    fn from(t: T) -> Self {
        Self::new(t)
    }
}

impl<T> From<T> for PyHexDigest<T> {
    #[inline]
    fn from(t: T) -> Self {
        Self::new(t)
    }
}

macro_rules! impl_into_py_object_py_digest_uint {
    ($t:ty, $size:expr) => {
        impl<'py> IntoPyObject<'py> for PyDigest<$t> {
            type Target = PyBytes;
            type Output = Bound<'py, Self::Target>;
            type Error = std::convert::Infallible;

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

#[inline]
fn encode_hex_ref<const N: usize, const S: usize>(bytes: &[u8; N]) -> [u8; S] {
    debug_assert!(S == N * 2, "S != N * 2");
    let mut out = [0u8; S];
    for i in 0..N {
        let b = bytes[i];
        out[i * 2] = HEX_CHARS_LOWER[(b >> 4) as usize];
        out[i * 2 + 1] = HEX_CHARS_LOWER[(b & 0x0f) as usize];
    }
    out
}

/// SUPA UNSAFE function to create a hex digest string from a digest byte array
#[inline]
fn pystring_hex_digest<'py, const N: usize>(
    py: Python<'py>,
    digest: &[u8; N],
) -> Bound<'py, PyString> {
    let hex_len = N * 2;
    debug_assert!(
        digest.len() == N,
        "Digest length does not match expected size: {} != {}",
        digest.len(),
        N
    );
    #[cfg(not(any(PyPy, GraalPy, Py_LIMITED_API)))]
    {
        #[expect(unsafe_code, clippy::cast_possible_wrap)]
        unsafe {
            let ptr = pyo3::ffi::PyUnicode_New(hex_len as isize, 127);
            debug_assert_eq!(
                pyo3::ffi::PyUnicode_KIND(ptr),
                pyo3::ffi::PyUnicode_1BYTE_KIND
            );
            let out = pyo3::ffi::PyUnicode_DATA(ptr).cast::<u8>();
            for (i, &b) in digest.iter().enumerate() {
                *out.add(i * 2) = HEX_CHARS_LOWER[(b >> 4) as usize];
                *out.add(i * 2 + 1) = HEX_CHARS_LOWER[(b & 0x0f) as usize];
            }
            core::ptr::write(out.add(hex_len), 0);
            Bound::from_owned_ptr(py, ptr).cast_into_unchecked()
        }
    }

    #[cfg(any(PyPy, GraalPy, Py_LIMITED_API))]
    {
        let mut out = vec![0u8; hex_len];
        for (i, &b) in digest.iter().enumerate() {
            out[i * 2] = HEX_CHARS_LOWER[(b >> 4) as usize];
            out[i * 2 + 1] = HEX_CHARS_LOWER[(b & 0x0f) as usize];
        }
        #[expect(unsafe_code)]
        let s = unsafe { std::str::from_utf8_unchecked(&out) };
        pystring_fast_new_ascii(py, s)
    }
}

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

macro_rules! impl_into_py_object_for_bytes_digest {
    (
        bin_size = $size:expr,
        hex_size = $hex_size:expr
    ) => {
        impl<'py> IntoPyObject<'py> for PyHexDigest<[u8; $size]> {
            type Target = PyString;
            type Output = Bound<'py, Self::Target>;
            type Error = std::convert::Infallible;

            #[inline]
            fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
                let bytes = encode_hex_ref::<$size, $hex_size>(&self.0);
                #[expect(unsafe_code)]
                let s = unsafe { std::str::from_utf8_unchecked(&bytes) };
                Ok(pystring_fast_new_ascii(py, s))
            }
        }
    };
}

impl<'py, const SIZE: usize> IntoPyObject<'py> for PyHexDigest<&[u8; SIZE]> {
    type Target = PyString;
    type Output = Bound<'py, Self::Target>;
    type Error = std::convert::Infallible;

    #[inline]
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        Ok(pystring_hex_digest(py, self.0))
    }
}

impl_into_py_object_for_bytes_digest!(bin_size = 20, hex_size = 40);
impl_into_py_object_for_bytes_digest!(bin_size = 28, hex_size = 56);

impl_into_py_object_for_bytes_digest!(bin_size = 32, hex_size = 64);
impl_into_py_object_for_bytes_digest!(bin_size = 48, hex_size = 96);
impl_into_py_object_for_bytes_digest!(bin_size = 64, hex_size = 128);
impl_into_py_object_for_bytes_digest!(bin_size = 128, hex_size = 256);
