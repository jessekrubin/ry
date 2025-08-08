//! Serializer for `PyAny`
//!
//! Based on a combination of `orjson`, `pythonize` and `rtoml`.

use pyo3::prelude::*;
use serde::ser::{Error as SerError, Serialize, Serializer};

use crate::any_repr::any_repr;
use crate::errors::pyerr2sererr;
use crate::ser::PySerializeContext;
#[cfg(feature = "ry")]
use crate::ser::rytypes;
use crate::ser::safe_impl::{
    SerializePyBool, SerializePyBytesLike, SerializePyDataclass, SerializePyDate,
    SerializePyDateTime, SerializePyDict, SerializePyFloat, SerializePyFrozenSet, SerializePyInt,
    SerializePyList, SerializePyMapping, SerializePyNone, SerializePySequence, SerializePySet,
    SerializePyStr, SerializePyTime, SerializePyTimeDelta, SerializePyTuple, SerializePyUuid,
};
use crate::type_cache::{PyObType, PyTypeCache};
use crate::{Depth, MAX_DEPTH};
use pyo3::types::{PyAnyMethods, PyDict, PyMapping, PySequence};
use pyo3::{Bound, intern};

pub struct SerializePyAny<'py> {
    pub(crate) obj: &'py Bound<'py, PyAny>,
    pub(crate) ctx: PySerializeContext<'py>,
    pub(crate) depth: Depth,
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
        let typeref = PyTypeCache::cached(py);
        let ctx = PySerializeContext::new(default, typeref);
        Self { obj, ctx, depth: 0 }
    }

    #[must_use]
    pub(crate) fn new_with_depth(
        obj: &'py Bound<'py, PyAny>,
        ctx: PySerializeContext<'py>,
        depth: Depth,
    ) -> Self {
        Self { obj, ctx, depth }
    }

    pub(crate) fn with_obj(&self, obj: &'py Bound<'py, PyAny>) -> Self {
        Self {
            obj,
            ctx: self.ctx,
            depth: self.depth + 1,
        }
    }
}

fn dataclass_fields<'a, 'py>(obj: &'a Bound<'py, PyAny>) -> Option<Bound<'py, PyDict>>
where
    'py: 'a,
{
    obj.getattr(intern!(obj.py(), "__dataclass_fields__")) // PyResult<Bound<PyAny>>
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

        let ob_type = self.ctx.typeref.obtype(self.obj);
        match ob_type {
            PyObType::None | PyObType::Ellipsis => SerializePyNone::new().serialize(serializer),
            PyObType::Bool => SerializePyBool::new(self.obj).serialize(serializer),
            PyObType::Int => SerializePyInt::new(self.obj).serialize(serializer),
            PyObType::Float => SerializePyFloat::new(self.obj).serialize(serializer),
            PyObType::String => SerializePyStr::new(self.obj).serialize(serializer),
            PyObType::List => SerializePyList::new(self.obj, self.ctx).serialize(serializer),
            PyObType::Tuple => SerializePyTuple::new(self.obj, self.ctx).serialize(serializer),
            PyObType::Dict => SerializePyDict::new_with_depth(self.obj, self.ctx, self.depth)
                .serialize(serializer),
            PyObType::Set => SerializePySet::new(self.obj, self.ctx).serialize(serializer),
            PyObType::FrozenSet => {
                SerializePyFrozenSet::new(self.obj, self.ctx).serialize(serializer)
            }
            PyObType::DateTime => SerializePyDateTime::new(self.obj).serialize(serializer),
            PyObType::Date => SerializePyDate::new(self.obj).serialize(serializer),
            PyObType::Time => SerializePyTime::new(self.obj).serialize(serializer),
            PyObType::Timedelta => SerializePyTimeDelta::new(self.obj).serialize(serializer),
            PyObType::Bytes | PyObType::ByteArray | PyObType::MemoryView => {
                SerializePyBytesLike::new(self.obj).serialize(serializer)
            }
            PyObType::PyUuid => SerializePyUuid::new(self.obj).serialize(serializer),
            // ------------------------------------------------------------
            // RY-TYPES
            // ------------------------------------------------------------
            // __HTTP__
            #[cfg(feature = "ryo3-http")]
            PyObType::RyHeaders => {
                rytypes::PyHeadersSerializer::new(self.obj).serialize(serializer)
            }
            #[cfg(feature = "ryo3-http")]
            PyObType::RyHttpStatus => {
                rytypes::PyHttpStatusSerializer::new(self.obj).serialize(serializer)
            }
            // __JIFF__
            #[cfg(feature = "ryo3-jiff")]
            PyObType::RyDate => rytypes::RyDateSerializer::new(self.obj).serialize(serializer),
            #[cfg(feature = "ryo3-jiff")]
            PyObType::RyDateTime => {
                rytypes::RyDateTimeSerializer::new(self.obj).serialize(serializer)
            }
            #[cfg(feature = "ryo3-jiff")]
            PyObType::RySignedDuration => {
                rytypes::RySignedDurationSerializer::new(self.obj).serialize(serializer)
            }
            #[cfg(feature = "ryo3-jiff")]
            PyObType::RyTime => rytypes::RyTimeSerializer::new(self.obj).serialize(serializer),
            #[cfg(feature = "ryo3-jiff")]
            PyObType::RyTimeSpan => rytypes::RySpanSerializer::new(self.obj).serialize(serializer),
            #[cfg(feature = "ryo3-jiff")]
            PyObType::RyTimestamp => {
                rytypes::RyTimestampSerializer::new(self.obj).serialize(serializer)
            }
            #[cfg(feature = "ryo3-jiff")]
            PyObType::RyTimeZone => {
                rytypes::RyTimeZoneSerializer::new(self.obj).serialize(serializer)
            }
            #[cfg(feature = "ryo3-jiff")]
            PyObType::RyZoned => rytypes::RyZonedSerializer::new(self.obj).serialize(serializer),
            // __ULID__
            #[cfg(feature = "ryo3-ulid")]
            PyObType::RyUlid => rytypes::PyUlidSerializer::new(self.obj).serialize(serializer),
            // __URL__
            #[cfg(feature = "ryo3-url")]
            PyObType::RyUrl => rytypes::PyUrlSerializer::new(self.obj).serialize(serializer),
            // __UUID__
            #[cfg(feature = "ryo3-uuid")]
            PyObType::RyUuid => rytypes::PyUuidSerializer::new(self.obj).serialize(serializer),
            // ------------------------------------------------------------
            // UNKNOWN
            // ------------------------------------------------------------
            PyObType::Unknown => {
                // if the object is a PyAny, we can try to serialize it as a dataclass or mapping
                if let Some(fields) = dataclass_fields(self.obj) {
                    let dc_serializer =
                        SerializePyDataclass::new(self.obj, self.ctx, self.depth + 1, fields);
                    dc_serializer.serialize(serializer)
                } else if let Ok(py_map) = self.obj.downcast::<PyMapping>() {
                    SerializePyMapping::new_with_depth(py_map, self.ctx, self.depth + 1)
                        .serialize(serializer)
                } else if let Ok(py_seq) = self.obj.downcast::<PySequence>() {
                    SerializePySequence::new_with_depth(py_seq, self.ctx, self.depth + 1)
                        .serialize(serializer)
                } else if let Some(default) = self.ctx.default {
                    // call the default transformer fn and attempt to then serialize the result
                    let r = default.call1((&self.obj,)).map_err(pyerr2sererr)?;
                    self.with_obj(&r).serialize(serializer)
                } else {
                    serde_err!("{} is not json-serializable", any_repr(self.obj))
                }
            }
        }
    }
}
