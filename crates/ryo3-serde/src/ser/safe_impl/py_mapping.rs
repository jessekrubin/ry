use pyo3::prelude::*;
use serde::ser::{Serialize, SerializeMap, Serializer};

use crate::errors::pyerr2sererr;
use crate::{Depth, SerializePyAny};

use crate::ser::PySerializeContext;
use crate::ser::safe_impl::py_mapping_key::SerializePyMappingKey;
use pyo3::types::PyDict;
use pyo3::{Bound, types::PyMapping};

pub(crate) struct SerializePyMapping<'a, 'py> {
    ctx: PySerializeContext<'py>,
    obj: &'a Bound<'py, PyMapping>,
    depth: Depth,
}

impl<'a, 'py> SerializePyMapping<'a, 'py> {
    pub(crate) fn new_with_depth(
        obj: &'a Bound<'py, PyMapping>,
        ctx: PySerializeContext<'py>,
        depth: Depth,
    ) -> Self {
        Self { ctx, obj, depth }
    }
}

impl Serialize for SerializePyMapping<'_, '_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let py_dict: &Bound<'_, PyDict> = self.obj.cast().map_err(pyerr2sererr)?;
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
