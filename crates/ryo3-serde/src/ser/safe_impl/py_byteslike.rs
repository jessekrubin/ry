use pyo3::prelude::*;
use serde::ser::{Serialize, Serializer};

use crate::errors::pyerr2sererr;

use pyo3::Bound;

pub(crate) struct SerializePyBytesLike<'a, 'py> {
    obj: &'a Bound<'py, PyAny>,
}

impl<'a, 'py> SerializePyBytesLike<'a, 'py> {
    pub(crate) fn new(obj: &'a Bound<'py, PyAny>) -> Self {
        Self { obj }
    }
}

impl Serialize for SerializePyBytesLike<'_, '_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self.obj.extract::<&[u8]>() {
            Ok(v) => v.serialize(serializer),
            Err(e) => Err(pyerr2sererr(e)),
        }
    }
}
