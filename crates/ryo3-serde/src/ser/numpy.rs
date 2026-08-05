#![allow(clippy::manual_let_else)]
use pyo3::buffer::PyUntypedBuffer;
use pyo3::prelude::*;
use serde::ser::{Serialize, SerializeSeq, Serializer};
use std::marker::PhantomData;

use crate::errors::pyerr2sererr;

#[derive(Copy, Clone)]
enum DTypeDispatch {
    Bool,
    I8,
    I16,
    I32,
    I64,
    U8,
    U16,
    U32,
    U64,
    F16,
    F32,
    F64,
}

impl DTypeDispatch {
    fn from_buffer_format(format: &std::ffi::CStr, itemsize: usize) -> Option<Self> {
        let fmt = format.to_str().ok()?;
        let mut chars = fmt.chars();

        let first = chars.next()?;
        let code = match first {
            '@' | '=' | '<' | '>' | '!' | '^' => chars.next()?,
            _ => first,
        };

        let dispatch = match (code, itemsize) {
            ('?', 1) => Self::Bool,
            ('b', 1) => Self::I8,
            ('h', 2) => Self::I16,
            ('i', 4) => Self::I32,
            ('l' | 'q', 8) => Self::I64,
            ('B', 1) => Self::U8,
            ('H', 2) => Self::U16,
            ('I', 4) => Self::U32,
            ('L' | 'Q', 8) => Self::U64,
            ('e', 2) => Self::F16,
            ('f', 4) => Self::F32,
            ('d', 8) => Self::F64,
            _ => return None,
        };
        Some(dispatch)
    }
}

trait CContigDTypeSerializer {
    const ITEM_SIZE: usize;

    fn serialize_scalar<S>(data: *const u8, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer;

    fn serialize_1d_contig<S>(
        data: *const u8,
        len: usize,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer;
}

trait StridedDTypeSerializer: CContigDTypeSerializer {
    fn serialize_1d_strided<S>(
        data: *const u8,
        len: usize,
        stride: isize,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer;
}

macro_rules! dtype_serializer {
    ($name:ident, $ty:ty, $method:ident) => {
        #[derive(Copy, Clone, Default)]
        struct $name;

        impl CContigDTypeSerializer for $name {
            const ITEM_SIZE: usize = std::mem::size_of::<$ty>();

            #[expect(unsafe_code)]
            fn serialize_scalar<S>(data: *const u8, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                let bytes = unsafe {
                    std::ptr::read_unaligned(data.cast::<[u8; std::mem::size_of::<$ty>()]>())
                };
                serializer.$method(<$ty>::from_ne_bytes(bytes))
            }

            #[expect(unsafe_code)]
            fn serialize_1d_contig<S>(
                data: *const u8,
                len: usize,
                serializer: S,
            ) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                let aligned = (data as usize) % std::mem::align_of::<$ty>() == 0;
                if aligned {
                    #[allow(clippy::cast_ptr_alignment)]
                    let typed = unsafe { std::slice::from_raw_parts(data.cast::<$ty>(), len) };
                    let mut seq = serializer.serialize_seq(Some(len))?;
                    for v in typed {
                        seq.serialize_element(v)?;
                    }
                    return seq.end();
                }

                let mut seq = serializer.serialize_seq(Some(len))?;
                for i in 0..len {
                    let ptr = unsafe { data.add(i * Self::ITEM_SIZE) };
                    let bytes = unsafe {
                        std::ptr::read_unaligned(ptr.cast::<[u8; std::mem::size_of::<$ty>()]>())
                    };
                    let v = <$ty>::from_ne_bytes(bytes);
                    seq.serialize_element(&v)?;
                }
                seq.end()
            }
        }

        impl StridedDTypeSerializer for $name {
            #[expect(unsafe_code)]
            fn serialize_1d_strided<S>(
                data: *const u8,
                len: usize,
                stride: isize,
                serializer: S,
            ) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                let mut seq = serializer.serialize_seq(Some(len))?;
                for i in 0..len {
                    let ptr = unsafe { data.offset(stride * (i.cast_signed())) };
                    let bytes = unsafe {
                        std::ptr::read_unaligned(ptr.cast::<[u8; std::mem::size_of::<$ty>()]>())
                    };
                    let v = <$ty>::from_ne_bytes(bytes);
                    seq.serialize_element(&v)?;
                }
                seq.end()
            }
        }
    };
}

#[derive(Copy, Clone, Default)]
struct BoolDType;

impl CContigDTypeSerializer for BoolDType {
    const ITEM_SIZE: usize = 1;

