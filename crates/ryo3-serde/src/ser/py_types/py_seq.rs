use pyo3::prelude::*;
use pyo3::types::{PyAny, PyFrozenSet, PyList, PySequence, PySet, PyTuple};
use serde::ser::{Serialize, SerializeSeq, SerializeTuple, Serializer};

use crate::constants::{Depth, MAX_DEPTH};
use crate::errors::pyerr2sererr;
use crate::ob_type::PyObType;
use crate::ser::py_types::{
    PyBoolSerializer, PyBytesLikeSerializer, PyDateSerializer, PyDateTimeSerializer,
    PyDictSerializer, PyFloatSerializer, PyIntSerializer, PyNoneSerializer, PyStrSerializer,
    PyTimeDeltaSerializer, PyTimeSerializer, PyUnknownSerializer, PyUuidSerializer,
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

macro_rules! serialize_seq_element {
    ($ob_type:expr, $seq:expr, $self:expr, $element:expr) => {
        match $ob_type {
            PyObType::None | PyObType::Ellipsis => {
                $seq.serialize_element(&PyNoneSerializer::new())?;
            }
            PyObType::Bool => {
                $seq.serialize_element(&PyBoolSerializer::new_unchecked($element))?;
            }
            PyObType::Int => {
                $seq.serialize_element(&PyIntSerializer::new_unchecked($element))?;
            }
            PyObType::Float => {
                $seq.serialize_element(&PyFloatSerializer::new_unchecked($element))?;
            }
            PyObType::String => {
                $seq.serialize_element(&PyStrSerializer::new_unchecked($element))?;
            }
            PyObType::List => {
                $seq.serialize_element(&PyListSerializer::new_unchecked(
                    $element,
                    $self.ctx,
                    $self.depth + 1,
                ))?;
            }
            PyObType::Tuple => {
                $seq.serialize_element(&PyTupleSerializer::new_unchecked(
                    $element,
                    $self.ctx,
                    $self.depth + 1,
                ))?;
            }
            PyObType::Dict => {
                $seq.serialize_element(&PyDictSerializer::new_unchecked(
                    $element,
                    $self.ctx,
                    $self.depth + 1,
                ))?;
            }
            PyObType::Set => {
                $seq.serialize_element(&PySetSerializer::new_unchecked(
                    $element,
                    $self.ctx,
                    $self.depth + 1,
                ))?;
            }
            PyObType::FrozenSet => {
                $seq.serialize_element(&PyFrozenSetSerializer::new_unchecked(
                    $element,
                    $self.ctx,
                    $self.depth + 1,
                ))?;
            }
            PyObType::DateTime => {
                $seq.serialize_element(&PyDateTimeSerializer::new_unchecked($element))?;
            }
            PyObType::Date => {
                $seq.serialize_element(&PyDateSerializer::new_unchecked($element))?;
            }
            PyObType::Time => {
                $seq.serialize_element(&PyTimeSerializer::new_unchecked($element))?;
            }
            PyObType::Timedelta => {
                $seq.serialize_element(&PyTimeDeltaSerializer::new_unchecked($element))?;
            }
            PyObType::Bytes | PyObType::ByteArray | PyObType::MemoryView => {
                $seq.serialize_element(&PyBytesLikeSerializer::new($element))?;
            }
            PyObType::PyUuid => {
                $seq.serialize_element(&PyUuidSerializer::new($element))?;
            }
            // ------------------------------------------------------------
            // RY-TYPES
            // ------------------------------------------------------------
            // __STD__
            #[cfg(feature = "ryo3-std")]
            PyObType::PyDuration => {
                $seq.serialize_element(&ry_types::PyDurationSerializer::new_unchecked($element))?;
            }

            #[cfg(feature = "ryo3-std")]
            PyObType::PyIpAddr => {
                $seq.serialize_element(&ry_types::PyIpAddrSerializer::new_unchecked($element))?;
            }
            #[cfg(feature = "ryo3-std")]
            PyObType::PyIpv4Addr => {
                $seq.serialize_element(&ry_types::PyIpv4AddrSerializer::new_unchecked($element))?;
            }
            #[cfg(feature = "ryo3-std")]
            PyObType::PyIpv6Addr => {
                $seq.serialize_element(&ry_types::PyIpv6AddrSerializer::new_unchecked($element))?;
            }
            #[cfg(feature = "ryo3-std")]
            PyObType::PySocketAddr => {
                $seq.serialize_element(&ry_types::PySocketAddrSerializer::new_unchecked($element))?;
            }
            #[cfg(feature = "ryo3-std")]
            PyObType::PySocketAddrV4 => {
                $seq.serialize_element(&ry_types::PySocketAddrV4Serializer::new_unchecked(
                    $element,
                ))?;
            }
            #[cfg(feature = "ryo3-std")]
            PyObType::PySocketAddrV6 => {
                $seq.serialize_element(&ry_types::PySocketAddrV6Serializer::new_unchecked(
                    $element,
                ))?;
            }

            // __HTTP__
            #[cfg(feature = "ryo3-http")]
            PyObType::RyHeaders => {
                $seq.serialize_element(&ry_types::PyHeadersSerializer::new_unchecked($element))?;
            }
            #[cfg(feature = "ryo3-http")]
            PyObType::RyHttpStatus => {
                $seq.serialize_element(&ry_types::PyHttpStatusSerializer::new_unchecked($element))?;
            }
            // __JIFF__
            #[cfg(feature = "ryo3-jiff")]
            PyObType::RyDate => {
                $seq.serialize_element(&ry_types::RyDateSerializer::new_unchecked($element))?;
            }
            #[cfg(feature = "ryo3-jiff")]
            PyObType::RyDateTime => {
                $seq.serialize_element(&ry_types::RyDateTimeSerializer::new_unchecked($element))?;
            }
            #[cfg(feature = "ryo3-jiff")]
            PyObType::RySignedDuration => {
                $seq.serialize_element(&ry_types::RySignedDurationSerializer::new_unchecked(
                    $element,
                ))?;
            }
            #[cfg(feature = "ryo3-jiff")]
            PyObType::RyTime => {
                $seq.serialize_element(&ry_types::RyTimeSerializer::new_unchecked($element))?;
            }
            #[cfg(feature = "ryo3-jiff")]
            PyObType::RyTimeSpan => {
                $seq.serialize_element(&ry_types::RySpanSerializer::new_unchecked($element))?;
            }
            #[cfg(feature = "ryo3-jiff")]
            PyObType::RyTimestamp => {
                $seq.serialize_element(&ry_types::RyTimestampSerializer::new_unchecked($element))?;
            }
            #[cfg(feature = "ryo3-jiff")]
            PyObType::RyTimeZone => {
                $seq.serialize_element(&ry_types::RyTimeZoneSerializer::new_unchecked($element))?;
            }
            #[cfg(feature = "ryo3-jiff")]
            PyObType::RyZoned => {
                $seq.serialize_element(&ry_types::RyZonedSerializer::new_unchecked($element))?;
            }
            // __ULID__
            #[cfg(feature = "ryo3-ulid")]
            PyObType::RyUlid => {
                $seq.serialize_element(&ry_types::PyUlidSerializer::new_unchecked($element))?;
            }
            // __URL__
            #[cfg(feature = "ryo3-url")]
            PyObType::RyUrl => {
                $seq.serialize_element(&ry_types::PyUrlSerializer::new_unchecked($element))?;
            }
            // __UUID__
            #[cfg(feature = "ryo3-uuid")]
            PyObType::RyUuid => {
                $seq.serialize_element(&ry_types::PyUuidSerializer::new_unchecked($element))?;
            }
            // ------------------------------------------------------------
            // UNKNOWN
            // ------------------------------------------------------------
            PyObType::Unknown => {
                $seq.serialize_element(&PyUnknownSerializer::new(
                    $element,
                    $self.ctx,
                    $self.depth + 1,
                ))?;
            }
        }
    };
}

// ----------------------------------------------------------------------------
// PyList
// ----------------------------------------------------------------------------
pub(crate) struct PyListSerializer<'a, 'py, T: SerializeTarget> {
    pub(crate) ctx: PySerializeContext<'py, T>,
    pub(crate) obj: Borrowed<'a, 'py, PyList>,
    pub(crate) depth: Depth,
}

impl<'a, 'py, T: SerializeTarget> PyListSerializer<'a, 'py, T> {
    #[inline]
    pub(crate) fn new(
        obj: Borrowed<'a, 'py, PyList>,
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
        let obj = unsafe { obj.cast_unchecked::<PyList>() };
        Self::new(obj, ctx, depth)
    }
}

impl<T: SerializeTarget> Serialize for PyListSerializer<'_, '_, T> {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if self.depth == MAX_DEPTH {
            return serde_err_recursion!();
        }
        let len = self.obj.len();
        if len == 0 {
            return serializer.serialize_seq(Some(0))?.end();
        }

        // MAIN THINGY
        let mut prev_ob_type_ptr = 0;
        let mut prev_ob_type = PyObType::Unknown;
        let mut seq = serializer.serialize_seq(Some(len))?;
        for element in self.obj.iter() {
            let element = element.as_borrowed();
            let type_ptr = element.get_type_ptr() as usize;
            let ob_type = if type_ptr == prev_ob_type_ptr {
                prev_ob_type
            } else {
                let t = self.ctx.typeref.ptr2type(type_ptr);
                prev_ob_type_ptr = type_ptr;
                prev_ob_type = t;
                t
            };
            serialize_seq_element!(ob_type, seq, self, element);
        }
        seq.end()
    }
}

