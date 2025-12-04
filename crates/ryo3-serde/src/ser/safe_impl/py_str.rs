use pyo3::prelude::*;
use serde::ser::{Serialize, Serializer};

use crate::errors::pyerr2sererr;
use pyo3::Bound;
use pyo3::types::PyString;

pub(crate) struct SerializePyStr<'a, 'py> {
    obj: &'a Bound<'py, PyAny>,
}

impl<'a, 'py> SerializePyStr<'a, 'py> {
    pub(crate) fn new(obj: &'a Bound<'py, PyAny>) -> Self {
        Self { obj }
    }
}

impl Serialize for SerializePyStr<'_, '_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let py_str = self.obj.cast_exact::<PyString>().map_err(pyerr2sererr)?;
        let s = py_str.to_str().map_err(pyerr2sererr)?;
        serializer.serialize_str(s)
    }
}

pub(crate) struct SerializePyStrSubclass<'a, 'py> {
    obj: &'a Bound<'py, PyString>,
}

impl<'a, 'py> SerializePyStrSubclass<'a, 'py> {
    pub(crate) fn new(obj: &'a Bound<'py, PyString>) -> Self {
        Self { obj }
    }
}

impl Serialize for SerializePyStrSubclass<'_, '_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = self.obj.to_str().map_err(pyerr2sererr)?;
        serializer.serialize_str(s)
    }
}
// impl Serialize for SerializePyStr<'_, '_> {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         let py_str: &Bound<'_, PyString> = unsafe { self.obj.cast_unchecked() };

//         #[expect(unsafe_code)]
//         unsafe {
//             let mut size: isize = 0;
//             let ptr = pyo3::ffi::PyUnicode_AsUTF8AndSize(py_str.as_ptr(), &mut size);
//             if ptr.is_null() {
//                 // Turn the Python error into SerError once; this path should be very rare.
//                 return Err(pyerr2sererr(PyErr::fetch(py_str.py())));
//             }
//             let slice = std::slice::from_raw_parts(ptr as *const u8, size as usize);
//             let s = std::str::from_utf8_unchecked(slice); // guaranteed by CPython for UTF-8 repr
//             serializer.serialize_str(s)
//         }
//     }
// }
