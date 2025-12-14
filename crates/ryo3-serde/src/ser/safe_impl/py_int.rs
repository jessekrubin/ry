use pyo3::prelude::*;
use serde::ser::{Serialize, Serializer};

use crate::errors::pyerr2sererr;

use pyo3::Bound;
use pyo3::types::PyInt;

pub(crate) struct SerializePyInt<'a, 'py> {
    obj: Borrowed<'a, 'py, PyAny>,
}

impl<'a, 'py> SerializePyInt<'a, 'py> {
    #[inline]
    pub(crate) fn new(obj: Borrowed<'a, 'py, PyAny>) -> Self {
        Self { obj }
    }
}

impl Serialize for SerializePyInt<'_, '_> {
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