// ----------------------------------------------------------------------------
// PyTuple
// ----------------------------------------------------------------------------
pub(crate) struct PyTupleSerializer<'a, 'py, T: SerializeTarget> {
    pub(crate) obj: Borrowed<'a, 'py, PyTuple>,
    pub(crate) ctx: PySerializeContext<'py, T>,
    pub(crate) depth: Depth,
}

impl<'a, 'py, T: SerializeTarget> PyTupleSerializer<'a, 'py, T> {
    #[inline]
    pub(crate) fn new(
        obj: Borrowed<'a, 'py, PyTuple>,
        ctx: PySerializeContext<'py, T>,
        depth: Depth,
    ) -> Self {
        Self { obj, ctx, depth }
    }

    #[inline]
    pub(crate) fn new_unchecked(
        obj: Borrowed<'a, 'py, PyAny>,
        ctx: PySerializeContext<'py, T>,
        depth: Depth,
    ) -> Self {
        #[expect(unsafe_code)]
        let obj = unsafe { obj.cast_unchecked::<PyTuple>() };
        Self::new(obj, ctx, depth)
    }
}
impl<T: SerializeTarget> Serialize for PyTupleSerializer<'_, '_, T> {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if self.depth == MAX_DEPTH {
            return serde_err_recursion!();
        }
        let len = self.obj.len();
        if len == 0 {
            return serializer.serialize_seq(Some(0))?.end();
        }

