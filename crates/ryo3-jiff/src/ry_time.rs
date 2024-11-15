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

use jiff::Zoned;
use pyo3::basic::CompareOp;
use pyo3::prelude::*;
use pyo3::types::PyType;
use std::str::FromStr;

use crate::ry_datetime::RyDateTime;
use pyo3::IntoPy;

#[derive(Debug, Clone)]
#[pyclass(name = "Time")]
pub struct RyTime(jiff::civil::Time);

impl From<jiff::civil::Time> for RyTime {
    fn from(value: jiff::civil::Time) -> Self {
        Self(value)
    }
}

#[pymethods]
impl RyTime {
    #[new]
    pub fn new(hour: i8, minute: i8, second: i8, nanosecond: i32) -> PyResult<Self> {
        jiff::civil::Time::new(hour, minute, second, nanosecond)
            .map(|t| crate::RyTime::from(t))
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

    fn to_datetime(&self, date: &crate::RyDate) -> RyDateTime {
        RyDateTime::from(self.0.to_datetime(date.0))
    }
}
