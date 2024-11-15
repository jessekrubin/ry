#![deny(clippy::all)]
#![deny(clippy::correctness)]
#![deny(clippy::panic)]
#![deny(clippy::perf)]
#![deny(clippy::pedantic)]
#![deny(clippy::style)]
#![deny(clippy::unwrap_used)]
#![warn(clippy::must_use_candidate)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::unnecessary_wraps)]
#![allow(clippy::needless_pass_by_value)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::unused_self)]

// use pyo3::types::PyTime;
use jiff::civil::DateTime;
use jiff::tz::Offset;
use jiff::{Timestamp, Zoned};
use pyo3::basic::CompareOp;
use pyo3::prelude::PyModule;
use pyo3::prelude::*;
use pyo3::types::PyType;
use std::str::FromStr;

#[derive(Debug, Clone)]
#[pyclass(name = "Timestamp", module = "ryo3")]
pub struct PyTimestamp(jiff::Timestamp);

impl From<Timestamp> for PyTimestamp {
    fn from(value: Timestamp) -> Self {
        PyTimestamp(value)
    }
}

#[pymethods]
impl PyTimestamp {
    #[new]
    #[pyo3(signature = (second = None, nanosecond = None))]
    pub fn new(second: Option<i64>, nanosecond: Option<i32>) -> PyResult<Self> {
        let s = second.unwrap_or(0);
        let ns = nanosecond.unwrap_or(0);
        Timestamp::new(s, ns)
            .map(|ts| PyTimestamp::from(ts))
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{}", e)))
    }

    #[classmethod]
    fn now(_cls: &Bound<'_, PyType>) -> Self {
        Self::from(Timestamp::now())
    }

    #[classmethod]
    fn parse(_cls: &Bound<'_, PyType>, s: &str) -> PyResult<Self> {
        Timestamp::from_str(s)
            .map(|ts| PyTimestamp::from(ts))
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{}", e)))
    }
    fn __richcmp__(&self, other: &Self, op: CompareOp, py: Python<'_>) -> PyObject {
        match op {
            CompareOp::Eq => (self.0 == other.0).into_py(py),
            CompareOp::Ne => (self.0 != other.0).into_py(py),
            CompareOp::Lt => (self.0 < other.0).into_py(py),
            CompareOp::Le => (self.0 <= other.0).into_py(py),
            CompareOp::Gt => (self.0 > other.0).into_py(py),
            CompareOp::Ge => (self.0 >= other.0).into_py(py),
        }
    }

    fn __hash__(&self) -> isize {
        self.0.as_microsecond() as isize
    }

    fn to_string(&self) -> String {
        self.0.to_string()
    }
    fn __str__(&self) -> String {
        format!("Timestamp<{}>", self.to_string())
    }

    fn as_microsecond(&self) -> i64 {
        self.0.as_microsecond()
    }

    fn as_millisecond(&self) -> i64 {
        self.0.as_millisecond()
    }

    fn as_nanosecond(&self) -> i128 {
        self.0.as_nanosecond()
    }
}

#[derive(Debug, Clone)]
#[pyclass(name = "Date")]
pub struct RyDate(jiff::civil::Date);

impl From<jiff::civil::Date> for RyDate {
    fn from(value: jiff::civil::Date) -> Self {
        RyDate(value)
    }
}

#[pymethods]
impl RyDate {
    #[new]
    pub fn new(year: i16, month: i8, day: i8) -> PyResult<Self> {
        jiff::civil::Date::new(year, month, day)
            .map(|d| RyDate::from(d))
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{}", e)))
    }

    #[classmethod]
    fn today(_cls: &Bound<'_, PyType>) -> Self {
        let z = jiff::civil::Date::from(Zoned::now());
        Self::from(z)
    }
    fn at(&self, hour: i8, minute: i8, second: i8, subsec_nanosecond: i32) -> RyDateTime {
        RyDateTime::from(self.0.at(hour, minute, second, subsec_nanosecond))
    }

    fn __richcmp__(&self, other: &Self, op: CompareOp, py: Python<'_>) -> PyObject {
        match op {
            CompareOp::Eq => (self.0 == other.0).into_py(py),
            CompareOp::Ne => (self.0 != other.0).into_py(py),
            CompareOp::Lt => (self.0 < other.0).into_py(py),
            CompareOp::Le => (self.0 <= other.0).into_py(py),
            CompareOp::Gt => (self.0 > other.0).into_py(py),
            CompareOp::Ge => (self.0 >= other.0).into_py(py),
        }
    }
    fn to_string(&self) -> String {
        self.0.to_string()
    }
    fn __str__(&self) -> String {
        format!("Date<{}>", self.to_string())
    }
}

