use crate::errors::map_py_err;

use crate::ser::SerializePyAny;
use pyo3::prelude::*;
use ryo3_uuid::PyUuid as RyUuid;
use serde::ser::{Serialize, SerializeMap, SerializeSeq};

#[inline(always)]
pub(crate) fn ry_uuid<'py, S>(ser: &SerializePyAny<'py>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let ry_uu = ser.obj.downcast::<RyUuid>().map_err(map_py_err)?;
    ry_uu.borrow().serialize(serializer)
}
