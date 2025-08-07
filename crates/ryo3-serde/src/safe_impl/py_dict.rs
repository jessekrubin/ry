use pyo3::prelude::*;
use serde::ser::{Serialize, SerializeMap, Serializer};

use crate::errors::pyerr2sererr;

use crate::SerializePyAny;
use crate::constants::Depth;
use crate::safe_impl::py_mapping_key::SerializePyMappingKey;
use crate::safe_impl::with_obj::ObjTypeRef;
use crate::ser::PySerializeContext;
use crate::type_cache::PyTypeCache;
use pyo3::Bound;
use pyo3::types::PyDict;

pub(crate) struct SerializePyDict<'a, 'py> {
    ctx: PySerializeContext<'py>,
    pub(crate) obj: &'a Bound<'py, PyAny>,
    pub(crate) depth: Depth,
}

// impl<'py> ObjTypeRef<'py> for SerializePyDict<'_, 'py> {
//     fn type_ref(&self) -> &'py PyTypeCache {
//         self
//     }
// }

impl<'a, 'py> SerializePyDict<'a, 'py> {
    pub(crate) fn new(obj: &'a Bound<'py, PyAny>, ctx: PySerializeContext<'py>) -> Self {
        Self {
            obj,
            ctx,
            depth: Depth::default(),
        }
    }

    pub(crate) fn new_with_depth(
        obj: &'a Bound<'py, PyAny>,
        ctx: PySerializeContext<'py>,
        depth: Depth,
    ) -> Self {
        Self { obj, ctx, depth }
    }
}

impl Serialize for SerializePyDict<'_, '_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let py_dict: &Bound<'_, PyDict> = self.obj.downcast().map_err(pyerr2sererr)?;
        let len = py_dict.len();
        if len == 0 {
            return serializer.serialize_map(Some(0))?.end();
        }
        let mut m = serializer.serialize_map(None)?;
        for (k, v) in py_dict {
            let sk = SerializePyMappingKey::new(&k);
            let sv = SerializePyAny::new_with_depth(&v, self.ctx, self.depth + 1);
            m.serialize_entry(&sk, &sv)?;
        }
        m.end()
    }
}
