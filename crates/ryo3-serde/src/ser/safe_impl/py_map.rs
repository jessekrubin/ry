use pyo3::prelude::*;
use serde::ser::{Serialize, SerializeMap, Serializer};

use crate::errors::pyerr2sererr;

use crate::constants::{Depth, MAX_DEPTH};
use crate::ob_type::PyObType;
use crate::ser::PySerializeContext;
use crate::ser::PyUnknownSerializer;
#[cfg(any(
    feature = "ryo3-http",
    feature = "ryo3-jiff",
    feature = "ryo3-ulid",
    feature = "ryo3-url",
    feature = "ryo3-uuid",
    feature = "ryo3-std"
))]
use crate::ser::rytypes;
use crate::ser::safe_impl::{
    PyBoolSerializer, PyBytesLikeSerializer, PyDataclassSerializer, PyDateSerializer,
    PyDateTimeSerializer, PyFloatSerializer, PyFrozenSetSerializer, PyIntSerializer,
    PyListSerializer, PyMappingKeySerializer, PyNoneSerializer, PySetSerializer, PyStrSerializer,
    PyTimeDeltaSerializer, PyTimeSerializer, PyTupleSerializer, PyUuidSerializer,
};
use crate::serde_err_recursion;
use pyo3::types::{PyDict, PyMapping};

pub(crate) struct PyDictSerializer<'a, 'py> {
    ctx: PySerializeContext<'py>,
    pub(crate) obj: Borrowed<'a, 'py, PyDict>,
    pub(crate) depth: Depth,
}

impl<'a, 'py> PyDictSerializer<'a, 'py> {
    #[inline]
    pub(crate) fn new(
        obj: Borrowed<'a, 'py, PyDict>,
        ctx: PySerializeContext<'py>,
        depth: Depth,
    ) -> Self {
        Self { ctx, obj, depth }
    }

    #[inline]
    #[expect(unsafe_code)]
    pub(crate) fn new_unchecked(
        obj: Borrowed<'a, 'py, PyAny>,
        ctx: PySerializeContext<'py>,
        depth: Depth,
    ) -> Self {
        let py_dict = unsafe { obj.cast_unchecked::<PyDict>() };
        Self::new(py_dict, ctx, depth)
    }
}

