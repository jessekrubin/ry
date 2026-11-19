use pyo3::prelude::*;
use pyo3::types::PyInt;
use ryo3_core::{PyIntRead, PyIntValue};
use serde::ser::{Serialize, Serializer};

pub(crate) struct PyIntSerializer<'a, 'py> {
    obj: Borrowed<'a, 'py, PyInt>,
}

impl<'a, 'py> PyIntSerializer<'a, 'py> {
    #[inline]
    pub(crate) fn new(obj: Borrowed<'a, 'py, PyInt>) -> Self {
        Self { obj }
    }

    #[inline]
    pub(crate) fn new_unchecked(obj: Borrowed<'a, 'py, PyAny>) -> Self {
        #[expect(unsafe_code)]
        let obj = unsafe { obj.cast_unchecked::<PyInt>() };
        Self::new(obj)
    }
}

impl Serialize for PyIntSerializer<'_, '_> {
    #[inline(always)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self.obj.read_int() {
            Some(PyIntValue::I64(v)) => serializer.serialize_i64(v),
            Some(PyIntValue::U64(v)) => serializer.serialize_u64(v),
            None => crate::serde_err!("pyint not representable as i64 or u64"),
        }
    }
}
