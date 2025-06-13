use crate::errors::map_py_err;

use crate::ser::SerializePyAny;
use pyo3::prelude::*;
use ryo3_uuid::PyUuid as RyUuid;
use serde::ser::Serialize;

#[inline]
pub(crate) fn ry_uuid<S>(ser: &SerializePyAny<'_>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let ry_uu = ser.obj.downcast::<RyUuid>().map_err(map_py_err)?;
    ry_uu.borrow().serialize(serializer)
}