// TODO: swap to serialize_entry
macro_rules! serialize_map_value {
    ($ob_type:expr, $map:expr, $self:expr, $value:expr) => {
        match $ob_type {
            PyObType::None | PyObType::Ellipsis => {
                $map.serialize_value(&PyNoneSerializer::new())?;
            }
            PyObType::Bool => {
                $map.serialize_value(&PyBoolSerializer::new_unchecked($value))?;
            }
            PyObType::Int => {
                $map.serialize_value(&PyIntSerializer::new_unchecked($value))?;
            }
            PyObType::Float => {
                $map.serialize_value(&PyFloatSerializer::new_unchecked($value))?;
            }
            PyObType::String => {
                $map.serialize_value(&PyStrSerializer::new_unchecked($value))?;
            }
            PyObType::List => {
                $map.serialize_value(&PyListSerializer::new_unchecked(
                    $value,
                    $self.ctx,
                    $self.depth + 1,
                ))?;
            }
            PyObType::Tuple => {
                $map.serialize_value(&PyTupleSerializer::new_unchecked(
                    $value,
                    $self.ctx,
                    $self.depth + 1,
                ))?;
            }
            PyObType::Dict => {
                $map.serialize_value(&PyDictSerializer::new_unchecked(
                    $value,
                    $self.ctx,
                    $self.depth + 1,
                ))?;
            }
            PyObType::Set => {
                $map.serialize_value(&PySetSerializer::new_unchecked($value, $self.ctx))?;
            }
            PyObType::FrozenSet => {
                $map.serialize_value(&PyFrozenSetSerializer::new_unchecked($value, $self.ctx))?;
            }
            PyObType::DateTime => {
                $map.serialize_value(&PyDateTimeSerializer::new_unchecked($value))?;
            }
            PyObType::Date => {
                $map.serialize_value(&PyDateSerializer::new_unchecked($value))?;
            }
            PyObType::Time => {
                $map.serialize_value(&PyTimeSerializer::new_unchecked($value))?;
            }
            PyObType::Timedelta => {
                $map.serialize_value(&PyTimeDeltaSerializer::new_unchecked($value))?;
            }
            PyObType::Bytes | PyObType::ByteArray | PyObType::MemoryView => {
                $map.serialize_value(&PyBytesLikeSerializer::new($value))?;
            }
            PyObType::PyUuid => {
                $map.serialize_value(&PyUuidSerializer::new($value))?;
            }
            PyObType::Dataclass => {
                $map.serialize_value(&PyDataclassSerializer::new($value, $self.ctx, $self.depth))?;
            }
            // ------------------------------------------------------------
            // RY-TYPES
            // ------------------------------------------------------------
            // __STD__
            #[cfg(feature = "ryo3-std")]
            PyObType::PyDuration => {
                $map.serialize_value(&rytypes::PyDurationSerializer::new_unchecked($value))?;
            }

            #[cfg(feature = "ryo3-std")]
            PyObType::PyIpAddr => {
                $map.serialize_value(&rytypes::PyIpAddrSerializer::new_unchecked($value))?;
            }
            #[cfg(feature = "ryo3-std")]
            PyObType::PyIpv4Addr => {
                $map.serialize_value(&rytypes::PyIpv4AddrSerializer::new_unchecked($value))?;
            }
            #[cfg(feature = "ryo3-std")]
            PyObType::PyIpv6Addr => {
                $map.serialize_value(&rytypes::PyIpv6AddrSerializer::new_unchecked($value))?;
            }
            #[cfg(feature = "ryo3-std")]
            PyObType::PySocketAddr => {
                $map.serialize_value(&rytypes::PySocketAddrSerializer::new_unchecked($value))?;
            }
            #[cfg(feature = "ryo3-std")]
            PyObType::PySocketAddrV4 => {
                $map.serialize_value(&rytypes::PySocketAddrV4Serializer::new_unchecked($value))?;
            }
            #[cfg(feature = "ryo3-std")]
            PyObType::PySocketAddrV6 => {
                $map.serialize_value(&rytypes::PySocketAddrV6Serializer::new_unchecked($value))?;
            }

            // __HTTP__
            #[cfg(feature = "ryo3-http")]
            PyObType::RyHeaders => {
                $map.serialize_value(&rytypes::PyHeadersSerializer::new_unchecked($value))?;
            }
            #[cfg(feature = "ryo3-http")]
            PyObType::RyHttpStatus => {
                $map.serialize_value(&rytypes::PyHttpStatusSerializer::new_unchecked($value))?;
            }
            // __JIFF__
            #[cfg(feature = "ryo3-jiff")]
            PyObType::RyDate => {
                $map.serialize_value(&rytypes::RyDateSerializer::new_unchecked($value))?;
            }
            #[cfg(feature = "ryo3-jiff")]
            PyObType::RyDateTime => {
                $map.serialize_value(&rytypes::RyDateTimeSerializer::new_unchecked($value))?;
            }
            #[cfg(feature = "ryo3-jiff")]
            PyObType::RySignedDuration => {
                $map.serialize_value(&rytypes::RySignedDurationSerializer::new_unchecked($value))?;
            }
            #[cfg(feature = "ryo3-jiff")]
            PyObType::RyTime => {
                $map.serialize_value(&rytypes::RyTimeSerializer::new_unchecked($value))?;
            }
            #[cfg(feature = "ryo3-jiff")]
            PyObType::RyTimeSpan => {
                $map.serialize_value(&rytypes::RySpanSerializer::new_unchecked($value))?;
            }
            #[cfg(feature = "ryo3-jiff")]
            PyObType::RyTimestamp => {
                $map.serialize_value(&rytypes::RyTimestampSerializer::new_unchecked($value))?;
            }
            #[cfg(feature = "ryo3-jiff")]
            PyObType::RyTimeZone => {
                $map.serialize_value(&rytypes::RyTimeZoneSerializer::new_unchecked($value))?;
            }
            #[cfg(feature = "ryo3-jiff")]
            PyObType::RyZoned => {
                $map.serialize_value(&rytypes::RyZonedSerializer::new_unchecked($value))?;
            }
            // __ULID__
            #[cfg(feature = "ryo3-ulid")]
            PyObType::RyUlid => {
                $map.serialize_value(&rytypes::PyUlidSerializer::new_unchecked($value))?;
            }
            // __URL__
            #[cfg(feature = "ryo3-url")]
            PyObType::RyUrl => {
                $map.serialize_value(&rytypes::PyUrlSerializer::new_unchecked($value))?;
            }
            // __UUID__
            #[cfg(feature = "ryo3-uuid")]
            PyObType::RyUuid => {
                $map.serialize_value(&rytypes::PyUuidSerializer::new_unchecked($value))?;
            }
            // ------------------------------------------------------------
            // UNKNOWN
            // ------------------------------------------------------------
            PyObType::Unknown => {
                $map.serialize_value(&PyUnknownSerializer::new_with_depth(
                    $value,
                    $self.ctx,
                    $self.depth + 1,
                ))?;
            }
        }
    };
}

