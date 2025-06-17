//! Serializer for `PyAny`
//!
//! Based on a combination of `orjson`, `pythonize` and `rtoml`.
use pyo3::prelude::*;
use serde::ser::{Error as SerError, Serialize, SerializeMap, Serializer};

use crate::any_repr::any_repr;
use crate::errors::pyerr2sererr;
use crate::pytypes::{
    bool_, byteslike, date, datetime, dict, float, frozenset, int, list, none, py_uuid, set, str,
    time, tuple,
};
#[cfg(feature = "ryo3-ulid")]
use crate::rytypes::ry_ulid;
#[cfg(feature = "ryo3-url")]
use crate::rytypes::ry_url;
#[cfg(feature = "ryo3-uuid")]
use crate::rytypes::ry_uuid;
#[cfg(feature = "ryo3-jiff")]
use crate::rytypes::{
    ry_date, ry_datetime, ry_signed_duration, ry_span, ry_time, ry_timestamp, ry_zoned,
};
use crate::type_cache::{PyObType, PyTypeCache};
use pyo3::types::{PyMapping, PySequence};
use pyo3::Bound;
use serde::ser::SerializeSeq;
pub struct SerializePyAny<'py> {
    pub(crate) obj: Bound<'py, PyAny>,
    pub(crate) none_value: Option<&'py str>,
    ob_type_lookup: &'py PyTypeCache,
}

impl<'py> SerializePyAny<'py> {
    #[must_use]
    pub fn new(py: Python<'py>, obj: Bound<'py, PyAny>, none_value: Option<&'py str>) -> Self {
        Self {
            obj,
            none_value,
            ob_type_lookup: PyTypeCache::cached(py),
        }
    }

    pub(crate) fn with_obj(&self, obj: Bound<'py, PyAny>) -> Self {
        Self {
            obj,
            none_value: self.none_value,
            ob_type_lookup: self.ob_type_lookup,
        }
    }
}

macro_rules! serde_err {
    ($($arg:tt)*) => {
        Err(SerError::custom(format_args!($($arg)*)))
    }
}

impl Serialize for SerializePyAny<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let lookup = self.ob_type_lookup;
        if let Some(ob_type) = lookup.obtype(&self.obj) {
            match ob_type {
                PyObType::None => none(self, serializer),
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
                PyObType::Bytes | PyObType::ByteArray => byteslike(self, serializer),
                PyObType::PyUuid => py_uuid(self, serializer),
                #[cfg(feature = "ryo3-uuid")]
                PyObType::RyUuid => ry_uuid(self, serializer),
                #[cfg(feature = "ryo3-ulid")]
                PyObType::RyUlid => ry_ulid(self, serializer), // ulid is treated as a uuid for now
                #[cfg(feature = "ryo3-url")]
                PyObType::RyUrl => ry_url(self, serializer),
                #[cfg(feature = "ryo3-jiff")]
                PyObType::RyDate => ry_date(self, serializer),
                #[cfg(feature = "ryo3-jiff")]
                PyObType::RyDateTime => ry_datetime(self, serializer),
                #[cfg(feature = "ryo3-jiff")]
                PyObType::RySignedDuration => ry_signed_duration(self, serializer),
                #[cfg(feature = "ryo3-jiff")]
                PyObType::RyTime => ry_time(self, serializer),
                #[cfg(feature = "ryo3-jiff")]
                PyObType::RyTimeSpan => ry_span(self, serializer),
                #[cfg(feature = "ryo3-jiff")]
                PyObType::RyTimestamp => ry_timestamp(self, serializer),
                #[cfg(feature = "ryo3-jiff")]
                PyObType::RyZoned => ry_zoned(self, serializer),
            }
        } else if let Ok(py_map) = self.obj.downcast::<PyMapping>() {
            SerializePyMapping::new(py_map).serialize(serializer)
        } else if let Ok(py_seq) = self.obj.downcast::<PySequence>() {
            SerializePySequence::new(py_seq).serialize(serializer)
        } else {
            serde_err!("{} is not json-serializable", any_repr(&self.obj))
        }
    }
}

// ===========================================================================
// PySequence ~ PySequence ~ PySequence ~ PySequence ~ PySequence ~ PySequence
// ===========================================================================
struct SerializePySequence<'a, 'py> {
    seq: &'a Bound<'py, PySequence>,
}

impl<'a, 'py> SerializePySequence<'a, 'py> {
    fn new(seq: &'a Bound<'py, PySequence>) -> Self {
        Self { seq }
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
            let item_ser = SerializePyAny::new(self.seq.py(), item, None);
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
}

impl<'a, 'py> SerializePyMapping<'a, 'py> {
    fn new(mapping: &'a Bound<'py, PyMapping>) -> Self {
        Self { mapping }
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
            let py = self.mapping.py();
            let keys = self.mapping.keys().map_err(pyerr2sererr)?;
            for k in keys {
                let k = crate::pytypes::mapping_key(&k)?;
                let val = self.mapping.get_item(k).map_err(pyerr2sererr)?;
                let v = SerializePyAny::new(py, val, None);
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
