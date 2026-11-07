//! Return types for `__richcmp__` and binary ops that may return `NotImplemented`.
//!
//! Returning `NotImplemented` directly avoids pyo3's fallback of building and
//! discarding a `TypeError` when an argument fails to extract.
//!
//! Converts to an owned `Bound<PyAny>`: `CPython` takes a new reference either
//! way, so a `Borrowed` output would not save an incref.
use pyo3::BoundObject;
use pyo3::prelude::*;
use pyo3::types::PyNotImplemented;

/// `T` or `NotImplemented`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PyNotImplementedOr<T> {
    Value(T),
    NotImplemented,
}

/// `__richcmp__` return value: `True`, `False` or `NotImplemented`.
pub type PyRichCmp = PyNotImplementedOr<bool>;

impl<T> From<T> for PyNotImplementedOr<T> {
    #[inline]
    fn from(value: T) -> Self {
        Self::Value(value)
    }
}

impl<'py, T> IntoPyObject<'py> for PyNotImplementedOr<T>
where
    T: IntoPyObject<'py>,
{
    type Target = PyAny;
    type Output = Bound<'py, PyAny>;
    type Error = T::Error;

    #[inline]
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        match self {
            Self::Value(v) => Ok(v.into_pyobject(py)?.into_bound().into_any()),
            Self::NotImplemented => Ok(PyNotImplemented::get(py).to_owned().into_any()),
        }
    }
}
