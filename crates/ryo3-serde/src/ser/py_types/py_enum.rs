use pyo3::prelude::*;
use pyo3::{Borrowed, PyAny, intern};
use serde::ser::{Serialize, Serializer};

use crate::errors::pyerr2sererr;
use crate::ser::PySerializeContext;
use crate::{Depth, PyAnySerializer};

pub(crate) struct PyEnumSerializer<'a, 'py> {
    ctx: PySerializeContext<'py>,
    obj: Borrowed<'a, 'py, PyAny>,
    depth: Depth,
}

impl<'a, 'py> PyEnumSerializer<'a, 'py> {
    #[inline]
    pub(crate) fn new(
        obj: Borrowed<'a, 'py, PyAny>,
        ctx: PySerializeContext<'py>,
        depth: Depth,
    ) -> Self {
        Self { ctx, obj, depth }
    }
}

impl Serialize for PyEnumSerializer<'_, '_> {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let value = self
            .obj
            .getattr(intern!(self.obj.py(), "value"))
            .map_err(pyerr2sererr)?;
        PyAnySerializer::new_with_depth(value.as_borrowed(), self.ctx, self.depth)
            .serialize(serializer)
    }
}
