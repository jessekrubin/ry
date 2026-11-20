//! python-strings module
//!
//! REF(s):
//! - <https://github.com/pydantic/jiter/blob/main/crates/jiter/src/py_string_cache.rs>
use pyo3::prelude::*;
use pyo3::types::PyString;

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

/// Faster py-string creation as done by jiter + orjson
///
/// # Safety
///
/// If `ascii_only` is true, `s` must be ASCII only.
#[must_use]
#[inline]
#[expect(unsafe_code)]
pub unsafe fn pystring_fast_new<'py>(
    py: Python<'py>,
    s: &str,
    ascii_only: bool,
) -> Bound<'py, PyString> {
    debug_assert!(!ascii_only || s.is_ascii());

    if ascii_only {
        // SAFETY: required by this function's contract.
        unsafe { pystring_ascii_new(py, s) }
    } else {
        PyString::new(py, s)
    }
}

/// Creates a new `PyString` from an ASCII string.
///
/// # Safety
///
/// `s` must be ASCII only (will debug-assert)
#[inline]
#[must_use]
#[expect(unsafe_code)]
pub unsafe fn pystring_fast_new_ascii<'py>(py: Python<'py>, s: &str) -> Bound<'py, PyString> {
    debug_assert!(s.is_ascii(), "pystring_fast_new_ascii expects ASCII");

    // SAFETY: required by this function's contract.
    unsafe { pystring_ascii_new(py, s) }
}

/// Creates a new `PyString` from an ASCII string.
///
/// # Safety
///
/// `s` must be ASCII only
#[expect(unsafe_code)]
#[inline]
#[must_use]
pub unsafe fn pystring_ascii_new<'py>(py: Python<'py>, s: &str) -> Bound<'py, PyString> {
    cfg_select! {
        any(PyPy, GraalPy, Py_LIMITED_API) => PyString::new(py, s),
        _ => {unsafe {
            let ptr = pyo3::ffi::PyUnicode_New(s.len().cast_signed(), 127);
            // convert the pointer BEFORE using the Unicode macros.
            // this handles allocation failure how `PyString::new` does, w/ a
            // panic on OOM and not doing ptr shenanigans on NULL.
            // REF: https://github.com/pydantic/jiter/issues/261
            let string: Bound<'py, PyString> = Bound::from_owned_ptr(py, ptr).cast_into_unchecked();
            let ptr = string.as_ptr();
            // REF: https://github.com/pydantic/jiter/pull/72#discussion_r1545485907
            debug_assert_eq!(
                pyo3::ffi::PyUnicode_KIND(ptr),
                pyo3::ffi::PyUnicode_1BYTE_KIND
            );
            let data_ptr = pyo3::ffi::PyUnicode_DATA(ptr).cast();
            core::ptr::copy_nonoverlapping(s.as_ptr(), data_ptr, s.len());
            core::ptr::write(data_ptr.add(s.len()), 0);
            string
        }}
    }
}

/// Faster py-string read inspired by orjson
///
/// # Safety
/// - Only use on actual `PyString` objects (not subclasses)
#[expect(unsafe_code)]
#[inline]
#[must_use]
pub unsafe fn pystr_read_fast_opt<'a>(ob: Borrowed<'a, '_, PyString>) -> Option<&'a str> {
    cfg_select! {
        any(PyPy, GraalPy, Py_LIMITED_API) => unsafe { crate::ffi::pyunicode::pyunicode_to_str_via_ffi(ob.as_ptr()) },
        _ => unsafe { crate::ffi::pyunicode::fast_pystr_read(ob.as_ptr()) },
    }
}

#[expect(unsafe_code)]
#[inline]
pub fn pystr_read_fast<'a>(ob: Borrowed<'a, '_, PyString>) -> PyResult<&'a str> {
    unsafe { pystr_read_fast_opt(ob) }.ok_or_else(|| PyErr::fetch(ob.py()))
}