#[derive(Debug, Clone)]
#[pyclass(name = "Time")]
pub struct RyDateTime(jiff::civil::DateTime);

impl From<DateTime> for RyDateTime {
    fn from(value: DateTime) -> Self {
        RyDateTime(value)
    }
}

#[pymethods]
impl RyDateTime {
    #[new]
    pub fn new(
        year: i16,
        month: i8,
        day: i8,
        hour: i8,
        minute: i8,
        second: i8,
        subsec_nanosecond: i32,
    ) -> PyResult<Self> {
        jiff::civil::DateTime::new(year, month, day, hour, minute, second, subsec_nanosecond)
            .map(|dt| RyDateTime::from(dt))
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{}", e)))
    }

    #[classmethod]
    fn now(_cls: &Bound<'_, PyType>) -> Self {
        Self::from(DateTime::from(Zoned::now()))
    }

    //     assert_eq!(d.year(), 2024);
    // assert_eq!(d.month(), 2);
    // assert_eq!(d.day(), 29);
    // assert_eq!(d.hour(), 21);
    // assert_eq!(d.minute(), 30);
    // assert_eq!(d.second(), 5);
    // assert_eq!(d.millisecond(), 123);
    // assert_eq!(d.microsecond(), 456);
    // assert_eq!(d.nanosecond(), 789);

    fn year(&self) -> i16 {
        self.0.year()
    }

    fn month(&self) -> i8 {
        self.0.month()
    }

    fn day(&self) -> i8 {
        self.0.day()
    }

    fn hour(&self) -> i8 {
        self.0.hour()
    }

    fn minute(&self) -> i8 {
        self.0.minute()
    }

    fn second(&self) -> i8 {
        self.0.second()
    }

    fn millisecond(&self) -> i16 {
        self.0.millisecond()
    }

    fn microsecond(&self) -> i16 {
        self.0.microsecond()
    }

    fn nanosecond(&self) -> i16 {
        self.0.nanosecond()
    }

    fn to_string(&self) -> String {
        self.0.to_string()
    }

    fn __str__(&self) -> String {
        format!("DateTime<{}>", self.to_string())
    }

    fn to_date(&self) -> RyDate {
        RyDate::from(self.0.date())
    }

    fn to_time(&self) -> RyTime {
        RyTime::from(self.0.time())
    }

    //
}

#[derive(Debug, Clone)]
#[pyclass(name = "Time")]
pub struct RyTime(jiff::civil::Time);

impl From<jiff::civil::Time> for RyTime {
    fn from(value: jiff::civil::Time) -> Self {
        RyTime(value)
    }
}

#[pymethods]
impl RyTime {
    #[new]
    pub fn new(hour: i8, minute: i8, second: i8, nanosecond: i32) -> PyResult<Self> {
        jiff::civil::Time::new(hour, minute, second, nanosecond)
            .map(|t| RyTime::from(t))
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{}", e)))
    }

    #[classmethod]
    fn now(_cls: &Bound<'_, PyType>) -> Self {
        let z = jiff::civil::Time::from(Zoned::now());
        Self::from(z)
    }

    fn on(&self, year: i16, month: i8, day: i8) -> RyDateTime {
        RyDateTime::from(self.0.on(year, month, day))
    }

    fn __richcmp__(&self, other: &Self, op: CompareOp, py: Python<'_>) -> PyObject {
        match op {
            CompareOp::Eq => (self.0 == other.0).into_py(py),
            CompareOp::Ne => (self.0 != other.0).into_py(py),
            CompareOp::Lt => (self.0 < other.0).into_py(py),
            CompareOp::Le => (self.0 <= other.0).into_py(py),
            CompareOp::Gt => (self.0 > other.0).into_py(py),
            CompareOp::Ge => (self.0 >= other.0).into_py(py),
        }
    }

    fn to_string(&self) -> String {
        self.0.to_string()
    }
    fn __str__(&self) -> String {
        format!("Time<{}>", self.to_string())
    }

    fn millisecond(&self) -> i16 {
        self.0.millisecond()
    }

    fn microsecond(&self) -> i16 {
        self.0.microsecond()
    }

    fn nanosecond(&self) -> i16 {
        self.0.nanosecond()
    }

    fn second(&self) -> i8 {
        self.0.second()
    }

    fn to_datetime(&self, date: &RyDate) -> RyDateTime {
        RyDateTime::from(self.0.to_datetime(date.0))
    }
}

