use crate::ry_datetime::RyDateTime;
use jiff::civil::Date;
use jiff::Zoned;
use pyo3::basic::CompareOp;
use pyo3::types::PyType;
use pyo3::{pyclass, pymethods, Bound, IntoPy, PyErr, PyObject, PyResult, Python};

#[derive(Debug, Clone)]
#[pyclass(name = "Date")]
pub struct RyDate(pub(crate) Date);

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
