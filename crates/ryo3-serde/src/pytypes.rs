use crate::errors::pyerr2sererr;

use crate::any_repr::any_repr;
use crate::py_serialize::SerializePyAny;
use crate::serde_err;
use pyo3::Bound;
use pyo3::prelude::*;
use pyo3::types::{
    PyBool, PyDate, PyDateTime, PyDict, PyFrozenSet, PyInt, PyIterator, PySet, PyString, PyTime,
    PyTzInfoAccess,
};
use pyo3::types::{PyList, PyTuple};
use serde::ser::{Error as SerError, Serialize, SerializeMap, SerializeSeq};

#[inline]
pub(crate) fn none<S>(_ser: &SerializePyAny<'_>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    // if let Some(none_value) = ser.none_value {
    //     serializer.serialize_str(none_value)
    // } else {
    // if no none_value is set, serialize as None
    serializer.serialize_none()
    // }
}

#[inline]
pub(crate) fn bool_<S>(ser: &SerializePyAny<'_>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let py_bool = ser.obj.downcast::<PyBool>().map_err(pyerr2sererr)?;
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
        .map_err(pyerr2sererr)?
        .extract()
        .map_err(pyerr2sererr)?;
    serializer.serialize_i64(v)
}

#[inline]
pub(crate) fn float<S>(ser: &SerializePyAny<'_>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let v: f64 = ser.obj.extract().map_err(pyerr2sererr)?;
    serializer.serialize_f64(v)
}

#[inline]
pub(crate) fn str<S>(ser: &SerializePyAny<'_>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let py_str: &Bound<'_, PyString> = ser.obj.downcast().map_err(pyerr2sererr)?;
    let s = py_str.to_str().map_err(pyerr2sererr)?;
    serializer.serialize_str(s)
}

// #[inline]
// pub(crate) fn str<S>(ser: &SerializePyAny<'_>, serializer: S) -> Result<S::Ok, S::Error>
// where
//     S: serde::Serializer,
// {
//     let py_str = ser.obj.extract::<PyBackedStr>().map_err(pyerr2sererr)?;
//     let s: &str = py_str.as_ref();
//     serializer.serialize_str(s)
// }

#[inline]
pub(crate) fn byteslike<S>(ser: &SerializePyAny<'_>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    match ser.obj.extract::<&[u8]>() {
        Ok(v) => v.serialize(serializer),
        Err(e) => Err(pyerr2sererr(e)),
    }
}

