use pyo3::prelude::*;
use serde::ser::{Serialize, Serializer};

use crate::errors::pyerr2sererr;

pub(crate) struct PyUuidSerializer<'a, 'py> {
    obj: Borrowed<'a, 'py, PyAny>,
}

impl<'a, 'py> PyUuidSerializer<'a, 'py> {
    #[inline]
    pub(crate) fn new(obj: Borrowed<'a, 'py, PyAny>) -> Self {
        Self { obj }
    }
}
/// Extract a uuid from a pyobject we KNOW is a uuid.
fn extract_uuid(obj: Borrowed<'_, '_, PyAny>) -> PyResult<uuid::Uuid> {
    let py = obj.py();
    let value = obj.getattr(pyo3::intern!(py, "int"))?;
    let value = value.extract::<u128>()?;
    Ok(uuid::Uuid::from_u128(value))
}

impl Serialize for PyUuidSerializer<'_, '_> {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        extract_uuid(self.obj)
            .map_err(pyerr2sererr)?
            .serialize(serializer)
    }
}
