//! Opt-in `NumPy` scalar and ndarray serialization.
//!
//! `NumPy` is intentionally imported only when the JSON API enables this mode.
//! Exact type pointers are cached and ndarray metadata is obtained from
//! `__array_struct__`; there is no Rust or Python runtime dependency on `NumPy`.

use std::ffi::c_int;
use std::marker::PhantomData;
use std::mem::size_of;
use std::ptr::NonNull;

use pyo3::ffi;
use pyo3::prelude::*;
use pyo3::types::{PyCapsule, PyCapsuleMethods};
use ryo3_numpy::{NumpyDType, NumpyObType, PyArrayInterface};
use serde::ser::{Serialize, SerializeSeq, Serializer};

use crate::errors::pyerr2sererr;
use crate::{Depth, MAX_DEPTH, serde_err_recursion};

const MAX_NDARRAY_DIMENSIONS: usize = 32;
const NPY_ARRAY_C_CONTIGUOUS: c_int = 0x0001;
const NPY_ARRAY_NOTSWAPPED: c_int = 0x0200;

enum NumpySerdeError {
    NotCContiguous,
    NotNativeEndian,
    UnsupportedDType(NumpyDType),
}

impl NumpySerdeError {
    fn as_str(&self) -> &str {
        match self {
            Self::NotCContiguous => "numpy array must be C-contiguous",
            Self::NotNativeEndian => "numpy array must have native byte order",
            Self::UnsupportedDType(dtype) => match dtype {
                NumpyDType::Bool => "numpy bool_ dtype is not supported",
                NumpyDType::I8 => "numpy int8 dtype is not supported",
                NumpyDType::I16 => "numpy int16 dtype is not supported",
                NumpyDType::I32 => "numpy int32 dtype is not supported",
                NumpyDType::I64 => "numpy int64 dtype is not supported",
                NumpyDType::U8 => "numpy uint8 dtype is not supported",
                NumpyDType::U16 => "numpy uint16 dtype is not supported",
                NumpyDType::U32 => "numpy uint32 dtype is not supported",
                NumpyDType::U64 => "numpy uint64 dtype is not supported",
                NumpyDType::F16 => "numpy float16 dtype is not supported",
                NumpyDType::F32 => "numpy float32 dtype is not supported",
                NumpyDType::F64 => "numpy float64 dtype is not supported",
            },
        }
    }
}

impl std::fmt::Display for NumpySerdeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

// convert to serde error for thingy err

// #[doc(hidden)]
// #[derive(Copy, Clone, Debug, Eq, PartialEq)]
// pub enum NumpyObType {
//     Array,
//     Scalar(NumpyDType),
// }

// pub(crate) struct NumpyTypeCache {
//     // Keep the module (and therefore its exact scalar/array classes) alive for
//     // as long as the cached raw type pointers can be consulted.
//     _module: Py<PyModule>,
//     array: usize,
//     bool_: usize,
//     int8: usize,
//     int16: usize,
//     int32: usize,
//     int64: usize,
//     uint8: usize,
//     uint16: usize,
//     uint32: usize,
//     uint64: usize,
//     float16: usize,
//     float32: usize,
//     float64: usize,
// }

// static NUMPY_TYPES: PyOnceLock<NumpyTypeCache> = PyOnceLock::new();

// impl NumpyTypeCache {
//     fn new(py: Python<'_>) -> PyResult<Self> {
//         #[cfg(any(PyPy, GraalPy, Py_LIMITED_API))]
//         {
//             let _ = py;
//             return Err(pyo3::exceptions::PyNotImplementedError::new_err(
//                 "NumPy serialization is supported on CPython only",
//             ));
//         }

//         #[cfg(not(any(PyPy, GraalPy, Py_LIMITED_API)))]
//         {
//             let numpy = py.import("numpy")?;
//             let ptr =
//                 |name: &str| -> PyResult<usize> { Ok(numpy.getattr(name)?.as_ptr() as usize) };
//             Ok(Self {
//                 _module: numpy.clone().unbind(),
//                 array: ptr("ndarray")?,
//                 bool_: ptr("bool_")?,
//                 int8: ptr("int8")?,
//                 int16: ptr("int16")?,
//                 int32: ptr("int32")?,
//                 int64: ptr("int64")?,
//                 uint8: ptr("uint8")?,
//                 uint16: ptr("uint16")?,
//                 uint32: ptr("uint32")?,
//                 uint64: ptr("uint64")?,
//                 float16: ptr("float16")?,
//                 float32: ptr("float32")?,
//                 float64: ptr("float64")?,
//             })
//         }
//     }

