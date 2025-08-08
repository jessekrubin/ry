use pyo3::prelude::*;
use serde::ser::{Serialize, SerializeSeq, Serializer};

use crate::errors::pyerr2sererr;

use crate::SerializePyAny;
use crate::constants::Depth;
use crate::ser::PySerializeContext;
use pyo3::Bound;
use pyo3::types::PyTuple;

pub(crate) struct SerializePyTuple<'a, 'py> {
    pub(crate) obj: &'a Bound<'py, PyAny>,
    pub(crate) ctx: PySerializeContext<'py>,
    pub(crate) depth: Depth,
}

impl<'a, 'py> SerializePyTuple<'a, 'py> {
    pub(crate) fn new(obj: &'a Bound<'py, PyAny>, ctx: PySerializeContext<'py>) -> Self {
        Self {
            obj,
            ctx,
            depth: Depth::default(),
        }
    }
}

impl Serialize for SerializePyTuple<'_, '_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let py_list: &Bound<'_, PyTuple> = self.obj.downcast().map_err(pyerr2sererr)?;
        let len = py_list.len();
        if len == 0 {
            serializer.serialize_seq(Some(0))?.end()
        } else {
            let mut seq = serializer.serialize_seq(Some(len))?;
            for element in py_list {
                seq.serialize_element(&SerializePyAny::new_with_depth(
                    &element,
                    self.ctx,
                    self.depth + 1,
                ))?;
            }
            seq.end()
        }
    }
}
