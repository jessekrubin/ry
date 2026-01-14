//! Serializer for `PyAny`
//!
//! Based on a combination of `orjson`, `pythonize` and `rtoml`.

use pyo3::prelude::*;
use serde::ser::{Serialize, Serializer};

use crate::ob_type::PyObType;
use crate::ob_type_cache::PyTypeCache;
use crate::ser::PySerializeContext;
use crate::ser::py_types::{
    PyBoolSerializer, PyBytesLikeSerializer, PyDateSerializer, PyDateTimeSerializer,
    PyDictSerializer, PyFloatSerializer, PyFrozenSetSerializer, PyIntSerializer, PyListSerializer,
    PyNoneSerializer, PySetSerializer, PyStrSerializer, PyTimeDeltaSerializer, PyTimeSerializer,
    PyTupleSerializer, PyUnknownSerializer, PyUuidSerializer,
};
#[cfg(feature = "ry")]
use crate::ser::ry_types;
use crate::{Depth, MAX_DEPTH, serde_err_recursion};
use pyo3::Bound;

pub struct PyAnySerializer<'a, 'py> {
    pub(crate) obj: Borrowed<'a, 'py, PyAny>,
    pub(crate) ctx: PySerializeContext<'py>,
    pub(crate) depth: Depth,
}

impl<'a, 'py> PyAnySerializer<'a, 'py> {
    #[must_use]
    pub fn new(obj: Borrowed<'a, 'py, PyAny>, default: Option<&'py Bound<'py, PyAny>>) -> Self {
        let py = obj.py();
        let typeref = PyTypeCache::cached(py);
        let ctx = PySerializeContext::new(default, typeref);
        Self { obj, ctx, depth: 0 }
    }

    #[must_use]
    pub(crate) fn new_with_depth(
        obj: Borrowed<'a, 'py, PyAny>,
        ctx: PySerializeContext<'py>,
        depth: Depth,
    ) -> Self {
        Self { obj, ctx, depth }
    }
}

