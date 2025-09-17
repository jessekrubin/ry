//! python-strings module
//!
//! REF(s):
//! - <https://github.com/pydantic/jiter/blob/main/crates/jiter/src/py_string_cache.rs>
use pyo3::prelude::*;
use pyo3::types::PyString;

/// Faster py-string creation as done by jiter + orjson
///
/// # Safety
///
/// Ascii only (as jiter ppl describe)
#[must_use]
pub fn pystring_fast_new<'py>(py: Python<'py>, s: &str, ascii_only: bool) -> Bound<'py, PyString> {
    if ascii_only {
        #[expect(unsafe_code)]
        #[cfg(not(any(PyPy, GraalPy, Py_LIMITED_API)))]
        unsafe {
            pystring_ascii_new(py, s)
        }

        #[cfg(any(PyPy, GraalPy, Py_LIMITED_API))]
        {
            pystring_ascii_new(py, s)
        }
    } else {
        PyString::new(py, s)
    }
}

/// Creates a new `PyString` from an ASCII string.
///
/// # Safety
///
/// `s` must be ASCII only
#[cfg(not(any(PyPy, GraalPy, Py_LIMITED_API)))]
#[expect(unsafe_code, clippy::cast_possible_wrap)]
#[must_use]
pub unsafe fn pystring_ascii_new<'py>(py: Python<'py>, s: &str) -> Bound<'py, PyString> {
    unsafe {
        let ptr = pyo3::ffi::PyUnicode_New(s.len() as isize, 127);
        // see https://github.com/pydantic/jiter/pull/72#discussion_r1545485907
        debug_assert_eq!(
            pyo3::ffi::PyUnicode_KIND(ptr),
            pyo3::ffi::PyUnicode_1BYTE_KIND
        );
        let data_ptr = pyo3::ffi::PyUnicode_DATA(ptr).cast();
        core::ptr::copy_nonoverlapping(s.as_ptr(), data_ptr, s.len());
        core::ptr::write(data_ptr.add(s.len()), 0);
        Bound::from_owned_ptr(py, ptr).cast_into_unchecked()
    }
}

#[cfg(any(PyPy, GraalPy, Py_LIMITED_API))]
pub fn pystring_ascii_new<'py>(py: Python<'py>, s: &str) -> Bound<'py, PyString> {
    PyString::new(py, s)
}