impl Serialize for PyDictSerializer<'_, '_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if self.depth == MAX_DEPTH {
            return serde_err_recursion!();
        }
        let py_dict = self.obj;
        let len = py_dict.len();
        if len == 0 {
            return serializer.serialize_map(Some(0))?.end();
        }
        let mut m = serializer.serialize_map(None)?;
        for (map_key, map_val) in py_dict.iter() {
            let map_key = map_key.as_borrowed();
            let map_val = map_val.as_borrowed();
            let sk = PyMappingKeySerializer::new(self.ctx, map_key);
            // let sv = PyAnySerializer::new_with_depth(&v, self.ctx, self.depth + 1);
            let ob_type = self.ctx.typeref.obtype(map_val);
            m.serialize_key(&sk)?;
            serialize_map_value!(ob_type, m, self, map_val);
        }
        m.end()
    }
}

// pub(crate) use serialize_map_value;
pub(crate) struct PyMappingSerializer<'a, 'py> {
    ctx: PySerializeContext<'py>,
    obj: Borrowed<'a, 'py, PyMapping>,
    depth: Depth,
}

impl<'a, 'py> PyMappingSerializer<'a, 'py> {
    pub(crate) fn new_with_depth(
        obj: Borrowed<'a, 'py, PyMapping>,
        ctx: PySerializeContext<'py>,
        depth: Depth,
    ) -> Self {
        Self { ctx, obj, depth }
    }
}

impl Serialize for PyMappingSerializer<'_, '_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let py_mapping = self.obj;
        let len = py_mapping.len().ok();
        if let Some(len) = len
            && len == 0
        {
            return serializer.serialize_map(Some(0))?.end();
        }
        let mut m = serializer.serialize_map(len)?;
        let keys = py_mapping.keys().map_err(pyerr2sererr)?;
        let values = py_mapping.values().map_err(pyerr2sererr)?;
        for (k, v) in keys.iter().zip(values.iter()) {
            let k = k.as_borrowed();
            let v = v.as_borrowed();
            let sk = PyMappingKeySerializer::new(self.ctx, k);
            let ob_type = self.ctx.typeref.obtype(v);
            m.serialize_key(&sk)?;
            serialize_map_value!(ob_type, m, self, v);
        }
        m.end()
    }
}
