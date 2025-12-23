use pyo3::prelude::*;
use pyo3::types::PyBool;
use serde::ser::{Serialize, Serializer};

pub(crate) struct PyBoolSerializer<'a, 'py> {
    obj: Borrowed<'a, 'py, PyBool>,
}

impl<'a, 'py> PyBoolSerializer<'a, 'py> {
    #[inline]
    pub(crate) fn new(obj: Borrowed<'a, 'py, PyBool>) -> Self {
        Self { obj }
    }

    #[inline]
    #[expect(unsafe_code)]
    pub(crate) fn new_unchecked(obj: Borrowed<'a, 'py, PyAny>) -> Self {
        let py_bool = unsafe { obj.cast_unchecked::<PyBool>() };
        Self { obj: py_bool }
    }
}

impl<'a, 'py> TryFrom<Borrowed<'a, 'py, PyAny>> for PyBoolSerializer<'a, 'py> {
    type Error = pyo3::CastError<'a, 'py>;

    #[inline]
    fn try_from(value: Borrowed<'a, 'py, PyAny>) -> Result<Self, Self::Error> {
        let py_bool = value.cast_exact::<PyBool>()?;
        Ok(Self::new(py_bool))
    }
}

// new_unsafe
// impl<'a, 'py> PyBoolSerializer<'a, 'py, true> {
//     #[inline]
//     pub(crate) fn new_unsafe(obj: Borrowed<'a, 'py, PyBool>) -> Self {
//         Self { obj }
//     }
// }

impl Serialize for PyBoolSerializer<'_, '_> {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let tf = self.obj.is_true();
        serializer.serialize_bool(tf)
    }
}

// impl Serialize for PyBoolSerializer<'_, '_, true> {
//     #[inline]
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         #[expect(unsafe_code)]
//         unsafe {
//             let istrue = ptr::eq(self.obj.as_ptr(), ffi::Py_True());
//             serializer.serialize_bool(istrue)
//         }
//     }
// }

// impl PySerializeUnsafe for PyBoolSerializer<'_, '_> {
//     fn serialize_unsafe<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         #[expect(unsafe_code)]
//         unsafe {
//             let istrue = ptr::eq(self.obj.as_ptr(), ffi::Py_True());
//             serializer.serialize_bool(istrue)
//         }
//     }
// }
