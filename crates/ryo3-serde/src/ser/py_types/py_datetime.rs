use pyo3::prelude::*;
use pyo3::types::{PyDate, PyDateTime, PyDelta, PyTime};
use serde::ser::{Serialize, Serializer};

use crate::errors::pyerr2sererr;

// ---------------------------------------------------------------------------
// python stdlib `datetime.date`
// ---------------------------------------------------------------------------
pub(crate) struct PyDateSerializer<'a, 'py> {
    obj: Borrowed<'a, 'py, PyDate>,
}

impl<'a, 'py> PyDateSerializer<'a, 'py> {
    #[inline]
    pub(crate) fn new(obj: Borrowed<'a, 'py, PyDate>) -> Self {
        Self { obj }
    }

    #[inline]
    pub(crate) fn new_unchecked(obj: Borrowed<'a, 'py, PyAny>) -> Self {
        #[expect(unsafe_code)]
        let obj = unsafe { obj.cast_unchecked::<PyDate>() };
        Self::new(obj)
    }
}

#[cfg(feature = "jiff")]
impl Serialize for PyDateSerializer<'_, '_> {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let date_jiff: jiff::civil::Date = self.obj.extract().map_err(pyerr2sererr)?;
        date_jiff.serialize(serializer)
    }
}

#[cfg(not(feature = "jiff"))]
impl Serialize for PyDateSerializer<'_, '_> {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let date_pystr = self.obj.str().map_err(pyerr2sererr)?;
        let date_str = date_pystr.to_str().map_err(pyerr2sererr)?;
        serializer.serialize_str(date_str)
    }
}

// ---------------------------------------------------------------------------
// python stdlib `datetime.time`
// ---------------------------------------------------------------------------
pub(crate) struct PyTimeSerializer<'a, 'py> {
    obj: Borrowed<'a, 'py, PyTime>,
}

impl<'a, 'py> PyTimeSerializer<'a, 'py> {
    #[inline]
    pub(crate) fn new(obj: Borrowed<'a, 'py, PyTime>) -> Self {
        Self { obj }
    }

    #[inline]
    pub(crate) fn new_unchecked(obj: Borrowed<'a, 'py, PyAny>) -> Self {
        #[expect(unsafe_code)]
        let obj = unsafe { obj.cast_unchecked::<PyTime>() };
        Self::new(obj)
    }
}

#[cfg(feature = "jiff")]
impl Serialize for PyTimeSerializer<'_, '_> {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let time_jiff: jiff::civil::Time = self.obj.extract().map_err(pyerr2sererr)?;
        time_jiff.serialize(serializer)
    }
}

#[cfg(not(feature = "jiff"))]
impl Serialize for PyTimeSerializer<'_, '_> {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let time_pystr = self.obj.str().map_err(pyerr2sererr)?;
        let time_str = time_pystr.to_str().map_err(pyerr2sererr)?;
        serializer.serialize_str(time_str)
    }
}

// ---------------------------------------------------------------------------
// python stdlib `datetime.datetime`
// ---------------------------------------------------------------------------
pub(crate) struct PyDateTimeSerializer<'a, 'py> {
    obj: Borrowed<'a, 'py, PyDateTime>,
}

impl<'a, 'py> PyDateTimeSerializer<'a, 'py> {
    #[inline]
    pub(crate) fn new(obj: Borrowed<'a, 'py, PyDateTime>) -> Self {
        Self { obj }
    }

    #[inline]
    pub(crate) fn new_unchecked(obj: Borrowed<'a, 'py, PyAny>) -> Self {
        #[expect(unsafe_code)]
        let obj = unsafe { obj.cast_unchecked::<PyDateTime>() };
        Self::new(obj)
    }
}

#[cfg(feature = "jiff")]
impl Serialize for PyDateTimeSerializer<'_, '_> {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use pyo3::types::PyTzInfoAccess;
        // has tz?
        // let has_tzinfo = dt.get_tzinfo().is_some();
        if let Some(_tzinfo) = self.obj.get_tzinfo() {
            let zdt: jiff::Zoned = self.obj.extract().map_err(pyerr2sererr)?;
            zdt.serialize(serializer)
        } else {
            // if no tzinfo, use jiff to serialize
            let dt: jiff::civil::DateTime = self.obj.extract().map_err(pyerr2sererr)?;
            dt.serialize(serializer)
        }
    }
}

#[cfg(not(feature = "jiff"))]
impl Serialize for PyDateTimeSerializer<'_, '_> {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let dt_pystr = self.obj.str().map_err(pyerr2sererr)?;
        let dt_str = dt_pystr.to_str().map_err(pyerr2sererr)?;
        // TODO: use jiff to do all the date-time formatting
        let iso_str = dt_str.replacen("+00:00", "Z", 1).replace(' ', "T");
        serializer.serialize_str(iso_str.as_ref())
    }
}

// ---------------------------------------------------------------------------
// python stdlib `datetime.timedelta`
// ---------------------------------------------------------------------------
#[cfg_attr(not(feature = "jiff"), expect(dead_code))]
pub(crate) struct PyTimeDeltaSerializer<'a, 'py> {
    obj: Borrowed<'a, 'py, PyDelta>,
}
impl<'a, 'py> PyTimeDeltaSerializer<'a, 'py> {
    #[inline]
    pub(crate) fn new(obj: Borrowed<'a, 'py, PyDelta>) -> Self {
        Self { obj }
    }

    #[inline]
    pub(crate) fn new_unchecked(obj: Borrowed<'a, 'py, PyAny>) -> Self {
        #[expect(unsafe_code)]
        let obj = unsafe { obj.cast_unchecked::<PyDelta>() };
        Self::new(obj)
    }
}

#[cfg(feature = "jiff")]
impl Serialize for PyTimeDeltaSerializer<'_, '_> {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let signed_duration: jiff::SignedDuration = self.obj.extract().map_err(pyerr2sererr)?;
        signed_duration.serialize(serializer)
    }
}

#[cfg(not(feature = "jiff"))]
impl Serialize for PyTimeDeltaSerializer<'_, '_> {
    #[inline]
    fn serialize<S>(&self, _serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        Err(serde::ser::Error::custom(
            "timedelta serialization requires the jiff feature",
        ))
    }
}
