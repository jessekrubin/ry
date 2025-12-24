use pyo3::prelude::*;
use serde::ser::{Serialize, Serializer};

use crate::errors::pyerr2sererr;
use pyo3::types::PyString;

pub(crate) struct PyStrSerializer<'a, 'py> {
    obj: Borrowed<'a, 'py, PyAny>,
}

impl<'a, 'py> PyStrSerializer<'a, 'py> {
    #[inline]
    pub(crate) fn new(obj: Borrowed<'a, 'py, PyAny>) -> Self {
        Self { obj }
    }
}

#[cfg(not(any(PyPy, GraalPy, Py_LIMITED_API)))]
impl Serialize for PyStrSerializer<'_, '_> {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use pystr_danger_zone::fast_pystr_read;

        #[expect(unsafe_code)]
        let s = unsafe { fast_pystr_read(self.obj.as_ptr()) };
        if let Some(s) = s {
            serializer.serialize_str(s)
        } else {
            // error here...
            crate::serde_err!("invalid str object")
        }
    }
}

#[cfg(any(PyPy, GraalPy, Py_LIMITED_API))]
impl Serialize for PyStrSerializer<'_, '_> {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let pystr = self.obj.cast_exact::<PyString>().map_err(pyerr2sererr)?;
        let s = pystr.to_str().map_err(pyerr2sererr)?;
        serializer.serialize_str(s)
    }
}

pub(crate) struct PyStrSubclassSerializer<'a, 'py> {
    obj: Borrowed<'a, 'py, PyString>,
}

impl<'a, 'py> PyStrSubclassSerializer<'a, 'py> {
    #[inline]
    pub(crate) fn new(obj: Borrowed<'a, 'py, PyString>) -> Self {
        Self { obj }
    }
}

impl Serialize for PyStrSubclassSerializer<'_, '_> {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = self.obj.to_str().map_err(pyerr2sererr)?;
        serializer.serialize_str(s)
    }
}

#[cfg(not(any(PyPy, GraalPy, Py_LIMITED_API)))]
mod pystr_danger_zone {
    //! Danger zone for fast str read based on orjson's approach.
    //!
    //! Using this yields a 10% ish speedup....
    //!
    //! REF: <https://github.com/ijl/orjson/blob/master/src/str/pystr.rs>

    use pyo3::ffi::{PyASCIIObject, PyCompactUnicodeObject, PyObject};

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
