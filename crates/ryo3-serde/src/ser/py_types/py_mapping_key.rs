use pyo3::prelude::*;
use serde::ser::{Serialize, Serializer};

use crate::ob_type::PyObType;
use crate::ser::PySerializeContext;
use crate::ser::py_types::{PyBoolSerializer, PyStrSerializer};
use crate::serde_err;

use crate::any_repr::any_repr;

pub(crate) struct PyMappingKeySerializer<'a, 'py> {
    pub(crate) ctx: PySerializeContext<'py>,
    obj: Borrowed<'a, 'py, PyAny>,
}

impl<'a, 'py> PyMappingKeySerializer<'a, 'py> {
    #[inline]
    pub(crate) fn new(ctx: PySerializeContext<'py>, obj: Borrowed<'a, 'py, PyAny>) -> Self {
        Self { ctx, obj }
    }
}

impl Serialize for PyMappingKeySerializer<'_, '_> {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let obtype = self.ctx.typeref.obtype_key(self.obj);
        match obtype {
            PyObType::String => PyStrSerializer::new(self.obj).serialize(serializer),
            PyObType::Bool => PyBoolSerializer::new(self.obj).serialize(serializer),
            _ => {
                let key_repr = any_repr(self.obj);
                serde_err!("{} is not serializable as map-key", key_repr)
            }
        }
    }
}
