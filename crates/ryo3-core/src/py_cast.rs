use pyo3::PyTypeInfo;
use pyo3::prelude::{Borrowed, Bound, PyAny, PyAnyMethods};

/// Extension methods for exact Python type checks w/o `::pyo3::CastError`
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
