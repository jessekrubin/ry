use pyo3::prelude::*;
use serde::ser::{Serialize, SerializeMap, Serializer};

use crate::errors::pyerr2sererr;
use crate::ser::safe_impl::serialize_map_value;
use crate::{Depth, SerializePyAny};

use crate::ser::PySerializeContext;
use crate::ser::safe_impl::{SerializePyMappingKey, SerializePyUuid};
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
        let py_mapping: &Bound<'_, PyMapping> = self.obj.cast().map_err(pyerr2sererr)?;
        let len = py_mapping.len().ok();
        if let Some(len) = len
            && len == 0
        {
            return serializer.serialize_map(Some(0))?.end();
        }
        let mut m = serializer.serialize_map(len)?;
        let keys = py_mapping.keys().map_err(pyerr2sererr)?;
        let values = py_mapping.values().map_err(pyerr2sererr)?;
        for (k, v) in keys.iter().zip(values.iter()) {
            let sk = SerializePyMappingKey::new(&k);
            // let sv = SerializePyAny::new_with_depth(&v, self.ctx, self.depth + 1);
            let ob_type = self.ctx.typeref.obtype(&v);
            m.serialize_key(&sk)?;
            serialize_map_value!(ob_type, m, self, v);
        }

        // for (k, element) in py_mapping {
        //     let sk = SerializePyMappingKey::new(&k);
        //     // let sv = SerializePyAny::new_with_depth(&v, self.ctx, self.depth + 1);
        //     let ob_type = self.ctx.typeref.obtype(&element);
        //     m.serialize_key(&sk)?;
        //     serialize_map_value!(ob_type, m, self, element);
        // }
        m.end()
    }
}
