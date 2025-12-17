use crate::errors::pyerr2sererr;
use crate::ser::PySerializeContext;
use crate::{Depth, PyAnySerializer};
use pyo3::Bound;
use pyo3::prelude::*;
use pyo3::types::PySequence;
use serde::ser::SerializeSeq;
use serde::ser::{Serialize, Serializer};

pub(crate) struct PySequenceSerializer<'a, 'py> {
    ctx: PySerializeContext<'py>,
    obj: &'a Bound<'py, PySequence>,
    depth: Depth,
}

impl<'a, 'py> PySequenceSerializer<'a, 'py> {
    pub(crate) fn new_with_depth(
        obj: &'a Bound<'py, PySequence>,
        ctx: PySerializeContext<'py>,
        depth: Depth,
    ) -> Self {
        Self { ctx, obj, depth }
    }
}

impl Serialize for PySequenceSerializer<'_, '_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let len = self.obj.len().map_err(pyerr2sererr)?;
        let mut seq = serializer.serialize_seq(Some(len))?;
        for i in 0..len {
            let item = self.obj.get_item(i).map_err(pyerr2sererr)?;
            let item_ser = PyAnySerializer::new_with_depth(&item, self.ctx, self.depth + 1);
            seq.serialize_element(&item_ser)?;
        }
        seq.end()
    }
}
