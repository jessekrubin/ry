use crate::pydatetime_conversions::jiff_time2pytime;
use crate::ry_datetime::RyDateTime;
use jiff::Zoned;
use pyo3::basic::CompareOp;
use pyo3::intern;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyTime, PyTuple, PyType};
use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, Clone)]
#[pyclass(name = "Time", module = "ryo3")]
pub struct RyTime(pub(crate) jiff::civil::Time);

#[pymethods]
impl RyTime {
    #[new]
    #[pyo3(signature = (hour=0, minute=0, second=0, nanosecond=0))]
    pub fn new(
        hour: Option<i8>,
        minute: Option<i8>,
        second: Option<i8>,
        nanosecond: Option<i32>,
    ) -> PyResult<Self> {
        jiff::civil::Time::new(
            hour.unwrap_or(0),
            minute.unwrap_or(0),
            second.unwrap_or(0),
            nanosecond.unwrap_or(0),
        )
        .map(crate::RyTime::from)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e}")))
    }

    #[classmethod]
    fn now(_cls: &Bound<'_, PyType>) -> Self {
        let z = jiff::civil::Time::from(Zoned::now());
        Self::from(z)
    }

    #[classmethod]
    fn parse(_cls: &Bound<'_, PyType>, s: &str) -> PyResult<Self> {
        jiff::civil::Time::from_str(s)
            .map(crate::RyTime::from)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e}")))
    }

    fn on(&self, year: i16, month: i8, day: i8) -> RyDateTime {
        RyDateTime::from(self.0.on(year, month, day))
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
        self.0.to_string()
    }

    fn __str__(&self) -> String {
        self.string()
    }

    fn __repr__(&self) -> String {
        format!("Time<{}>", self.0)
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

    fn to_datetime(&self, date: &crate::RyDate) -> RyDateTime {
        RyDateTime::from(self.0.to_datetime(date.0))
    }

    fn to_pytime<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyTime>> {
        let dt = jiff_time2pytime(py, &self.0)?;
        Ok(dt)
    }

    fn astuple<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyTuple>> {
        PyTuple::new(
            py,
            vec![
                i32::from(self.0.hour()),
                i32::from(self.0.minute()),
                i32::from(self.0.second()),
                self.0.subsec_nanosecond(),
            ],
        )
    }

    fn asdict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        let dict = PyDict::new(py);
        dict.set_item(intern!(py, "hour"), self.0.hour())?;
        dict.set_item(intern!(py, "minute"), self.0.minute())?;
        dict.set_item(intern!(py, "second"), self.0.second())?;
        dict.set_item(intern!(py, "nanosecond"), self.0.nanosecond())?;
        Ok(dict)
    }
}

impl Display for RyTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Time<{}>", self.0)
    }
}
impl From<jiff::civil::Time> for RyTime {
    fn from(value: jiff::civil::Time) -> Self {
        Self(value)
    }
}