//     pub(crate) fn cached(py: Python<'_>) -> PyResult<&'static Self> {
//         NUMPY_TYPES.get_or_try_init(py, || Self::new(py))
//     }

//     #[inline]
//     fn obtype(&self, ptr: usize) -> Option<NumpyObType> {
//         macro_rules! scalar {
//             ($field:ident, $dtype:ident) => {
//                 if ptr == self.$field {
//                     return Some(NumpyObType::Scalar(NumpyDType::$dtype));
//                 }
//             };
//         }

//         if ptr == self.array {
//             return Some(NumpyObType::Array);
//         }
//         scalar!(bool_, Bool);
//         scalar!(int8, I8);
//         scalar!(int16, I16);
//         scalar!(int32, I32);
//         scalar!(int64, I64);
//         scalar!(uint8, U8);
//         scalar!(uint16, U16);
//         scalar!(uint32, U32);
//         scalar!(uint64, U64);
//         scalar!(float16, F16);
//         scalar!(float32, F32);
//         scalar!(float64, F64);
//         None
//     }
// }

trait CContigDTypeSerializer<const ALIGNED: bool> {
    const ITEM_SIZE: usize;

    fn serialize_1d<S>(data: *const u8, len: usize, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer;
}

// Keep primitive serialization explicit. This macro avoids a per-element dtype
// branch while still allowing Serde to inline the concrete numeric method.
macro_rules! primitive_dtype {
    ($name:ident, $ty:ty, $method:ident) => {
        #[derive(Copy, Clone, Default)]
        struct $name;

        impl<const ALIGNED: bool> CContigDTypeSerializer<ALIGNED> for $name {
            const ITEM_SIZE: usize = size_of::<$ty>();

            #[expect(unsafe_code)]
            #[allow(clippy::cast_ptr_alignment)]
            #[inline]
            fn serialize_1d<S>(
                data: *const u8,
                len: usize,
                serializer: S,
            ) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                let mut seq = serializer.serialize_seq(Some(len))?;
                for i in 0..len {
                    let ptr = unsafe { data.add(i * size_of::<$ty>()).cast::<$ty>() };
                    let value = if ALIGNED {
                        unsafe { ptr.read() }
                    } else {
                        unsafe { ptr.read_unaligned() }
                    };
                    seq.serialize_element(&SerializePrimitive(value))?;
                }
                seq.end()
            }
        }

        impl Serialize for SerializePrimitive<$ty> {
            #[inline]
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                serializer.$method(self.0)
            }
        }
    };
}

struct SerializePrimitive<T>(T);

primitive_dtype!(I8DType, i8, serialize_i8);
primitive_dtype!(I16DType, i16, serialize_i16);
primitive_dtype!(I32DType, i32, serialize_i32);
primitive_dtype!(I64DType, i64, serialize_i64);
primitive_dtype!(U8DType, u8, serialize_u8);
primitive_dtype!(U16DType, u16, serialize_u16);
primitive_dtype!(U32DType, u32, serialize_u32);
primitive_dtype!(U64DType, u64, serialize_u64);
primitive_dtype!(F32DType, f32, serialize_f32);
primitive_dtype!(F64DType, f64, serialize_f64);

#[derive(Copy, Clone, Default)]
struct BoolDType;

impl<const ALIGNED: bool> CContigDTypeSerializer<ALIGNED> for BoolDType {
    const ITEM_SIZE: usize = 1;

    #[expect(unsafe_code)]
    #[inline]
    fn serialize_1d<S>(data: *const u8, len: usize, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(len))?;
        for i in 0..len {
            let value = unsafe { data.add(i).read() } != 0;
            seq.serialize_element(&value)?;
        }
        seq.end()
    }
}

struct NdArraySerializer<'a, D, const ALIGNED: bool> {
    data: *const u8,
    shape: &'a [usize],
    _dtype: PhantomData<D>,
}

impl<'a, D, const ALIGNED: bool> NdArraySerializer<'a, D, ALIGNED> {
    #[inline]
    fn new(data: *const u8, shape: &'a [usize]) -> Self {
        Self {
            data,
            shape,
            _dtype: PhantomData,
        }
    }
}

