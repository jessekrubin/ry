use crate::ry_time::RyTime;
use crate::ry_timezone::RyTimeZone;
use crate::ry_zoned::RyZoned;
use crate::RyDate;
use jiff::civil::DateTime;
use jiff::Zoned;
use pyo3::types::PyType;
use pyo3::{pyclass, pymethods, Bound, PyErr, PyResult};
use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, Clone)]
#[pyclass(name = "DateTime")]
pub struct RyDateTime(pub(crate) DateTime);

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
        DateTime::new(year, month, day, hour, minute, second, subsec_nanosecond)
            .map(RyDateTime::from)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e}")))
    }

    #[classmethod]
    fn now(_cls: &Bound<'_, PyType>) -> Self {
        Self::from(DateTime::from(Zoned::now()))
    }
    #[classmethod]
    fn parse(_cls: &Bound<'_, PyType>, s: &str) -> PyResult<Self> {
        DateTime::from_str(s)
            .map(RyDateTime::from)
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

    fn __str__(&self) -> String {
        self.to_string()
    }

    fn __repr__(&self) -> String {
        format!("DateTime(year={}, month={}, day={}, hour={}, minute={}, second={}, millisecond={}, microsecond={}, nanosecond={})", self.year(), self.month(), self.day(), self.hour(), self.minute(), self.second(), self.millisecond(), self.microsecond(), self.nanosecond())
    }

    fn to_date(&self) -> RyDate {
        RyDate::from(self.0.date())
    }

    fn time(&self) -> RyTime {
        RyTime::from(self.0.time())
    }

    fn date(&self) -> RyDate {
        RyDate::from(self.0.date())
    }

    fn intz(&self, time_zone_name: &str) -> PyResult<RyZoned> {
        self.0
            .intz(time_zone_name)
            .map(RyZoned::from)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e}")))
    }

    fn to_zoned(&self, tz: RyTimeZone) -> PyResult<RyZoned> {
        self.0
            .to_zoned(tz.0)
            .map(RyZoned::from)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e}")))
    }

    fn first_of_month(&self) -> RyDateTime {
        RyDateTime::from(self.0.first_of_month())
    }
    fn last_of_month(&self) -> RyDateTime {
        RyDateTime::from(self.0.last_of_month())
    }
}

impl Display for RyDateTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
