use crate::ry_timezone::RyTimeZone;
use crate::ry_zoned::RyZoned;
use jiff::{Timestamp, Zoned};
use pyo3::basic::CompareOp;
use pyo3::prelude::*;
use pyo3::types::PyType;
use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, Clone)]
#[pyclass(name = "Timestamp", module = "ryo3")]
pub struct RyTimestamp(pub(crate) Timestamp);

#[pymethods]
impl RyTimestamp {
    #[new]
    #[pyo3(signature = (second = None, nanosecond = None))]
    pub fn new(second: Option<i64>, nanosecond: Option<i32>) -> PyResult<Self> {
        let s = second.unwrap_or(0);
        let ns = nanosecond.unwrap_or(0);
        Timestamp::new(s, ns)
            .map(RyTimestamp::from)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e}")))
    }

    #[classmethod]
    fn now(_cls: &Bound<'_, PyType>) -> Self {
        Self::from(Timestamp::now())
    }

    #[classmethod]
    fn parse(_cls: &Bound<'_, PyType>, s: &str) -> PyResult<Self> {
        Timestamp::from_str(s)
            .map(RyTimestamp::from)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e}")))
    }

    #[classmethod]
    fn from_millisecond(_cls: &Bound<'_, PyType>, milisecond: i64) -> PyResult<RyTimestamp> {
        Timestamp::from_millisecond(milisecond)
            .map(RyTimestamp::from)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e}")))
    }

    fn to_zoned(&self, time_zone: RyTimeZone) -> RyZoned {
        RyZoned::from(Zoned::new(self.0, time_zone.0))
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
        self.__str__()
    }

    fn __str__(&self) -> String {
        self.0.to_string()
    }

    fn __repr__(&self) -> String {
        format!("Timestamp<{}>", self.string())
    }

    fn as_second(&self) -> i64 {
        self.0.as_second()
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
impl Display for RyTimestamp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl From<Timestamp> for RyTimestamp {
    fn from(value: Timestamp) -> Self {
        RyTimestamp(value)
    }
}
