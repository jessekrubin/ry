use crate::SerializePyAny;
use crate::constants::{Depth, MAX_DEPTH};
use crate::errors::pyerr2sererr;
use crate::ob_type::PyObType;
use crate::ser::PySerializeContext;
use crate::ser::rytypes;
use crate::ser::safe_impl::{
    SerializePyBool, SerializePyBytesLike, SerializePyDataclass, SerializePyDate,
    SerializePyDateTime, SerializePyDict, SerializePyFloat, SerializePyInt, SerializePyNone,
    SerializePyStr, SerializePyTime, SerializePyTimeDelta, SerializePyUuid,
};
use crate::serde_err_recursion;
use pyo3::Bound;
use pyo3::prelude::*;
use pyo3::types::{PyAny, PyFrozenSet, PyIterator, PyList, PySequence, PySet, PyTuple};
use serde::ser::{Serialize, SerializeSeq, SerializeTuple, Serializer};

macro_rules! serialize_seq_element {
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

// ----------------------------------------------------------------------------
// PyList
// ----------------------------------------------------------------------------
pub(crate) struct SerializePyList<'a, 'py> {
    pub(crate) ctx: PySerializeContext<'py>,
    pub(crate) obj: &'a Bound<'py, PyAny>,
    pub(crate) depth: Depth,
}

impl<'a, 'py> SerializePyList<'a, 'py> {
    pub(crate) fn new(
        obj: &'a Bound<'py, PyAny>,
        ctx: PySerializeContext<'py>,
        depth: Depth,
    ) -> Self {
        Self { ctx, obj, depth }
    }
}
impl Serialize for SerializePyList<'_, '_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if self.depth == MAX_DEPTH {
            return serde_err_recursion!();
        }
        let py_list: &Bound<'_, PyList> = self.obj.cast_exact().map_err(pyerr2sererr)?;
        let len = py_list.len();
        if len == 0 {
            serializer.serialize_seq(Some(0))?.end()
        } else {
            let mut seq = serializer.serialize_seq(Some(len))?;
            for element in py_list {
                let ob_type = self.ctx.typeref.obtype(&element);
                serialize_seq_element!(ob_type, seq, self, element);
            }
            seq.end()
        }
    }
}

// ----------------------------------------------------------------------------
// PyTuple
// ----------------------------------------------------------------------------
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
        } else {
            let mut tup = serializer.serialize_tuple(len)?;
            for element in py_tuple {
                let ob_type = self.ctx.typeref.obtype(&element);
                serialize_seq_element!(ob_type, tup, self, element);
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

// ----------------------------------------------------------------------------
// PySet
// ----------------------------------------------------------------------------
pub(crate) struct SerializePySet<'a, 'py> {
    pub(crate) ctx: PySerializeContext<'py>,
    pub(crate) obj: &'a Bound<'py, PyAny>,
    pub(crate) depth: Depth,
    // default: Option<&'py Bound<'py, PyAny>>,
    // ob_type_lookup: &'py PyTypeCache,
}

impl<'a, 'py> SerializePySet<'a, 'py> {
    pub(crate) fn new(obj: &'a Bound<'py, PyAny>, ctx: PySerializeContext<'py>) -> Self {
        Self {
            obj,
            ctx,
            depth: Depth::default(),
        }
    }
}

impl Serialize for SerializePySet<'_, '_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let py_set: &Bound<'_, PyAny> = self.obj.cast::<PySet>().map_err(pyerr2sererr)?;
        let len = py_set.len().map_err(pyerr2sererr)?;
        if len == 0 {
            return serializer.serialize_seq(Some(0))?.end();
        }
        let py_iter = PyIterator::from_object(py_set).expect("set is always iterable");
        let mut seq = serializer.serialize_seq(Some(len))?;
        for element in py_iter {
            let pyany = element.map_err(pyerr2sererr)?;
            let ob_type = self.ctx.typeref.obtype(&pyany);
            serialize_seq_element!(ob_type, seq, self, pyany);
        }
        seq.end()
    }
}

