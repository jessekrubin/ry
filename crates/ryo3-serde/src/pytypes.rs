use crate::errors::map_py_err;

use crate::any_repr::any_repr;
use crate::ser::SerializePyAny;
use crate::serde_err;
use pyo3::prelude::*;
use pyo3::types::{PyBool, PyDate, PyDateTime, PyDict, PyInt, PyString, PyTime};
use pyo3::types::{PyList, PyTuple};
use pyo3::Bound;
use ryo3_uuid::uuid;
use serde::ser::{Error as SerError, Serialize, SerializeMap, SerializeSeq};

#[inline]
pub(crate) fn none<S>(ser: &SerializePyAny<'_>, serializer: S) -> Result<S::Ok, S::Error>
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

#[inline]
pub(crate) fn bool_<S>(ser: &SerializePyAny<'_>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let py_bool = ser.obj.downcast::<PyBool>().map_err(map_py_err)?;
    let v: bool = py_bool.is_true();
    serializer.serialize_bool(v)
}

#[inline]
pub(crate) fn int<S>(ser: &SerializePyAny<'_>, serializer: S) -> Result<S::Ok, S::Error>
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

#[inline]
pub(crate) fn float<S>(ser: &SerializePyAny<'_>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let v: f64 = ser.obj.extract().map_err(map_py_err)?;
    serializer.serialize_f64(v)
}

#[inline]
pub(crate) fn str<S>(ser: &SerializePyAny<'_>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let py_str: &Bound<'_, PyString> = ser.obj.downcast().map_err(map_py_err)?;
    let s = py_str.to_str().map_err(map_py_err)?;
    serializer.serialize_str(s)
}

#[inline]
pub(crate) fn byteslike<S>(ser: &SerializePyAny<'_>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    match ser.obj.extract::<&[u8]>() {
        Ok(v) => v.serialize(serializer),
        Err(e) => Err(map_py_err(e)),
    }
}

// ============================================================================
// LIST
// ============================================================================
#[inline]
pub(crate) fn list<S>(ser: &SerializePyAny<'_>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let py_list: &Bound<'_, PyList> = ser.obj.downcast().map_err(map_py_err)?;
    let len = py_list.len();
    if len == 0 {
        serializer.serialize_seq(Some(0))?.end()
    } else {
        let mut seq = serializer.serialize_seq(Some(len))?;
        for element in py_list {
            seq.serialize_element(&ser.with_obj(element))?;
        }
        seq.end()
    }
}

// ============================================================================
// TUPLE
// ============================================================================
#[inline]
#[expect(clippy::similar_names)]
pub(crate) fn tuple<S>(ser: &SerializePyAny<'_>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let py_tuple: &Bound<'_, PyTuple> = ser.obj.downcast().map_err(map_py_err)?;
    let len = py_tuple.len();
    let mut seq = serializer.serialize_seq(Some(len))?;
    for element in py_tuple {
        // if self.none_value.is_some() || !element.is_none() {
        seq.serialize_element(&ser.with_obj(element))?;
        // }
    }
    seq.end()
}

// ============================================================================
// DICT
// ============================================================================

#[inline]
fn mapping_key<'py, E: SerError>(key: &'py Bound<'py, PyAny>) -> Result<&'py str, E> {
    if let Ok(py_string) = key.downcast::<PyString>() {
        py_string.to_str().map_err(map_py_err)
    } else if let Ok(key) = key.extract::<bool>() {
        Ok(if key { "true" } else { "false" })
    } else {
        let key_repr = any_repr(key);
        serde_err!("{} is not JSON-serializable as map-key", key_repr)
    }
}

#[inline]
pub(crate) fn dict<S>(ser: &SerializePyAny<'_>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let py_dict: &Bound<'_, PyDict> = ser.obj.downcast().map_err(map_py_err)?;
    let len = py_dict.len();
    if len == 0 {
        return serializer.serialize_map(Some(0))?.end();
    }
    let mut m = serializer.serialize_map(Some(len))?;
    for (k, v) in py_dict {
        m.serialize_entry(mapping_key(&k)?, &ser.with_obj(v))?;
    }
    m.end()
}

// ============================================================================
// uuid.UUID
// ============================================================================
#[inline]
pub(crate) fn py_uuid<S>(ser: &SerializePyAny<'_>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let uu = ryo3_uuid::CPythonUuid::extract_bound(&ser.obj)
        // .map_err(|e| serde_err!("Failed to extract CPythonUuid: {}", e))
        .map(uuid::Uuid::from)
        .map_err(map_py_err)?;
    serializer.serialize_str(&uu.hyphenated().to_string())
}

// ============================================================================
// datetime.date
// ============================================================================
pub(crate) fn date<S>(ser: &SerializePyAny<'_>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let py_date: &Bound<'_, PyDate> = ser.obj.downcast().map_err(map_py_err)?;
    let date_pystr = py_date.str().map_err(map_py_err)?;
    let date_str = date_pystr.to_str().map_err(map_py_err)?;
    serializer.serialize_str(date_str)
}

// ============================================================================
// datetime.datetime
// ============================================================================
#[inline]
pub(crate) fn datetime<S>(ser: &SerializePyAny<'_>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let py_dt: &Bound<'_, PyDateTime> = ser.obj.downcast().map_err(map_py_err)?;
    let dt_pystr = py_dt.str().map_err(map_py_err)?;
    let dt_str = dt_pystr.to_str().map_err(map_py_err)?;
    // TODO: use jiff to do all the date-time formatting
    let iso_str = dt_str.replacen("+00:00", "Z", 1).replace(' ', "T");
    serializer.serialize_str(iso_str.as_ref())
}

// ============================================================================
// datetime.time
// ============================================================================
#[inline]
pub(crate) fn time<S>(ser: &SerializePyAny<'_>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let py_time: &Bound<'_, PyTime> = ser.obj.downcast().map_err(map_py_err)?;
    let time_pystr = py_time.str().map_err(map_py_err)?;
    let time_str = time_pystr.to_str().map_err(map_py_err)?;
    serializer.serialize_str(time_str)
}
