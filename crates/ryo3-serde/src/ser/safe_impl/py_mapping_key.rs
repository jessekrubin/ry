use pyo3::prelude::*;
use serde::ser::{Serialize, Serializer};

use crate::ob_type::PyObType;
use crate::ser::PySerializeContext;
use crate::ser::safe_impl::{SerializePyBool, SerializePyStr};
use crate::serde_err;

use crate::any_repr::any_repr;
use pyo3::Bound;

pub(crate) struct SerializePyMappingKey<'a, 'py> {
    pub(crate) ctx: PySerializeContext<'py>,
    obj: &'a Bound<'py, PyAny>,
}

impl<'a, 'py> SerializePyMappingKey<'a, 'py> {
    #[inline]
    pub(crate) fn new(ctx: PySerializeContext<'py>, obj: &'a Bound<'py, PyAny>) -> Self {
        Self { ctx, obj }
    }
}

impl Serialize for SerializePyMappingKey<'_, '_> {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let obtype = self.ctx.typeref.obtype_key(self.obj);
        match obtype {
            PyObType::Bool => SerializePyBool::new(self.obj).serialize(serializer),
            PyObType::String => SerializePyStr::new(self.obj).serialize(serializer),
            _ => {
                let key_repr = any_repr(self.obj);
                serde_err!("{} is not serializable as map-key", key_repr)
            }
        }
    }
}
