//! int/number helpers
use pyo3::prelude::*;
use pyo3::types::PyInt;

/// `PyNumber_Check`: has `__index__`/`__int__`/`__float__` (or is complex)
///
/// Fast check if `ob.extract::<u64>` et al would fail
#[inline]
#[must_use]
pub fn pynumber_check(ob: Borrowed<'_, '_, PyAny>) -> bool {
    ob.is_exact_instance_of::<PyInt>() || {
        #[expect(unsafe_code, reason = "valid object + attached")]
        unsafe {
            pyo3::ffi::PyNumber_Check(ob.as_ptr()) != 0
        }
    }
}

/// `PyIndex_Check`: is an int or has `__index__`
///
/// NOTE: this is stricter than [`pynumber_check`]
#[inline]
#[must_use]
pub fn pyindex_check(ob: Borrowed<'_, '_, PyAny>) -> bool {
    ob.is_exact_instance_of::<PyInt>() || {
        #[expect(unsafe_code, reason = "valid object + attached")]
        unsafe {
            pyo3::ffi::PyIndex_Check(ob.as_ptr()) != 0
        }
    }
}

/// value of a python int that fits in 64 bits
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PyIntValue {
    I64(i64),
    /// only for values > `i64::MAX`
    U64(u64),
}

/// Python int reading w/o `OverflowError`s ~ just `None` if it doesn't fit.
///
/// On cpython (`ryo3_pylong_layout`) the int's digits are read directly (like
/// orjson, ormsgpack and python-libipld) sans allocation(!).
///
/// NOT on cpython, int reading goes through ye olde ffi/pyo3 where only
/// out-of-range ints raise (+ get cleared).
///
/// `read_i128`/`read_u128` are separate (not on [`PyIntValue`]) to keep the
/// 64bit path petite.
pub trait PyIntRead {
    /// read as `i64` or `u64` or `None` if the int is too big for both
    fn read_int(&self) -> Option<PyIntValue>;

    /// read as `i128` or `None` if it doesn't fit
    fn read_i128(&self) -> Option<i128>;

    /// read as `u128` or `None` if it doesn't fit (or is negative)
    fn read_u128(&self) -> Option<u128>;
}

impl PyIntRead for Borrowed<'_, '_, PyInt> {
    #[inline]
    fn read_int(&self) -> Option<PyIntValue> {
        dangerzone::read_int(*self)
    }

    #[inline]
    fn read_i128(&self) -> Option<i128> {
        dangerzone::read_i128(*self)
    }

    #[inline]
    fn read_u128(&self) -> Option<u128> {
        dangerzone::read_u128(*self)
    }
}

impl PyIntRead for Bound<'_, PyInt> {
    #[inline]
    fn read_int(&self) -> Option<PyIntValue> {
        self.as_borrowed().read_int()
    }

    #[inline]
    fn read_i128(&self) -> Option<i128> {
        self.as_borrowed().read_i128()
    }

    #[inline]
    fn read_u128(&self) -> Option<u128> {
        self.as_borrowed().read_u128()
    }
}

