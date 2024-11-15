use crate::ry_time::RyTime;
use crate::RyDate;
use jiff::civil::DateTime;
use jiff::Zoned;
use pyo3::types::PyType;
use pyo3::{pyclass, pymethods, Bound, PyErr, PyResult};

#[derive(Debug, Clone)]
#[pyclass(name = "Time")]
pub struct RyDateTime(DateTime);

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
            .map(|dt| RyDateTime::from(dt))
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{}", e)))
    }

    #[classmethod]
    fn now(_cls: &Bound<'_, PyType>) -> Self {
        Self::from(DateTime::from(Zoned::now()))
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
}
