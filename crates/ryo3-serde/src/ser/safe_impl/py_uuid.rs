use pyo3::prelude::*;
use serde::ser::{Serialize, Serializer};

use crate::errors::pyerr2sererr;

use pyo3::Bound;

pub(crate) struct SerializePyUuid<'a, 'py> {
    obj: &'a Bound<'py, PyAny>,
}

impl<'a, 'py> SerializePyUuid<'a, 'py> {
    pub(crate) fn new(obj: &'a Bound<'py, PyAny>) -> Self {
        Self { obj }
    }
}

impl Serialize for SerializePyUuid<'_, '_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let uu: uuid::Uuid = self.obj.extract().map_err(pyerr2sererr)?;
        serializer.serialize_str(&uu.hyphenated().to_string())
    }
}
