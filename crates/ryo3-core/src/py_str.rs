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
        _ => {
            unsafe {
                let ptr = pyo3::ffi::PyUnicode_New(s.len().cast_signed(), 127);
                // convert the pointer BEFORE using the Unicode macros.
                // this handles allocation failure how `PyString::new` does, w/ a
                // panic on OOM and not doing ptr shenanigans on NULL.
                // REF: https://github.com/pydantic/jiter/issues/261
                let string: Bound<'py, PyString> =
                    Bound::from_owned_ptr(py, ptr).cast_into_unchecked();
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
            }
        }
    }
}

/// `PyUnicode_AsUTF8AndSize` fallback ~ what pyo3 does internally w/o tying
/// the result to the `to_str()` wrapper
#[inline]
#[expect(unsafe_code)]
unsafe fn pyunicode_to_str_via_ffi<'a>(op: *mut pyo3::ffi::PyObject) -> Option<&'a str> {
    unsafe {
        let mut size: pyo3::ffi::Py_ssize_t = 0;
        let ptr = pyo3::ffi::PyUnicode_AsUTF8AndSize(op, &raw mut size).cast::<u8>();
        if ptr.is_null() {
            None
        } else {
            Some(core::str::from_utf8_unchecked(core::slice::from_raw_parts(
                ptr,
                size.cast_unsigned(),
            )))
        }
    }
}

#[cfg(not(any(PyPy, GraalPy, Py_LIMITED_API)))]
mod dangerzone {
    #![expect(clippy::inline_always)]
    //! Danger zone for fast str read based on orjson's approach.
    //!
    //! Using this yields a 10% ish speedup....
    //!
    //! `PyASCIIObject.state` bitfield layout (AFAICT):
    //! - with-gil: `interned:2` so `kind` starts at bit 2
    //! - sans-gil (Py_GIL_DISABLED): `interned` is a whole `unsigned char` (atomic)
    //!   so `kind` starts at bit 8
    //!
    //! `compact`/`ascii` follow `kind:3` (`+3`/`+4`)
    //!
    //! REF:
    //!   - orjson: <https://github.com/ijl/orjson/blob/master/src/ffi/pystrref/object.rs>
    //!   - cpython: <https://github.com/python/cpython/blob/3357cb57dcdf758ed22d886035f4a0ab408d8f7a/Include/cpython/unicodeobject.h#L117-L122>
    use pyo3::ffi::PyObject;

    use super::pyunicode_to_str_via_ffi;

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
    pub(crate) unsafe fn fast_pystr_read<'a>(op: *mut PyObject) -> Option<&'a str> {
        cfg_select! {
            target_endian = "little" => {
                // local use for little-endian only ~ good no more warning
                use pyo3::ffi::{PyASCIIObject, PyCompactUnicodeObject};
                const STATE_KIND_SHIFT: usize = cfg_select! {
                    all(Py_3_14, Py_GIL_DISABLED) => 8,
                    _ => 2,
                };
                const STATE_COMPACT: u32 = 1 << (STATE_KIND_SHIFT + 3);
                const STATE_COMPACT_ASCII: u32 =
                    1 << STATE_KIND_SHIFT | STATE_COMPACT | 1 << (STATE_KIND_SHIFT + 4);
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
                        let len =
                            isize_to_usize((*op.cast::<PyCompactUnicodeObject>()).utf8_length);
                        let s = str_from_slice_fn(ptr, len);
                        Some(s)
                    } else {
                        pyunicode_to_str_via_ffi(op)
                    }
                }
            }
            _ => {
                // no big-endian fast path (orjson & ormsgpack dont support it)
                unsafe { pyunicode_to_str_via_ffi(op) }
            }
        }
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
        any(PyPy, GraalPy, Py_LIMITED_API) => unsafe { pyunicode_to_str_via_ffi(ob.as_ptr()) },
        _ => unsafe { dangerzone::fast_pystr_read(ob.as_ptr()) },
    }
}

#[expect(unsafe_code)]
#[inline]
pub fn pystr_read_fast<'a>(ob: Borrowed<'a, '_, PyString>) -> PyResult<&'a str> {
    unsafe { pystr_read_fast_opt(ob) }.ok_or_else(|| PyErr::fetch(ob.py()))
}
