use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList, PyMapping};
use serde::ser::{Serialize, SerializeMap, Serializer};

use crate::constants::{Depth, MAX_DEPTH};
use crate::errors::pyerr2sererr;
use crate::ob_type::PyObType;
use crate::ser::json::json_map_key_str;
use crate::ser::py_types::{
    PyBoolSerializer, PyBytesLikeSerializer, PyDateSerializer, PyDateTimeSerializer,
    PyFloatSerializer, PyFrozenSetSerializer, PyIntSerializer, PyListSerializer,
    PyMappingKeySerializer, PyNoneSerializer, PySetSerializer, PyStrSerializer,
    PyTimeDeltaSerializer, PyTimeSerializer, PyTupleSerializer, PyUnknownSerializer,
    PyUuidSerializer,
};
#[cfg(any(
    feature = "ryo3-http",
    feature = "ryo3-jiff",
    feature = "ryo3-ulid",
    feature = "ryo3-url",
    feature = "ryo3-uuid",
    feature = "ryo3-std"
))]
use crate::ser::ry_types;
use crate::ser::{PySerializeContext, SerializeTarget};
use crate::serde_err_recursion;

pub(crate) struct PyDictSerializer<'a, 'py, T: SerializeTarget> {
    ctx: PySerializeContext<'py, T>,
    pub(crate) obj: Borrowed<'a, 'py, PyDict>,
    pub(crate) depth: Depth,
}

impl<'a, 'py, T: SerializeTarget> PyDictSerializer<'a, 'py, T> {
    #[inline]
    pub(crate) fn new(
        obj: Borrowed<'a, 'py, PyDict>,
        ctx: PySerializeContext<'py, T>,
        depth: Depth,
    ) -> Self {
        Self { ctx, obj, depth }
    }

    #[inline]
    pub(crate) fn new_unchecked(
        obj: Borrowed<'a, 'py, PyAny>,
        ctx: PySerializeContext<'py, T>,
        depth: Depth,
    ) -> Self {
        #[expect(unsafe_code)]
        let obj = unsafe { obj.cast_unchecked::<PyDict>() };
        Self::new(obj, ctx, depth)
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
                $map.serialize_value(&PySetSerializer::new_unchecked(
                    $value,
                    $self.ctx,
                    $self.depth + 1,
                ))?;
            }
            PyObType::FrozenSet => {
                $map.serialize_value(&PyFrozenSetSerializer::new_unchecked(
                    $value,
                    $self.ctx,
                    $self.depth + 1,
                ))?;
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
            // ------------------------------------------------------------
            // RY-TYPES
            // ------------------------------------------------------------
            // __STD__
            #[cfg(feature = "ryo3-std")]
            PyObType::PyDuration => {
                $map.serialize_value(&ry_types::PyDurationSerializer::new_unchecked($value))?;
            }

            #[cfg(feature = "ryo3-std")]
            PyObType::PyIpAddr => {
                $map.serialize_value(&ry_types::PyIpAddrSerializer::new_unchecked($value))?;
            }
            #[cfg(feature = "ryo3-std")]
            PyObType::PyIpv4Addr => {
                $map.serialize_value(&ry_types::PyIpv4AddrSerializer::new_unchecked($value))?;
            }
            #[cfg(feature = "ryo3-std")]
            PyObType::PyIpv6Addr => {
                $map.serialize_value(&ry_types::PyIpv6AddrSerializer::new_unchecked($value))?;
            }
            #[cfg(feature = "ryo3-std")]
            PyObType::PySocketAddr => {
                $map.serialize_value(&ry_types::PySocketAddrSerializer::new_unchecked($value))?;
            }
            #[cfg(feature = "ryo3-std")]
            PyObType::PySocketAddrV4 => {
                $map.serialize_value(&ry_types::PySocketAddrV4Serializer::new_unchecked($value))?;
            }
            #[cfg(feature = "ryo3-std")]
            PyObType::PySocketAddrV6 => {
                $map.serialize_value(&ry_types::PySocketAddrV6Serializer::new_unchecked($value))?;
            }

            // __HTTP__
            #[cfg(feature = "ryo3-http")]
            PyObType::RyHeaders => {
                $map.serialize_value(&ry_types::PyHeadersSerializer::new_unchecked($value))?;
            }
            #[cfg(feature = "ryo3-http")]
            PyObType::RyHttpStatus => {
                $map.serialize_value(&ry_types::PyHttpStatusSerializer::new_unchecked($value))?;
            }
            // __JIFF__
            #[cfg(feature = "ryo3-jiff")]
            PyObType::RyDate => {
                $map.serialize_value(&ry_types::RyDateSerializer::new_unchecked($value))?;
            }
            #[cfg(feature = "ryo3-jiff")]
            PyObType::RyDateTime => {
                $map.serialize_value(&ry_types::RyDateTimeSerializer::new_unchecked($value))?;
            }
            #[cfg(feature = "ryo3-jiff")]
            PyObType::RySignedDuration => {
                $map.serialize_value(&ry_types::RySignedDurationSerializer::new_unchecked($value))?;
            }
            #[cfg(feature = "ryo3-jiff")]
            PyObType::RyTime => {
                $map.serialize_value(&ry_types::RyTimeSerializer::new_unchecked($value))?;
            }
            #[cfg(feature = "ryo3-jiff")]
            PyObType::RyTimeSpan => {
                $map.serialize_value(&ry_types::RySpanSerializer::new_unchecked($value))?;
            }
            #[cfg(feature = "ryo3-jiff")]
            PyObType::RyTimestamp => {
                $map.serialize_value(&ry_types::RyTimestampSerializer::new_unchecked($value))?;
            }
            #[cfg(feature = "ryo3-jiff")]
            PyObType::RyTimeZone => {
                $map.serialize_value(&ry_types::RyTimeZoneSerializer::new_unchecked($value))?;
            }
            #[cfg(feature = "ryo3-jiff")]
            PyObType::RyZoned => {
                $map.serialize_value(&ry_types::RyZonedSerializer::new_unchecked($value))?;
            }
            // __ULID__
            #[cfg(feature = "ryo3-ulid")]
            PyObType::RyUlid => {
                $map.serialize_value(&ry_types::PyUlidSerializer::new_unchecked($value))?;
            }
            // __URL__
            #[cfg(feature = "ryo3-url")]
            PyObType::RyUrl => {
                $map.serialize_value(&ry_types::PyUrlSerializer::new_unchecked($value))?;
            }
            // __UUID__
            #[cfg(feature = "ryo3-uuid")]
            PyObType::RyUuid => {
                $map.serialize_value(&ry_types::PyUuidSerializer::new_unchecked($value))?;
            }
            // ------------------------------------------------------------
            // UNKNOWN
            // ------------------------------------------------------------
            PyObType::Unknown => {
                $map.serialize_value(&PyUnknownSerializer::new(
                    $value,
                    $self.ctx,
                    $self.depth + 1,
                ))?;
            }
        }
    };
}

