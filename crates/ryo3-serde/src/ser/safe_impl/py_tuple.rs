use pyo3::prelude::*;
use serde::ser::{Serialize, SerializeSeq, SerializeTuple, Serializer};

use crate::constants::{Depth, MAX_DEPTH};
use crate::errors::pyerr2sererr;
use crate::ob_type::PyObType;
use crate::ser::safe_impl::{
    SerializePyBool, SerializePyBytesLike, SerializePyDataclass, SerializePyDate,
    SerializePyDateTime, SerializePyDict, SerializePyFloat, SerializePyFrozenSet, SerializePyInt,
    SerializePyList, SerializePyNone, SerializePySet, SerializePyStr, SerializePyTime,
    SerializePyTimeDelta, SerializePyUuid,
};
use crate::ser::{PySerializeContext, rytypes};
use crate::{SerializePyAny, serde_err_recursion};

use pyo3::types::PyTuple;

pub(crate) struct SerializePyTuple<'a, 'py> {
    pub(crate) obj: &'a Bound<'py, PyAny>,
    pub(crate) ctx: PySerializeContext<'py>,
    pub(crate) depth: Depth,
}

impl<'a, 'py> SerializePyTuple<'a, 'py> {
    pub(crate) fn new(
        obj: &'a Bound<'py, PyAny>,
        ctx: PySerializeContext<'py>,
        depth: Depth,
    ) -> Self {
        Self { obj, ctx, depth }
    }
}

