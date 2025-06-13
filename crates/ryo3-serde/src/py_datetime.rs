use crate::errors::map_py_err;
use crate::ser::SerializePyAny;
use pyo3::prelude::*;
use pyo3::prelude::*;
use pyo3::types::{PyDate, PyDateTime, PyTime};
use pyo3::Bound;
use serde::ser::{SerializeMap, SerializeSeq, Serializer};

pub fn date<'py, S>(ser: &SerializePyAny<'py>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let py_date: &Bound<'_, PyDate> = ser.obj.downcast().map_err(map_py_err)?;
    let date_pystr = py_date.str().map_err(map_py_err)?;
    let date_str = date_pystr.to_str().map_err(map_py_err)?;
    serializer.serialize_str(date_str)
}

pub fn datetime<'py, S>(ser: &SerializePyAny<'py>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let py_dt: &Bound<'_, PyDateTime> = ser.obj.downcast().map_err(map_py_err)?;
    let dt_pystr = py_dt.str().map_err(map_py_err)?;
    let dt_str = dt_pystr.to_str().map_err(map_py_err)?;
    // TODO: use jiff to do all the date-time formatting
    let iso_str = dt_str.replacen("+00:00", "Z", 1).replace(' ', "T");
    serializer.serialize_str(iso_str.as_ref())
}

pub fn time<'py, S>(ser: &SerializePyAny<'py>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let py_time: &Bound<'_, PyTime> = ser.obj.downcast().map_err(map_py_err)?;
    let time_pystr = py_time.str().map_err(map_py_err)?;
    let time_str = time_pystr.to_str().map_err(map_py_err)?;
    serializer.serialize_str(time_str)
}
