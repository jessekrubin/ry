use pyo3::Borrowed;
use pyo3::prelude::*;
use serde::ser::{Serialize, Serializer};

pub(crate) struct PyBoolSerializer<'a, 'py> {
    obj: Borrowed<'a, 'py, pyo3::types::PyBool>,
}

impl<'a, 'py> PyBoolSerializer<'a, 'py> {
    #[inline]
    pub(crate) fn new(obj: Borrowed<'a, 'py, pyo3::types::PyBool>) -> Self {
        Self { obj }
    }

    #[inline]
    pub(crate) fn new_unchecked(obj: Borrowed<'a, 'py, PyAny>) -> Self {
        #[expect(unsafe_code)]
        let obj = unsafe { obj.cast_unchecked::<pyo3::types::PyBool>() };
        Self::new(obj)
    }
}

#[cfg(not(any(PyPy, GraalPy, Py_LIMITED_API)))]
impl Serialize for PyBoolSerializer<'_, '_> {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        #[expect(unsafe_code)]
        unsafe {
            let istrue = std::ptr::eq(self.obj.as_ptr(), pyo3::ffi::Py_True());
            serializer.serialize_bool(istrue)
        }
    }
}

#[cfg(any(PyPy, GraalPy, Py_LIMITED_API))]
impl Serialize for PyBoolSerializer<'_, '_> {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_bool(self.obj.is_true())
    }
}
