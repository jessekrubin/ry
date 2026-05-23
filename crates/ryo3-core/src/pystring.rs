//! python-strings module
//!
//! REF(s):
//! - <https://github.com/pydantic/jiter/blob/main/crates/jiter/src/py_string_cache.rs>
use pyo3::prelude::*;
use pyo3::types::PyString;

pub struct PyAsciiStr<'s>(&'s str);

impl<'py> IntoPyObject<'py> for PyAsciiStr<'_> {
    type Target = pyo3::types::PyString;
    type Output = Bound<'py, Self::Target>;
    type Error = std::convert::Infallible;

    #[cfg_attr(not(any(PyPy, GraalPy, Py_LIMITED_API)), expect(unsafe_code))]
    #[inline]
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        debug_assert!(
            self.0.is_ascii(),
            "PyAsciiStr(ing) must be ascii only: {:?}",
            self.0
        );
        #[cfg(not(any(PyPy, GraalPy, Py_LIMITED_API)))]
        {
            unsafe { Ok(pystring_ascii_new(py, self.0)) }
        }
        #[cfg(any(PyPy, GraalPy, Py_LIMITED_API))]
        {
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

impl From<String> for PyAsciiString {
    #[inline]
    fn from(s: String) -> Self {
        debug_assert!(s.is_ascii(), "PyAsciiString must be ascii only: {s:?}");
        Self(s)
    }
}

pub struct PyAsciiString(String);

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
/// Ascii only (as jiter ppl describe)
#[must_use]
#[inline]
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
/// `s` must be ASCII only (will debug-assert)
#[inline]
#[must_use]
pub fn pystring_fast_new_ascii<'py>(py: Python<'py>, s: &str) -> Bound<'py, PyString> {
    debug_assert!(s.is_ascii(), "pystring_fast_new_ascii expects ASCII");
    #[cfg(not(any(PyPy, GraalPy, Py_LIMITED_API)))]
    #[expect(unsafe_code)]
    unsafe {
        pystring_ascii_new(py, s)
    }
    #[cfg(any(PyPy, GraalPy, Py_LIMITED_API))]
    {
        pystring_ascii_new(py, s)
    }
}

/// Creates a new `PyString` from an ASCII string.
///
/// # Safety
///
/// `s` must be ASCII only
#[cfg(not(any(PyPy, GraalPy, Py_LIMITED_API)))]
#[expect(unsafe_code, clippy::cast_possible_wrap)]
#[inline]
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
#[must_use]
#[inline]
pub fn pystring_ascii_new<'py>(py: Python<'py>, s: &str) -> Bound<'py, PyString> {
    PyString::new(py, s)
}

#[cfg(not(any(PyPy, GraalPy, Py_LIMITED_API)))]
mod pystr_danger_zone {
    #![expect(clippy::inline_always)]
    //! Danger zone for fast str read based on orjson's approach.
    //!
    //! Using this yields a 10% ish speedup....
    //!
    //! REF: <https://github.com/ijl/orjson/blob/master/src/str/pystr.rs>

    use pyo3::ffi::PyObject;

    #[cfg(all(target_endian = "little", Py_3_14, Py_GIL_DISABLED))]
    const STATE_KIND_SHIFT: usize = 8;
    #[cfg(all(target_endian = "little", not(all(Py_3_14, Py_GIL_DISABLED))))]
    const STATE_KIND_SHIFT: usize = 2;
    #[cfg(target_endian = "little")]
    const STATE_COMPACT: u32 = 1 << 5;
    #[cfg(target_endian = "little")]
    const STATE_COMPACT_ASCII: u32 =
        1 << STATE_KIND_SHIFT | 1 << (STATE_KIND_SHIFT + 3) | 1 << (STATE_KIND_SHIFT + 4);

    #[inline(always)]
    #[allow(clippy::cast_sign_loss)]
    pub(crate) fn isize_to_usize(val: isize) -> usize {
        debug_assert!(val >= 0);
        val as usize
    }

    #[inline(always)]
    #[expect(unsafe_code)]
    unsafe fn str_from_slice_fn(ptr: *const u8, size: usize) -> &'static str {
        unsafe { core::str::from_utf8_unchecked(core::slice::from_raw_parts(ptr, size)) }
    }

    #[inline(always)]
    #[expect(unsafe_code)]
    unsafe fn to_str_via_ffi(op: *mut PyObject) -> Option<&'static str> {
        let mut size: isize = 0;
        let ptr = unsafe { pyo3::ffi::PyUnicode_AsUTF8AndSize(op, &raw mut size) };
        if ptr.is_null() {
            None
        } else {
            let s = unsafe { str_from_slice_fn(ptr.cast::<u8>(), isize_to_usize(size)) };
            Some(s)
        }
    }

    #[inline(always)]
    #[expect(unsafe_code)]
    #[cfg(target_endian = "little")]
    pub(crate) unsafe fn fast_pystr_read<'a>(op: *mut PyObject) -> Option<&'a str> {
        // local use for little-endian only ~ good no more warning
        use pyo3::ffi::{PyASCIIObject, PyCompactUnicodeObject};
        unsafe {
            let state = (*op.cast::<PyASCIIObject>()).state;
            if state & STATE_COMPACT_ASCII == STATE_COMPACT_ASCII {
                let ptr = op.cast::<PyASCIIObject>().add(1).cast::<u8>();
                let len = isize_to_usize((*op.cast::<PyASCIIObject>()).length);
                let s = str_from_slice_fn(ptr, len);
                Some(s)
            } else if (state & STATE_COMPACT) != 0
                && (*op.cast::<PyCompactUnicodeObject>()).utf8_length > 0
            {
                let ptr = ((*op.cast::<PyCompactUnicodeObject>()).utf8).cast::<u8>();
                let len = isize_to_usize((*op.cast::<PyCompactUnicodeObject>()).utf8_length);
                let s = str_from_slice_fn(ptr, len);
                Some(s)
            } else {
                to_str_via_ffi(op)
            }
        }
    }

    #[inline(always)]
    #[expect(unsafe_code)]
    #[cfg(not(target_endian = "little"))]
    pub(crate) unsafe fn fast_pystr_read<'a>(op: *mut PyObject) -> Option<&'a str> {
        unsafe {
            // TODO: maybe implement big-endian fast path later orjson does not?
            to_str_via_ffi(op)
        }
    }
}

pub(crate) fn fast_pystr_read<'a>(op: Borrowed<'_, '_, PyString>) -> Option<&'a str> {
    #[cfg(not(any(PyPy, GraalPy, Py_LIMITED_API)))]
    {
        #[expect(unsafe_code)]
        unsafe {
            pystr_danger_zone::fast_pystr_read(op.as_ptr())
        }
    }
    #[cfg(any(PyPy, GraalPy, Py_LIMITED_API))]
    {
        op.to_str().ok()
    }
}
