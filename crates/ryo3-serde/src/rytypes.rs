use crate::errors::pyerr2sererr;

use crate::py_serialize::SerializePyAny;
use pyo3::prelude::*;
use ryo3_uuid::PyUuid as RyUuid;
use serde::ser::Serialize;

#[inline]
pub(crate) fn ry_uuid<S>(ser: &SerializePyAny<'_>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let ry_uu = ser.obj.downcast::<RyUuid>().map_err(pyerr2sererr)?;
    ry_uu.borrow().serialize(serializer)
}
