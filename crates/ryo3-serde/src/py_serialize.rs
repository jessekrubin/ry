//! Serializer for `PyAny`
//!
//! Based on a combination of `orjson`, `pythonize` and `rtoml`.

use pyo3::prelude::*;
use serde::ser::{Error as SerError, Serialize, SerializeMap, Serializer};

use crate::any_repr::any_repr;
use crate::errors::pyerr2sererr;
use crate::pytypes::{
    bool_, byteslike, date, datetime, dict, float, frozenset, int, list, none, py_uuid, set, str,
    time, timedelta, tuple,
};
#[cfg(feature = "ry")]
use crate::rytypes;
// #[cfg(feature = "ryo3-ulid")]
// use crate::rytypes::ry_ulid;
// #[cfg(feature = "ryo3-url")]
// use crate::rytypes::ry_url;
// #[cfg(feature = "ryo3-uuid")]
// use crate::rytypes::ry_uuid;
// #[cfg(feature = "ryo3-jiff")]
// use crate::rytypes::{
//     ry_date, ry_datetime, ry_signed_duration, ry_span, ry_time, ry_timestamp, ry_timezone, ry_zoned,
// };
// #[cfg(feature = "ryo3-http")]
// use crate::rytypes::{ry_headers, ry_http_status};
use crate::type_cache::{PyObType, PyTypeCache};
use pyo3::sync::GILOnceCell;
use pyo3::types::{PyAnyMethods, PyDict, PyMapping, PySequence, PyString};
use pyo3::{Bound, intern};
use serde::ser::SerializeSeq;

type Depth = u8;
const MAX_DEPTH: Depth = 255;
pub struct SerializePyAny<'py> {
    pub(crate) obj: &'py Bound<'py, PyAny>,
    pub(crate) depth: Depth,
    default: Option<&'py Bound<'py, PyAny>>,
    ob_type_lookup: &'py PyTypeCache,
}

macro_rules! serde_err {
    ($($arg:tt)*) => {
        Err(SerError::custom(format_args!($($arg)*)))
    }
}

impl<'py> SerializePyAny<'py> {
    #[must_use]
    pub fn new(obj: &'py Bound<'py, PyAny>, default: Option<&'py Bound<'py, PyAny>>) -> Self {
        let py = obj.py();
        Self {
            obj,
            default,
            depth: 0,
            ob_type_lookup: PyTypeCache::cached(py),
        }
    }

    #[must_use]
    pub fn new_with_depth(
        obj: &'py Bound<'py, PyAny>,
        default: Option<&'py Bound<'py, PyAny>>,
        depth: Depth,
    ) -> Self {
        let py = obj.py();
        Self {
            obj,
            default,
            ob_type_lookup: PyTypeCache::cached(py),
            depth,
        }
    }

    pub(crate) fn with_obj(&self, obj: &'py Bound<'py, PyAny>) -> Self {
        Self {
            obj,
            ob_type_lookup: self.ob_type_lookup,
            default: self.default,
            depth: self.depth + 1,
        }
    }
}
fn dataclass_fields<'a, 'py>(obj: &'a Bound<'py, PyAny>) -> Option<Bound<'py, PyDict>>
where
    'py: 'a, // keep lifetimes compatible
{
    obj.getattr("__dataclass_fields__") // PyResult<Bound<PyAny>>
        .ok()? // Option<Bound<PyAny>>
        .downcast_into::<PyDict>() // PyResult<Bound<PyDict>>
        .ok() // Option<Bound<PyDict>>
}