// ----------------------------------------------------------------------------
// PyFrozenSet
// ----------------------------------------------------------------------------
pub(crate) struct SerializePyFrozenSet<'a, 'py> {
    pub(crate) ctx: PySerializeContext<'py>,
    pub(crate) obj: &'a Bound<'py, PyAny>,
    pub(crate) depth: Depth,
}

impl<'a, 'py> SerializePyFrozenSet<'a, 'py> {
    pub(crate) fn new(obj: &'a Bound<'py, PyAny>, ctx: PySerializeContext<'py>) -> Self {
        Self {
            obj,
            ctx,
            depth: Depth::default(),
        }
    }
}

impl Serialize for SerializePyFrozenSet<'_, '_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let py_frozenset: &Bound<'_, PyAny> =
            self.obj.cast::<PyFrozenSet>().map_err(pyerr2sererr)?;
        let len = py_frozenset.len().map_err(pyerr2sererr)?;
        if len == 0 {
            return serializer.serialize_seq(Some(0))?.end();
        }
        let py_iter = PyIterator::from_object(py_frozenset).expect("frozenset is always iterable");
        let mut seq = serializer.serialize_seq(Some(len))?;
        for element in py_iter {
            let pyany = element.map_err(pyerr2sererr)?;
            let ob_type = self.ctx.typeref.obtype(&pyany);
            serialize_seq_element!(ob_type, seq, self, pyany);
        }
        seq.end()
    }
}

// ----------------------------------------------------------------------------
// PySequence
// ----------------------------------------------------------------------------

pub(crate) struct SerializePySequence<'a, 'py> {
    ctx: PySerializeContext<'py>,
    obj: &'a Bound<'py, PySequence>,
    depth: Depth,
}

impl<'a, 'py> SerializePySequence<'a, 'py> {
    pub(crate) fn new_with_depth(
        obj: &'a Bound<'py, PySequence>,
        ctx: PySerializeContext<'py>,
        depth: Depth,
    ) -> Self {
        Self { ctx, obj, depth }
    }
}

impl Serialize for SerializePySequence<'_, '_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let len = self.obj.len().map_err(pyerr2sererr)?;
        let mut seq = serializer.serialize_seq(Some(len))?;
        for i in 0..len {
            let item = self.obj.get_item(i).map_err(pyerr2sererr)?;
            let item_ser = SerializePyAny::new_with_depth(&item, self.ctx, self.depth + 1);
            seq.serialize_element(&item_ser)?;
        }
        seq.end()
    }
}

// // impl Serialize for SerializePyList<'_, '_> {
// //     #[inline(always)]
// //     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
// //     where
// //         S: Serializer,
// //     {
// //         if self.depth == MAX_DEPTH {
// //             return serde_err_recursion!();
// //         }

// //         // Safe downcast; keep Bound<PyList> to ensure GIL + lifetime correctness.
// //         let py_list: &Bound<'_, PyList> = self.obj.cast_exact().map_err(pyerr2sererr)?;
// //         let ptr = py_list.as_ptr();

// //         // SAFETY: `py_list` is a valid `PyList` while GIL is held.
// //         let len = unsafe { ffi::PyList_GET_SIZE(ptr) as usize };

// //         if len == 0 {
// //             return serializer.serialize_seq(Some(0))?.end();
// //         }

// //         let mut seq = serializer.serialize_seq(Some(len))?;
// //         let py = py_list.py();

// //         // Tight, index-based loop without creating a Python iterator.
// //         for i in 0..len {
// //             // SAFETY: `0 <= i < len`, `ptr` is a valid list; CPython guarantees
// //             // elements are valid borrowed references for the duration of the borrow.
// //             let item = unsafe { ffi::PyList_GET_ITEM(ptr, i as isize) };
// //             // make into Bound<PyAny> without incref
// //             // let element: Bound<'_, PyAny> = unsafe { Bound::from_borrowed_ptr(py, item) };
// //             debug_assert!(!item.is_null());

