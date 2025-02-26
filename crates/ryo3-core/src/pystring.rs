//! python-strings module
//!
//! REF(s):
//! - <https://github.com/pydantic/jiter/blob/main/crates/jiter/src/py_string_cache.rs>
use pyo3::prelude::*;
use pyo3::types::PyString;

/// Faster py-string creation as done by jiter + orjson
#[must_use]
pub fn pystring_fast_new<'py>(py: Python<'py>, s: &str, ascii_only: bool) -> Bound<'py, PyString> {
    if ascii_only {
        #[expect(unsafe_code)]
        unsafe {
            pystring_ascii_new(py, s)
        }
    } else {
        PyString::new(py, s)
    }
}

/// Faster creation of `PyString` from an ASCII string, inspired by
/// <https://github.com/ijl/orjson/blob/3.10.0/src/str/create.rs#L41>
#[expect(unsafe_code)]
#[expect(clippy::cast_possible_wrap)]
#[cfg(not(any(PyPy, GraalPy)))]
unsafe fn pystring_ascii_new<'py>(py: Python<'py>, s: &str) -> Bound<'py, PyString> {
    // disabled on everything except tier-1 platforms because of a crash in the built wheels from CI,
    // see https://github.com/pydantic/jiter/pull/175
    let ptr = pyo3::ffi::PyUnicode_New(s.len() as isize, 127);
    debug_assert_eq!(
        pyo3::ffi::PyUnicode_KIND(ptr),
        pyo3::ffi::PyUnicode_1BYTE_KIND
    );
    let data_ptr = pyo3::ffi::PyUnicode_DATA(ptr).cast();
    core::ptr::copy_nonoverlapping(s.as_ptr(), data_ptr, s.len());
    core::ptr::write(data_ptr.add(s.len()), 0);
    Bound::from_owned_ptr(py, ptr).downcast_into_unchecked()
}

// unoptimized version (albeit not that much slower) on other platforms
#[expect(unsafe_code)]
#[cfg(any(PyPy, GraalPy))]
unsafe fn pystring_ascii_new<'py>(py: Python<'py>, s: &str) -> Bound<'py, PyString> {
    PyString::new(py, s)
}
