use crate::errors::map_py_err;
use crate::ser::SerializePyAny;
use pyo3::prelude::*;
use pyo3::types::{PyBool, PyInt, PyString};
use serde::Serialize;

pub fn none<'py, S>(ser: &SerializePyAny<'py>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    if let Some(none_value) = ser.none_value {
        serializer.serialize_str(none_value)
    } else {
        // if no none_value is set, serialize as None
        serializer.serialize_none()
    }
}

pub fn bool<'py, S>(ser: &SerializePyAny<'py>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let py_bool = ser.obj.downcast::<PyBool>().map_err(map_py_err)?;
    let v: bool = py_bool.is_true();
    serializer.serialize_bool(v)
}

pub fn int<'py, S>(ser: &SerializePyAny<'py>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let v: i64 = ser
        .obj
        .downcast::<PyInt>()
        .map_err(map_py_err)?
        .extract()
        .map_err(map_py_err)?;
    serializer.serialize_i64(v)
}

pub fn float<'py, S>(ser: &SerializePyAny<'py>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let v: f64 = ser.obj.extract().map_err(map_py_err)?;
    serializer.serialize_f64(v)
}

pub fn str<'py, S>(ser: &SerializePyAny<'py>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let py_str: &Bound<'_, PyString> = ser.obj.downcast().map_err(map_py_err)?;
    let s = py_str.to_str().map_err(map_py_err)?;
    serializer.serialize_str(s)
}

pub fn byteslike<'py, S>(ser: &SerializePyAny<'py>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    match ser.obj.extract::<&[u8]>() {
        Ok(v) => v.serialize(serializer),
        Err(e) => Err(map_py_err(e)),
    }
    // let py_bytes = ser.obj.downcast::<PyBytes>().map_err(map_py_err)?;
    // let bytes = py_bytes.as_bytes();
    // serializer.serialize_bytes(bytes)
}
