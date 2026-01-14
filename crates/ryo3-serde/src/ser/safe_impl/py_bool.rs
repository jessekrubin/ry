use pyo3::prelude::*;
use serde::ser::{Serialize, Serializer};
use std::ptr;

use crate::ser::traits::PySerializeUnsafe;
use pyo3::{Borrowed, ffi};

pub(crate) struct PyBoolSerializer<'a, 'py> {
    obj: Borrowed<'a, 'py, PyAny>,
}

impl<'a, 'py> PyBoolSerializer<'a, 'py> {
    #[inline]
    pub(crate) fn new(obj: Borrowed<'a, 'py, PyAny>) -> Self {
        Self { obj }
    }
}

#[cfg(not(any(PyPy, GraalPy, Py_LIMITED_API)))]
impl Serialize for PyBoolSerializer<'_, '_> {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        #[expect(unsafe_code)]
        unsafe {
            let istrue = ptr::eq(self.obj.as_ptr(), ffi::Py_True());
            serializer.serialize_bool(istrue)
        }
    }
}

#[cfg(any(PyPy, GraalPy, Py_LIMITED_API))]
impl Serialize for PyBoolSerializer<'_, '_> {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let tf = self
            .obj
            .cast_exact::<pyo3::types::PyBool>()
            .map_err(crate::errors::pyerr2sererr)?
            .is_true();
        serializer.serialize_bool(tf)
    }
}

impl PySerializeUnsafe for PyBoolSerializer<'_, '_> {
    fn serialize_unsafe<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        #[expect(unsafe_code)]
        unsafe {
            let istrue = ptr::eq(self.obj.as_ptr(), ffi::Py_True());
            serializer.serialize_bool(istrue)
        }
    }
}
