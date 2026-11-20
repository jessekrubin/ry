//! ascii-only str newtypes ~ skip the utf8 scan when making a `PyString`
use pyo3::prelude::*;

use crate::pystring_ascii_new;

pub struct PyAsciiStr<'s>(&'s str);

impl<'a> PyAsciiStr<'a> {
    /// creates a new `PyAsciiStr` from an ASCII string.
    ///
    /// # Panics
    ///
    /// Panics if `s` is not ASCII.
    #[inline]
    #[must_use]
    pub fn new(s: &'a str) -> Self {
        assert!(s.is_ascii(), "PyAsciiStr(ing) must be ascii only: {s:?}");
        Self(s)
    }

    /// # Safety
    ///
    /// `s` must be ASCII only (will debug-assert)
    #[expect(unsafe_code)]
    #[inline]
    #[must_use]
    pub unsafe fn new_unchecked(s: &'a str) -> Self {
        debug_assert!(s.is_ascii(), "PyAsciiStr(ing) must be ascii only: {s:?}");
        Self(s)
    }
}

impl<'py> IntoPyObject<'py> for PyAsciiStr<'_> {
    type Target = pyo3::types::PyString;
    type Output = Bound<'py, Self::Target>;
    type Error = std::convert::Infallible;

    #[inline]
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        debug_assert!(
            self.0.is_ascii(),
            "PyAsciiStr(ing) must be ascii only: {:?}",
            self.0
        );
        #[expect(unsafe_code)]
        unsafe {
            Ok(pystring_ascii_new(py, self.0))
        }
    }
}

impl<'s> From<&'s str> for PyAsciiStr<'s> {
    #[inline]
    fn from(s: &'s str) -> Self {
        debug_assert!(s.is_ascii(), "PyAsciiStr(ing) must be ascii only: {s:?}");
        Self(s)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyAsciiString(String);

impl PyAsciiString {
    /// creates a new `PyAsciiString` from an ASCII string.
    ///
    /// # Panics
    ///
    /// Panics if `s` is not ASCII.
    #[inline]
    #[must_use]
    pub fn new(s: String) -> Self {
        assert!(s.is_ascii(), "PyAsciiString must be ascii only: {s:?}");
        Self(s)
    }

    /// # Safety
    ///
    /// `s` must be ASCII only (will debug-assert)
    #[expect(unsafe_code)]
    #[inline]
    #[must_use]
    pub unsafe fn new_unchecked(s: String) -> Self {
        debug_assert!(s.is_ascii(), "PyAsciiString must be ascii only: {s:?}");
        Self(s)
    }
}

impl From<String> for PyAsciiString {
    #[inline]
    #[expect(unsafe_code)]
    fn from(s: String) -> Self {
        unsafe { Self::new_unchecked(s) }
    }
}

impl<'py> IntoPyObject<'py> for &PyAsciiString {
    type Target = pyo3::types::PyString;
    type Output = Bound<'py, Self::Target>;
    type Error = std::convert::Infallible;

    #[inline]
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        PyAsciiStr(&self.0).into_pyobject(py)
    }
}

impl<'py> IntoPyObject<'py> for PyAsciiString {
    type Target = pyo3::types::PyString;
    type Output = Bound<'py, Self::Target>;
    type Error = std::convert::Infallible;

    #[inline]
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        PyAsciiStr(&self.0).into_pyobject(py)
    }
}
