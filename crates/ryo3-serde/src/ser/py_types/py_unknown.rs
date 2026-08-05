use pyo3::prelude::*;
use pyo3::types::{PyMapping, PySequence, PyString};
use serde::ser::{Serialize, Serializer};

use crate::any_repr::any_repr;
use crate::errors::pyerr2sererr;
use crate::ser::dataclass::is_dataclass;
use crate::ser::numpy::PyNumpySerializer;
use crate::ser::py_types::{
    PyDataclassSerializer, PyEnumSerializer, PyMappingSerializer, PySequenceSerializer,
    PyStrSubclassSerializer,
};
use crate::ser::{PySerializeContext, PySerializeTarget, SerdeTarget};
use crate::{Depth, PyAnySerializer, serde_err};

pub(crate) struct PyUnknownSerializer<'a, 'py, T = SerdeTarget>
where
    T: PySerializeTarget,
{
    pub(crate) ctx: PySerializeContext<'py, T>,
    obj: Borrowed<'a, 'py, PyAny>,
    pub(crate) depth: Depth,
}

impl<'a, 'py, T> PyUnknownSerializer<'a, 'py, T>
where
    T: PySerializeTarget,
{
    #[inline]
    pub(crate) fn new(
        obj: Borrowed<'a, 'py, PyAny>,
        ctx: PySerializeContext<'py, T>,
        depth: Depth,
    ) -> Self {
        Self { ctx, obj, depth }
    }
}

impl<T> Serialize for PyUnknownSerializer<'_, '_, T>
where
    T: PySerializeTarget,
{
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if self.ctx.numpy {
            let numpy_types =
                ryo3_numpy::NumpyTypeCache::cached(self.obj.py()).map_err(pyerr2sererr)?;
            if let Some(numpy_type) = numpy_types.obtype(self.obj.get_type_ptr() as usize) {
                return PyNumpySerializer::new(self.obj, numpy_type, self.depth)
                    .serialize(serializer);
            }
        }

        if let Ok(pystr_subclass) = self.obj.cast::<PyString>() {
            PyStrSubclassSerializer::new(pystr_subclass).serialize(serializer)
        } else if self.ctx.typeref.is_enum(self.obj) {
            PyEnumSerializer::new(self.obj, self.ctx, self.depth).serialize(serializer)
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
