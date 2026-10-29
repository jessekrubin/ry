//! `Bytes` known to be utf-8 (AKA a zero-copy string)
//!
//! used by sse (event `data` sliced out of the response chunk) and multipart
//! (python `str` values handed to reqwest w/o copying)
use std::ops::Deref;

use bytes::Bytes;
use pyo3::prelude::*;
use pyo3::pybacked::PyBackedStr;
use pyo3::types::PyString;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Utf8Bytes(Bytes);

impl Utf8Bytes {
    #[inline]
    pub(crate) const fn from_static(s: &'static str) -> Self {
        Self(Bytes::from_static(s.as_bytes()))
    }

    /// zero-copy for valid utf-8; invalid bytes get replaced (and copied)
    pub(crate) fn from_bytes_lossy(bytes: Bytes) -> Self {
        if std::str::from_utf8(&bytes).is_ok() {
            Self(bytes)
        } else {
            Self(Bytes::from(String::from_utf8_lossy(&bytes).into_owned()))
        }
    }

    #[inline]
    pub(crate) fn as_str(&self) -> &str {
        // SAFETY: only ever built from valid utf-8
        #[expect(unsafe_code)]
        unsafe {
            std::str::from_utf8_unchecked(&self.0)
        }
    }

    #[inline]
    pub(crate) fn as_bytes(&self) -> &Bytes {
        &self.0
    }
}

impl Deref for Utf8Bytes {
    type Target = str;

    #[inline]
    fn deref(&self) -> &str {
        self.as_str()
    }
}

/// zero-copy ~ the `Bytes` own the python `str`
impl TryFrom<Bound<'_, PyString>> for Utf8Bytes {
    type Error = PyErr;

    #[inline]
    fn try_from(s: Bound<'_, PyString>) -> PyResult<Self> {
        PyBackedStr::try_from(s).map(|s| Self(Bytes::from_owner(s)))
    }
}

#[cfg(test)]
impl From<&str> for Utf8Bytes {
    fn from(s: &str) -> Self {
        Self(Bytes::copy_from_slice(s.as_bytes()))
    }
}

#[cfg(test)]
impl From<String> for Utf8Bytes {
    fn from(s: String) -> Self {
        Self(Bytes::from(s))
    }
}
