//! `PyString` fast read ~ orjson-style direct `PyASCIIObject` access
//!
//! Using this yields a 10% ish speedup....
//!
//! `PyASCIIObject.state` bitfield layout (AFAICT):
//! - with-gil: `interned:2` SOOOOO `kind` starts at the 2nd-bit
//! - sans-gil (`Py_GIL_DISABLED`): `interned` is a whole `unsigned char` (atomic)
//!   so `kind` starts at the 8th-bit
//!
//! `compact`/`ascii` follow `kind:3` (`+3`/`+4`)
//!
//! REF:
//!   - orjson: <https://github.com/ijl/orjson/blob/master/src/ffi/pystrref/object.rs>
//!   - cpython: <https://github.com/python/cpython/blob/3357cb57dcdf758ed22d886035f4a0ab408d8f7a/Include/cpython/unicodeobject.h#L117-L122>
use pyo3::ffi::PyObject;

/// `PyUnicode_AsUTF8AndSize` fallback ~ what pyo3 does internally w/o tying
/// the result to the `to_str()` wrapper
#[inline]
#[expect(unsafe_code)]
pub(crate) unsafe fn pyunicode_to_str_via_ffi<'a>(op: *mut PyObject) -> Option<&'a str> {
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
#[expect(clippy::inline_always)]
#[inline(always)]
#[expect(unsafe_code)]
pub(crate) unsafe fn fast_pystr_read<'a>(op: *mut PyObject) -> Option<&'a str> {
    cfg_select! {
        target_endian = "little" => {
            // local use for little-endian only ~ good no more warning
            use pyo3::ffi::{PyASCIIObject, PyCompactUnicodeObject};

            #[inline(always)]
            #[allow(clippy::cast_sign_loss)]
            fn isize_to_usize(val: isize) -> usize {
                debug_assert!(val >= 0);
                val as usize
            }

            #[inline(always)]
            #[expect(unsafe_code)]
            unsafe fn str_from_slice_fn<'s>(ptr: *const u8, size: usize) -> &'s str {
                unsafe { core::str::from_utf8_unchecked(core::slice::from_raw_parts(ptr, size)) }
            }

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
                    let len = isize_to_usize((*op.cast::<PyCompactUnicodeObject>()).utf8_length);
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
