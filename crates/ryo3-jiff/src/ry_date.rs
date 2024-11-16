use crate::ry_datetime::RyDateTime;
use crate::ry_time::RyTime;
use crate::ry_timezone::RyTimeZone;
use crate::ry_zoned::RyZoned;
use jiff::civil::Date;
use jiff::Zoned;
use pyo3::basic::CompareOp;
use pyo3::types::PyType;
use pyo3::{pyclass, pymethods, Bound, IntoPy, PyErr, PyObject, PyResult, Python};
use std::fmt::Display;

#[derive(Debug, Clone)]
#[pyclass(name = "Date")]
pub struct RyDate(pub(crate) Date);

#[pymethods]
impl RyDate {
    #[new]
    pub fn new(year: i16, month: i8, day: i8) -> PyResult<Self> {
        Date::new(year, month, day)
            .map(RyDate::from)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e}")))
    }

    #[classmethod]
    fn today(_cls: &Bound<'_, PyType>) -> Self {
        let z = jiff::civil::Date::from(Zoned::now());
        Self::from(z)
    }
    fn at(&self, hour: i8, minute: i8, second: i8, subsec_nanosecond: i32) -> RyDateTime {
        RyDateTime::from(self.0.at(hour, minute, second, subsec_nanosecond))
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

    fn to_datetime(&self, time: RyTime) -> RyDateTime {
        RyDateTime::from(self.0.to_datetime(time.0))
    }

    fn to_zoned(&self, tz: RyTimeZone) -> PyResult<RyZoned> {
        self.0
            .to_zoned(tz.0)
            .map(RyZoned::from)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e}")))
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
    fn string(&self) -> String {
        self.0.to_string()
    }
    fn __str__(&self) -> String {
        format!("Date<{self}>")
    }
}

impl Display for RyDate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Date<{}>", self.0)
    }
}
impl From<Date> for RyDate {
    fn from(value: Date) -> Self {
        RyDate(value)
    }
}