// //             // Convert the borrowed `PyObject*` into a `Bound<PyAny>` **without** incref.
// //             // SAFETY: `item` is a *borrowed* reference tied to `py_list`/GIL; we do not
// //             // store it past the loop, and we don’t decref it.
// //             let element: Bound<'_, PyAny> = unsafe { Bound::from_borrowed_ptr(py, item) };

// //             // Faster than going through `self.ctx.typeref.obtype(&element)`.
// //             // This is just a raw pointer read; no Python-call overhead.
// //             // SAFETY: `item` is a valid PyObject*; `ob_type` is always non-null.
// //             let ob_type_ptr = unsafe { (*item).ob_type };
// //             debug_assert!(!ob_type_ptr.is_null());
// //             let ob_type = self.ctx.typeref.ptr2type(ob_type_ptr as usize, &element);

// //             // Your existing dispatch; if you can change the macro to accept a raw
// //             // `*mut ffi::PyTypeObject` you’ll avoid re-touching `element` here.
// //             serialize_seq_element!(ob_type, seq, self, element);
// //         }

// //         seq.end()
// //     }
// // }

// impl Serialize for SerializePyList<'_, '_> {
//     #[inline(always)]
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         if self.depth == MAX_DEPTH {
//             return serde_err_recursion!();
//         }

//         let py_list: &Bound<'_, PyList> = self.obj.cast_exact().map_err(pyerr2sererr)?;
//         let len = py_list.len();
//         let mut seq = serializer.serialize_seq(Some(len))?;
//         if len == 0 {
//             return seq.end();
//         }

//         let py = py_list.py();
//         let lptr = py_list.as_ptr();

//         unsafe {
//             let first = ffi::PyList_GET_ITEM(lptr, 0);
//             // Example: exact-int fast path
//             if ffi::PyLong_CheckExact(first) != 0 {
//                 for i in 0..len {
//                     let it = ffi::PyList_GET_ITEM(lptr, i as isize);

//                     // Fast int to i64; handle overflow by falling back.
//                     let mut overflow: std::os::raw::c_int = 0;
//                     let v = ffi::PyLong_AsLongLongAndOverflow(it, &mut overflow);
//                     if overflow != 0 || (v == -1 && !ffi::PyErr_Occurred().is_null()) {
//                         ffi::PyErr_Clear();
//                         // Fallback for huge ints: use your generic machinery
//                         let el = Bound::from_borrowed_ptr(py, it);
//                         let ob_type = self.ctx.typeref.obtype(&el);
//                         serialize_seq_element!(ob_type, seq, self, el);
//                         continue;
//                     }
//                     // zero-copy into serde
//                     seq.serialize_element(&v)?;
//                 }
//                 return seq.end();
//             }

//             // Example: exact-str fast path
//             if ffi::PyUnicode_CheckExact(first) != 0 {
//                 for i in 0..len {
//                     let it = ffi::PyList_GET_ITEM(lptr, i as isize);
//                     let mut size: isize = 0;
//                     let c = ffi::PyUnicode_AsUTF8AndSize(it, &mut size);
//                     if c.is_null() {
//                         ffi::PyErr_Clear();
//                         let el = Bound::from_borrowed_ptr(py, it);
//                         let ob_type = self.ctx.typeref.obtype(&el);
//                         serialize_seq_element!(ob_type, seq, self, el);
//                         continue;
//                     }
//                     let s = std::slice::from_raw_parts(c as *const u8, size as usize);
//                     // Safety: CPython guarantees UTF-8 here.
//                     let s = std::str::from_utf8_unchecked(s);
//                     seq.serialize_element(&s)?;
//                 }
//                 return seq.end();
//             }
//         }

//         // Generic fallback
//         for element in py_list {
//             let ob_type = self.ctx.typeref.obtype(&element);
//             serialize_seq_element!(ob_type, seq, self, element);
//         }
//         seq.end()
//     }
// }
