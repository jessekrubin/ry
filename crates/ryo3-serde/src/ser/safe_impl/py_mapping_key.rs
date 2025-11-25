use pyo3::prelude::*;
use pyo3::types::PyBool;
use serde::ser::{Serialize, Serializer};

use crate::errors::pyerr2sererr;
use crate::serde_err;

use crate::any_repr::any_repr;
use pyo3::Bound;

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
        use pyo3::types::PyString;

        if let Ok(py_str) = self.obj.cast::<PyString>() {
            let s = py_str.to_str().map_err(pyerr2sererr)?;
            serializer.serialize_str(s)
        } else if let Ok(py_bool) = self.obj.cast::<PyBool>() {
            let b = py_bool.is_true();
            serializer.serialize_bool(b)
        } else {
            let key_repr = any_repr(self.obj);
            serde_err!("{} is not serializable as map-key", key_repr)
        }
    }
}
