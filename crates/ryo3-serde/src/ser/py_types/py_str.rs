use pyo3::prelude::*;
use pyo3::types::PyString;
use ryo3_core::pystr_read_fast_opt;
use serde::ser::{Serialize, Serializer};

use crate::errors::pyerr2sererr;

pub(crate) struct PyStrSerializer<'a, 'py> {
    obj: Borrowed<'a, 'py, PyString>,
}

impl<'a, 'py> PyStrSerializer<'a, 'py> {
    #[inline]
    pub(crate) fn new(obj: Borrowed<'a, 'py, PyString>) -> Self {
        Self { obj }
    }

    #[inline]
    pub(crate) fn new_unchecked(obj: Borrowed<'a, 'py, PyAny>) -> Self {
        #[expect(unsafe_code)]
        let obj = unsafe { obj.cast_unchecked::<PyString>() };
        Self::new(obj)
    }
}

impl Serialize for PyStrSerializer<'_, '_> {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        #[expect(unsafe_code)]
        let s = unsafe { pystr_read_fast_opt(self.obj) };
        if let Some(s) = s {
            serializer.serialize_str(s)
        } else {
            // error here...
            crate::serde_err!("invalid str object")
        }
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