    #[expect(unsafe_code)]
    fn serialize_scalar<S>(data: *const u8, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_bool(unsafe { std::ptr::read_unaligned(data) } != 0)
    }

    #[expect(unsafe_code)]
    fn serialize_1d_contig<S>(data: *const u8, len: usize, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(len))?;
        for i in 0..len {
            let ptr = unsafe { data.add(i) };
            let v = unsafe { std::ptr::read_unaligned(ptr) } != 0;
            seq.serialize_element(&v)?;
        }
        seq.end()
    }
}

impl StridedDTypeSerializer for BoolDType {
    #[expect(unsafe_code)]
    fn serialize_1d_strided<S>(
        data: *const u8,
        len: usize,
        stride: isize,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(len))?;
        for i in 0..len {
            let ptr = unsafe { data.offset(stride * (i.cast_signed())) };
            let v = unsafe { std::ptr::read_unaligned(ptr) } != 0;
            seq.serialize_element(&v)?;
        }
        seq.end()
    }
}

dtype_serializer!(I8DType, i8, serialize_i8);
dtype_serializer!(I16DType, i16, serialize_i16);
dtype_serializer!(I32DType, i32, serialize_i32);
dtype_serializer!(I64DType, i64, serialize_i64);
dtype_serializer!(U8DType, u8, serialize_u8);
dtype_serializer!(U16DType, u16, serialize_u16);
dtype_serializer!(U32DType, u32, serialize_u32);
dtype_serializer!(U64DType, u64, serialize_u64);
dtype_serializer!(F32DType, f32, serialize_f32);
dtype_serializer!(F64DType, f64, serialize_f64);

#[inline]
fn is_native_endian(format: &std::ffi::CStr) -> bool {
    let fmt = match format.to_str() {
        Ok(fmt) => fmt,
        Err(_) => return false,
    };
    let mut chars = fmt.chars();
    let first = match chars.next() {
        Some(c) => c,
        None => return false,
    };
    match first {
        '<' => cfg!(target_endian = "little"),
        '>' | '!' => cfg!(target_endian = "big"),
        '^' => false,
        '@' | '=' | _ => true,
    }
}

struct NdArraySerializer<'a, D, const C_CONTIG: bool> {
    data: *const u8,
    shape: &'a [usize],
    strides: &'a [isize],
    _dtype: PhantomData<D>,
}

// pub(crate) struct NumpyTypes {
//     pub array: *mut PyTypeObject,
//     pub float64: *mut PyTypeObject,
//     pub float32: *mut PyTypeObject,
//     pub float16: *mut PyTypeObject,
//     pub int64: *mut PyTypeObject,
//     pub int32: *mut PyTypeObject,
//     pub int16: *mut PyTypeObject,
//     pub int8: *mut PyTypeObject,
//     pub uint64: *mut PyTypeObject,
//     pub uint32: *mut PyTypeObject,
//     pub uint16: *mut PyTypeObject,
//     pub uint8: *mut PyTypeObject,
//     pub bool_: *mut PyTypeObject,
//     pub datetime64: *mut PyTypeObject,
// }

impl<'a, D, const C_CONTIG: bool> NdArraySerializer<'a, D, C_CONTIG> {
    #[inline]
    fn new(data: *const u8, shape: &'a [usize], strides: &'a [isize]) -> Self {
        Self {
            data,
            shape,
            strides,
            _dtype: PhantomData,
        }
    }

    #[expect(unsafe_code)]
    #[inline]
    fn element_ptr(&self, index: usize) -> *const u8 {
        unsafe { self.data.offset(self.strides[0] * (index.cast_signed())) }
    }
}

