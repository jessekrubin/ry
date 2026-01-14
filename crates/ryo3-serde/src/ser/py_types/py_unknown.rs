use pyo3::{
    prelude::*,
    types::{PyMapping, PySequence, PyString},
};
use serde::ser::{Serialize, Serializer};

use crate::{
    Depth, PyAnySerializer,
    any_repr::any_repr,
    errors::pyerr2sererr,
    ser::{
        PySerializeContext,
        dataclass::is_dataclass,
        py_types::{
            PyDataclassSerializer, PyMappingSerializer, PySequenceSerializer,
            PyStrSubclassSerializer,
        },
    },
    serde_err,
};

pub(crate) struct PyUnknownSerializer<'a, 'py> {
    pub(crate) ctx: PySerializeContext<'py>,
    obj: Borrowed<'a, 'py, PyAny>,
    pub(crate) depth: Depth,
}

impl<'a, 'py> PyUnknownSerializer<'a, 'py> {
    #[inline]
    pub(crate) fn new(
        obj: Borrowed<'a, 'py, PyAny>,
        ctx: PySerializeContext<'py>,
        depth: Depth,
    ) -> Self {
        Self { ctx, obj, depth }
    }
}

impl Serialize for PyUnknownSerializer<'_, '_> {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if let Ok(pystr_subclass) = self.obj.cast::<PyString>() {
            PyStrSubclassSerializer::new(pystr_subclass).serialize(serializer)
        } else if is_dataclass(self.obj) {
            PyDataclassSerializer::new(self.obj, self.ctx, self.depth).serialize(serializer)
        } else if let Ok(py_map) = self.obj.cast::<PyMapping>() {
            PyMappingSerializer::new_with_depth(py_map, self.ctx, self.depth).serialize(serializer)
        } else if let Ok(py_seq) = self.obj.cast::<PySequence>() {
            PySequenceSerializer::new_with_depth(py_seq, self.ctx, self.depth).serialize(serializer)
        } else if let Some(default) = self.ctx.default {
            // call the default transformer fn and attempt to then serialize the result
            let r = default.call1((&self.obj,)).map_err(pyerr2sererr)?;
            let any_serializer =
                PyAnySerializer::new_with_depth(r.as_borrowed(), self.ctx, self.depth);
            any_serializer.serialize(serializer)
        } else {
            serde_err!("{} is not json-serializable", any_repr(self.obj))
        }
    }
}
