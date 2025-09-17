use pyo3::prelude::*;
use serde::ser::{Serialize, Serializer};

use crate::errors::pyerr2sererr;

use pyo3::Bound;
use pyo3::types::{PyDate, PyDateTime, PyTime, PyTzInfoAccess};

// ---------------------------------------------------------------------------
// python stdlib `datetime.date`
// ---------------------------------------------------------------------------
pub(crate) struct SerializePyDate<'a, 'py> {
    obj: &'a Bound<'py, PyAny>,
}

impl<'a, 'py> SerializePyDate<'a, 'py> {
    pub(crate) fn new(obj: &'a Bound<'py, PyAny>) -> Self {
        Self { obj }
    }
}

impl Serialize for SerializePyDate<'_, '_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let py_date: &Bound<'_, PyDate> = self.obj.cast().map_err(pyerr2sererr)?;
        let date_pystr = py_date.str().map_err(pyerr2sererr)?;
        let date_str = date_pystr.to_str().map_err(pyerr2sererr)?;
        serializer.serialize_str(date_str)
    }
}

// ---------------------------------------------------------------------------
// python stdlib `datetime.date`
// ---------------------------------------------------------------------------
pub(crate) struct SerializePyTime<'a, 'py> {
    obj: &'a Bound<'py, PyAny>,
}

impl<'a, 'py> SerializePyTime<'a, 'py> {
    pub(crate) fn new(obj: &'a Bound<'py, PyAny>) -> Self {
        Self { obj }
    }
}

impl Serialize for SerializePyTime<'_, '_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let py_time: &Bound<'_, PyTime> = self.obj.cast().map_err(pyerr2sererr)?;
        let time_pystr = py_time.str().map_err(pyerr2sererr)?;
        let time_str = time_pystr.to_str().map_err(pyerr2sererr)?;
        serializer.serialize_str(time_str)
    }
}

// ---------------------------------------------------------------------------
// python stdlib `datetime.datetime`
// ---------------------------------------------------------------------------
pub(crate) struct SerializePyDateTime<'a, 'py> {
    obj: &'a Bound<'py, PyAny>,
}

impl<'a, 'py> SerializePyDateTime<'a, 'py> {
    pub(crate) fn new(obj: &'a Bound<'py, PyAny>) -> Self {
        Self { obj }
    }
}
#[cfg(feature = "jiff")]
impl Serialize for SerializePyDateTime<'_, '_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let py_dt: &Bound<'_, PyDateTime> = self.obj.cast().map_err(pyerr2sererr)?;
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
}

#[cfg(not(feature = "jiff"))]
impl Serialize for SerializePyDateTime<'_, '_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let py_dt: &Bound<'_, PyDateTime> = ser.obj.cast().map_err(pyerr2sererr)?;
        let dt_pystr = py_dt.str().map_err(pyerr2sererr)?;
        let dt_str = dt_pystr.to_str().map_err(pyerr2sererr)?;
        // TODO: use jiff to do all the date-time formatting
        let iso_str = dt_str.replacen("+00:00", "Z", 1).replace(' ', "T");
        serializer.serialize_str(iso_str.as_ref())
    }
}

// ---------------------------------------------------------------------------
// python stdlib `datetime.timedelta`
// ---------------------------------------------------------------------------
pub(crate) struct SerializePyTimeDelta<'a, 'py> {
    obj: &'a Bound<'py, PyAny>,
}
impl<'a, 'py> SerializePyTimeDelta<'a, 'py> {
    pub(crate) fn new(obj: &'a Bound<'py, PyAny>) -> Self {
        Self { obj }
    }
}

#[cfg(feature = "jiff")]
impl Serialize for SerializePyTimeDelta<'_, '_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let py_timedelta: &Bound<'_, pyo3::types::PyDelta> =
            self.obj.cast().map_err(pyerr2sererr)?;
        let signed_duration: jiff::SignedDuration = py_timedelta.extract().map_err(pyerr2sererr)?;
        signed_duration.serialize(serializer)
    }
}

#[cfg(not(feature = "jiff"))]
impl Serialize for SerializePyTimeDelta<'_, '_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        Err(SerError::custom(
            "timedelta serialization requires the jiff feature",
        ))
    }
}
