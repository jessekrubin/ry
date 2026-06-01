//! `PyO3` casting extension methods.
//!
//! `PyO3`'s casting methods return `Result<T, pyo3::CastError>`, whereas these
//! extension traits return `Option<T>` to avoid constructing a [`pyo3::CastError`].
use pyo3::prelude::{Borrowed, Bound, PyAny, PyAnyMethods};
use pyo3::{PyTypeCheck, PyTypeInfo};

pub trait PyCastOpt<'a> {
    type Output<T: PyTypeCheck + 'a>;

    /// Casts to `T` if the object's Python type is `T` or a subtype of `T`.
    ///
    /// Unlike `PyO3`'s `cast`, this returns `None` on a mismatch without constructing
    /// a [`pyo3::CastError`].
    fn cast_opt<T>(self) -> Option<Self::Output<T>>
    where
        T: PyTypeCheck + 'a;
}

impl<'a, 'py> PyCastOpt<'a> for &'a Bound<'py, PyAny> {
    type Output<T: PyTypeCheck + 'a> = &'a Bound<'py, T>;

    #[inline]
    fn cast_opt<T>(self) -> Option<Self::Output<T>>
    where
        T: PyTypeCheck + 'a,
    {
        if self.is_instance_of::<T>() {
            // SAFETY: the Python type was checked immediately above.
            #[expect(unsafe_code, reason = "wenodis")]
            unsafe {
                Some(self.cast_unchecked())
            }
        } else {
            None
        }
    }
}

impl<'a, 'py> PyCastOpt<'a> for Borrowed<'a, 'py, PyAny> {
    type Output<T: PyTypeCheck + 'a> = Borrowed<'a, 'py, T>;

    #[inline]
    fn cast_opt<T>(self) -> Option<Self::Output<T>>
    where
        T: PyTypeCheck + 'a,
    {
        if self.is_instance_of::<T>() {
            // SAFETY: the Python type was checked immediately above.
            #[expect(unsafe_code, reason = "wenodis")]
            unsafe {
                Some(self.cast_unchecked())
            }
        } else {
            None
        }
    }
}

/// Extension methods for exact Python type checks which do not need a diagnostic error.
///
/// Use [`pyo3::Bound::cast_exact`] or [`pyo3::Borrowed::cast_exact`] instead
/// when the [`pyo3::CastError`] will be returned to the caller.
pub trait PyCastExactOpt<'a> {
    type Output<T: PyTypeInfo + 'a>;

    /// Casts to `T` if the object's Python type is exactly `T`.
    ///
    /// `PyO3`'s `cast_exact` returns a `Result<Self::Output<T>, ::pyo3::CastError>`,
    /// where this method returns `Option<Self::Output<T>>` and simply returns
    /// `None` if the type check faails saving the overhead of constructing
    /// a `CastError` (ref-count-bumping)
    fn cast_exact_opt<T>(self) -> Option<Self::Output<T>>
    where
        T: PyTypeInfo + 'a;
}

impl<'a, 'py> PyCastExactOpt<'a> for &'a Bound<'py, PyAny> {
    type Output<T: PyTypeInfo + 'a> = &'a Bound<'py, T>;

    #[inline]
    fn cast_exact_opt<T>(self) -> Option<Self::Output<T>>
    where
        T: PyTypeInfo + 'a,
    {
        if self.is_exact_instance_of::<T>() {
            // SAFETY: the exact Python type was checked immediately above.
            #[expect(unsafe_code, reason = "wenodis")]
            unsafe {
                Some(self.cast_unchecked())
            }
        } else {
            None
        }
    }
}

impl<'a, 'py> PyCastExactOpt<'a> for Borrowed<'a, 'py, PyAny> {
    type Output<T: PyTypeInfo + 'a> = Borrowed<'a, 'py, T>;

    #[inline]
    fn cast_exact_opt<T>(self) -> Option<Self::Output<T>>
    where
        T: PyTypeInfo + 'a,
    {
        if self.is_exact_instance_of::<T>() {
            // SAFETY: the exact Python type was checked immediately above.
            #[expect(unsafe_code, reason = "wenodis")]
            unsafe {
                Some(self.cast_unchecked())
            }
        } else {
            None
        }
    }
}
