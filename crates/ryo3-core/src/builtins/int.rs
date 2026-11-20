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
        crate::ffi::pylong::read_int(*self)
    }

    #[inline]
    fn read_i128(&self) -> Option<i128> {
        crate::ffi::pylong::read_i128(*self)
    }

    #[inline]
    fn read_u128(&self) -> Option<u128> {
        crate::ffi::pylong::read_u128(*self)
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
