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