// ~ ~ ~ NO BORROWED ITER ~ ~ ~
// ~ ~ ~ NO BORROWED ITER ~ ~ ~
// ~ ~ ~ NO BORROWED ITER ~ ~ ~
// impl Serialize for PyDictSerializer<'_, '_> {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         if self.depth == MAX_DEPTH {
//             return serde_err_recursion!();
//         }
//         let len = self.obj.len();
//         if len == 0 {
//             return serializer.serialize_map(Some(0))?.end();
//         }
//         let mut m = serializer.serialize_map(None)?;
//         let mut prev_ob_type_ptr = 0;
//         let mut prev_ob_type = PyObType::Unknown;
//         for (map_key, map_val) in self.obj.iter() {
//             let map_key = map_key.as_borrowed();
//             let map_val = map_val.as_borrowed();
//             let sk = PyMappingKeySerializer::new(self.ctx, map_key);
//             let type_ptr = map_val.get_type_ptr() as usize;
//             let ob_type = if type_ptr == prev_ob_type_ptr {
//                 prev_ob_type
//             } else {
//                 let t = self.ctx.typeref.ptr2type(type_ptr);
//                 prev_ob_type_ptr = type_ptr;
//                 prev_ob_type = t;
//                 t
//             };
//             m.serialize_key(&sk)?;
//             serialize_map_value!(ob_type, m, self, map_val);
//         }
//         m.end()
//     }
// }

impl<T: SerializeTarget> Serialize for PyDictSerializer<'_, '_, T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if self.depth == MAX_DEPTH {
            return serde_err_recursion!();
        }
        let len = self.obj.len();
        if len == 0 {
            return serializer.serialize_map(Some(0))?.end();
        }

        if T::SORT_KEYS {
            return PyJsonSortedDictSerializer::new(self.obj, self.ctx, self.depth, len)
                .serialize(serializer);
        }

        let mut m = serializer.serialize_map(None)?;
        let mut prev_val_ob_type_ptr = 0;
        let mut prev_val_ob_type = PyObType::Unknown;

        for (map_key, map_val) in ryo3_core::py_dict::BorrowedDictIter::new(self.obj) {
            let type_ptr = map_val.get_type_ptr() as usize;
            if type_ptr != prev_val_ob_type_ptr {
                prev_val_ob_type_ptr = type_ptr;
                prev_val_ob_type = self.ctx.typeref.ptr2type(type_ptr);
            }
            let sk = PyMappingKeySerializer::new(self.ctx, map_key);
            m.serialize_key(&sk)?;
            serialize_map_value!(prev_val_ob_type, m, self, map_val);
        }
        m.end()
    }
}

