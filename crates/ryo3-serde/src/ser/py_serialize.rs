//! Serializer for `PyAny`
//!
//! Based on a combination of `orjson`, `pythonize` and `rtoml`.

use pyo3::prelude::*;
use serde::ser::{Serialize, Serializer};

use crate::any_repr::any_repr;
use crate::errors::pyerr2sererr;
use crate::ob_type::PyObType;
use crate::ob_type_cache::PyTypeCache;
use crate::ser::PySerializeContext;
#[cfg(feature = "ry")]
use crate::ser::rytypes;
use crate::ser::safe_impl::{
    SerializePyBool, SerializePyBytesLike, SerializePyDataclass, SerializePyDate,
    SerializePyDateTime, SerializePyDict, SerializePyFloat, SerializePyFrozenSet, SerializePyInt,
    SerializePyList, SerializePyMapping, SerializePyNone, SerializePySequence, SerializePySet,
    SerializePyStr, SerializePyStrSubclass, SerializePyTime, SerializePyTimeDelta,
    SerializePyTuple, SerializePyUuid,
};
use crate::{Depth, MAX_DEPTH, serde_err, serde_err_recursion};
use pyo3::Bound;
use pyo3::types::{PyAnyMethods, PyMapping, PySequence, PyString};

pub struct SerializePyAny<'py> {
    pub(crate) obj: &'py Bound<'py, PyAny>,
    pub(crate) ctx: PySerializeContext<'py>,
    pub(crate) depth: Depth,
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
impl Serialize for SerializePyAny<'_> {
    #[expect(clippy::too_many_lines)]
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if self.depth == MAX_DEPTH {
            return serde_err_recursion!();
        }
        let ob_type = self.ctx.typeref.obtype(self.obj);
        let ob_borrowed = self.obj.as_borrowed();
        match ob_type {
            PyObType::None | PyObType::Ellipsis => SerializePyNone::new().serialize(serializer),
            PyObType::Bool => SerializePyBool::new(ob_borrowed).serialize(serializer),
            PyObType::Int => SerializePyInt::new(ob_borrowed).serialize(serializer),
            PyObType::Float => SerializePyFloat::new(ob_borrowed).serialize(serializer),
            PyObType::String => SerializePyStr::new(ob_borrowed).serialize(serializer),
            PyObType::List => {
                SerializePyList::new(ob_borrowed, self.ctx, self.depth).serialize(serializer)
            }
            PyObType::Tuple => {
                SerializePyTuple::new(ob_borrowed, self.ctx, self.depth).serialize(serializer)
            }
            PyObType::Dict => {
                SerializePyDict::new(ob_borrowed, self.ctx, self.depth).serialize(serializer)
            }
            PyObType::Set => SerializePySet::new(ob_borrowed, self.ctx).serialize(serializer),
            PyObType::FrozenSet => {
                SerializePyFrozenSet::new(ob_borrowed, self.ctx).serialize(serializer)
            }
            PyObType::DateTime => SerializePyDateTime::new(ob_borrowed).serialize(serializer),
            PyObType::Date => SerializePyDate::new(ob_borrowed).serialize(serializer),
            PyObType::Time => SerializePyTime::new(ob_borrowed).serialize(serializer),
            PyObType::Timedelta => SerializePyTimeDelta::new(ob_borrowed).serialize(serializer),
            PyObType::Bytes | PyObType::ByteArray | PyObType::MemoryView => {
                SerializePyBytesLike::new(ob_borrowed).serialize(serializer)
            }
            PyObType::PyUuid => SerializePyUuid::new(ob_borrowed).serialize(serializer),
            PyObType::Dataclass => {
                SerializePyDataclass::new(ob_borrowed, self.ctx, self.depth).serialize(serializer)
            }
            // ------------------------------------------------------------
            // RY-TYPES
            // ------------------------------------------------------------
            // __STD__
            #[cfg(feature = "ryo3-std")]
            PyObType::PyDuration => {
                rytypes::PyDurationSerializer::new(ob_borrowed).serialize(serializer)
            }

            #[cfg(feature = "ryo3-std")]
            PyObType::PyIpAddr => rytypes::PyIpAddrSerializer::new(ob_borrowed).serialize(serializer),
            #[cfg(feature = "ryo3-std")]
            PyObType::PyIpv4Addr => {
                rytypes::PyIpv4AddrSerializer::new(ob_borrowed).serialize(serializer)
            }
            #[cfg(feature = "ryo3-std")]
            PyObType::PyIpv6Addr => {
                rytypes::PyIpv6AddrSerializer::new(ob_borrowed).serialize(serializer)
            }
            #[cfg(feature = "ryo3-std")]
            PyObType::PySocketAddr => {
                rytypes::PySocketAddrSerializer::new(ob_borrowed).serialize(serializer)
            }
            #[cfg(feature = "ryo3-std")]
            PyObType::PySocketAddrV4 => {
                rytypes::PySocketAddrV4Serializer::new(ob_borrowed).serialize(serializer)
            }
            #[cfg(feature = "ryo3-std")]
            PyObType::PySocketAddrV6 => {
                rytypes::PySocketAddrV6Serializer::new(ob_borrowed).serialize(serializer)
            }

            // __HTTP__
            #[cfg(feature = "ryo3-http")]
            PyObType::RyHeaders => {
                rytypes::PyHeadersSerializer::new(ob_borrowed).serialize(serializer)
            }
            #[cfg(feature = "ryo3-http")]
            PyObType::RyHttpStatus => {
                rytypes::PyHttpStatusSerializer::new(ob_borrowed).serialize(serializer)
            }
            // __JIFF__
            #[cfg(feature = "ryo3-jiff")]
            PyObType::RyDate => rytypes::RyDateSerializer::new(ob_borrowed).serialize(serializer),
            #[cfg(feature = "ryo3-jiff")]
            PyObType::RyDateTime => {
                rytypes::RyDateTimeSerializer::new(ob_borrowed).serialize(serializer)
            }
            #[cfg(feature = "ryo3-jiff")]
            PyObType::RySignedDuration => {
                rytypes::RySignedDurationSerializer::new(ob_borrowed).serialize(serializer)
            }
            #[cfg(feature = "ryo3-jiff")]
            PyObType::RyTime => rytypes::RyTimeSerializer::new(ob_borrowed).serialize(serializer),
            #[cfg(feature = "ryo3-jiff")]
            PyObType::RyTimeSpan => rytypes::RySpanSerializer::new(ob_borrowed).serialize(serializer),
            #[cfg(feature = "ryo3-jiff")]
            PyObType::RyTimestamp => {
                rytypes::RyTimestampSerializer::new(ob_borrowed).serialize(serializer)
            }
            #[cfg(feature = "ryo3-jiff")]
            PyObType::RyTimeZone => {
                rytypes::RyTimeZoneSerializer::new(ob_borrowed).serialize(serializer)
            }
            #[cfg(feature = "ryo3-jiff")]
            PyObType::RyZoned => rytypes::RyZonedSerializer::new(ob_borrowed).serialize(serializer),
            // __ULID__
            #[cfg(feature = "ryo3-ulid")]
            PyObType::RyUlid => rytypes::PyUlidSerializer::new(ob_borrowed).serialize(serializer),
            // __URL__
            #[cfg(feature = "ryo3-url")]
            PyObType::RyUrl => rytypes::PyUrlSerializer::new(ob_borrowed).serialize(serializer),
            // __UUID__
            #[cfg(feature = "ryo3-uuid")]
            PyObType::RyUuid => rytypes::PyUuidSerializer::new(ob_borrowed).serialize(serializer),
            // ------------------------------------------------------------
            // UNKNOWN
            // ------------------------------------------------------------
            PyObType::Unknown => {
                if let Ok(pystr_subclass) = self.obj.cast::<PyString>() {
                    SerializePyStrSubclass::new(pystr_subclass).serialize(serializer)
                } else if let Ok(py_map) = self.obj.cast::<PyMapping>() {
                    SerializePyMapping::new_with_depth(py_map, self.ctx, self.depth)
                        .serialize(serializer)
                } else if let Ok(py_seq) = self.obj.cast::<PySequence>() {
                    SerializePySequence::new_with_depth(py_seq, self.ctx, self.depth)
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