impl Serialize for PyAnySerializer<'_, '_> {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if self.depth == MAX_DEPTH {
            return serde_err_recursion!();
        }
        let ob_type = self.ctx.typeref.obtype(self.obj);
        match ob_type {
            PyObType::None | PyObType::Ellipsis => PyNoneSerializer::new().serialize(serializer),
            PyObType::Bool => PyBoolSerializer::new(self.obj).serialize(serializer),
            PyObType::Int => PyIntSerializer::new(self.obj).serialize(serializer),
            PyObType::Float => PyFloatSerializer::new(self.obj).serialize(serializer),
            PyObType::String => PyStrSerializer::new(self.obj).serialize(serializer),
            PyObType::List => {
                PyListSerializer::new(self.obj, self.ctx, self.depth).serialize(serializer)
            }
            PyObType::Tuple => {
                PyTupleSerializer::new(self.obj, self.ctx, self.depth).serialize(serializer)
            }
            PyObType::Dict => {
                PyDictSerializer::new(self.obj, self.ctx, self.depth).serialize(serializer)
            }
            PyObType::Set => PySetSerializer::new(self.obj, self.ctx).serialize(serializer),
            PyObType::FrozenSet => {
                PyFrozenSetSerializer::new(self.obj, self.ctx).serialize(serializer)
            }
            PyObType::DateTime => PyDateTimeSerializer::new(self.obj).serialize(serializer),
            PyObType::Date => PyDateSerializer::new(self.obj).serialize(serializer),
            PyObType::Time => PyTimeSerializer::new(self.obj).serialize(serializer),
            PyObType::Timedelta => PyTimeDeltaSerializer::new(self.obj).serialize(serializer),
            PyObType::Bytes | PyObType::ByteArray | PyObType::MemoryView => {
                PyBytesLikeSerializer::new(self.obj).serialize(serializer)
            }
            PyObType::PyUuid => PyUuidSerializer::new(self.obj).serialize(serializer),
            // now handled in Unknown
            // PyObType::Dataclass => {
            //     PyDataclassSerializer::new(self.obj, self.ctx, self.depth).serialize(serializer)
            // }
            // ------------------------------------------------------------
            // RY-TYPES
            // ------------------------------------------------------------
            // __STD__
            #[cfg(feature = "ryo3-std")]
            PyObType::PyDuration => {
                ry_types::PyDurationSerializer::new(self.obj).serialize(serializer)
            }

            #[cfg(feature = "ryo3-std")]
            PyObType::PyIpAddr => ry_types::PyIpAddrSerializer::new(self.obj).serialize(serializer),
            #[cfg(feature = "ryo3-std")]
            PyObType::PyIpv4Addr => {
                ry_types::PyIpv4AddrSerializer::new(self.obj).serialize(serializer)
            }
            #[cfg(feature = "ryo3-std")]
            PyObType::PyIpv6Addr => {
                ry_types::PyIpv6AddrSerializer::new(self.obj).serialize(serializer)
            }
            #[cfg(feature = "ryo3-std")]
            PyObType::PySocketAddr => {
                ry_types::PySocketAddrSerializer::new(self.obj).serialize(serializer)
            }
            #[cfg(feature = "ryo3-std")]
            PyObType::PySocketAddrV4 => {
                ry_types::PySocketAddrV4Serializer::new(self.obj).serialize(serializer)
            }
            #[cfg(feature = "ryo3-std")]
            PyObType::PySocketAddrV6 => {
                ry_types::PySocketAddrV6Serializer::new(self.obj).serialize(serializer)
            }

            // __HTTP__
            #[cfg(feature = "ryo3-http")]
            PyObType::RyHeaders => {
                ry_types::PyHeadersSerializer::new(self.obj).serialize(serializer)
            }
            #[cfg(feature = "ryo3-http")]
            PyObType::RyHttpStatus => {
                ry_types::PyHttpStatusSerializer::new(self.obj).serialize(serializer)
            }
            // __JIFF__
            #[cfg(feature = "ryo3-jiff")]
            PyObType::RyDate => ry_types::RyDateSerializer::new(self.obj).serialize(serializer),
            #[cfg(feature = "ryo3-jiff")]
            PyObType::RyDateTime => {
                ry_types::RyDateTimeSerializer::new(self.obj).serialize(serializer)
            }
            #[cfg(feature = "ryo3-jiff")]
            PyObType::RySignedDuration => {
                ry_types::RySignedDurationSerializer::new(self.obj).serialize(serializer)
            }
            #[cfg(feature = "ryo3-jiff")]
            PyObType::RyTime => ry_types::RyTimeSerializer::new(self.obj).serialize(serializer),
            #[cfg(feature = "ryo3-jiff")]
            PyObType::RyTimeSpan => ry_types::RySpanSerializer::new(self.obj).serialize(serializer),
            #[cfg(feature = "ryo3-jiff")]
            PyObType::RyTimestamp => {
                ry_types::RyTimestampSerializer::new(self.obj).serialize(serializer)
            }
            #[cfg(feature = "ryo3-jiff")]
            PyObType::RyTimeZone => {
                ry_types::RyTimeZoneSerializer::new(self.obj).serialize(serializer)
            }
            #[cfg(feature = "ryo3-jiff")]
            PyObType::RyZoned => ry_types::RyZonedSerializer::new(self.obj).serialize(serializer),
            // __ULID__
            #[cfg(feature = "ryo3-ulid")]
            PyObType::RyUlid => ry_types::PyUlidSerializer::new(self.obj).serialize(serializer),
            // __URL__
            #[cfg(feature = "ryo3-url")]
            PyObType::RyUrl => ry_types::PyUrlSerializer::new(self.obj).serialize(serializer),
            // __UUID__
            #[cfg(feature = "ryo3-uuid")]
            PyObType::RyUuid => ry_types::PyUuidSerializer::new(self.obj).serialize(serializer),
            // ------------------------------------------------------------
            // UNKNOWN
            // ------------------------------------------------------------
            PyObType::Unknown => {
                PyUnknownSerializer::new(self.obj, self.ctx, self.depth).serialize(serializer)
            }
        }
    }
}
