use pyo3::prelude::*;
use serde::ser::{Serialize, SerializeMap, SerializeSeq, Serializer};

use crate::errors::pyerr2sererr;

use crate::SerializePyAny;
use crate::constants::Depth;
use crate::safe_impl::with_obj::ObjTypeRef;
use crate::type_cache::PyTypeCache;
use pyo3::Bound;
use pyo3::types::PyTuple;

pub(crate) struct SerializePyTuple<'a, 'py> {
    pub(crate) obj: &'a Bound<'py, PyAny>,
    pub(crate) depth: Depth,
    default: Option<&'py Bound<'py, PyAny>>,
    ob_type_lookup: &'py PyTypeCache,
}

impl<'a, 'py> ObjTypeRef<'py> for SerializePyTuple<'a, 'py> {
    fn type_ref(&self) -> &'py PyTypeCache {
        self.ob_type_lookup
    }
}

impl<'a, 'py> SerializePyTuple<'a, 'py> {
    pub(crate) fn new(
        obj: &'a Bound<'py, PyAny>,
        ob_type_lookup: &'py PyTypeCache,
        default: Option<&'py Bound<'py, PyAny>>,
    ) -> Self {
        Self {
            obj,
            ob_type_lookup,
            depth: Depth::default(),
            default,
        }
    }
}

impl Serialize for SerializePyTuple<'_, '_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let py_list: &Bound<'_, PyTuple> = self.obj.downcast().map_err(pyerr2sererr)?;
        let len = py_list.len();
        if len == 0 {
            serializer.serialize_seq(Some(0))?.end()
        } else {
            let mut seq = serializer.serialize_seq(Some(len))?;
            for element in py_list {
                seq.serialize_element(&SerializePyAny::new_with_depth(
                    &element,
                    self.default,
                    self.depth + 1,
                    self.ob_type_lookup,
                ))?;
            }
            seq.end()
        }
    }
}
