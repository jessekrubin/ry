use crate::dev::{JiffUnit, RyDateTimeRound};
use crate::pydatetime_conversions::jiff_zoned2pydatetime;
use crate::ry_datetime::RyDateTime;
use crate::ry_span::RySpan;
use crate::ry_time::RyTime;
use crate::ry_timestamp::RyTimestamp;
use crate::ry_timezone::RyTimeZone;
use crate::RyDate;
use jiff::{Zoned, ZonedRound};
use pyo3::basic::CompareOp;
use pyo3::types::{PyDateTime, PyType};
use pyo3::{pyclass, pymethods, Bound, FromPyObject, PyErr, PyResult, Python};
use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, Clone)]
#[pyclass(name = "Zoned", module = "ryo3")]
pub struct RyZoned(pub(crate) Zoned);

#[pymethods]
impl RyZoned {
    #[new]
    #[pyo3(signature = (timestamp, time_zone))]
    pub fn new(timestamp: &RyTimestamp, time_zone: RyTimeZone) -> PyResult<Self> {
        let ts = timestamp.0;
        let tz = time_zone.0;
        Ok(RyZoned::from(Zoned::new(ts, tz)))
    }

    #[classmethod]
    #[pyo3(signature = (tz=None))]
    fn now(_cls: &Bound<'_, PyType>, tz: Option<&str>) -> PyResult<Self> {
        if let Some(tz) = tz {
            Zoned::now()
                .intz(tz)
                .map(RyZoned::from)
                .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e}")))
        } else {
            Ok(Self::from(Zoned::now()))
        }
    }

    #[classmethod]
    fn utcnow(_cls: &Bound<'_, PyType>) -> PyResult<Self> {
        Zoned::now()
            .intz("UTC")
            .map(RyZoned::from)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e}")))
    }

    #[classmethod]
    fn parse(_cls: &Bound<'_, PyType>, s: &str) -> PyResult<Self> {
        Zoned::from_str(s)
            .map(RyZoned::from)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e}")))
    }

    #[classmethod]
    fn strptime(_cls: &Bound<'_, PyType>, format: &str, input: &str) -> PyResult<Self> {
        Zoned::strptime(format, input)
            .map(RyZoned::from)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e}")))
    }

    fn strftime(&self, format: &str) -> String {
        self.0.strftime(format).to_string()
    }

    fn __richcmp__(&self, other: &Self, op: CompareOp) -> PyResult<bool> {
        match op {
            CompareOp::Eq => Ok(self.0 == other.0),
            CompareOp::Ne => Ok(self.0 != other.0),
            CompareOp::Lt => Ok(self.0 < other.0),
            CompareOp::Le => Ok(self.0 <= other.0),
            CompareOp::Gt => Ok(self.0 > other.0),
            CompareOp::Ge => Ok(self.0 >= other.0),
        }
    }

    fn string(&self) -> String {
        self.__str__()
    }

    fn __str__(&self) -> String {
        self.0.to_string()
    }

    fn __repr__(&self) -> String {
        format!("Zoned<{}>", self.0)
    }

    fn timestamp(&self) -> RyTimestamp {
        RyTimestamp::from(self.0.timestamp())
    }

    fn date(&self) -> RyDate {
        RyDate::from(self.0.date())
    }

    fn time(&self) -> RyTime {
        RyTime::from(self.0.time())
    }

    fn datetime(&self) -> RyDateTime {
        RyDateTime::from(self.0.datetime())
    }

    fn to_pydatetime<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDateTime>> {
        jiff_zoned2pydatetime(py, &self.0)
    }

    fn intz(&self, tz: &str) -> PyResult<Self> {
        self.0
            .intz(tz)
            .map(RyZoned::from)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e}")))
    }

    fn inutc(&self) -> PyResult<Self> {
        self.0
            .intz("UTC")
            .map(RyZoned::from)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e}")))
    }

    fn to_rfc2822(&self) -> PyResult<String> {
        jiff::fmt::rfc2822::to_string(&self.0)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e}")))
    }

    #[staticmethod]
    fn from_rfc2822(s: &str) -> PyResult<Self> {
        jiff::fmt::rfc2822::parse(s)
            .map(RyZoned::from)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e}")))
    }

    fn __sub__(&self, other: &Self) -> RySpan {
        RySpan::from(&self.0 - &other.0)
    }

    fn checked_add(&self, span: &RySpan) -> PyResult<Self> {
        self.0
            .checked_add(span.0)
            .map(RyZoned::from)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e}")))
    }

    fn timezone(&self) -> RyTimeZone {
        RyTimeZone::from(self.0.time_zone())
    }

    fn round(&self, option: IntoZonedRound) -> PyResult<Self> {
        self.0
            .round(option)
            .map(RyZoned::from)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e}")))
    }

    fn year(&self) -> i16 {
        self.0.year()
    }

    fn month(&self) -> i8 {
        self.0.month()
    }

    fn day(&self) -> i8 {
        self.0.day()
    }

    // TODO: weekdays
    // fn weekday(&self) -> i8 {
    //     self.0.weekday()
    // }

    fn hour(&self) -> i8 {
        self.0.hour()
    }

    fn minute(&self) -> i8 {
        self.0.minute()
    }

    fn second(&self) -> i8 {
        self.0.second()
    }

    fn microsecond(&self) -> i16 {
        self.0.microsecond()
    }

    fn millisecond(&self) -> i16 {
        self.0.millisecond()
    }

    fn nanosecond(&self) -> i16 {
        self.0.nanosecond()
    }

    fn subsec_nanosecond(&self) -> i32 {
        self.0.subsec_nanosecond()
    }
}

impl Display for RyZoned {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl From<Zoned> for RyZoned {
    fn from(value: Zoned) -> Self {
        RyZoned(value)
    }
}

#[derive(Debug, Clone, FromPyObject)]
enum IntoZonedRound {
    DateTimeRound(RyDateTimeRound),
    JiffUnit(JiffUnit),
}

impl From<IntoZonedRound> for ZonedRound {
    fn from(val: IntoZonedRound) -> Self {
        match val {
            // TODO: this is ugly
            IntoZonedRound::DateTimeRound(round) => ZonedRound::new()
                .smallest(round.smallest.0)
                .mode(round.mode.0)
                .increment(round.increment),
            IntoZonedRound::JiffUnit(unit) => unit.0.into(),
        }
    }
}