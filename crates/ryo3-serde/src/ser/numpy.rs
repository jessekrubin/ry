//! Opt-in `NumPy` scalar and ndarray serialization.
//!
//! `NumPy` is intentionally imported only when the JSON API enables this mode.
//! Exact type pointers are cached and ndarray metadata is obtained from
//! `__array_struct__`; there is no Rust or Python runtime dependency on `NumPy`.

use pyo3::prelude::*;
use ryo3_numpy::NumpyObType;
use serde::ser::{Serialize, Serializer};

mod ndarray {

    use std::ffi::{c_char, c_int};
    use std::marker::PhantomData;
    use std::mem::size_of;
    use std::ptr::NonNull;

    use pyo3::prelude::*;
    use pyo3::types::{PyCapsule, PyCapsuleMethods};
    use ryo3_numpy::{NumpyDType, PyArrayInterface};
    use serde::ser::{Serialize, SerializeSeq, Serializer};
    const MAX_NDARRAY_DIMENSIONS: usize = 32;
    const NPY_ARRAY_C_CONTIGUOUS: c_int = 0x0001;
    const NPY_ARRAY_NOTSWAPPED: c_int = 0x0200;
    use crate::errors::pyerr2sererr;
    enum NpArraySerdeError {
        Static(&'static str),
        UnsupportedDType { kind: c_char, itemsize: c_int },
    }

    impl NpArraySerdeError {
        const MALFORMED_ARRAY_METADATA: Self =
            Self::Static("numpy array has malformed __array_struct__ metadata");
        const ZERO_DIMENSIONAL_ARRAY: Self =
            Self::Static("numpy zero-dimensional arrays are not supported; use a numpy scalar");
        const INVALID_DIMENSION: Self = Self::Static("numpy array dimension is invalid");
        const TOO_MANY_DIMENSIONS: Self =
            Self::Static("numpy arrays with more than 32 dimensions are not supported");
        const NOT_C_CONTIGUOUS: Self = Self::Static("numpy array must be C-contiguous");
        const NOT_NATIVE_ENDIAN: Self = Self::Static("numpy array must have native byte order");
        const NULL_SHAPE: Self = Self::Static("numpy array shape pointer is null");
        const UNSUPPORTED_FLOAT16_DTYPE: Self =
            Self::Static("numpy float16 dtype is not supported");
        const INVALID_SHAPE: Self = Self::Static("numpy array shape is invalid");
        const SHAPE_OVERFLOW: Self = Self::Static("numpy array shape overflow");
        const OFFSET_OVERFLOW: Self = Self::Static("numpy array offset overflow");
        const BYTE_LENGTH_OVERFLOW: Self = Self::Static("numpy array byte length overflow");
        const NULL_DATA: Self = Self::Static("numpy array data pointer is null");
    }

