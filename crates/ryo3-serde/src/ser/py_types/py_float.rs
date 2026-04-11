use pyo3::Borrowed;
use pyo3::prelude::*;
use pyo3::types::PyFloat;
use serde::ser::{Serialize, Serializer};

pub(crate) struct PyFloatSerializer<'a, 'py> {
    obj: Borrowed<'a, 'py, PyFloat>,
}

impl<'a, 'py> PyFloatSerializer<'a, 'py> {
    pub(crate) fn new(obj: Borrowed<'a, 'py, PyFloat>) -> Self {
        Self { obj }
    }

    pub(crate) fn new_unchecked(obj: Borrowed<'a, 'py, PyAny>) -> Self {
        #[expect(unsafe_code)]
        let obj = unsafe { obj.cast_unchecked::<PyFloat>() };
        Self::new(obj)
    }
}

#[cfg(not(any(PyPy, GraalPy, Py_LIMITED_API)))]
impl Serialize for PyFloatSerializer<'_, '_> {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_f64(self.obj.value())
    }
}

#[cfg(any(PyPy, GraalPy, Py_LIMITED_API))]
impl Serialize for PyFloatSerializer<'_, '_> {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use crate::errors::pyerr2sererr;
        let v: f64 = self.obj.extract().map_err(pyerr2sererr)?;
        serializer.serialize_f64(v)
    }
}