// ============================================================================
// LIST
// ============================================================================
#[inline]
#[expect(clippy::similar_names)]
pub(crate) fn list<S>(ser: &SerializePyAny<'_>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let py_list: &Bound<'_, PyList> = ser.obj.downcast().map_err(pyerr2sererr)?;
    let len = py_list.len();
    if len == 0 {
        serializer.serialize_seq(Some(0))?.end()
    } else {
        let mut seq = serializer.serialize_seq(Some(len))?;
        for element in py_list {
            seq.serialize_element(&ser.with_obj(&element))?;
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
    let py_tuple: &Bound<'_, PyTuple> = ser.obj.downcast().map_err(pyerr2sererr)?;
    let len = py_tuple.len();
    let mut seq = serializer.serialize_seq(Some(len))?;
    for element in py_tuple {
        seq.serialize_element(&ser.with_obj(&element))?;
    }
    seq.end()
}

// ============================================================================
// DICT
// ============================================================================

#[inline]
pub(crate) fn mapping_key<'py, E: SerError>(key: &'py Bound<'py, PyAny>) -> Result<&'py str, E> {
    if let Ok(py_string) = key.downcast::<PyString>() {
        py_string.to_str().map_err(pyerr2sererr)
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
    let py_dict: &Bound<'_, PyDict> = ser.obj.downcast().map_err(pyerr2sererr)?;
    let len = py_dict.len();
    if len == 0 {
        return serializer.serialize_map(Some(0))?.end();
    }
    let mut m = serializer.serialize_map(Some(len))?;
    for (k, v) in py_dict {
        m.serialize_entry(mapping_key(&k)?, &ser.with_obj(&v))?;
    }
    m.end()
}

// ============================================================================
// SET and FROZENSET
// ============================================================================
#[inline]
#[expect(clippy::similar_names)]
pub(crate) fn set<S>(ser: &SerializePyAny<'_>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let py_set: &Bound<'_, PyAny> = ser.obj.downcast::<PySet>().map_err(pyerr2sererr)?;
    let len = py_set.len().map_err(pyerr2sererr)?;
    if len == 0 {
        return serializer.serialize_seq(Some(0))?.end();
    }
    let py_iter = PyIterator::from_object(py_set).expect("set is always iterable");
    let mut seq = serializer.serialize_seq(Some(len))?;
    for element in py_iter {
        let pyany = element.map_err(pyerr2sererr)?;
        seq.serialize_element(&ser.with_obj(&pyany))?;
    }
    seq.end()
}

#[inline]
#[expect(clippy::similar_names)]
pub(crate) fn frozenset<S>(ser: &SerializePyAny<'_>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let py_frozenset: &Bound<'_, PyAny> =
        ser.obj.downcast::<PyFrozenSet>().map_err(pyerr2sererr)?;
    let len = py_frozenset.len().map_err(pyerr2sererr)?;
    if len == 0 {
        return serializer.serialize_seq(Some(0))?.end();
    }
    let py_iter = PyIterator::from_object(py_frozenset).expect("frozenset is always iterable");
    let mut seq = serializer.serialize_seq(Some(len))?;
    for element in py_iter {
        let pyany = element.map_err(pyerr2sererr)?;
        seq.serialize_element(&ser.with_obj(&pyany))?;
    }
    seq.end()
}
// ============================================================================
// uuid.UUID
// ============================================================================
#[inline]
pub(crate) fn py_uuid<S>(ser: &SerializePyAny<'_>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let uu: uuid::Uuid = ser.obj.extract().map_err(pyerr2sererr)?;
    serializer.serialize_str(&uu.hyphenated().to_string())
}

// ============================================================================
// datetime.date
// ============================================================================
pub(crate) fn date<S>(ser: &SerializePyAny<'_>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let py_date: &Bound<'_, PyDate> = ser.obj.downcast().map_err(pyerr2sererr)?;
    let date_pystr = py_date.str().map_err(pyerr2sererr)?;
    let date_str = date_pystr.to_str().map_err(pyerr2sererr)?;
    serializer.serialize_str(date_str)
}

// ============================================================================
// datetime.datetime
// ============================================================================
#[cfg(feature = "jiff")]
#[inline]
pub(crate) fn datetime<S>(ser: &SerializePyAny<'_>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let py_dt: &Bound<'_, PyDateTime> = ser.obj.downcast().map_err(pyerr2sererr)?;
    // has tz?
    // let has_tzinfo = dt.get_tzinfo().is_some();
    if let Some(_tzinfo) = py_dt.get_tzinfo() {
        let zdt: jiff::Zoned = py_dt.extract().map_err(pyerr2sererr)?;
        zdt.serialize(serializer)
    } else {
        // if no tzinfo, use jiff to serialize
        let dt: jiff::civil::DateTime = py_dt.extract().map_err(pyerr2sererr)?;
        dt.serialize(serializer)
    }
}

#[cfg(not(feature = "jiff"))]
#[inline]
pub(crate) fn datetime<S>(ser: &SerializePyAny<'_>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let py_dt: &Bound<'_, PyDateTime> = ser.obj.downcast().map_err(pyerr2sererr)?;
    let dt_pystr = py_dt.str().map_err(pyerr2sererr)?;
    let dt_str = dt_pystr.to_str().map_err(pyerr2sererr)?;
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
    let py_time: &Bound<'_, PyTime> = ser.obj.downcast().map_err(pyerr2sererr)?;
    let time_pystr = py_time.str().map_err(pyerr2sererr)?;
    let time_str = time_pystr.to_str().map_err(pyerr2sererr)?;
    serializer.serialize_str(time_str)
}

// ============================================================================
// datetime.timedelta
// ============================================================================
#[cfg(feature = "jiff")]
#[inline]
pub(crate) fn timedelta<S>(ser: &SerializePyAny<'_>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let py_timedelta: &Bound<'_, pyo3::types::PyDelta> =
        ser.obj.downcast().map_err(pyerr2sererr)?;
    let signed_duration: jiff::SignedDuration = py_timedelta.extract().map_err(pyerr2sererr)?;
    signed_duration.serialize(serializer)
}

#[cfg(not(feature = "jiff"))]
#[inline]
pub(crate) fn timedelta<S>(_ser: &SerializePyAny<'_>, _serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    Err(SerError::custom(
        "timedelta serialization requires the jiff feature",
    ))
}

// ============================================================================
// datetime.tzinfo
// ============================================================================
// #[cfg(feature = "jiff")]
// #[inline]
// pub(crate) fn tzinfo<S>(ser: &SerializePyAny<'_>, serializer: S) -> Result<S::Ok, S::Error>
// where
//     S: serde::Serializer,
// {
//     println!("{:?}", ser.obj);
//     let tz: jiff::tz::TimeZone = ser.obj.extract().map_err(pyerr2sererr)?;
//     jiff::fmt::serde::tz::required::serialize(&tz, serializer)
// }
//
// #[cfg(not(feature = "jiff"))]
// #[inline]
// pub(crate) fn tzinfo<S>(_ser: &SerializePyAny<'_>, _serializer: S) -> Result<S::Ok, S::Error>
// where
//     S: serde::Serializer,
// {
//     Err(SerError::custom(
//         "tzinfo serialization requires the jiff feature",
//     ))
// }