    impl std::fmt::Display for NpArraySerdeError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Static(message) => f.write_str(message),
                Self::UnsupportedDType { kind, itemsize } => write!(
                    f,
                    "numpy dtype is not supported (kind={kind}, itemsize={itemsize})"
                ),
            }
        }
    }

    trait CContigDTypeSerializer<const ALIGNED: bool> {
        fn serialize_1d<S>(data: *const u8, len: usize, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer;
    }
    macro_rules! sererr_return {
        ($err:expr) => {
            return Err(serde::ser::Error::custom($err));
        };
    }

    // This macro avoids a per-element dtype branch while still allowing Serde to
    // inline each primitive's concrete `Serialize` implementation.
    macro_rules! impl_array_serializer_for_dtype {
        ($ty:ty) => {
            impl<const ALIGNED: bool> CContigDTypeSerializer<ALIGNED> for $ty {
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
                    let mut ptr = data.cast::<Self>();
                    for _ in 0..len {
                        let value = if ALIGNED {
                            unsafe { ptr.read() }
                        } else {
                            unsafe { ptr.read_unaligned() }
                        };
                        seq.serialize_element(&value)?;
                        ptr = unsafe { ptr.add(1) };
                    }
                    seq.end()
                }
            }
        };
    }

    impl_array_serializer_for_dtype!(i8);
    impl_array_serializer_for_dtype!(i16);
    impl_array_serializer_for_dtype!(i32);
    impl_array_serializer_for_dtype!(i64);
    impl_array_serializer_for_dtype!(u8);
    impl_array_serializer_for_dtype!(u16);
    impl_array_serializer_for_dtype!(u32);
    impl_array_serializer_for_dtype!(u64);
    impl_array_serializer_for_dtype!(f32);
    impl_array_serializer_for_dtype!(f64);

    impl<const ALIGNED: bool> CContigDTypeSerializer<ALIGNED> for bool {
        #[expect(unsafe_code)]
        #[inline]
        fn serialize_1d<S>(data: *const u8, len: usize, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let mut seq = serializer.serialize_seq(Some(len))?;
            let mut ptr = data;
            for _ in 0..len {
                let value = unsafe { ptr.read() } != 0;
                seq.serialize_element(&value)?;
                ptr = unsafe { ptr.add(1) };
            }

            seq.end()
        }
    }

    pub(super) struct NdArraySerializer<'a, D, const ALIGNED: bool> {
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
                    .ok_or_else(|| serde::ser::Error::custom(NpArraySerdeError::SHAPE_OVERFLOW))
            })?;
            let step = sub_elems
                .checked_mul(size_of::<D>())
                .ok_or_else(|| serde::ser::Error::custom(NpArraySerdeError::OFFSET_OVERFLOW))?;

            let mut seq = serializer.serialize_seq(Some(len))?;
            let mut data = self.data;
            for _ in 0..len {
                let item = Self::new(data, sub_shape);
                seq.serialize_element(&item)?;
                data = data.wrapping_add(step);
            }
            seq.end()
        }
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
        macro_rules! serialize_dtype_array {
            ($dtype:ty) => {
                NdArraySerializer::<$dtype, ALIGNED>::new(data, shape).serialize(serializer)
            };
        }

        match dtype {
            NumpyDType::Bool => serialize_dtype_array!(bool),
            NumpyDType::I8 => serialize_dtype_array!(i8),
            NumpyDType::I16 => serialize_dtype_array!(i16),
            NumpyDType::I32 => serialize_dtype_array!(i32),
            NumpyDType::I64 => serialize_dtype_array!(i64),
            NumpyDType::U8 => serialize_dtype_array!(u8),
            NumpyDType::U16 => serialize_dtype_array!(u16),
            NumpyDType::U32 => serialize_dtype_array!(u32),
            NumpyDType::U64 => serialize_dtype_array!(u64),
            NumpyDType::F16 => Err(serde::ser::Error::custom(
                NpArraySerdeError::UNSUPPORTED_FLOAT16_DTYPE,
            )),
            NumpyDType::F32 => serialize_dtype_array!(f32),
            NumpyDType::F64 => serialize_dtype_array!(f64),
        }
    }

    pub(super) struct PyNumpyArraySerializer<'a, 'py> {
        pub(super) obj: Borrowed<'a, 'py, PyAny>,
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
                sererr_return!(NpArraySerdeError::MALFORMED_ARRAY_METADATA);
            }
            if interface.nd < 1 {
                sererr_return!(NpArraySerdeError::ZERO_DIMENSIONAL_ARRAY);
            }
            let dimensions = usize::try_from(interface.nd)
                .map_err(|_| serde::ser::Error::custom(NpArraySerdeError::INVALID_DIMENSION))?;
            if dimensions > MAX_NDARRAY_DIMENSIONS {
                sererr_return!(NpArraySerdeError::TOO_MANY_DIMENSIONS);
            }
            if interface.flags & NPY_ARRAY_C_CONTIGUOUS == 0 {
                sererr_return!(NpArraySerdeError::NOT_C_CONTIGUOUS);
            }
            if interface.flags & NPY_ARRAY_NOTSWAPPED == 0 {
                sererr_return!(NpArraySerdeError::NOT_NATIVE_ENDIAN);
            }
            if interface.shape.is_null() {
                sererr_return!(NpArraySerdeError::NULL_SHAPE);
            }

            let dtype =
                NumpyDType::from_typekind_and_itemsize(interface.typekind, interface.itemsize)
                    .ok_or_else(|| {
                        serde::ser::Error::custom(NpArraySerdeError::UnsupportedDType {
                            kind: interface.typekind,
                            itemsize: interface.itemsize,
                        })
                    })?;

            let raw_shape = unsafe { std::slice::from_raw_parts(interface.shape, dimensions) };
            // numpy sometimes has a 32 dim lim
            let mut shape_storage = [0usize; MAX_NDARRAY_DIMENSIONS];
            let mut element_count = 1usize;
            for (index, &dimension) in raw_shape.iter().enumerate() {
                let dimension = usize::try_from(dimension)
                    .map_err(|_| serde::ser::Error::custom(NpArraySerdeError::INVALID_SHAPE))?;
                element_count = element_count
                    .checked_mul(dimension)
                    .ok_or_else(|| serde::ser::Error::custom(NpArraySerdeError::SHAPE_OVERFLOW))?;
                shape_storage[index] = dimension;
            }
            element_count.checked_mul(dtype.itemsize()).ok_or_else(|| {
                serde::ser::Error::custom(NpArraySerdeError::BYTE_LENGTH_OVERFLOW)
            })?;

            // SO this shit was a thing that was confusing to figure out, but
            // you need that `NonNull::<u8>::dangling().as_ptr()` for empty arrays
            let data = if element_count == 0 {
                NonNull::<u8>::dangling().as_ptr().cast_const()
            } else {
                if interface.data.is_null() {
                    sererr_return!(NpArraySerdeError::NULL_DATA);
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
}
mod scalar {
    use pyo3::prelude::*;
    use ryo3_numpy::NumpyDType;
    use serde::ser::{Serialize, Serializer};
    pub(super) struct PyNumpyScalarSerializer<'a, 'py> {
        pub(super) obj: Borrowed<'a, 'py, PyAny>,
        pub(super) dtype: NumpyDType,
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
                    .add(size_of::<pyo3::ffi::PyObject>())
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
}

pub(crate) struct PyNumpySerializer<'a, 'py> {
    obj: Borrowed<'a, 'py, PyAny>,
    obtype: NumpyObType,
}

impl<'a, 'py> PyNumpySerializer<'a, 'py> {
    pub(crate) fn new(obj: Borrowed<'a, 'py, PyAny>, obtype: NumpyObType) -> Self {
        Self { obj, obtype }
    }
}

impl Serialize for PyNumpySerializer<'_, '_> {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self.obtype {
            NumpyObType::Array => {
                ndarray::PyNumpyArraySerializer { obj: self.obj }.serialize(serializer)
            }
            NumpyObType::Scalar(dtype) => scalar::PyNumpyScalarSerializer {
                obj: self.obj,
                dtype,
            }
            .serialize(serializer),
        }
    }
}
