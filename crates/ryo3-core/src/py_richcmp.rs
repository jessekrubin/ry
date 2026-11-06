//! `__richcmp__` return value
//!
//! returning this instead of `PyResult<bool>` lets `__richcmp__` hand back
//! `NotImplemented` for types it doesn't know w/o making a `TypeError` (and
//! the borrowed `True`/`False`/`NotImplemented` singletons cost nothing)
use pyo3::BoundObject;
use pyo3::prelude::*;
use pyo3::types::{PyBool, PyNotImplemented};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PyRichCmp {
    Bool(bool),
    NotImplemented,
}

impl From<bool> for PyRichCmp {
    #[inline]
    fn from(b: bool) -> Self {
        Self::Bool(b)
    }
}

impl<'py> IntoPyObject<'py> for PyRichCmp {
    type Target = PyAny;
    type Output = Borrowed<'py, 'py, PyAny>;
    type Error = std::convert::Infallible;

    #[inline]
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        match self {
            Self::Bool(b) => Ok(PyBool::new(py, b).into_any()),
            Self::NotImplemented => Ok(PyNotImplemented::get(py).into_any()),
        }
    }
}
