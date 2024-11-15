use jiff::Timestamp;
use pyo3::basic::CompareOp;
use pyo3::prelude::*;
use pyo3::types::PyType;
use std::str::FromStr;

#[derive(Debug, Clone)]
#[pyclass(name = "Timestamp", module = "ryo3")]
pub struct RyTimestamp(pub(crate) Timestamp);

impl From<Timestamp> for RyTimestamp {
    fn from(value: Timestamp) -> Self {
        RyTimestamp(value)
    }
}

#[pymethods]
impl RyTimestamp {
    #[new]
    #[pyo3(signature = (second = None, nanosecond = None))]
    pub fn new(second: Option<i64>, nanosecond: Option<i32>) -> PyResult<Self> {
        let s = second.unwrap_or(0);
        let ns = nanosecond.unwrap_or(0);
        Timestamp::new(s, ns)
            .map(|ts| RyTimestamp::from(ts))
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{}", e)))
    }

    #[classmethod]
    fn now(_cls: &Bound<'_, PyType>) -> Self {
        Self::from(Timestamp::now())
    }

    #[classmethod]
    fn parse(_cls: &Bound<'_, PyType>, s: &str) -> PyResult<Self> {
        Timestamp::from_str(s)
            .map(|ts| RyTimestamp::from(ts))
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