macro_rules! serialize_tuple_element {
    ($ob_type:expr, $seq:expr, $self:expr, $element:expr) => {
        match $ob_type {
            PyObType::None | PyObType::Ellipsis => {
                $seq.serialize_element(&SerializePyNone::new())?;
            }
            PyObType::Bool => {
                $seq.serialize_element(&SerializePyBool::new(&$element))?;
            }
            PyObType::Int => {
                $seq.serialize_element(&SerializePyInt::new(&$element))?;
            }
            PyObType::Float => {
                $seq.serialize_element(&SerializePyFloat::new(&$element))?;
            }
            PyObType::String => {
                $seq.serialize_element(&SerializePyStr::new(&$element))?;
            }
            PyObType::List => {
                $seq.serialize_element(&SerializePyList::new(
                    &$element,
                    $self.ctx,
                    $self.depth + 1,
                ))?;
            }
            PyObType::Tuple => {
                $seq.serialize_element(&SerializePyTuple::new(
                    &$element,
                    $self.ctx,
                    $self.depth + 1,
                ))?;
            }
            PyObType::Dict => {
                $seq.serialize_element(&SerializePyDict::new(
                    &$element,
                    $self.ctx,
                    $self.depth + 1,
                ))?;
            }
            PyObType::Set => {
                $seq.serialize_element(&SerializePySet::new(&$element, $self.ctx))?;
            }
            PyObType::FrozenSet => {
                $seq.serialize_element(&SerializePyFrozenSet::new(&$element, $self.ctx))?;
            }
            PyObType::DateTime => {
                $seq.serialize_element(&SerializePyDateTime::new(&$element))?;
            }
            PyObType::Date => {
                $seq.serialize_element(&SerializePyDate::new(&$element))?;
            }
            PyObType::Time => {
                $seq.serialize_element(&SerializePyTime::new(&$element))?;
            }
            PyObType::Timedelta => {
                $seq.serialize_element(&SerializePyTimeDelta::new(&$element))?;
            }
            PyObType::Bytes | PyObType::ByteArray | PyObType::MemoryView => {
                $seq.serialize_element(&SerializePyBytesLike::new(&$element))?;
            }
            PyObType::PyUuid => {
                $seq.serialize_element(&SerializePyUuid::new(&$element))?;
            }
            PyObType::Dataclass => {
                $seq.serialize_element(&SerializePyDataclass::new(
                    &$element,
                    $self.ctx,
                    $self.depth,
                ))?;
            }
            // ------------------------------------------------------------
            // RY-TYPES
            // ------------------------------------------------------------
            // __STD__
            #[cfg(feature = "ryo3-std")]
            PyObType::PyDuration => {
                $seq.serialize_element(&rytypes::PyDurationSerializer::new(&$element))?;
            }

            #[cfg(feature = "ryo3-std")]
            PyObType::PyIpAddr => {
                $seq.serialize_element(&rytypes::PyIpAddrSerializer::new(&$element))?;
            }
            #[cfg(feature = "ryo3-std")]
            PyObType::PyIpv4Addr => {
                $seq.serialize_element(&rytypes::PyIpv4AddrSerializer::new(&$element))?;
            }
            #[cfg(feature = "ryo3-std")]
            PyObType::PyIpv6Addr => {
                $seq.serialize_element(&rytypes::PyIpv6AddrSerializer::new(&$element))?;
            }
            #[cfg(feature = "ryo3-std")]
            PyObType::PySocketAddr => {
                $seq.serialize_element(&rytypes::PySocketAddrSerializer::new(&$element))?;
            }
            #[cfg(feature = "ryo3-std")]
            PyObType::PySocketAddrV4 => {
                $seq.serialize_element(&rytypes::PySocketAddrV4Serializer::new(&$element))?;
            }
            #[cfg(feature = "ryo3-std")]
            PyObType::PySocketAddrV6 => {
                $seq.serialize_element(&rytypes::PySocketAddrV6Serializer::new(&$element))?;
            }

            // __HTTP__
            #[cfg(feature = "ryo3-http")]
            PyObType::RyHeaders => {
                $seq.serialize_element(&rytypes::PyHeadersSerializer::new(&$element))?;
            }
            #[cfg(feature = "ryo3-http")]
            PyObType::RyHttpStatus => {
                $seq.serialize_element(&rytypes::PyHttpStatusSerializer::new(&$element))?;
            }
            // __JIFF__
            #[cfg(feature = "ryo3-jiff")]
            PyObType::RyDate => {
                $seq.serialize_element(&rytypes::RyDateSerializer::new(&$element))?;
            }
            #[cfg(feature = "ryo3-jiff")]
            PyObType::RyDateTime => {
                $seq.serialize_element(&rytypes::RyDateTimeSerializer::new(&$element))?;
            }
            #[cfg(feature = "ryo3-jiff")]
            PyObType::RySignedDuration => {
                $seq.serialize_element(&rytypes::RySignedDurationSerializer::new(&$element))?;
            }
            #[cfg(feature = "ryo3-jiff")]
            PyObType::RyTime => {
                $seq.serialize_element(&rytypes::RyTimeSerializer::new(&$element))?;
            }
            #[cfg(feature = "ryo3-jiff")]
            PyObType::RyTimeSpan => {
                $seq.serialize_element(&rytypes::RySpanSerializer::new(&$element))?;
            }
            #[cfg(feature = "ryo3-jiff")]
            PyObType::RyTimestamp => {
                $seq.serialize_element(&rytypes::RyTimestampSerializer::new(&$element))?;
            }
            #[cfg(feature = "ryo3-jiff")]
            PyObType::RyTimeZone => {
                $seq.serialize_element(&rytypes::RyTimeZoneSerializer::new(&$element))?;
            }
            #[cfg(feature = "ryo3-jiff")]
            PyObType::RyZoned => {
                $seq.serialize_element(&rytypes::RyZonedSerializer::new(&$element))?;
            }
            // __ULID__
            #[cfg(feature = "ryo3-ulid")]
            PyObType::RyUlid => {
                $seq.serialize_element(&rytypes::PyUlidSerializer::new(&$element))?;
            }
            // __URL__
            #[cfg(feature = "ryo3-url")]
            PyObType::RyUrl => {
                $seq.serialize_element(&rytypes::PyUrlSerializer::new(&$element))?;
            }
            // __UUID__
            #[cfg(feature = "ryo3-uuid")]
            PyObType::RyUuid => {
                $seq.serialize_element(&rytypes::PyUuidSerializer::new(&$element))?;
            }
            // ------------------------------------------------------------
            // UNKNOWN
            // ------------------------------------------------------------
            PyObType::Unknown => {
                $seq.serialize_element(&SerializePyAny::new_with_depth(
                    &$element,
                    $self.ctx,
                    $self.depth + 1,
                ))?;
            }
        }
    };
}

impl Serialize for SerializePyTuple<'_, '_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if self.depth == MAX_DEPTH {
            return serde_err_recursion!();
        }
        let py_tuple: &Bound<'_, PyTuple> = self.obj.cast().map_err(pyerr2sererr)?;
        let len = py_tuple.len();
        if len == 0 {
            serializer.serialize_seq(Some(0))?.end()
        } else if len <= 16 {
            let mut tup = serializer.serialize_tuple(len)?;
            for element in py_tuple {
                let ob_type = self.ctx.typeref.obtype(&element);
                serialize_tuple_element!(ob_type, tup, self, element);
            }
            tup.end()
        } else {
            let mut seq = serializer.serialize_seq(Some(len))?;
            for element in py_tuple {
                let ob_type = self.ctx.typeref.obtype(&element);
                serialize_tuple_element!(ob_type, seq, self, element);
            }
            seq.end()
        }
    }
}
