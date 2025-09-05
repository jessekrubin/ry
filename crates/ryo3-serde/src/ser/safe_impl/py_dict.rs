use pyo3::prelude::*;
use serde::ser::{Serialize, SerializeMap, Serializer};

use crate::errors::pyerr2sererr;

use crate::constants::{Depth, MAX_DEPTH};
use crate::ob_type::PyObType;
use crate::ser::safe_impl::py_mapping_key::SerializePyMappingKey;
use crate::ser::safe_impl::{
    SerializePyBool, SerializePyBytesLike, SerializePyDataclass, SerializePyDate,
    SerializePyDateTime, SerializePyFloat, SerializePyFrozenSet, SerializePyInt, SerializePyList,
    SerializePyNone, SerializePySet, SerializePyStr, SerializePyTime, SerializePyTimeDelta,
    SerializePyTuple, SerializePyUuid,
};
use crate::ser::types::SerError;
use crate::ser::{PySerializeContext, rytypes};
use crate::{SerializePyAny, serde_err_recursion};
use pyo3::Bound;
use pyo3::types::PyDict;

pub(crate) struct SerializePyDict<'a, 'py> {
    ctx: PySerializeContext<'py>,
    pub(crate) obj: &'a Bound<'py, PyAny>,
    pub(crate) depth: Depth,
}

impl<'a, 'py> SerializePyDict<'a, 'py> {
    pub(crate) fn new(
        obj: &'a Bound<'py, PyAny>,
        ctx: PySerializeContext<'py>,
        depth: Depth,
    ) -> Self {
        Self { ctx, obj, depth }
    }
}

