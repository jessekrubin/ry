use pyo3::prelude::*;
use serde::ser::{Serialize, SerializeSeq, SerializeTuple, Serializer};

use crate::constants::{Depth, MAX_DEPTH};
use crate::errors::pyerr2sererr;
use crate::ob_type::PyObType;
use crate::ser::safe_impl::{
    PyBoolSerializer, PyBytesLikeSerializer, PyDataclassSerializer, PyDateSerializer,
    PyDateTimeSerializer, PyDictSerializer, PyFloatSerializer, PyFrozenSetSerializer, PyIntSerializer,
    PyListSerializer, PyNoneSerializer, PySetSerializer, PyStrSerializer, PyTimeSerializer,
    PyTimeDeltaSerializer, PyUuidSerializer,
};
use crate::ser::{PySerializeContext, rytypes};
use crate::{PyAnySerializer, serde_err_recursion};

use pyo3::types::PyTuple;

pub(crate) struct PyTupleSerializer<'a, 'py> {
    pub(crate) obj: &'a Bound<'py, PyAny>,
    pub(crate) ctx: PySerializeContext<'py>,
    pub(crate) depth: Depth,
}

impl<'a, 'py> PyTupleSerializer<'a, 'py> {
    pub(crate) fn new(
        obj: Borrowed<'a, 'py, PyAny>,
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
                $seq.serialize_element(&PyNoneSerializer::new())?;
            }
            PyObType::Bool => {
                $seq.serialize_element(&PyBoolSerializer::new(&$element))?;
            }
            PyObType::Int => {
                $seq.serialize_element(&PyIntSerializer::new(&$element))?;
            }
            PyObType::Float => {
                $seq.serialize_element(&PyFloatSerializer::new(&$element))?;
            }
            PyObType::String => {
                $seq.serialize_element(&PyStrSerializer::new(&$element))?;
            }
            PyObType::List => {
                $seq.serialize_element(&PyListSerializer::new(
                    &$element,
                    $self.ctx,
                    $self.depth + 1,
                ))?;
            }
            PyObType::Tuple => {
                $seq.serialize_element(&PyTupleSerializer::new(
                    &$element,
                    $self.ctx,
                    $self.depth + 1,
                ))?;
            }
            PyObType::Dict => {
                $seq.serialize_element(&PyDictSerializer::new(
                    &$element,
                    $self.ctx,
                    $self.depth + 1,
                ))?;
            }
            PyObType::Set => {
                $seq.serialize_element(&PySetSerializer::new(&$element, $self.ctx))?;
            }
            PyObType::FrozenSet => {
                $seq.serialize_element(&PyFrozenSetSerializer::new(&$element, $self.ctx))?;
            }
            PyObType::DateTime => {
                $seq.serialize_element(&PyDateTimeSerializer::new(&$element))?;
            }
            PyObType::Date => {
                $seq.serialize_element(&PyDateSerializer::new(&$element))?;
            }
            PyObType::Time => {
                $seq.serialize_element(&PyTimeSerializer::new(&$element))?;
            }
            PyObType::Timedelta => {
                $seq.serialize_element(&PyTimeDeltaSerializer::new(&$element))?;
            }
            PyObType::Bytes | PyObType::ByteArray | PyObType::MemoryView => {
                $seq.serialize_element(&PyBytesLikeSerializer::new(&$element))?;
            }
            PyObType::PyUuid => {
                $seq.serialize_element(&PyUuidSerializer::new(&$element))?;
            }
            PyObType::Dataclass => {
                $seq.serialize_element(&PyDataclassSerializer::new(
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
                $seq.serialize_element(&PyAnySerializer::new_with_depth(
                    &$element,
                    $self.ctx,
                    $self.depth + 1,
                ))?;
            }
        }
    };
}

impl Serialize for PyTupleSerializer<'_, '_> {
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
        } else  {
            let mut tup = serializer.serialize_tuple(len)?;
            for element in py_tuple {
                let ob_type = self.ctx.typeref.obtype(&element);
                serialize_tuple_element!(ob_type, tup, self, element);
            }
            tup.end()
        }
        // else {
        //     let mut seq = serializer.serialize_seq(Some(len))?;
        //     for element in py_tuple {
        //         let ob_type = self.ctx.typeref.obtype(&element);
        //         serialize_tuple_element!(ob_type, seq, self, element);
        //     }
        //     seq.end()
        // }
    }
}