impl<D, const C_CONTIG: bool> Serialize for NdArraySerializer<'_, D, C_CONTIG>
where
    D: StridedDTypeSerializer,
{
    #[expect(unsafe_code)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if self.shape.is_empty() {
            return D::serialize_scalar(self.data, serializer);
        }

        if self.shape.len() == 1 {
            if C_CONTIG {
                return D::serialize_1d_contig(self.data, self.shape[0], serializer);
            }
            return D::serialize_1d_strided(self.data, self.shape[0], self.strides[0], serializer);
        }

        let len = self.shape[0];
        let mut seq = serializer.serialize_seq(Some(len))?;
        let sub_shape = &self.shape[1..];

        if C_CONTIG {
            let sub_elems = sub_shape.iter().try_fold(1usize, |acc, &dim| {
                acc.checked_mul(dim)
                    .ok_or_else(|| serde::ser::Error::custom("numpy array shape overflow"))
            })?;
            let step = sub_elems
                .checked_mul(D::ITEM_SIZE)
                .ok_or_else(|| serde::ser::Error::custom("numpy array stride overflow"))?;

            for i in 0..len {
                let offset = i
                    .checked_mul(step)
                    .ok_or_else(|| serde::ser::Error::custom("numpy array offset overflow"))?;
                let elem = NdArraySerializer::<D, true>::new(
                    unsafe { self.data.add(offset) },
                    sub_shape,
                    self.strides,
                );
                seq.serialize_element(&elem)?;
            }
        } else {
            let sub_strides = &self.strides[1..];
            for i in 0..len {
                let elem =
                    NdArraySerializer::<D, false>::new(self.element_ptr(i), sub_shape, sub_strides);
                seq.serialize_element(&elem)?;
            }
        }
        seq.end()
    }
}

