use pyo3::prelude::*;
use serde::ser::{Serialize, Serializer};

use crate::errors::pyerr2sererr;
use crate::serde_err;

use crate::any_repr::any_repr;
use pyo3::Bound;
use pyo3::types::PyString;

pub(crate) struct SerializePyMappingKey<'a, 'py> {
    obj: &'a Bound<'py, PyAny>,
}

impl<'a, 'py> SerializePyMappingKey<'a, 'py> {
    pub(crate) fn new(obj: &'a Bound<'py, PyAny>) -> Self {
        Self { obj }
    }
}

impl Serialize for SerializePyMappingKey<'_, '_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if let Ok(py_string) = self.obj.cast::<PyString>() {
            let v = py_string.to_str().map_err(pyerr2sererr)?;
            serializer.serialize_str(v)
        } else if let Ok(key) = self.obj.extract::<bool>() {
            // Ok(if key { "true" } else { "false" })
            serializer.serialize_bool(key)
        } else {
            let key_repr = any_repr(self.obj);
            serde_err!("{} is not JSON-serializable as map-key", key_repr)
        }
    }
}
