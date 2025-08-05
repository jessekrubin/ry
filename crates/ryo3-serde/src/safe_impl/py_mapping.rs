use pyo3::prelude::*;
use serde::ser::{Error as SerError, Serialize, SerializeMap, Serializer};

use crate::errors::pyerr2sererr;
use crate::{Depth, SerializePyAny};

use pyo3::{Bound, types::PyMapping};

pub(crate) struct SerializePyMapping<'a, 'py> {
    mapping: &'a Bound<'py, PyMapping>,
    depth: Depth,
    default: Option<&'py Bound<'py, PyAny>>,
}

impl<'a, 'py> SerializePyMapping<'a, 'py> {
    pub(crate) fn new_with_depth(
        mapping: &'a Bound<'py, PyMapping>,
        default: Option<&'py Bound<'py, PyAny>>,
        depth: Depth,
    ) -> Self {
        Self {
            mapping,
            depth,
            default,
        }
    }
}

impl Serialize for SerializePyMapping<'_, '_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let len = self.mapping.len().ok();
        if let Some(len) = len {
            let mut m = serializer.serialize_map(Some(len))?;
            let keys = self.mapping.keys().map_err(pyerr2sererr)?;
            for k in keys {
                let k = crate::pytypes::mapping_key(&k)?;
                let val = self.mapping.get_item(k).map_err(pyerr2sererr)?;
                let v = SerializePyAny::new_with_depth(&val, self.default, self.depth + 1);
                m.serialize_entry(k, &v).map_err(pyerr2sererr)?;
            }
            m.end()
        } else {
            Err(S::Error::custom(
                "SerializePyMapping: Length of mapping is not known.",
            ))
        }
    }
}