impl<D, const ALIGNED: bool> Serialize for NdArraySerializer<'_, D, ALIGNED>
where
    D: CContigDTypeSerializer<ALIGNED>,
{
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if self.shape.len() == 1 {
            return D::serialize_1d(self.data, self.shape[0], serializer);
        }

        let len = self.shape[0];
        let sub_shape = &self.shape[1..];
        let sub_elems = sub_shape.iter().try_fold(1usize, |acc, &dim| {
            acc.checked_mul(dim)
                .ok_or_else(|| serde::ser::Error::custom("numpy array shape overflow"))
        })?;
        let step = sub_elems
            .checked_mul(D::ITEM_SIZE)
            .ok_or_else(|| serde::ser::Error::custom("numpy array offset overflow"))?;

        let mut seq = serializer.serialize_seq(Some(len))?;
        for i in 0..len {
            let offset = i
                .checked_mul(step)
                .ok_or_else(|| serde::ser::Error::custom("numpy array offset overflow"))?;
            let item = Self::new(self.data.wrapping_add(offset), sub_shape);
            seq.serialize_element(&item)?;
        }
        seq.end()
    }
}

fn serialize_array_typed<S, D, const ALIGNED: bool>(
    data: *const u8,
    shape: &[usize],
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    D: CContigDTypeSerializer<ALIGNED>,
{
    NdArraySerializer::<D, ALIGNED>::new(data, shape).serialize(serializer)
}

fn serialize_array_layout<S, const ALIGNED: bool>(
    dtype: NumpyDType,
    data: *const u8,
    shape: &[usize],
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match dtype {
        NumpyDType::Bool => serialize_array_typed::<S, BoolDType, ALIGNED>(data, shape, serializer),
        NumpyDType::I8 => serialize_array_typed::<S, I8DType, ALIGNED>(data, shape, serializer),
        NumpyDType::I16 => serialize_array_typed::<S, I16DType, ALIGNED>(data, shape, serializer),
        NumpyDType::I32 => serialize_array_typed::<S, I32DType, ALIGNED>(data, shape, serializer),
        NumpyDType::I64 => serialize_array_typed::<S, I64DType, ALIGNED>(data, shape, serializer),
        NumpyDType::U8 => serialize_array_typed::<S, U8DType, ALIGNED>(data, shape, serializer),
        NumpyDType::U16 => serialize_array_typed::<S, U16DType, ALIGNED>(data, shape, serializer),
        NumpyDType::U32 => serialize_array_typed::<S, U32DType, ALIGNED>(data, shape, serializer),
        NumpyDType::U64 => serialize_array_typed::<S, U64DType, ALIGNED>(data, shape, serializer),
        NumpyDType::F16 => Err(serde::ser::Error::custom(
            NumpySerdeError::UnsupportedDType(NumpyDType::F16),
        )),
        NumpyDType::F32 => serialize_array_typed::<S, F32DType, ALIGNED>(data, shape, serializer),
        NumpyDType::F64 => serialize_array_typed::<S, F64DType, ALIGNED>(data, shape, serializer),
    }
}

struct PyNumpyArraySerializer<'a, 'py> {
    obj: Borrowed<'a, 'py, PyAny>,
    depth: Depth,
}

impl Serialize for PyNumpyArraySerializer<'_, '_> {
    #[expect(unsafe_code)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let py = self.obj.py();
        let capsule_obj = self
            .obj
            .getattr(pyo3::intern!(py, "__array_struct__"))
            .map_err(pyerr2sererr)?;
        let capsule = capsule_obj.cast::<PyCapsule>().map_err(pyerr2sererr)?;
        let interface_ptr = capsule.pointer_checked(None).map_err(pyerr2sererr)?;
        let interface = unsafe { interface_ptr.cast::<PyArrayInterface>().as_ref() };

        if interface.two != 2 {
            return Err(serde::ser::Error::custom(
                "numpy array has malformed __array_struct__ metadata",
            ));
        }
        if interface.nd < 1 {
            return Err(serde::ser::Error::custom(
                "numpy zero-dimensional arrays are not supported; use a numpy scalar",
            ));
        }
        let dimensions = usize::try_from(interface.nd)
            .map_err(|_| serde::ser::Error::custom("numpy array dimension is invalid"))?;
        if dimensions > MAX_NDARRAY_DIMENSIONS {
            return Err(serde::ser::Error::custom(
                "numpy arrays with more than 32 dimensions are not supported",
            ));
        }
        if self.depth as usize + dimensions >= MAX_DEPTH as usize {
            return serde_err_recursion!();
        }
        if interface.flags & NPY_ARRAY_C_CONTIGUOUS == 0 {
            return Err(serde::ser::Error::custom(NumpySerdeError::NotCContiguous));
        }
        if interface.flags & NPY_ARRAY_NOTSWAPPED == 0 {
            return Err(serde::ser::Error::custom(NumpySerdeError::NotNativeEndian));
        }
        if interface.shape.is_null() {
            return Err(serde::ser::Error::custom(
                "numpy array shape pointer is null",
            ));
        }

        let dtype = NumpyDType::from_typekind_and_itemsize(interface.typekind, interface.itemsize)
            .ok_or_else(|| {
                serde::ser::Error::custom(format_args!(
                    "numpy dtype is not supported (kind={}, itemsize={})",
                    interface.typekind, interface.itemsize
                ))
            })?;

        let raw_shape = unsafe { std::slice::from_raw_parts(interface.shape, dimensions) };
        // NumPy itself caps dimensions at 32, so keep shape metadata on the
        // stack and avoid an allocation for every serialized array.
        let mut shape_storage = [0usize; MAX_NDARRAY_DIMENSIONS];
        let mut element_count = 1usize;
        for (index, &dimension) in raw_shape.iter().enumerate() {
            let dimension = usize::try_from(dimension)
                .map_err(|_| serde::ser::Error::custom("numpy array shape is invalid"))?;
            element_count = element_count
                .checked_mul(dimension)
                .ok_or_else(|| serde::ser::Error::custom("numpy array shape overflow"))?;
            shape_storage[index] = dimension;
        }
        element_count
            .checked_mul(dtype.itemsize())
            .ok_or_else(|| serde::ser::Error::custom("numpy array byte length overflow"))?;

        let data = if element_count == 0 {
            NonNull::<u8>::dangling().as_ptr().cast_const()
        } else {
            if interface.data.is_null() {
                return Err(serde::ser::Error::custom(
                    "numpy array data pointer is null",
                ));
            }
            interface.data.cast::<u8>()
        };

        let shape = &shape_storage[..dimensions];
        if (data as usize).is_multiple_of(dtype.itemsize()) {
            serialize_array_layout::<S, true>(dtype, data, shape, serializer)
        } else {
            serialize_array_layout::<S, false>(dtype, data, shape, serializer)
        }
    }
}

