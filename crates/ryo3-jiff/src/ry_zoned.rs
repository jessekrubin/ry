use crate::ry_span::RySpan;
use crate::ry_timestamp::RyTimestamp;
use crate::ry_timezone::RyTimeZone;
use crate::RyDate;
use jiff::Zoned;
use pyo3::basic::CompareOp;
use pyo3::types::PyType;
use pyo3::{pyclass, pymethods, Bound, IntoPy, PyErr, PyObject, PyResult, Python};
use std::str::FromStr;

#[derive(Debug, Clone)]
#[pyclass(name = "Zoned", module = "ryo3")]
pub struct RyZoned(pub(crate) Zoned);

impl From<Zoned> for RyZoned {
    fn from(value: Zoned) -> Self {
        RyZoned(value)
    }
}

#[pymethods]
impl RyZoned {
    #[new]
    #[pyo3(signature = (timestamp, time_zone))]
    pub fn new(timestamp: RyTimestamp, time_zone: RyTimeZone) -> PyResult<Self> {
        let ts = timestamp.0;
        let tz = time_zone.0;
        Ok(RyZoned::from(Zoned::new(ts, tz)))
    }

    #[classmethod]
    fn now(_cls: &Bound<'_, PyType>) -> Self {
        Self::from(Zoned::now())
    }

    #[classmethod]
    fn parse(_cls: &Bound<'_, PyType>, s: &str) -> PyResult<Self> {
        Zoned::from_str(s)
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

    fn to_string(&self) -> String {
        self.0.to_string()
    }

    fn __str__(&self) -> String {
        format!("Zoned<{}>", self.0)
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
}
