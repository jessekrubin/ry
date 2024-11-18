use jiff::SignedDuration;
use pyo3::basic::CompareOp;
use pyo3::types::PyType;
use pyo3::{pyclass, pymethods, Bound, IntoPy, PyErr, PyObject, PyResult, Python};
use std::str::FromStr;

#[derive(Debug, Clone)]
#[pyclass(name = "SignedDuration")]
pub struct RySignedDuration(pub(crate) SignedDuration);

#[pymethods]
impl RySignedDuration {
    #[new]
    fn new(secs: i64, nanos: i32) -> Self {
        Self(SignedDuration::new(secs, nanos))
    }

    #[classmethod]
    fn parse(_cls: &Bound<'_, PyType>, s: &str) -> PyResult<Self> {
        SignedDuration::from_str(s)
            .map(RySignedDuration::from)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e}")))
    }

    fn __abs__(&self) -> Self {
        Self(self.0.abs())
    }

    fn __str__(&self) -> String {
        self.__repr__()
    }

    fn __repr__(&self) -> String {
        format!(
            "SignedDuration(secs={}, nanos={})",
            self.0.as_secs(),
            self.0.subsec_nanos()
        )
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
    fn __add__(&self, other: RySignedDuration) -> Self {
        Self(self.0 + other.0)
    }

    fn __sub__(&self, other: RySignedDuration) -> Self {
        Self(self.0 - other.0)
    }
}

impl From<jiff::SignedDuration> for RySignedDuration {
    fn from(d: jiff::SignedDuration) -> Self {
        Self(d)
    }
}
