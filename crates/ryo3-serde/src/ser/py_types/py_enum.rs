use pyo3::prelude::*;
use pyo3::{Borrowed, PyAny, intern};
use serde::ser::{Serialize, Serializer};

use crate::errors::pyerr2sererr;
use crate::ser::{PySerializeContext, PySerializeTarget, SerdeTarget};
use crate::{Depth, PyAnySerializer};

pub(crate) struct PyEnumSerializer<'a, 'py, T = SerdeTarget>
where
    T: PySerializeTarget,
{
    ctx: PySerializeContext<'py, T>,
    obj: Borrowed<'a, 'py, PyAny>,
    depth: Depth,
}

impl<'a, 'py, T> PyEnumSerializer<'a, 'py, T>
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

impl<T> Serialize for PyEnumSerializer<'_, '_, T>
where
    T: PySerializeTarget,
{
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // TODO: use `getattr_opt` ?
        let value = self
            .obj
            .getattr(intern!(self.obj.py(), "value"))
            .map_err(pyerr2sererr)?;
        PyAnySerializer::new_with_depth(value.as_borrowed(), self.ctx, self.depth)
            .serialize(serializer)
    }
}