#[derive(Debug, Clone)]
#[pyclass(name = "TimeZone")]
pub struct PyTimeZone(jiff::tz::TimeZone);
// Source
// pub fn get(time_zone_name: &str) -> Result<TimeZone, Error>
// A convenience function for performing a time zone database lookup for the given time zone identifier. It uses the default global time zone database via tz::db().
//
// Errors
// This returns an error if the given time zone identifier could not be found in the default TimeZoneDatabase.
//
// Example
// use jiff::{tz::TimeZone, Timestamp};
//
// let tz = TimeZone::get("Japan")?;
// assert_eq!(
//     tz.to_datetime(Timestamp::UNIX_EPOCH).to_string(),
//     "1970-01-01T09:00:00",
// );

impl From<jiff::tz::TimeZone> for PyTimeZone {
    fn from(value: jiff::tz::TimeZone) -> Self {
        PyTimeZone(value)
    }
}

#[pymethods]
impl PyTimeZone {
    #[new]
    pub fn new(time_zone_name: &str) -> PyResult<Self> {
        jiff::tz::TimeZone::get(time_zone_name)
            .map(|tz| PyTimeZone::from(tz))
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{}", e)))
    }

    #[classmethod]
    fn utc(_cls: &Bound<'_, PyType>) -> Self {
        Self::from(jiff::tz::TimeZone::fixed(Offset::UTC))
    }

    #[classmethod]
    fn system(_cls: &Bound<'_, PyType>) -> Self {
        Self::from(jiff::tz::TimeZone::system())
    }

    fn iana_name(&self) -> Option<&str> {
        self.0.iana_name()
    }

    fn __str__(&self) -> String {
        // TODO; figure out good repr
        let iana_name = self.0.iana_name();
        match iana_name {
            Some(name) => format!("TimeZone<{}>", name),
            None => "TimeZone<None>".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
#[pyclass(name = "Zoned")]
pub struct PyZoned(jiff::Zoned);

impl From<Zoned> for PyZoned {
    fn from(value: Zoned) -> Self {
        PyZoned(value)
    }
}

#[pymethods]
impl PyZoned {
    #[new]
    #[pyo3(signature = (timestamp, time_zone))]
    pub fn new(timestamp: PyTimestamp, time_zone: PyTimeZone) -> PyResult<Self> {
        let ts = timestamp.0;
        let tz = time_zone.0;
        Ok(PyZoned::from(Zoned::new(ts, tz)))
    }

    #[classmethod]
    fn now(_cls: &Bound<'_, PyType>) -> Self {
        Self::from(Zoned::now())
    }

    #[classmethod]
    fn parse(_cls: &Bound<'_, PyType>, s: &str) -> PyResult<Self> {
        Zoned::from_str(s)
            .map(|z| PyZoned::from(z))
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{}", e)))
    }

    fn __richcmp__(&self, other: &Self, op: CompareOp, py: Python<'_>) -> PyObject {
        match op {
            CompareOp::Eq => (self.0 == other.0).into_py(py),
            CompareOp::Ne => (self.0 != other.0).into_py(py),
            CompareOp::Lt => (self.0 < other.0).into_py(py),
            CompareOp::Le => (self.0 <= other.0).into_py(py),
            CompareOp::Gt => (self.0 > other.0).into_py(py),
            CompareOp::Ge => (self.0 >= other.0).into_py(py),
        }
    }

    fn to_string(&self) -> String {
        self.0.to_string()
    }

    fn date(&self) -> RyDate {
        RyDate::from(self.0.date())
    }
}

// methods
#[pyfunction]
pub fn date(year: i16, month: i8, day: i8) -> PyResult<RyDate> {
    RyDate::new(year, month, day)
}

#[pyfunction]
pub fn time(hour: i8, minute: i8, second: i8, nanosecond: i32) -> PyResult<RyTime> {
    RyTime::new(hour, minute, second, nanosecond)
}

#[pyfunction]
pub fn datetime(
    year: i16,
    month: i8,
    day: i8,
    hour: i8,
    minute: i8,
    second: i8,
    subsec_nanosecond: i32,
) -> PyResult<RyDateTime> {
    RyDateTime::new(year, month, day, hour, minute, second, subsec_nanosecond)
}

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // m.add_function(wrap_pyfunction!(, m)?)?;

    m.add_class::<PyTimestamp>()?;
    m.add_class::<RyDate>()?;
    m.add_class::<RyDateTime>()?;
    m.add_class::<RyTime>()?;
    m.add_class::<PyTimeZone>()?;
    m.add_class::<PyZoned>()?;

    m.add_function(wrap_pyfunction!(date, m)?)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]

    #[test]
    fn test_dev() {
        assert_eq!(1 + 1, 2)
    }
}
