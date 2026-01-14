use pyo3::prelude::*;
use serde::ser::{Serialize, Serializer};

use crate::errors::pyerr2sererr;

use crate::ser::traits::PySerializeUnsafe;
use pyo3::Borrowed;

pub(crate) struct PyFloatSerializer<'a, 'py> {
    obj: Borrowed<'a, 'py, PyAny>,
}

impl<'a, 'py> PyFloatSerializer<'a, 'py> {
    pub(crate) fn new(obj: Borrowed<'a, 'py, PyAny>) -> Self {
        Self { obj }
    }
}

impl Serialize for PyFloatSerializer<'_, '_> {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let v: f64 = self.obj.extract().map_err(pyerr2sererr)?;
        serializer.serialize_f64(v)
    }
}

impl PySerializeUnsafe for PyFloatSerializer<'_, '_> {
    #[inline]
    fn serialize_unsafe<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use pyo3::ffi::PyFloat_AsDouble;

        #[expect(unsafe_code)]
        let f = unsafe { PyFloat_AsDouble(self.obj.as_ptr()) };
        serializer.serialize_f64(f)
    }
}
