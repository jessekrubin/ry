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

use pyo3::prelude::*;

use crate::PyNotImplementedOr;

/// A type a binary op accepts as its other operand.
pub trait FromPyOperand: Sized {
    /// `None` if `ob` is not a supported operand (the op returns `NotImplemented`).
    ///
    /// Rejecting unsupported types should be cheap: check types before
    /// extracting and do not build errors.
    fn from_py_operand(ob: Borrowed<'_, '_, PyAny>) -> Option<Self>;
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

impl<'py, T> FromPyObject<'_, 'py> for PyOperandArg<T>
where
    T: FromPyOperand,
{
    type Error = Infallible;

    #[inline]
    fn extract(ob: Borrowed<'_, 'py, PyAny>) -> Result<Self, Self::Error> {
        Ok(Self(T::from_py_operand(ob)))
    }
}