        let mut tup = serializer.serialize_tuple(len)?;
        let mut prev_ob_type_ptr = 0;
        let mut prev_ob_type = PyObType::Unknown;

        for element in self.obj.iter_borrowed() {
            let type_ptr = element.get_type_ptr() as usize;
            if type_ptr != prev_ob_type_ptr {
                prev_ob_type_ptr = type_ptr;
                prev_ob_type = self.ctx.typeref.ptr2type(type_ptr);
            }
            serialize_seq_element!(prev_ob_type, tup, self, element);
        }
        tup.end()
    }
}

// ----------------------------------------------------------------------------
// PySet
// ----------------------------------------------------------------------------
pub(crate) struct PySetSerializer<'a, 'py, T: SerializeTarget> {
    pub(crate) ctx: PySerializeContext<'py, T>,
    pub(crate) obj: Borrowed<'a, 'py, PySet>,
    pub(crate) depth: Depth,
}

impl<'a, 'py, T: SerializeTarget> PySetSerializer<'a, 'py, T> {
    #[inline]
    pub(crate) fn new(
        obj: Borrowed<'a, 'py, PySet>,
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
        let obj = unsafe { obj.cast_unchecked::<PySet>() };
        Self::new(obj, ctx, depth)
    }
}

impl<T: SerializeTarget> Serialize for PySetSerializer<'_, '_, T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let len = self.obj.len();
        if len == 0 {
            return serializer.serialize_seq(Some(0))?.end();
        }
        let mut seq = serializer.serialize_seq(Some(len))?;
        for element in self.obj.iter() {
            let element = element.as_borrowed();
            let ob_type = self.ctx.typeref.obtype(element);
            serialize_seq_element!(ob_type, seq, self, element);
        }
        seq.end()
    }
}

// ----------------------------------------------------------------------------
// PyFrozenSet
// ----------------------------------------------------------------------------
pub(crate) struct PyFrozenSetSerializer<'a, 'py, T: SerializeTarget> {
    pub(crate) ctx: PySerializeContext<'py, T>,
    pub(crate) obj: Borrowed<'a, 'py, PyFrozenSet>,
    pub(crate) depth: Depth,
}

impl<'a, 'py, T: SerializeTarget> PyFrozenSetSerializer<'a, 'py, T> {
    #[inline]
    pub(crate) fn new(
        obj: Borrowed<'a, 'py, PyFrozenSet>,
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
        let obj = unsafe { obj.cast_unchecked::<PyFrozenSet>() };
        Self::new(obj, ctx, depth)
    }
}

impl<T: SerializeTarget> Serialize for PyFrozenSetSerializer<'_, '_, T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let len = self.obj.len();
        if len == 0 {
            return serializer.serialize_seq(Some(0))?.end();
        }
        let mut seq = serializer.serialize_seq(Some(len))?;
        for element in self.obj.iter() {
            let element = element.as_borrowed();
            let ob_type = self.ctx.typeref.obtype(element);
            serialize_seq_element!(ob_type, seq, self, element);
        }
        seq.end()
    }
}

// ----------------------------------------------------------------------------
// PySequence
// ----------------------------------------------------------------------------

pub(crate) struct PySequenceSerializer<'a, 'py, T: SerializeTarget> {
    ctx: PySerializeContext<'py, T>,
    obj: Borrowed<'a, 'py, PySequence>,
    depth: Depth,
}

impl<'a, 'py, T: SerializeTarget> PySequenceSerializer<'a, 'py, T> {
    pub(crate) fn new_with_depth(
        obj: Borrowed<'a, 'py, PySequence>,
        ctx: PySerializeContext<'py, T>,
        depth: Depth,
    ) -> Self {
        Self { ctx, obj, depth }
    }
}

impl<T: SerializeTarget> Serialize for PySequenceSerializer<'_, '_, T> {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let len = self.obj.len().map_err(pyerr2sererr)?;
        let mut seq = serializer.serialize_seq(Some(len))?;
        for i in 0..len {
            let bound_pyany = self.obj.get_item(i).map_err(pyerr2sererr)?;
            let borrowed_pyany = bound_pyany.as_borrowed();
            let ob_type = self.ctx.typeref.obtype(borrowed_pyany);
            serialize_seq_element!(ob_type, seq, self, borrowed_pyany);
        }
        seq.end()
    }
}
