use pyo3::prelude::*;
use pyo3::types::PyInt;
use serde::ser::{Serialize, Serializer};

use crate::errors::pyerr2sererr;

pub(crate) struct PyIntSerializer<'a, 'py> {
    obj: Borrowed<'a, 'py, PyInt>,
}

impl<'a, 'py> PyIntSerializer<'a, 'py> {
    #[inline]
    pub(crate) fn new(obj: Borrowed<'a, 'py, PyInt>) -> Self {
        Self { obj }
    }

    #[inline]
    pub(crate) fn new_unchecked(obj: Borrowed<'a, 'py, PyAny>) -> Self {
        #[expect(unsafe_code)]
        let obj = unsafe { obj.cast_unchecked::<PyInt>() };
        Self::new(obj)
    }
}

impl<'a, 'py> TryFrom<Borrowed<'a, 'py, PyAny>> for PyIntSerializer<'a, 'py> {
    type Error = pyo3::CastError<'a, 'py>;

    #[inline]
    fn try_from(value: Borrowed<'a, 'py, PyAny>) -> Result<Self, Self::Error> {
        let obj = value.cast_exact::<PyInt>()?;
        Ok(Self::new(obj))
    }
}

impl Serialize for PyIntSerializer<'_, '_> {
    #[inline(always)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let v: i64 = self.obj.extract().map_err(pyerr2sererr)?;
        serializer.serialize_i64(v)
    }
}
