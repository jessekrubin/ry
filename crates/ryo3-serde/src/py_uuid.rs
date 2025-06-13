use crate::errors::map_py_err;
use crate::ser::SerializePyAny;
use pyo3::prelude::*;
use pyo3::prelude::*;
use ryo3_uuid::uuid;
use serde::ser::{SerializeMap, SerializeSeq, Serializer};

pub fn py_uuid<'py, S>(ser: &SerializePyAny<'py>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let uu = ryo3_uuid::CPythonUuid::extract_bound(&ser.obj)
        // .map_err(|e| serde_err!("Failed to extract CPythonUuid: {}", e))
        .map(|u| uuid::Uuid::from(u))
        .map_err(|e| map_py_err(e))?;
    serializer.serialize_str(&uu.hyphenated().to_string())
}
