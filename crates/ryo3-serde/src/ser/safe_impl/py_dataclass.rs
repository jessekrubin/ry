use pyo3::sync::PyOnceLock;
use pyo3::types::PyString;
use pyo3::{intern, prelude::*};
use serde::ser::{Error as SerError, Serialize, SerializeMap, Serializer};

use crate::errors::pyerr2sererr;
use crate::{Depth, MAX_DEPTH, SerializePyAny, serde_err, serde_err_recursion};

use crate::ser::PySerializeContext;
use crate::ser::dataclass::dataclass_fields;
use pyo3::{Bound, types::PyDict};

pub(crate) struct SerializePyDataclass<'a, 'py> {
    ctx: PySerializeContext<'py>,
    obj: &'a Bound<'py, PyAny>,
    depth: Depth,
}

impl<'a, 'py> SerializePyDataclass<'a, 'py> {
    pub(crate) fn new(
        obj: &'a Bound<'py, PyAny>,
        ctx: PySerializeContext<'py>,
        depth: Depth,
    ) -> Self {
        Self { ctx, obj, depth }
    }
}

// as done in pydantic-core: https://github.com/pydantic/pydantic-core/blob/5f0b5a8b26691b7a1e3de07cb409b21bb174929c/src/serializers/shared.rs#L591
static DC_FIELD_MARKER: PyOnceLock<Py<PyAny>> = PyOnceLock::new();
/// needed to match the logic from dataclasses.fields `tuple(f for f in fields.values() if f._field_type is _FIELD)`
fn get_field_marker(py: Python<'_>) -> PyResult<&Bound<'_, PyAny>> {
    DC_FIELD_MARKER.import(py, "dataclasses", "_FIELD")
}

impl Serialize for SerializePyDataclass<'_, '_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if self.depth == MAX_DEPTH {
            return serde_err_recursion!();
        }

        let py = self.obj.py();
        // check for __dict__
        if let Ok(dunder_dict) = self.obj.getattr(intern!(py, "__dict__")) {
            if let Ok(dict) = dunder_dict.cast_into::<PyDict>() {
                // serialize the __dict__ as a dict
                SerializePyAny::new_with_depth(&dict, self.ctx, self.depth + 1)
                    .serialize(serializer)
            } else {
                serde_err!("__dict__ is not a dict")
            }
        } else if let Some(fields) = dataclass_fields(self.obj) {
            let field_marker = get_field_marker(py).map_err(pyerr2sererr)?;
            let mut map = serializer.serialize_map(None)?;
            for (field_name, field) in fields.iter() {
                // check if the field is a dataclass field
                let field_type = field
                    .getattr(intern!(py, "_field_type"))
                    .map_err(pyerr2sererr)?;
                if field_type.is(field_marker) {
                    // this is a dataclass field
                    let field_name_py_str =
                        field_name.cast_into::<PyString>().map_err(pyerr2sererr)?;
                    let value = self.obj.getattr(&field_name_py_str).map_err(pyerr2sererr)?;
                    let field_ser =
                        SerializePyAny::new_with_depth(&value, self.ctx, self.depth + 1);

                    // actual string
                    let s = field_name_py_str
                        .to_str()
                        .map_err(|_| SerError::custom("field name is not a valid UTF-8 string"))?;
                    map.serialize_entry(s, &field_ser)?;
                }
            }
            map.end()
        } else {
            serde_err!("object is not a dataclass instance")
        }
    }
}
