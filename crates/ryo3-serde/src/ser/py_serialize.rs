//! Serializer for `PyAny`
//!
//! Based on a combination of `orjson`, `pythonize` and `rtoml`.

use pyo3::prelude::*;
use serde::ser::{Serialize, Serializer};

use crate::ob_type::PyObType;
use crate::ob_type_cache::PyTypeCache;
use crate::ser::PySerializeContext;
use crate::ser::py_unknown::PyUnknownSerializer;
#[cfg(feature = "ry")]
use crate::ser::rytypes;
use crate::ser::safe_impl::{
    PyBoolSerializer, PyBytesLikeSerializer, PyDataclassSerializer, PyDateSerializer,
    PyDateTimeSerializer, PyDictSerializer, PyFloatSerializer, PyFrozenSetSerializer,
    PyIntSerializer, PyListSerializer, PyNoneSerializer, PySetSerializer, PyStrSerializer,
    PyTimeDeltaSerializer, PyTimeSerializer, PyTupleSerializer, PyUuidSerializer,
};
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
        match ob_type {
            PyObType::None | PyObType::Ellipsis => PyNoneSerializer::new().serialize(serializer),
            PyObType::Bool => PyBoolSerializer::new_unchecked(self.obj).serialize(serializer),
            PyObType::Int => PyIntSerializer::new_unchecked(self.obj).serialize(serializer),
            PyObType::Float => PyFloatSerializer::new_unchecked(self.obj).serialize(serializer),
            PyObType::String => PyStrSerializer::new_unchecked(self.obj).serialize(serializer),
            PyObType::List => PyListSerializer::new_unchecked(self.obj, self.ctx, self.depth)
                .serialize(serializer),
            PyObType::Tuple => PyTupleSerializer::new_unchecked(self.obj, self.ctx, self.depth)
                .serialize(serializer),
            PyObType::Dict => PyDictSerializer::new_unchecked(self.obj, self.ctx, self.depth)
                .serialize(serializer),
            PyObType::Set => {
                PySetSerializer::new_unchecked(self.obj, self.ctx).serialize(serializer)
            }
            PyObType::FrozenSet => {
                PyFrozenSetSerializer::new_unchecked(self.obj, self.ctx).serialize(serializer)
            }
            PyObType::DateTime => {
                PyDateTimeSerializer::new_unchecked(self.obj).serialize(serializer)
            }
            PyObType::Date => PyDateSerializer::new_unchecked(self.obj).serialize(serializer),
            PyObType::Time => PyTimeSerializer::new_unchecked(self.obj).serialize(serializer),
            PyObType::Timedelta => {
                PyTimeDeltaSerializer::new_unchecked(self.obj).serialize(serializer)
            }
            PyObType::Bytes | PyObType::ByteArray | PyObType::MemoryView => {
                PyBytesLikeSerializer::new(self.obj).serialize(serializer)
            }
            PyObType::PyUuid => PyUuidSerializer::new(self.obj).serialize(serializer),
            PyObType::Dataclass => {
                PyDataclassSerializer::new(self.obj, self.ctx, self.depth).serialize(serializer)
            }
            // ------------------------------------------------------------
            // RY-TYPES
            // ------------------------------------------------------------
            // __STD__
            #[cfg(feature = "ryo3-std")]
            PyObType::PyDuration => {
                rytypes::PyDurationSerializer::new_unchecked(self.obj).serialize(serializer)
            }

            #[cfg(feature = "ryo3-std")]
            PyObType::PyIpAddr => {
                rytypes::PyIpAddrSerializer::new_unchecked(self.obj).serialize(serializer)
            }
            #[cfg(feature = "ryo3-std")]
            PyObType::PyIpv4Addr => {
                rytypes::PyIpv4AddrSerializer::new_unchecked(self.obj).serialize(serializer)
            }
            #[cfg(feature = "ryo3-std")]
            PyObType::PyIpv6Addr => {
                rytypes::PyIpv6AddrSerializer::new_unchecked(self.obj).serialize(serializer)
            }
            #[cfg(feature = "ryo3-std")]
            PyObType::PySocketAddr => {
                rytypes::PySocketAddrSerializer::new_unchecked(self.obj).serialize(serializer)
            }
            #[cfg(feature = "ryo3-std")]
            PyObType::PySocketAddrV4 => {
                rytypes::PySocketAddrV4Serializer::new_unchecked(self.obj).serialize(serializer)
            }
            #[cfg(feature = "ryo3-std")]
            PyObType::PySocketAddrV6 => {
                rytypes::PySocketAddrV6Serializer::new_unchecked(self.obj).serialize(serializer)
            }

            // __HTTP__
            #[cfg(feature = "ryo3-http")]
            PyObType::RyHeaders => {
                rytypes::PyHeadersSerializer::new_unchecked(self.obj).serialize(serializer)
            }
            #[cfg(feature = "ryo3-http")]
            PyObType::RyHttpStatus => {
                rytypes::PyHttpStatusSerializer::new_unchecked(self.obj).serialize(serializer)
            }
            // __JIFF__
            #[cfg(feature = "ryo3-jiff")]
            PyObType::RyDate => {
                rytypes::RyDateSerializer::new_unchecked(self.obj).serialize(serializer)
            }
            #[cfg(feature = "ryo3-jiff")]
            PyObType::RyDateTime => {
                rytypes::RyDateTimeSerializer::new_unchecked(self.obj).serialize(serializer)
            }
            #[cfg(feature = "ryo3-jiff")]
            PyObType::RySignedDuration => {
                rytypes::RySignedDurationSerializer::new_unchecked(self.obj).serialize(serializer)
            }
            #[cfg(feature = "ryo3-jiff")]
            PyObType::RyTime => {
                rytypes::RyTimeSerializer::new_unchecked(self.obj).serialize(serializer)
            }
            #[cfg(feature = "ryo3-jiff")]
            PyObType::RyTimeSpan => {
                rytypes::RySpanSerializer::new_unchecked(self.obj).serialize(serializer)
            }
            #[cfg(feature = "ryo3-jiff")]
            PyObType::RyTimestamp => {
                rytypes::RyTimestampSerializer::new_unchecked(self.obj).serialize(serializer)
            }
            #[cfg(feature = "ryo3-jiff")]
            PyObType::RyTimeZone => {
                rytypes::RyTimeZoneSerializer::new_unchecked(self.obj).serialize(serializer)
            }
            #[cfg(feature = "ryo3-jiff")]
            PyObType::RyZoned => {
                rytypes::RyZonedSerializer::new_unchecked(self.obj).serialize(serializer)
            }
            // __ULID__
            #[cfg(feature = "ryo3-ulid")]
            PyObType::RyUlid => {
                rytypes::PyUlidSerializer::new_unchecked(self.obj).serialize(serializer)
            }
            // __URL__
            #[cfg(feature = "ryo3-url")]
            PyObType::RyUrl => {
                rytypes::PyUrlSerializer::new_unchecked(self.obj).serialize(serializer)
            }
            // __UUID__
            #[cfg(feature = "ryo3-uuid")]
            PyObType::RyUuid => {
                rytypes::PyUuidSerializer::new_unchecked(self.obj).serialize(serializer)
            }
            // ------------------------------------------------------------
            // UNKNOWN
            // ------------------------------------------------------------
            PyObType::Unknown => PyUnknownSerializer::from(self).serialize(serializer),
            // PyObType::Unknown => {
            //     if let Ok(pystr_subclass) = self.obj.cast::<PyString>() {
            //         PyStrSubclassSerializer::new(pystr_subclass).serialize(serializer)
            //     } else if let Ok(py_map) = self.obj.cast::<PyMapping>() {
            //         PyMappingSerializer::new_with_depth(py_map, self.ctx, self.depth)
            //             .serialize(serializer)
            //     } else if let Ok(py_seq) = self.obj.cast::<PySequence>() {
            //         PySequenceSerializer::new_with_depth(py_seq, self.ctx, self.depth)
            //             .serialize(serializer)
            //     } else if let Some(default) = self.ctx.default {
            //         // call the default transformer fn and attempt to then serialize the result
            //         let r = default.call1((&self.obj,)).map_err(pyerr2sererr)?;
            //         self.with_obj(r.as_borrowed()).serialize(serializer)
            //     } else {
            //         serde_err!("{} is not json-serializable", any_repr(self.obj))
            //     }
            // }
        }
    }
}
