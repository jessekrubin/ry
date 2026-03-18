use pyo3::Borrowed;
use pyo3::prelude::*;
use serde::ser::{Serialize, Serializer};

pub(crate) struct PyFloatSerializer<'a, 'py> {
    obj: Borrowed<'a, 'py, PyAny>,
}

impl<'a, 'py> PyFloatSerializer<'a, 'py> {
    pub(crate) fn new(obj: Borrowed<'a, 'py, PyAny>) -> Self {
        Self { obj }
    }
}

#[cfg(not(any(PyPy, GraalPy, Py_LIMITED_API)))]
impl Serialize for PyFloatSerializer<'_, '_> {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // WENODIS: this is fo sho a float. checked by the caller
        #[expect(unsafe_code)]
        let f = unsafe { self.obj.cast_unchecked::<pyo3::types::PyFloat>() }.value();
        serializer.serialize_f64(f)
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
