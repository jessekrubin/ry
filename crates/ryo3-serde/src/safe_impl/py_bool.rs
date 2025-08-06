use pyo3::prelude::*;
use serde::ser::{Serialize, Serializer};

use crate::errors::pyerr2sererr;

use pyo3::Bound;
use pyo3::types::PyBool;

pub(crate) struct SerializePyBool<'a, 'py> {
    obj: &'a Bound<'py, PyAny>,
}

impl<'a, 'py> SerializePyBool<'a, 'py> {
    pub(crate) fn new(obj: &'a Bound<'py, PyAny>) -> Self {
        Self { obj }
    }
}

impl Serialize for SerializePyBool<'_, '_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let tf = self
            .obj
            .downcast::<PyBool>()
            .map(pyo3::types::PyBoolMethods::is_true)
            .map_err(pyerr2sererr)?;
        serializer.serialize_bool(tf)
    }
}