#[inline]
fn serialize_ndarray_with_dtype_and_layout<S, D, const C_CONTIG: bool>(
    data: *const u8,
    shape: &[usize],
    strides: &[isize],
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    D: StridedDTypeSerializer,
{
    NdArraySerializer::<D, C_CONTIG>::new(data, shape, strides).serialize(serializer)
}

pub(crate) struct PyNumpyArraySerializer<'a, 'py> {
    obj: Borrowed<'a, 'py, PyAny>,
}

impl<'a, 'py> PyNumpyArraySerializer<'a, 'py> {
    #[inline]
    pub(crate) fn new(obj: Borrowed<'a, 'py, PyAny>) -> Self {
        Self { obj }
    }
}

impl Serialize for PyNumpyArraySerializer<'_, '_> {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let buf = PyUntypedBuffer::get(self.obj.as_any()).map_err(pyerr2sererr)?;
        if !is_native_endian(buf.format()) {
            return Err(serde::ser::Error::custom(
                "numpy array is not native-endian",
            ));
        }

        if !buf.is_c_contiguous() && !buf.is_fortran_contiguous() {
            return Err(serde::ser::Error::custom(
                "numpy array is not C- or Fortran-contiguous",
            ));
        }

        if buf.dimensions() > isize::MAX as usize {
            return Err(serde::ser::Error::custom(
                "numpy array has too many dimensions",
            ));
        }

        let dispatch = match DTypeDispatch::from_buffer_format(buf.format(), buf.item_size()) {
            Some(dispatch) => dispatch,
            None => {
                return Err(serde::ser::Error::custom("numpy dtype is not supported"));
            }
        };

        let nd = buf.dimensions();
        let data = buf.buf_ptr() as *const u8;
        if data.is_null() {
            return Err(serde::ser::Error::custom(
                "numpy array data pointer is null",
            ));
        }

        let shape = buf.shape();
        let mut shape_usize = Vec::with_capacity(nd);
        for dim in shape.iter().take(nd) {
            shape_usize.push(*dim);
        }

        let strides = buf.strides();
        let mut strides_isize = Vec::with_capacity(nd);
        for stride in strides.iter().take(nd) {
            strides_isize.push(*stride);
        }

        let c_contig = buf.is_c_contiguous();

        match dispatch {
            DTypeDispatch::Bool => {
                if c_contig {
                    serialize_ndarray_with_dtype_and_layout::<S, BoolDType, true>(
                        data,
                        &shape_usize,
                        &strides_isize,
                        serializer,
                    )
                } else {
                    serialize_ndarray_with_dtype_and_layout::<S, BoolDType, false>(
                        data,
                        &shape_usize,
                        &strides_isize,
                        serializer,
                    )
                }
            }
            DTypeDispatch::I8 => {
                if c_contig {
                    serialize_ndarray_with_dtype_and_layout::<S, I8DType, true>(
                        data,
                        &shape_usize,
                        &strides_isize,
                        serializer,
                    )
                } else {
                    serialize_ndarray_with_dtype_and_layout::<S, I8DType, false>(
                        data,
                        &shape_usize,
                        &strides_isize,
                        serializer,
                    )
                }
            }
            DTypeDispatch::I16 => {
                if c_contig {
                    serialize_ndarray_with_dtype_and_layout::<S, I16DType, true>(
                        data,
                        &shape_usize,
                        &strides_isize,
                        serializer,
                    )
                } else {
                    serialize_ndarray_with_dtype_and_layout::<S, I16DType, false>(
                        data,
                        &shape_usize,
                        &strides_isize,
                        serializer,
                    )
                }
            }
            DTypeDispatch::I32 => {
                if c_contig {
                    serialize_ndarray_with_dtype_and_layout::<S, I32DType, true>(
                        data,
                        &shape_usize,
                        &strides_isize,
                        serializer,
                    )
                } else {
                    serialize_ndarray_with_dtype_and_layout::<S, I32DType, false>(
                        data,
                        &shape_usize,
                        &strides_isize,
                        serializer,
                    )
                }
            }
            DTypeDispatch::I64 => {
                if c_contig {
                    serialize_ndarray_with_dtype_and_layout::<S, I64DType, true>(
                        data,
                        &shape_usize,
                        &strides_isize,
                        serializer,
                    )
                } else {
                    serialize_ndarray_with_dtype_and_layout::<S, I64DType, false>(
                        data,
                        &shape_usize,
                        &strides_isize,
                        serializer,
                    )
                }
            }
            DTypeDispatch::U8 => {
                if c_contig {
                    serialize_ndarray_with_dtype_and_layout::<S, U8DType, true>(
                        data,
                        &shape_usize,
                        &strides_isize,
                        serializer,
                    )
                } else {
                    serialize_ndarray_with_dtype_and_layout::<S, U8DType, false>(
                        data,
                        &shape_usize,
                        &strides_isize,
                        serializer,
                    )
                }
            }
            DTypeDispatch::U16 => {
                if c_contig {
                    serialize_ndarray_with_dtype_and_layout::<S, U16DType, true>(
                        data,
                        &shape_usize,
                        &strides_isize,
                        serializer,
                    )
                } else {
                    serialize_ndarray_with_dtype_and_layout::<S, U16DType, false>(
                        data,
                        &shape_usize,
                        &strides_isize,
                        serializer,
                    )
                }
            }
            DTypeDispatch::U32 => {
                if c_contig {
                    serialize_ndarray_with_dtype_and_layout::<S, U32DType, true>(
                        data,
                        &shape_usize,
                        &strides_isize,
                        serializer,
                    )
                } else {
                    serialize_ndarray_with_dtype_and_layout::<S, U32DType, false>(
                        data,
                        &shape_usize,
                        &strides_isize,
                        serializer,
                    )
                }
            }
            DTypeDispatch::U64 => {
                if c_contig {
                    serialize_ndarray_with_dtype_and_layout::<S, U64DType, true>(
                        data,
                        &shape_usize,
                        &strides_isize,
                        serializer,
                    )
                } else {
                    serialize_ndarray_with_dtype_and_layout::<S, U64DType, false>(
                        data,
                        &shape_usize,
                        &strides_isize,
                        serializer,
                    )
                }
            }
            DTypeDispatch::F16 => {
                // not supported
                Err(serde::ser::Error::custom(
                    "numpy float16 dtype is not supported (tbd)",
                ))
            }
            DTypeDispatch::F32 => {
                if c_contig {
                    serialize_ndarray_with_dtype_and_layout::<S, F32DType, true>(
                        data,
                        &shape_usize,
                        &strides_isize,
                        serializer,
                    )
                } else {
                    serialize_ndarray_with_dtype_and_layout::<S, F32DType, false>(
                        data,
                        &shape_usize,
                        &strides_isize,
                        serializer,
                    )
                }
            }
            DTypeDispatch::F64 => {
                if c_contig {
                    serialize_ndarray_with_dtype_and_layout::<S, F64DType, true>(
                        data,
                        &shape_usize,
                        &strides_isize,
                        serializer,
                    )
                } else {
                    serialize_ndarray_with_dtype_and_layout::<S, F64DType, false>(
                        data,
                        &shape_usize,
                        &strides_isize,
                        serializer,
                    )
                }
            }
        }
    }
}
