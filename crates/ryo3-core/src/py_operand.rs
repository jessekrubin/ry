//! Binary-op arguments that never fail to extract.
//!
//! When a binary-op argument fails to extract, pyo3 returns `NotImplemented`
//! only after building and annotating a `TypeError` it then discards.
//! [`PyOperandArg<T>`] cannot fail: it holds `Some(T)` for supported operands
//! and `None` otherwise, and [`PyOperandArg::map`] / [`PyOperandArg::try_map`]
//! turn that into a [`PyNotImplementedOr`] return value.
//!
//! ```ignore
//! fn __add__(&self, other: PyOperandArg<Operand>) -> PyResult<PyNotImplementedOr<Self>> {
//!     other.try_map(|rhs| self.try_add(rhs))
//! }
//! ```
use std::convert::Infallible;
use std::time::Duration;

use pyo3::PyTypeInfo;
use pyo3::prelude::*;
use pyo3::types::{PyDelta, PyFloat};

use crate::{PyNotImplementedOr, pyindex_check};

/// A type a binary op accepts as its other operand.
///
/// NOTE: this is used really as a cheap `Option` extractor, so it can be used
/// as the backend for `FromPyObject` impls that just add an error message;
/// `Self::from_py_operand(ob).ok_or_else(..)`
///
/// Need them lifetimes (`'a` & `'py`) for `Borrowed<'a, 'py, T>`
pub trait FromPyOperand<'a, 'py>: Sized {
    /// `None` if `ob` is not a supported operand (the op returns `NotImplemented`).
    ///
    /// Rejecting unsupported types should be cheap: check types before
    /// extracting and do not build errors.
    fn from_py_operand(ob: Borrowed<'a, 'py, PyAny>) -> Option<Self>;
}

/// Binary-op argument that never fails to extract (scroll up).
pub struct PyOperandArg<T>(Option<T>);

impl<T> PyOperandArg<T> {
    #[inline]
    pub fn into_option(self) -> Option<T> {
        self.0
    }

    /// `f(operand)`, or `NotImplemented` if unsupported.
    #[inline]
    pub fn map<U>(self, f: impl FnOnce(T) -> U) -> PyNotImplementedOr<U> {
        match self.0 {
            Some(v) => PyNotImplementedOr::Value(f(v)),
            None => PyNotImplementedOr::NotImplemented,
        }
    }

    /// `f(operand)?`, or `NotImplemented` if unsupported.
    #[inline]
    pub fn try_map<U, E>(
        self,
        f: impl FnOnce(T) -> Result<U, E>,
    ) -> Result<PyNotImplementedOr<U>, E> {
        match self.0 {
            Some(v) => f(v).map(PyNotImplementedOr::Value),
            None => Ok(PyNotImplementedOr::NotImplemented),
        }
    }
}

impl<'a, 'py, T> FromPyObject<'a, 'py> for PyOperandArg<T>
where
    T: FromPyOperand<'a, 'py>,
{
    type Error = Infallible;

    #[inline]
    fn extract(ob: Borrowed<'a, 'py, PyAny>) -> Result<Self, Self::Error> {
        Ok(Self(T::from_py_operand(ob)))
    }
}

/// An instance of exactly `T` (no subclasses).
impl<'a, 'py, T> FromPyOperand<'a, 'py> for Borrowed<'a, 'py, T>
where
    T: PyTypeInfo + 'a,
{
    #[inline]
    fn from_py_operand(ob: Borrowed<'a, 'py, PyAny>) -> Option<Self> {
        use crate::PyCastExactOpt;
        ob.cast_exact_opt::<T>()
    }
}

/// Implements `FromPyOperand` for py-int-like objects (`int` or has `__index__`)
macro_rules! impl_int_operand {
    ($($ty:ty),*) => {$(
        impl FromPyOperand<'_, '_> for $ty {
            #[inline]
            fn from_py_operand(ob: Borrowed<'_, '_, PyAny>) -> Option<Self> {
                if pyindex_check(ob) {
                    ob.extract().ok()
                } else {
                    None
                }
            }
        }
    )*};
}