struct PyNumpyScalarSerializer<'a, 'py> {
    obj: Borrowed<'a, 'py, PyAny>,
    dtype: NumpyDType,
}

impl PyNumpyScalarSerializer<'_, '_> {
    #[expect(unsafe_code)]
    #[inline]
    unsafe fn value<T: Copy>(&self) -> T {
        // NumPy's fixed-width scalar structs are `PyObject_HEAD` followed by
        // the scalar payload. Exact cached type matching is required before
        // reaching this method; subclasses must never use this layout.
        let ptr = unsafe {
            self.obj
                .as_ptr()
                .cast::<u8>()
                .add(size_of::<ffi::PyObject>())
                .cast::<T>()
        };
        unsafe { ptr.read_unaligned() }
    }
}

impl Serialize for PyNumpyScalarSerializer<'_, '_> {
    #[expect(unsafe_code)]
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self.dtype {
            NumpyDType::Bool => serializer.serialize_bool(unsafe { self.value::<u8>() } != 0),
            NumpyDType::I8 => serializer.serialize_i8(unsafe { self.value() }),
            NumpyDType::I16 => serializer.serialize_i16(unsafe { self.value() }),
            NumpyDType::I32 => serializer.serialize_i32(unsafe { self.value() }),
            NumpyDType::I64 => serializer.serialize_i64(unsafe { self.value() }),
            NumpyDType::U8 => serializer.serialize_u8(unsafe { self.value() }),
            NumpyDType::U16 => serializer.serialize_u16(unsafe { self.value() }),
            NumpyDType::U32 => serializer.serialize_u32(unsafe { self.value() }),
            NumpyDType::U64 => serializer.serialize_u64(unsafe { self.value() }),
            NumpyDType::F16 => Err(serde::ser::Error::custom(
                "numpy float16 scalar is not supported",
            )),
            NumpyDType::F32 => serializer.serialize_f32(unsafe { self.value() }),
            NumpyDType::F64 => serializer.serialize_f64(unsafe { self.value() }),
        }
    }
}

pub(crate) struct PyNumpySerializer<'a, 'py> {
    obj: Borrowed<'a, 'py, PyAny>,
    obtype: NumpyObType,
    depth: Depth,
}

impl<'a, 'py> PyNumpySerializer<'a, 'py> {
    pub(crate) fn new(obj: Borrowed<'a, 'py, PyAny>, obtype: NumpyObType, depth: Depth) -> Self {
        Self { obj, obtype, depth }
    }
}

impl Serialize for PyNumpySerializer<'_, '_> {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self.obtype {
            NumpyObType::Array => PyNumpyArraySerializer {
                obj: self.obj,
                depth: self.depth,
            }
            .serialize(serializer),
            NumpyObType::Scalar(dtype) => PyNumpyScalarSerializer {
                obj: self.obj,
                dtype,
            }
            .serialize(serializer),
        }
    }
}
