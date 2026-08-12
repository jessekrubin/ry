use pyo3::prelude::*;
use serde::ser::{Serialize, Serializer};

use crate::any_repr::any_repr;
use crate::ob_type::PyObType;
use crate::ser::py_types::{PyBoolSerializer, PyStrSerializer};
use crate::ser::{PySerializeContext, PySerializeTarget, SerdeTarget};
use crate::serde_err;

pub(crate) struct PyMappingKeySerializer<'a, 'py, T = SerdeTarget>
where
    T: PySerializeTarget,
{
    pub(crate) ctx: PySerializeContext<'py, T>,
    obj: Borrowed<'a, 'py, PyAny>,
}

impl<'a, 'py, T> PyMappingKeySerializer<'a, 'py, T>
where
    T: PySerializeTarget,
{
    #[inline]
    pub(crate) fn new(ctx: PySerializeContext<'py, T>, obj: Borrowed<'a, 'py, PyAny>) -> Self {
        Self { ctx, obj }
    }
}

impl<T> Serialize for PyMappingKeySerializer<'_, '_, T>
where
    T: PySerializeTarget,
{
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let obtype = self.ctx.typeref.obtype_key(self.obj);
        match obtype {
            PyObType::String => PyStrSerializer::new_unchecked(self.obj).serialize(serializer),
            PyObType::Bool => PyBoolSerializer::new_unchecked(self.obj).serialize(serializer),
            _ => {
                let key_repr = any_repr(self.obj);
                serde_err!("{} is not serializable as map-key", key_repr)
            }
        }
    }
}