impl Serialize for SerializePyAny<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if self.depth == MAX_DEPTH {
            return Err(SerError::custom("recursion"));
        }

        let lookup = self.ob_type_lookup;
        if let Some(ob_type) = lookup.obtype(self.obj) {
            match ob_type {
                PyObType::None | PyObType::Ellipsis => none(self, serializer),
                PyObType::Bool => bool_(self, serializer),
                PyObType::Int => int(self, serializer),
                PyObType::Float => float(self, serializer),
                PyObType::String => str(self, serializer),
                PyObType::List => list(self, serializer),
                PyObType::Tuple => tuple(self, serializer),
                PyObType::Dict => dict(self, serializer),
                PyObType::Set => set(self, serializer),
                PyObType::Frozenset => frozenset(self, serializer),
                PyObType::DateTime => datetime(self, serializer),
                PyObType::Date => date(self, serializer),
                PyObType::Time => time(self, serializer),
                PyObType::Timedelta => timedelta(self, serializer),
                PyObType::Bytes | PyObType::ByteArray | PyObType::MemoryView => {
                    byteslike(self, serializer)
                }
                PyObType::PyUuid => py_uuid(self, serializer),
                // ------------------------------------------------------------
                // RY-TYPES
                // ------------------------------------------------------------
                // __UUID__
                #[cfg(feature = "ryo3-uuid")]
                PyObType::RyUuid => rytypes::ry_uuid(self, serializer),
                // __ULID__
                #[cfg(feature = "ryo3-ulid")]
                PyObType::RyUlid => rytypes::ry_ulid(self, serializer), // ulid is treated as a uuid for now
                // __URL__
                #[cfg(feature = "ryo3-url")]
                PyObType::RyUrl => rytypes::ry_url(self, serializer),
                // __HTTP__
                #[cfg(feature = "ryo3-http")]
                PyObType::RyHeaders => rytypes::ry_headers(self, serializer),
                #[cfg(feature = "ryo3-http")]
                PyObType::RyHttpStatus => rytypes::ry_http_status(self, serializer),
                // __JIFF__
                #[cfg(feature = "ryo3-jiff")]
                PyObType::RyDate => rytypes::ry_date(self, serializer),
                #[cfg(feature = "ryo3-jiff")]
                PyObType::RyDateTime => rytypes::ry_datetime(self, serializer),
                #[cfg(feature = "ryo3-jiff")]
                PyObType::RySignedDuration => rytypes::ry_signed_duration(self, serializer),
                #[cfg(feature = "ryo3-jiff")]
                PyObType::RyTime => rytypes::ry_time(self, serializer),
                #[cfg(feature = "ryo3-jiff")]
                PyObType::RyTimeSpan => rytypes::ry_span(self, serializer),
                #[cfg(feature = "ryo3-jiff")]
                PyObType::RyTimestamp => rytypes::ry_timestamp(self, serializer),
                #[cfg(feature = "ryo3-jiff")]
                PyObType::RyTimeZone => rytypes::ry_timezone(self, serializer),
                #[cfg(feature = "ryo3-jiff")]
                PyObType::RyZoned => rytypes::ry_zoned(self, serializer),
            }
        } else if let Some(fields) = dataclass_fields(self.obj) {
            let dc_serializer =
                DataclassSerializer::new(self.obj, self.default, self.depth + 1, fields);
            dc_serializer.serialize(serializer)
        } else if let Ok(py_map) = self.obj.downcast::<PyMapping>() {
            SerializePyMapping::new_with_depth(py_map, self.default, self.depth + 1)
                .serialize(serializer)
        } else if let Ok(py_seq) = self.obj.downcast::<PySequence>() {
            SerializePySequence::new_with_depth(py_seq, self.default, self.depth + 1)
                .serialize(serializer)
        } else if let Some(default) = self.default {
            // call the default transformer fn and attempt to then serialize the result
            let r = default.call1((&self.obj,)).map_err(pyerr2sererr)?;
            self.with_obj(&r).serialize(serializer)
        } else {
            serde_err!("{} is not json-serializable", any_repr(self.obj))
        }
    }
}

// ===========================================================================
// PySequence ~ PySequence ~ PySequence ~ PySequence ~ PySequence ~ PySequence
// ===========================================================================
struct SerializePySequence<'a, 'py> {
    seq: &'a Bound<'py, PySequence>,
    depth: Depth,
    default: Option<&'py Bound<'py, PyAny>>,
}

