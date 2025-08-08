use pyo3::prelude::*;
use serde::ser::{Serialize, Serializer};
use std::mem::offset_of;

use crate::errors::pyerr2sererr;

use crate::ser::traits::PySerializeUnsafe;
use pyo3::Bound;

pub(crate) struct SerializePyFloat<'a, 'py> {
    obj: &'a Bound<'py, PyAny>,
}

impl<'a, 'py> SerializePyFloat<'a, 'py> {
    pub(crate) fn new(obj: &'a Bound<'py, PyAny>) -> Self {
        Self { obj }
    }
}

impl Serialize for SerializePyFloat<'_, '_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let v: f64 = self.obj.extract().map_err(pyerr2sererr)?;
        serializer.serialize_f64(v)
    }
}

impl PySerializeUnsafe for SerializePyFloat<'_, '_> {
    fn serialize_unsafe<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        unsafe {}
    }
}
