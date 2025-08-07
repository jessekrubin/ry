use crate::errors::pyerr2sererr;
use crate::ser::PySerializeContext;
use crate::{Depth, SerializePyAny};
use pyo3::Bound;
use pyo3::prelude::*;
use pyo3::types::PySequence;
use serde::ser::SerializeSeq;
use serde::ser::{Serialize, Serializer};

pub(crate) struct SerializePySequence<'a, 'py> {
    ctx: PySerializeContext<'py>,
    obj: &'a Bound<'py, PySequence>,
    depth: Depth,
}

impl<'a, 'py> SerializePySequence<'a, 'py> {
    pub(crate) fn new_with_depth(
        obj: &'a Bound<'py, PySequence>,
        ctx: PySerializeContext<'py>,
        depth: Depth,
    ) -> Self {
        Self { ctx, obj, depth }
    }
}

impl Serialize for SerializePySequence<'_, '_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let len = self.obj.len().map_err(pyerr2sererr)?;
        let mut seq = serializer.serialize_seq(Some(len))?;
        for i in 0..len {
            let item = self.obj.get_item(i).map_err(pyerr2sererr)?;
            let item_ser = SerializePyAny::new_with_depth(&item, self.ctx, self.depth + 1);
            seq.serialize_element(&item_ser)?;
        }
        seq.end()
    }
}