#[cfg(ryo3_pylong_layout)]
mod dangerzone {
    //! reads up to 3 digits (all of i64/u64) or 5 digits (all of i128/u128)
    //!
    //! names are copied from cpython's `longintrepr.h` (+ `pycore_long.h` for
    //! `SIGN_NEGATIVE`) ~ 30-bit digits (cfg `ryo3_pylong_layout` is set by
    //! `build.rs` for 64-bit non-abi3 cpython only):
    //! - 3.11: `ob_size` (in the var-object header) is `sign * ndigits`
    //! - 3.12+: `lv_tag` is `ndigits << _PyLong_NON_SIZE_BITS | sign` where
    //!   sign is 0 (positive), 1 (`SIGN_ZERO`), 2 (`SIGN_NEGATIVE`)
    //!
    //! REF(s):
    //! - <https://github.com/python/cpython/blob/v3.12.0/Include/cpython/longintrepr.h>
    //! - <https://github.com/python/cpython/blob/v3.12.0/Include/internal/pycore_long.h>
    //! - <https://github.com/ijl/orjson/blob/master/src/ffi/pyintref.rs>
    #![expect(clippy::inline_always)]
    #![expect(
        non_camel_case_types,
        non_upper_case_globals,
        reason = "cpython names (`digit`, `PyLong_SHIFT`, ...)"
    )]
    // `_PyLong_*` consts only exist (+ get used) on 3.12+
    #![cfg_attr(
        Py_3_12,
        expect(
            clippy::used_underscore_items,
            reason = "cpython names (`_PyLong_SIGN_MASK`, ...)"
        )
    )]
    use pyo3::prelude::*;
    use pyo3::types::PyInt;

    use super::PyIntValue;

    type digit = u32;
    const PyLong_SHIFT: usize = 30;

    #[cfg(Py_3_12)]
    const _PyLong_SIGN_MASK: usize = 3;
    #[cfg(Py_3_12)]
    const _PyLong_NON_SIZE_BITS: usize = 3;
    #[cfg(Py_3_12)]
    const SIGN_NEGATIVE: usize = 2;

    #[cfg(Py_3_12)]
    #[repr(C)]
    struct _PyLongValue {
        lv_tag: usize,
        // really `digit ob_digit[ndigits]`
        ob_digit: [digit; 1],
    }

    #[cfg(Py_3_12)]
    #[repr(C)]
    struct PyLongObject {
        ob_base: pyo3::ffi::PyObject,
        long_value: _PyLongValue,
    }

    #[cfg(not(Py_3_12))]
    #[repr(C)]
    struct PyLongObject {
        ob_base: pyo3::ffi::PyVarObject,
        ob_digit: [digit; 1],
    }

    /// sign & digits (least significant first) of a cpython int
    struct CPythonLongParts<'a> {
        negative: bool,
        digits: &'a [digit],
    }

    impl<'a> CPythonLongParts<'a> {
        #[inline(always)]
        #[expect(unsafe_code)]
        fn new(ob: Borrowed<'a, '_, PyInt>) -> Self {
            let long = ob.as_ptr().cast::<PyLongObject>();
            // SAFETY: `ob` is a cpython int (or subclass) so the header is
            // a-ok and it has `ndigits`-digits
            unsafe {
                cfg_select! {
                    Py_3_12 => {
                        let tag = (*long).long_value.lv_tag;
                        let ptr = (&raw const (*long).long_value.ob_digit).cast::<digit>();
                        Self {
                            negative: tag & _PyLong_SIGN_MASK == SIGN_NEGATIVE,
                            digits: core::slice::from_raw_parts(ptr, tag >> _PyLong_NON_SIZE_BITS),
                        }
                    }
                    _ => {
                        let size = (*long).ob_base.ob_size;
                        let ptr = (&raw const (*long).ob_digit).cast::<digit>();
                        Self {
                            negative: size < 0,
                            digits: core::slice::from_raw_parts(ptr, size.unsigned_abs()),
                        }
                    }
                }
            }
        }
    }

    // inlined 4 serde
    #[inline(always)]
    pub(super) fn read_int(ob: Borrowed<'_, '_, PyInt>) -> Option<PyIntValue> {
        let CPythonLongParts { negative, digits } = CPythonLongParts::new(ob);
        let mag = match *digits {
            [] => 0,
            [d0] => u64::from(d0),
            [d0, d1] => u64::from(d0) | u64::from(d1) << PyLong_SHIFT,
            // 3 digits is 90 bits ~ top digit gets 4 bits for a u64
            [d0, d1, d2] if d2 < 1 << (64 - 2 * PyLong_SHIFT) => {
                u64::from(d0) | u64::from(d1) << PyLong_SHIFT | u64::from(d2) << (2 * PyLong_SHIFT)
            }
            _ => return None,
        };
        if negative {
            // NOTE: 2^63 wraps to i64::MIN
            (mag <= 1 << 63).then(|| PyIntValue::I64(mag.cast_signed().wrapping_neg()))
        } else if let Ok(v) = i64::try_from(mag) {
            Some(PyIntValue::I64(v))
        } else {
            Some(PyIntValue::U64(mag))
        }
    }

    /// (magnitude aka abs value, negative) for ints that fit in 128 bits
    #[inline]
    fn read_mag128(ob: Borrowed<'_, '_, PyInt>) -> Option<(u128, bool)> {
        let CPythonLongParts { negative, digits } = CPythonLongParts::new(ob);
        // 5 digits is 150 bits (assuming the 30 bit thing)
        //  the top digit gets 8 bits for a u128 (aka the most signif digit)
        if digits.len() > 5
            || digits
                .get(4)
                .is_some_and(|&top| top >= 1 << (128 - 4 * PyLong_SHIFT))
        {
            return None;
        }
        let mag = digits
            .iter()
            .rev()
            .fold(0, |mag, &d| mag << PyLong_SHIFT | u128::from(d));
        Some((mag, negative))
    }

    #[inline]
    pub(super) fn read_i128(ob: Borrowed<'_, '_, PyInt>) -> Option<i128> {
        let (mag, negative) = read_mag128(ob)?;
        if negative {
            // 2^127 wraps to i128::MIN
            (mag <= 1 << 127).then(|| mag.cast_signed().wrapping_neg())
        } else {
            i128::try_from(mag).ok()
        }
    }

    #[inline]
    pub(super) fn read_u128(ob: Borrowed<'_, '_, PyInt>) -> Option<u128> {
        let (mag, negative) = read_mag128(ob)?;
        (!negative).then_some(mag)
    }
}

#[cfg(not(ryo3_pylong_layout))]
mod dangerzone {
    use pyo3::ffi;
    use pyo3::prelude::*;
    use pyo3::types::PyInt;

    use super::PyIntValue;

    #[inline]
    #[expect(unsafe_code)]
    pub(super) fn read_int(ob: Borrowed<'_, '_, PyInt>) -> Option<PyIntValue> {
        let op = ob.as_ptr();
        // SAFETY: `ob` is a live int and is attached ('tis borrowed)
        unsafe {
            let mut overflow = 0;
            let v = ffi::PyLong_AsLongLongAndOverflow(op, &raw mut overflow);
            match overflow {
                0 => Some(PyIntValue::I64(v)),
                1 => {
                    // only > u64::MAX errors here (no non-erroring api for it)
                    let u = ffi::PyLong_AsUnsignedLongLong(op);
                    if u == u64::MAX && !ffi::PyErr_Occurred().is_null() {
                        ffi::PyErr_Clear();
                        None
                    } else {
                        Some(PyIntValue::U64(u))
                    }
                }
                _ => None,
            }
        }
    }

    #[inline]
    pub(super) fn read_i128(ob: Borrowed<'_, '_, PyInt>) -> Option<i128> {
        ob.extract().ok()
    }

    #[inline]
    pub(super) fn read_u128(ob: Borrowed<'_, '_, PyInt>) -> Option<u128> {
        ob.extract().ok()
    }
}