impl Serialize for SerializePyDict<'_, '_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if self.depth == MAX_DEPTH {
            return serde_err_recursion!();
        }
        let py_dict: &Bound<'_, PyDict> = self.obj.downcast_exact().map_err(pyerr2sererr)?;
        let len = py_dict.len();
        if len == 0 {
            return serializer.serialize_map(Some(0))?.end();
        }
        let mut m = serializer.serialize_map(None)?;
        for (k, element) in py_dict {
            let sk = SerializePyMappingKey::new(&k);
            // let sv = SerializePyAny::new_with_depth(&v, self.ctx, self.depth + 1);
            let ob_type = self.ctx.typeref.obtype(&element);
            match ob_type {
                PyObType::None | PyObType::Ellipsis => {
                    m.serialize_entry(&sk, &SerializePyNone::new())?;
                }
                PyObType::Bool => {
                    m.serialize_entry(&sk, &SerializePyBool::new(&element))?;
                }
                PyObType::Int => {
                    m.serialize_entry(&sk, &SerializePyInt::new(&element))?;
                }
                PyObType::Float => {
                    m.serialize_entry(&sk, &SerializePyFloat::new(&element))?;
                }
                PyObType::String => {
                    m.serialize_entry(&sk, &SerializePyStr::new(&element))?;
                }
                PyObType::List => {
                    m.serialize_entry(
                        &sk,
                        &SerializePyList::new(&element, self.ctx, self.depth + 1),
                    )?;
                }
                PyObType::Tuple => {
                    m.serialize_entry(
                        &sk,
                        &SerializePyTuple::new(&element, self.ctx, self.depth + 1),
                    )?;
                }
                PyObType::Dict => {
                    m.serialize_entry(
                        &sk,
                        &SerializePyDict::new(&element, self.ctx, self.depth + 1),
                    )?;
                }
                PyObType::Set => {
                    m.serialize_entry(&sk, &SerializePySet::new(&element, self.ctx))?;
                }
                PyObType::FrozenSet => {
                    m.serialize_entry(&sk, &SerializePyFrozenSet::new(&element, self.ctx))?;
                }
                PyObType::DateTime => {
                    m.serialize_entry(&sk, &SerializePyDateTime::new(&element))?;
                }
                PyObType::Date => {
                    m.serialize_entry(&sk, &SerializePyDate::new(&element))?;
                }
                PyObType::Time => {
                    m.serialize_entry(&sk, &SerializePyTime::new(&element))?;
                }
                PyObType::Timedelta => {
                    m.serialize_entry(&sk, &SerializePyTimeDelta::new(&element))?;
                }
                PyObType::Bytes | PyObType::ByteArray | PyObType::MemoryView => {
                    m.serialize_entry(&sk, &SerializePyBytesLike::new(&element))?;
                }
                PyObType::PyUuid => {
                    m.serialize_entry(&sk, &SerializePyUuid::new(&element))?;
                }
                PyObType::Dataclass => {
                    m.serialize_entry(
                        &sk,
                        &SerializePyDataclass::new(&element, self.ctx, self.depth),
                    )?;
                }
                // ------------------------------------------------------------
                // RY-TYPES
                // ------------------------------------------------------------
                // __STD__
                #[cfg(feature = "ryo3-std")]
                PyObType::PyDuration => {
                    m.serialize_entry(&sk, &rytypes::PyDurationSerializer::new(&element))?;
                }

                #[cfg(feature = "ryo3-std")]
                PyObType::PyIpAddr => {
                    m.serialize_entry(&sk, &rytypes::PyIpAddrSerializer::new(&element))?;
                }
                #[cfg(feature = "ryo3-std")]
                PyObType::PyIpv4Addr => {
                    m.serialize_entry(&sk, &rytypes::PyIpv4AddrSerializer::new(&element))?;
                }
                #[cfg(feature = "ryo3-std")]
                PyObType::PyIpv6Addr => {
                    m.serialize_entry(&sk, &rytypes::PyIpv6AddrSerializer::new(&element))?;
                }
                #[cfg(feature = "ryo3-std")]
                PyObType::PySocketAddr => {
                    m.serialize_entry(&sk, &rytypes::PySocketAddrSerializer::new(&element))?;
                }
                #[cfg(feature = "ryo3-std")]
                PyObType::PySocketAddrV4 => {
                    m.serialize_entry(&sk, &rytypes::PySocketAddrV4Serializer::new(&element))?;
                }
                #[cfg(feature = "ryo3-std")]
                PyObType::PySocketAddrV6 => {
                    m.serialize_entry(&sk, &rytypes::PySocketAddrV6Serializer::new(&element))?;
                }

                // __HTTP__
                #[cfg(feature = "ryo3-http")]
                PyObType::RyHeaders => {
                    m.serialize_entry(&sk, &rytypes::PyHeadersSerializer::new(&element))?;
                }
                #[cfg(feature = "ryo3-http")]
                PyObType::RyHttpStatus => {
                    m.serialize_entry(&sk, &rytypes::PyHttpStatusSerializer::new(&element))?;
                }
                // __JIFF__
                #[cfg(feature = "ryo3-jiff")]
                PyObType::RyDate => {
                    m.serialize_entry(&sk, &rytypes::RyDateSerializer::new(&element))?;
                }
                #[cfg(feature = "ryo3-jiff")]
                PyObType::RyDateTime => {
                    m.serialize_entry(&sk, &rytypes::RyDateTimeSerializer::new(&element))?;
                }
                #[cfg(feature = "ryo3-jiff")]
                PyObType::RySignedDuration => {
                    m.serialize_entry(&sk, &rytypes::RySignedDurationSerializer::new(&element))?;
                }
                #[cfg(feature = "ryo3-jiff")]
                PyObType::RyTime => {
                    m.serialize_entry(&sk, &rytypes::RyTimeSerializer::new(&element))?;
                }
                #[cfg(feature = "ryo3-jiff")]
                PyObType::RyTimeSpan => {
                    m.serialize_entry(&sk, &rytypes::RySpanSerializer::new(&element))?;
                }
                #[cfg(feature = "ryo3-jiff")]
                PyObType::RyTimestamp => {
                    m.serialize_entry(&sk, &rytypes::RyTimestampSerializer::new(&element))?;
                }
                #[cfg(feature = "ryo3-jiff")]
                PyObType::RyTimeZone => {
                    m.serialize_entry(&sk, &rytypes::RyTimeZoneSerializer::new(&element))?;
                }
                #[cfg(feature = "ryo3-jiff")]
                PyObType::RyZoned => {
                    m.serialize_entry(&sk, &rytypes::RyZonedSerializer::new(&element))?;
                }
                // __ULID__
                #[cfg(feature = "ryo3-ulid")]
                PyObType::RyUlid => {
                    m.serialize_entry(&sk, &rytypes::PyUlidSerializer::new(&element))?;
                }
                // __URL__
                #[cfg(feature = "ryo3-url")]
                PyObType::RyUrl => {
                    m.serialize_entry(&sk, &rytypes::PyUrlSerializer::new(&element))?;
                }
                // __UUID__
                #[cfg(feature = "ryo3-uuid")]
                PyObType::RyUuid => {
                    m.serialize_entry(&sk, &rytypes::PyUuidSerializer::new(&element))?;
                }
                // ------------------------------------------------------------
                // UNKNOWN
                // ------------------------------------------------------------
                PyObType::Unknown => {
                    m.serialize_entry(
                        &sk,
                        &SerializePyAny::new_with_depth(&element, self.ctx, self.depth + 1),
                    )?;
                }
            }
        }
        m.end()
    }
}
