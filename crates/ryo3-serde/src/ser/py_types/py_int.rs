use pyo3::prelude::*;
use pyo3::types::PyInt;
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
        if let Ok(v) = self.obj.extract::<i64>() {
            serializer.serialize_i64(v)
        } else if let Ok(v) = self.obj.extract::<u64>() {
            serializer.serialize_u64(v)
        } else {
            crate::serde_err!("pyint not representable as i64 or u64")
        }
    }
}
