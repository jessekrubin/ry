use crate::PyAnySerializer;
use crate::constants::{Depth, MAX_DEPTH};
use crate::errors::pyerr2sererr;
use crate::ob_type::PyObType;
use crate::ser::PySerializeContext;
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
    PyDateTimeSerializer, PyDictSerializer, PyFloatSerializer, PyIntSerializer, PyNoneSerializer,
    PyStrSerializer, PyTimeDeltaSerializer, PyTimeSerializer, PyUuidSerializer,
};
use crate::serde_err_recursion;
use pyo3::prelude::*;
use pyo3::types::{PyAny, PyFrozenSet, PyList, PySequence, PySet, PyTuple};
use serde::ser::{Serialize, SerializeSeq, SerializeTuple, Serializer};

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
                $seq.serialize_element(&PySetSerializer::new_unchecked($element, $self.ctx))?;
            }
            PyObType::FrozenSet => {
                $seq.serialize_element(&PyFrozenSetSerializer::new_unchecked($element, $self.ctx))?;
            }
            PyObType::DateTime => {
                $seq.serialize_element(&PyDateTimeSerializer::new($element))?;
            }
            PyObType::Date => {
                $seq.serialize_element(&PyDateSerializer::new($element))?;
            }
            PyObType::Time => {
                $seq.serialize_element(&PyTimeSerializer::new($element))?;
            }
            PyObType::Timedelta => {
                $seq.serialize_element(&PyTimeDeltaSerializer::new($element))?;
            }
            PyObType::Bytes | PyObType::ByteArray | PyObType::MemoryView => {
                $seq.serialize_element(&PyBytesLikeSerializer::new($element))?;
            }
            PyObType::PyUuid => {
                $seq.serialize_element(&PyUuidSerializer::new($element))?;
            }
            PyObType::Dataclass => {
                $seq.serialize_element(&PyDataclassSerializer::new(
                    $element,
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
                $seq.serialize_element(&rytypes::PyDurationSerializer::new($element))?;
            }

            #[cfg(feature = "ryo3-std")]
            PyObType::PyIpAddr => {
                $seq.serialize_element(&rytypes::PyIpAddrSerializer::new($element))?;
            }
            #[cfg(feature = "ryo3-std")]
            PyObType::PyIpv4Addr => {
                $seq.serialize_element(&rytypes::PyIpv4AddrSerializer::new($element))?;
            }
            #[cfg(feature = "ryo3-std")]
            PyObType::PyIpv6Addr => {
                $seq.serialize_element(&rytypes::PyIpv6AddrSerializer::new($element))?;
            }
            #[cfg(feature = "ryo3-std")]
            PyObType::PySocketAddr => {
                $seq.serialize_element(&rytypes::PySocketAddrSerializer::new($element))?;
            }
            #[cfg(feature = "ryo3-std")]
            PyObType::PySocketAddrV4 => {
                $seq.serialize_element(&rytypes::PySocketAddrV4Serializer::new($element))?;
            }
            #[cfg(feature = "ryo3-std")]
            PyObType::PySocketAddrV6 => {
                $seq.serialize_element(&rytypes::PySocketAddrV6Serializer::new($element))?;
            }

            // __HTTP__
            #[cfg(feature = "ryo3-http")]
            PyObType::RyHeaders => {
                $seq.serialize_element(&rytypes::PyHeadersSerializer::new($element))?;
            }
            #[cfg(feature = "ryo3-http")]
            PyObType::RyHttpStatus => {
                $seq.serialize_element(&rytypes::PyHttpStatusSerializer::new($element))?;
            }
            // __JIFF__
            #[cfg(feature = "ryo3-jiff")]
            PyObType::RyDate => {
                $seq.serialize_element(&rytypes::RyDateSerializer::new($element))?;
            }
            #[cfg(feature = "ryo3-jiff")]
            PyObType::RyDateTime => {
                $seq.serialize_element(&rytypes::RyDateTimeSerializer::new($element))?;
            }
            #[cfg(feature = "ryo3-jiff")]
            PyObType::RySignedDuration => {
                $seq.serialize_element(&rytypes::RySignedDurationSerializer::new($element))?;
            }
            #[cfg(feature = "ryo3-jiff")]
            PyObType::RyTime => {
                $seq.serialize_element(&rytypes::RyTimeSerializer::new($element))?;
            }
            #[cfg(feature = "ryo3-jiff")]
            PyObType::RyTimeSpan => {
                $seq.serialize_element(&rytypes::RySpanSerializer::new($element))?;
            }
            #[cfg(feature = "ryo3-jiff")]
            PyObType::RyTimestamp => {
                $seq.serialize_element(&rytypes::RyTimestampSerializer::new($element))?;
            }
            #[cfg(feature = "ryo3-jiff")]
            PyObType::RyTimeZone => {
                $seq.serialize_element(&rytypes::RyTimeZoneSerializer::new($element))?;
            }
            #[cfg(feature = "ryo3-jiff")]
            PyObType::RyZoned => {
                $seq.serialize_element(&rytypes::RyZonedSerializer::new($element))?;
            }
            // __ULID__
            #[cfg(feature = "ryo3-ulid")]
            PyObType::RyUlid => {
                $seq.serialize_element(&rytypes::PyUlidSerializer::new($element))?;
            }
            // __URL__
            #[cfg(feature = "ryo3-url")]
            PyObType::RyUrl => {
                $seq.serialize_element(&rytypes::PyUrlSerializer::new($element))?;
            }
            // __UUID__
            #[cfg(feature = "ryo3-uuid")]
            PyObType::RyUuid => {
                $seq.serialize_element(&rytypes::PyUuidSerializer::new($element))?;
            }
            // ------------------------------------------------------------
            // UNKNOWN
            // ------------------------------------------------------------
            PyObType::Unknown => {
                $seq.serialize_element(&PyAnySerializer::new_with_depth(
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
pub(crate) struct PyListSerializer<'a, 'py> {
    pub(crate) ctx: PySerializeContext<'py>,
    pub(crate) obj: Borrowed<'a, 'py, PyList>,
    pub(crate) depth: Depth,
}

impl<'a, 'py> PyListSerializer<'a, 'py> {
    #[inline]
    pub(crate) fn new(
        obj: Borrowed<'a, 'py, PyList>,
        ctx: PySerializeContext<'py>,
        depth: Depth,
    ) -> Self {
        Self { ctx, obj, depth }
    }

    #[expect(unsafe_code)]
    #[inline]
    pub(crate) fn new_unchecked(
        obj: Borrowed<'a, 'py, PyAny>,
        ctx: PySerializeContext<'py>,
        depth: Depth,
    ) -> Self {
        let py_list = unsafe { obj.cast_unchecked::<PyList>() };
        Self::new(py_list, ctx, depth)
    }
}

impl Serialize for PyListSerializer<'_, '_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if self.depth == MAX_DEPTH {
            return serde_err_recursion!();
        }
        let len = self.obj.len();
        if len == 0 {
            serializer.serialize_seq(Some(0))?.end()
        } else {
            let mut seq = serializer.serialize_seq(Some(len))?;
            for element in self.obj.iter() {
                let element = element.as_borrowed();
                let ob_type = self.ctx.typeref.obtype(element);
                serialize_seq_element!(ob_type, seq, self, element);
            }
            seq.end()
        }
    }
}

// ----------------------------------------------------------------------------
// PyTuple
// ----------------------------------------------------------------------------
pub(crate) struct PyTupleSerializer<'a, 'py> {
    pub(crate) obj: Borrowed<'a, 'py, PyTuple>,
    pub(crate) ctx: PySerializeContext<'py>,
    pub(crate) depth: Depth,
}

impl<'a, 'py> PyTupleSerializer<'a, 'py> {
    #[inline]
    pub(crate) fn new(
        obj: Borrowed<'a, 'py, PyTuple>,
        ctx: PySerializeContext<'py>,
        depth: Depth,
    ) -> Self {
        Self { obj, ctx, depth }
    }

    #[expect(unsafe_code)]
    #[inline]
    pub(crate) fn new_unchecked(
        obj: Borrowed<'a, 'py, PyAny>,
        ctx: PySerializeContext<'py>,
        depth: Depth,
    ) -> Self {
        let py_tuple = unsafe { obj.cast_unchecked::<PyTuple>() };
        Self::new(py_tuple, ctx, depth)
    }
}

impl Serialize for PyTupleSerializer<'_, '_> {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if self.depth == MAX_DEPTH {
            return serde_err_recursion!();
        }
        let py_tuple = self.obj;
        let len = py_tuple.len();
        if len == 0 {
            serializer.serialize_seq(Some(0))?.end()
        } else {
            let mut tup = serializer.serialize_tuple(len)?;
            for element in py_tuple.iter() {
                let element = element.as_borrowed();
                let ob_type = self.ctx.typeref.obtype(element);
                serialize_seq_element!(ob_type, tup, self, element);
            }
            tup.end()
        }
    }
}

// ----------------------------------------------------------------------------
// PySet
// ----------------------------------------------------------------------------
pub(crate) struct PySetSerializer<'a, 'py> {
    pub(crate) ctx: PySerializeContext<'py>,
    pub(crate) obj: Borrowed<'a, 'py, PySet>,
    pub(crate) depth: Depth,
}

impl<'a, 'py> PySetSerializer<'a, 'py> {
    #[inline]
    pub(crate) fn new(obj: Borrowed<'a, 'py, PySet>, ctx: PySerializeContext<'py>) -> Self {
        Self {
            obj,
            ctx,
            depth: Depth::default(),
        }
    }

    #[expect(unsafe_code)]
    #[inline]
    pub(crate) fn new_unchecked(
        obj: Borrowed<'a, 'py, PyAny>,
        ctx: PySerializeContext<'py>,
    ) -> Self {
        let py_set = unsafe { obj.cast_unchecked::<PySet>() };
        Self::new(py_set, ctx)
    }
}

impl Serialize for PySetSerializer<'_, '_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let py_set = self.obj;
        let len = py_set.len();
        if len == 0 {
            return serializer.serialize_seq(Some(0))?.end();
        }
        // TODO: Can't have non hashable elements in set or frozenset so could optimize checks for those
        // let py_iter = PyIterator::from_object(py_set).expect("set is always iterable");
        let mut seq = serializer.serialize_seq(Some(len))?;
        for element in py_set.iter() {
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
pub(crate) struct PyFrozenSetSerializer<'a, 'py> {
    pub(crate) ctx: PySerializeContext<'py>,
    pub(crate) obj: Borrowed<'a, 'py, PyFrozenSet>,
    pub(crate) depth: Depth,
}

impl<'a, 'py> PyFrozenSetSerializer<'a, 'py> {
    #[inline]
    pub(crate) fn new(obj: Borrowed<'a, 'py, PyFrozenSet>, ctx: PySerializeContext<'py>) -> Self {
        Self {
            obj,
            ctx,
            depth: Depth::default(),
        }
    }

    #[expect(unsafe_code)]
    #[inline]
    pub(crate) fn new_unchecked(
        obj: Borrowed<'a, 'py, PyAny>,
        ctx: PySerializeContext<'py>,
    ) -> Self {
        let py_frozenset = unsafe { obj.cast_unchecked::<PyFrozenSet>() };
        Self::new(py_frozenset, ctx)
    }
}

impl Serialize for PyFrozenSetSerializer<'_, '_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let py_frozenset = self.obj;
        let len = py_frozenset.len();
        if len == 0 {
            return serializer.serialize_seq(Some(0))?.end();
        }
        // let py_iter = PyIterator::from_object(py_frozenset).expect("frozenset is always iterable");
        let mut seq = serializer.serialize_seq(Some(len))?;
        for element in py_frozenset.iter() {
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

pub(crate) struct PySequenceSerializer<'a, 'py> {
    ctx: PySerializeContext<'py>,
    obj: Borrowed<'a, 'py, PySequence>,
    depth: Depth,
}

impl<'a, 'py> PySequenceSerializer<'a, 'py> {
    #[inline]
    pub(crate) fn new_with_depth(
        obj: Borrowed<'a, 'py, PySequence>,
        ctx: PySerializeContext<'py>,
        depth: Depth,
    ) -> Self {
        Self { ctx, obj, depth }
    }
}

impl Serialize for PySequenceSerializer<'_, '_> {
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
