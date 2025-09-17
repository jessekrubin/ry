use pyo3::prelude::*;
use serde::ser::{Serialize, Serializer};

use crate::errors::pyerr2sererr;
use pyo3::Bound;
use pyo3::types::PyString;

pub(crate) struct SerializePyStr<'a, 'py> {
    obj: &'a Bound<'py, PyAny>,
}

impl<'a, 'py> SerializePyStr<'a, 'py> {
    pub(crate) fn new(obj: &'a Bound<'py, PyAny>) -> Self {
        Self { obj }
    }
}

impl Serialize for SerializePyStr<'_, '_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let py_str: &Bound<'_, PyString> = self.obj.cast().map_err(pyerr2sererr)?;
        let s = py_str.to_str().map_err(pyerr2sererr)?;
        serializer.serialize_str(s)
    }
}