impl<'a, 'py> SerializePySequence<'a, 'py> {
    fn new_with_depth(
        seq: &'a Bound<'py, PySequence>,
        default: Option<&'py Bound<'py, PyAny>>,
        depth: Depth,
    ) -> Self {
        Self {
            seq,
            depth,
            default,
        }
    }
}

impl Serialize for SerializePySequence<'_, '_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let len = self.seq.len().map_err(pyerr2sererr)?;
        let mut seq = serializer.serialize_seq(Some(len))?;
        for i in 0..len {
            let item = self.seq.get_item(i).map_err(pyerr2sererr)?;
            let item_ser = SerializePyAny::new_with_depth(&item, self.default, self.depth + 1);
            seq.serialize_element(&item_ser)?;
        }
        seq.end()
    }
}
// ===========================================================================
// PyMapping ~ PyMapping ~ PyMapping ~ PyMapping ~ PyMapping ~ PyMapping
// ===========================================================================

struct SerializePyMapping<'a, 'py> {
    mapping: &'a Bound<'py, PyMapping>,
    depth: Depth,
    default: Option<&'py Bound<'py, PyAny>>,
}

impl<'a, 'py> SerializePyMapping<'a, 'py> {
    fn new_with_depth(
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

// ===========================================================================
// DATACLASSES
// ===========================================================================

struct DataclassSerializer<'a, 'py> {
    obj: &'a Bound<'py, PyAny>,
    default: Option<&'py Bound<'py, PyAny>>,
    fields: Bound<'py, PyDict>,
    depth: Depth,
}

impl<'a, 'py> DataclassSerializer<'a, 'py> {
    fn new(
        obj: &'a Bound<'py, PyAny>,
        default: Option<&'py Bound<'py, PyAny>>,
        depth: Depth,
        fields: Bound<'py, PyDict>,
    ) -> Self {
        Self {
            obj,
            default,
            fields,
            depth,
        }
    }
}
// as done in pydantic-core: https://github.com/pydantic/pydantic-core/blob/5f0b5a8b26691b7a1e3de07cb409b21bb174929c/src/serializers/shared.rs#L591
static DC_FIELD_MARKER: GILOnceCell<PyObject> = GILOnceCell::new();
/// needed to match the logic from dataclasses.fields `tuple(f for f in fields.values() if f._field_type is _FIELD)`
fn get_field_marker(py: Python<'_>) -> PyResult<&Bound<'_, PyAny>> {
    DC_FIELD_MARKER.import(py, "dataclasses", "_FIELD")
}

impl Serialize for DataclassSerializer<'_, '_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if self.depth == MAX_DEPTH {
            return Err(SerError::custom("recursion"));
        }

        // check for __dict__
        if let Ok(dunder_dict) = self.obj.getattr("__dict__") {
            if let Ok(dict) = dunder_dict.downcast_into::<PyDict>() {
                // serialize the __dict__ as a dict
                SerializePyAny::new_with_depth(&dict, self.default, self.depth + 1)
                    .serialize(serializer)
            } else {
                serde_err!("__dict__ is not a dict")
            }
        } else {
            let py = self.obj.py();
            let field_marker = get_field_marker(py).map_err(pyerr2sererr)?;
            let mut map = serializer.serialize_map(None)?;
            for (field_name, field) in self.fields.iter() {
                // check if the field is a dataclass field
                let field_type = field
                    .getattr(intern!(py, "_field_type"))
                    .map_err(pyerr2sererr)?;
                if field_type.is(field_marker) {
                    // this is a dataclass field
                    let field_name_py_str = field_name
                        .downcast_into::<PyString>()
                        .map_err(pyerr2sererr)?;
                    let value = self.obj.getattr(&field_name_py_str).map_err(pyerr2sererr)?;
                    let field_ser =
                        SerializePyAny::new_with_depth(&value, self.default, self.depth + 1);

                    // actual string
                    let s = field_name_py_str
                        .to_str()
                        .map_err(|_| SerError::custom("field name is not a valid UTF-8 string"))?;
                    map.serialize_entry(s, &field_ser)?;
                }
            }
            map.end()
        }
    }
}