struct PyJsonSortedDictSerializer<'a, 'py, T: SerializeTarget> {
    ctx: PySerializeContext<'py, T>,
    obj: Borrowed<'a, 'py, PyDict>,
    depth: Depth,
    length: usize,
}

impl<'a, 'py, T: SerializeTarget> PyJsonSortedDictSerializer<'a, 'py, T> {
    fn new(
        obj: Borrowed<'a, 'py, PyDict>,
        ctx: PySerializeContext<'py, T>,
        depth: Depth,
        length: usize,
    ) -> Self {
        Self {
            ctx,
            obj,
            depth,
            length,
        }
    }
}

impl<T: SerializeTarget> Serialize for PyJsonSortedDictSerializer<'_, '_, T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut entries = Vec::with_capacity(self.length);

        for (map_key, map_val) in ryo3_core::py_dict::BorrowedDictIter::new(self.obj) {
            entries.push((json_map_key_str::<S::Error, T>(self.ctx, map_key)?, map_val));
        }

        entries.sort_by_key(|(key, _)| *key);

        let mut m = serializer.serialize_map(Some(entries.len()))?;
        let mut prev_val_ob_type_ptr = 0;
        let mut prev_val_ob_type = PyObType::Unknown;

        for (map_key, map_val) in entries {
            let type_ptr = map_val.get_type_ptr() as usize;
            if type_ptr != prev_val_ob_type_ptr {
                prev_val_ob_type_ptr = type_ptr;
                prev_val_ob_type = self.ctx.typeref.ptr2type(type_ptr);
            }
            m.serialize_key(map_key)?;
            serialize_map_value!(prev_val_ob_type, m, self, map_val);
        }
        m.end()
    }
}

// pub(crate) use serialize_map_value;
pub(crate) struct PyMappingSerializer<'a, 'py, T: SerializeTarget> {
    ctx: PySerializeContext<'py, T>,
    obj: Borrowed<'a, 'py, PyMapping>,
    depth: Depth,
}

impl<'a, 'py, T: SerializeTarget> PyMappingSerializer<'a, 'py, T> {
    pub(crate) fn new_with_depth(
        obj: Borrowed<'a, 'py, PyMapping>,
        ctx: PySerializeContext<'py, T>,
        depth: Depth,
    ) -> Self {
        Self { ctx, obj, depth }
    }
}

impl<T: SerializeTarget> Serialize for PyMappingSerializer<'_, '_, T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let py_mapping = self.obj.cast::<PyMapping>().map_err(pyerr2sererr)?;
        let len = py_mapping.len().ok();
        if let Some(len) = len
            && len == 0
        {
            return serializer.serialize_map(Some(0))?.end();
        }
        let keys = py_mapping.keys().map_err(pyerr2sererr)?;
        let values = py_mapping.values().map_err(pyerr2sererr)?;
        if T::SORT_KEYS {
            return PyJsonSortedMappingSerializer::new(keys, values, self.ctx, self.depth, len)
                .serialize(serializer);
        }

        let mut m = serializer.serialize_map(len)?;
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

struct PyJsonSortedMappingSerializer<'py, T: SerializeTarget> {
    ctx: PySerializeContext<'py, T>,
    keys: Bound<'py, PyList>,
    values: Bound<'py, PyList>,
    depth: Depth,
    len: Option<usize>,
}

impl<'py, T: SerializeTarget> PyJsonSortedMappingSerializer<'py, T> {
    fn new(
        keys: Bound<'py, PyList>,
        values: Bound<'py, PyList>,
        ctx: PySerializeContext<'py, T>,
        depth: Depth,
        len: Option<usize>,
    ) -> Self {
        Self {
            ctx,
            keys,
            values,
            depth,
            len,
        }
    }
}

impl<T: SerializeTarget> Serialize for PyJsonSortedMappingSerializer<'_, T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut entries = Vec::with_capacity(self.len.unwrap_or(0));
        for (k, v) in self.keys.iter().zip(self.values.iter()) {
            json_map_key_str::<S::Error, T>(self.ctx, k.as_borrowed())?;
            entries.push((k, v));
        }
        entries.sort_by(|(a, _), (b, _)| {
            let a =
                json_map_key_str::<S::Error, T>(self.ctx, a.as_borrowed()).expect("JSON map key");
            let b =
                json_map_key_str::<S::Error, T>(self.ctx, b.as_borrowed()).expect("JSON map key");
            a.cmp(b)
        });

        let mut m = serializer.serialize_map(self.len)?;
        for (k, v) in entries {
            let k = json_map_key_str::<S::Error, T>(self.ctx, k.as_borrowed())?;
            let v = v.as_borrowed();
            let ob_type = self.ctx.typeref.obtype(v);
            m.serialize_key(k)?;
            serialize_map_value!(ob_type, m, self, v);
        }
        m.end()
    }
}
