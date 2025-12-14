use pyo3::prelude::*;
use serde::ser::{Serialize, Serializer};
use std::ptr;

use crate::errors::pyerr2sererr;
use crate::ser::traits::PySerializeUnsafe;
use pyo3::types::PyBool;
use pyo3::{Borrowed, Bound, ffi};

pub(crate) struct SerializePyBool<'a, 'py> {
    obj: Borrowed<'a, 'py, PyAny>,
}

impl<'a, 'py> SerializePyBool<'a, 'py> {
    #[inline]
    pub(crate) fn new(obj: Borrowed<'a, 'py, PyAny>) -> Self {
        Self { obj }
    }
}

impl Serialize for SerializePyBool<'_, '_> {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let tf = self
            .obj
            .cast_exact::<PyBool>()
            .map_err(pyerr2sererr)?
            .is_true();
        serializer.serialize_bool(tf)
    }
}

impl PySerializeUnsafe for SerializePyBool<'_, '_> {
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
