use pyo3::prelude::*;
use serde::ser::{Serialize, Serializer};

use crate::errors::pyerr2sererr;

use pyo3::types::PyInt;

pub(crate) struct PyIntSerializer<'a, 'py> {
    obj: Borrowed<'a, 'py, PyAny>,
}

impl<'a, 'py> PyIntSerializer<'a, 'py> {
    #[inline]
    pub(crate) fn new(obj: Borrowed<'a, 'py, PyAny>) -> Self {
        Self { obj }
    }
}

impl Serialize for PyIntSerializer<'_, '_> {
    #[inline(always)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let v: i64 = self
            .obj
            .cast_exact::<PyInt>()
            .map_err(pyerr2sererr)?
            .extract()
            .map_err(pyerr2sererr)?;
        serializer.serialize_i64(v)
    }
}