// just need i32, u32, i64 and i128 (2026-11-07)
impl_int_operand!(i32, i64, i128, u32);

/// Exact `float` (ints != floats)
///
/// NOTE: might have to chain an `int` operand first
impl FromPyOperand<'_, '_> for f64 {
    #[inline]
    fn from_py_operand(ob: Borrowed<'_, '_, PyAny>) -> Option<Self> {
        use crate::PyCastExactOpt;
        ob.cast_exact_opt::<PyFloat>().map(|f| f.value())
    }
}

/// Exact `datetime.timedelta` (no subclasses ~ they're dumb)
impl FromPyOperand<'_, '_> for Duration {
    #[inline]
    fn from_py_operand(ob: Borrowed<'_, '_, PyAny>) -> Option<Self> {
        use crate::PyCastExactOpt;
        ob.cast_exact_opt::<PyDelta>()?.extract().ok()
    }
}

/// Chain-able operand extract ~ very sexy fugue-state-jesse work from 2am
///
/// Each variant/fn's arg type is the operand type tried, in order, first wins;
/// this example is taken from the duration impl for multiplication:
///
/// ```ignore
/// use pyo3::prelude::*;
/// use ryo3_core::{FromPyOperand, PyNotImplementedOr, PyOperandArg, PyOperandExt};
///
/// enum MulInput {
///     Int(u32),   // `int`
///     Float(f64), // `float`
/// }
///
/// impl FromPyOperand<'_, '_> for MulInput {
///     fn from_py_operand(obj: Borrowed<'_, '_, PyAny>) -> Option<Self> {
///         // order matters; int first so `2` stays an int
///         obj.operand(Self::Int)
///             .or(Self::Float)
///             .ok() // finalize ~ Option<Self>
///     }
/// }
///
/// // and in pymethods can be used like this
/// fn __mul__(&self, other: PyOperandArg<MulInput>) -> PyResult<PyNotImplementedOr<Self>> {
///     other.try_map(|rhs| self.try_mul(rhs)) // unsupported -> NotImplemented
/// }
/// ```
///
/// Or start empty w/ `operand_match()` so every variant is an `.or(...)` line
/// (easier to shuffle around):
///
/// ```ignore
/// obj.operand_match()
///     .or(Self::Int)
///     .or(Self::Float)
///     .ok()
/// ```
pub trait PyOperandExt<'a, 'py>: Sized {
    /// Empty match ~ nothing tried yet.
    fn operand_match<O>(self) -> PyOperandMatch<'a, 'py, O>;

    /// Match starting with `T` (same as `operand_match().or(f)`).
    #[inline]
    fn operand<T, O>(self, f: impl FnOnce(T) -> O) -> PyOperandMatch<'a, 'py, O>
    where
        T: FromPyOperand<'a, 'py>,
    {
        self.operand_match().or(f)
    }
}

impl<'a, 'py> PyOperandExt<'a, 'py> for Borrowed<'a, 'py, PyAny> {
    #[inline]
    fn operand_match<O>(self) -> PyOperandMatch<'a, 'py, O> {
        PyOperandMatch {
            ob: self,
            out: None,
        }
    }
}

/// Builder from [`PyOperandExt::operand`] / [`PyOperandExt::operand_match`].
#[must_use]
pub struct PyOperandMatch<'a, 'py, O> {
    ob: Borrowed<'a, 'py, PyAny>,
    out: Option<O>,
}

impl<'a, 'py, O> PyOperandMatch<'a, 'py, O> {
    /// Try `T` next (skipped once something matched).
    #[inline]
    pub fn or<T: FromPyOperand<'a, 'py>>(mut self, f: impl FnOnce(T) -> O) -> Self {
        if self.out.is_none() {
            self.out = T::from_py_operand(self.ob).map(f);
        }
        self
    }

    /// Finalize the match (if any)
    #[inline]
    pub fn ok(self) -> Option<O> {
        self.out
    }
}
