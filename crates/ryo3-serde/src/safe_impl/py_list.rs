use pyo3::prelude::*;
use serde::ser::{Serialize, SerializeSeq, Serializer};

use crate::SerializePyAny;
use crate::constants::Depth;
use crate::errors::pyerr2sererr;
use crate::safe_impl::with_obj::ObjTypeRef;
use crate::ser::PySerializeContext;
use crate::type_cache::PyTypeCache;
use pyo3::Bound;
use pyo3::types::PyList;

pub(crate) struct SerializePyList<'a, 'py> {
    pub(crate) ctx: PySerializeContext<'py>,
    pub(crate) obj: &'a Bound<'py, PyAny>,
    pub(crate) depth: Depth,
    // default: Option<&'py Bound<'py, PyAny>>,
    // ob_type_lookup: &'py PyTypeCache,
}

impl<'py> ObjTypeRef<'py> for SerializePyList<'_, 'py> {
    fn type_ref(&self) -> &'py PyTypeCache {
        self.ctx.typeref
    }
}

impl<'a, 'py> SerializePyList<'a, 'py> {
    pub(crate) fn new(obj: &'a Bound<'py, PyAny>, ctx: PySerializeContext<'py>) -> Self {
        Self {
            ctx,
            obj,
            depth: Depth::default(),
        }
    }
}

impl Serialize for SerializePyList<'_, '_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let py_list: &Bound<'_, PyList> = self.obj.downcast().map_err(pyerr2sererr)?;
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
