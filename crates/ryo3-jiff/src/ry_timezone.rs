use crate::errors::map_py_value_err;
use crate::pydatetime_conversions::timezone2pyobect;
use crate::ry_datetime::RyDateTime;
use crate::ry_offset::RyOffset;
use crate::ry_timestamp::RyTimestamp;
use crate::ry_zoned::RyZoned;
use crate::JiffTimeZone;
use jiff::tz::{Offset, TimeZone};
use jiff::Timestamp;
use pyo3::prelude::*;
use pyo3::types::{PyType, PyTzInfo};
use ryo3_macros::err_py_not_impl;
use std::fmt::Debug;
use std::hash::{DefaultHasher, Hash, Hasher};

#[derive(Debug, Clone)]
#[pyclass(name = "TimeZone", module = "ryo3", frozen)]
pub struct RyTimeZone(pub(crate) TimeZone);

impl From<TimeZone> for RyTimeZone {
    fn from(value: TimeZone) -> Self {
        RyTimeZone(value)
    }
}

impl From<&TimeZone> for RyTimeZone {
    fn from(value: &TimeZone) -> Self {
        RyTimeZone(value.clone())
    }
}

#[pymethods]
impl RyTimeZone {
    #[new]
    pub fn py_new(time_zone_name: &str) -> PyResult<Self> {
        TimeZone::get(time_zone_name)
            .map(RyTimeZone::from)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e}")))
    }

    #[classmethod]
    fn utc(_cls: &Bound<'_, PyType>) -> Self {
        Self::from(TimeZone::fixed(Offset::UTC))
    }

    #[classmethod]
    fn try_system(_cls: &Bound<'_, PyType>) -> PyResult<Self> {
        TimeZone::try_system()
            .map(RyTimeZone::from)
            .map_err(map_py_value_err)
    }

    #[classmethod]
    fn system(_cls: &Bound<'_, PyType>) -> PyResult<Self> {
        TimeZone::try_system()
            .map(RyTimeZone::from)
            .map_err(map_py_value_err)
    }

    fn iana_name(&self) -> Option<&str> {
        self.0.iana_name()
    }

    #[getter]
    fn name(&self) -> Option<&str> {
        self.iana_name()
    }

    fn __repr__(&self) -> String {
        let iana_name = self.0.iana_name();
        if let Some(name) = iana_name {
            format!("TimeZone(\"{name}\")")
        } else {
            // REALLY NOT SURE IF THIS IS CORRECT
            let offset = self.0.to_offset(Timestamp::now());
            format!("TimeZone('{offset}')")
        }
    }

    fn __hash__(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.__str__().hash(&mut hasher);
        hasher.finish()
    }

    fn __str__(&self) -> String {
        if let Some(name) = self.iana_name() {
            name.to_string()
        } else {
            // REALLY NOT SURE IF THIS IS CORRECT
            let offset = self.0.to_offset(Timestamp::now());
            format!("{offset}")
        }
    }

    fn __eq__(&self, other: TimeZoneEquality) -> bool {
        match other {
            TimeZoneEquality::TimeZone(other) => {
                self.0.eq(&other.0) || self.0.iana_name() == other.0.iana_name()
            }
            TimeZoneEquality::Str(other) => self.0.iana_name() == Some(other.as_str()),
        }
    }

    fn to_pytzinfo<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        timezone2pyobect(py, &self.0)
    }

    #[classmethod]
    fn from_pytzinfo(_cls: &Bound<'_, PyType>, d: &Bound<'_, PyTzInfo>) -> PyResult<Self> {
        let jiff_tz: JiffTimeZone = d.extract()?;
        Ok(Self::from(jiff_tz.0))
    }

    // =====================================================================
    // CLASS METHODS
    // =====================================================================

    #[classmethod]
    fn fixed(_cls: &Bound<'_, PyType>, offset: &RyOffset) -> Self {
        Self::from(TimeZone::fixed(offset.0))
    }

    #[classmethod]
    fn posix(_cls: &Bound<'_, PyType>, string: &str) -> PyResult<Self> {
        TimeZone::posix(string)
            .map(RyTimeZone::from)
            .map_err(map_py_value_err)
    }

    #[classmethod]
    fn get(_cls: &Bound<'_, PyType>, s: &str) -> PyResult<RyTimeZone> {
        TimeZone::get(s).map(Self::from).map_err(map_py_value_err)
    }

    #[classmethod]
    fn tzif(_cls: &Bound<'_, PyType>, name: &str, data: &[u8]) -> PyResult<RyTimeZone> {
        TimeZone::tzif(name, data)
            .map(RyTimeZone::from)
            .map_err(map_py_value_err)
    }

    fn to_datetime(&self, timestamp: &RyTimestamp) -> RyDateTime {
        RyDateTime::from(self.0.to_datetime(timestamp.0))
    }

    /// Return `Offset` from TimeZone
    fn to_offset(&self, timestamp: &RyTimestamp) -> RyOffset {
        RyOffset::from(self.0.to_offset(timestamp.0))
    }

    /// Return `Timestamp` from TimeZone given a `DateTime`
    fn to_timestamp(&self, datetime: &RyDateTime) -> Result<RyTimestamp, PyErr> {
        self.0
            .to_timestamp(datetime.0)
            .map(RyTimestamp::from)
            .map_err(map_py_value_err)
    }

    /// Return `Zoned` from TimeZone given a `DateTime`
    fn to_zoned(&self, datetime: &RyDateTime) -> PyResult<RyZoned> {
        self.0
            .to_zoned(datetime.0)
            .map(RyZoned::from)
            .map_err(map_py_value_err)
    }

    // ===============
    // NOT IMPLEMENTED
    // ===============
    fn to_ambiguous_timestamp(&self) -> PyResult<()> {
        err_py_not_impl!()
    }
    fn to_ambiguous_zoned(&self) -> PyResult<()> {
        err_py_not_impl!()
    }
}

#[derive(Debug, Clone, FromPyObject)]
enum TimeZoneEquality {
    TimeZone(RyTimeZone),
    Str(String),
}
