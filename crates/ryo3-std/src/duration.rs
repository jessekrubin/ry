use pyo3::basic::CompareOp;
use pyo3::prelude::{PyModule, PyModuleMethods};
use pyo3::types::{PyDelta, PyType};
use pyo3::{pyclass, pymethods, Bound, FromPyObject, IntoPyObject, PyResult, Python};
use std::time::Duration;

#[derive(Debug, Clone)]
#[pyclass(name = "Duration", module = "ryo3")]
pub struct PyDuration(pub Duration);

#[pymethods]
impl PyDuration {
    #[new]
    fn new(secs: u64, nanos: u32) -> Self {
        let dur = Duration::new(secs, nanos);
        PyDuration(dur)
    }

    fn __str__(&self) -> String {
        format!(
            "Duration(secs={}, nanos={})",
            self.0.as_secs(),
            self.0.subsec_nanos()
        )
    }

    fn __repr__(&self) -> String {
        format!(
            "Duration(secs={}, nanos={})",
            self.0.as_secs(),
            self.0.subsec_nanos()
        )
    }

    fn dbg(&self) -> String {
        format!("Duration<{:?}>", self.0)
    }

    #[classmethod]
    fn zero(_cls: &Bound<'_, PyType>) -> Self {
        PyDuration(Duration::new(0, 0))
    }

    #[classmethod]
    fn from_secs(_cls: &Bound<'_, PyType>, secs: u64) -> Self {
        PyDuration(Duration::from_secs(secs))
    }

    #[classmethod]
    fn from_millis(_cls: &Bound<'_, PyType>, millis: u64) -> Self {
        PyDuration(Duration::from_millis(millis))
    }

    #[classmethod]
    fn from_micros(_cls: &Bound<'_, PyType>, micros: u64) -> Self {
        PyDuration(Duration::from_micros(micros))
    }

    #[classmethod]
    fn from_nanos(_cls: &Bound<'_, PyType>, nanos: u64) -> Self {
        PyDuration(Duration::from_nanos(nanos))
    }

    #[getter]
    fn secs(&self) -> u64 {
        self.0.as_secs()
    }

    #[getter]
    fn nanos(&self) -> u32 {
        self.0.subsec_nanos()
    }

    fn as_secs(&self) -> f64 {
        self.0.as_secs_f64()
    }

    fn as_millis(&self) -> u128 {
        self.0.as_millis()
    }

    fn as_micros(&self) -> u128 {
        self.0.as_micros()
    }

    fn as_nanos(&self) -> u128 {
        self.0.as_nanos()
    }

    fn __richcmp__(&self, other: PyDurationComparable, op: CompareOp) -> PyResult<bool> {
        match other {
            PyDurationComparable::PyDuration(other) => match op {
                CompareOp::Eq => Ok(self.0 == other.0),
                CompareOp::Ne => Ok(self.0 != other.0),
                CompareOp::Lt => Ok(self.0 < other.0),
                CompareOp::Le => Ok(self.0 <= other.0),
                CompareOp::Gt => Ok(self.0 > other.0),
                CompareOp::Ge => Ok(self.0 >= other.0),
            },
            PyDurationComparable::Duration(other) => match op {
                CompareOp::Eq => Ok(self.0 == other),
                CompareOp::Ne => Ok(self.0 != other),
                CompareOp::Lt => Ok(self.0 < other),
                CompareOp::Le => Ok(self.0 <= other),
                CompareOp::Gt => Ok(self.0 > other),
                CompareOp::Ge => Ok(self.0 >= other),
            },
        }
    }

    fn to_pytimedelta<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDelta>> {
        self.0.into_pyobject(py)
    }

    #[classmethod]
    fn from_pytimedelta(_cls: &Bound<'_, PyType>, delta: Duration) -> PyResult<Self> {
        Ok(PyDuration(delta))
    }

    #[getter]
    fn days(&self) -> u64 {
        self.0.as_secs() / 86400
    }

    #[getter]
    fn seconds(&self) -> u64 {
        self.0.as_secs() % 86400
    }

    #[getter]
    fn microseconds(&self) -> u32 {
        self.0.subsec_micros()
    }

    fn __add__(&self, other: PyDurationComparable) -> PyDuration {
        match other {
            PyDurationComparable::PyDuration(other) => PyDuration(self.0 + other.0),
            PyDurationComparable::Duration(other) => PyDuration(self.0 + other),
        }
    }

    fn __sub__(&self, other: PyDurationComparable) -> PyDuration {
        match other {
            PyDurationComparable::PyDuration(other) => PyDuration(self.0 - other.0),
            PyDurationComparable::Duration(other) => PyDuration(self.0 - other),
        }
    }
}

#[derive(Debug, Clone, FromPyObject)]
enum PyDurationComparable {
    PyDuration(PyDuration),
    Duration(Duration),
}

impl From<Duration> for PyDuration {
    fn from(d: Duration) -> Self {
        PyDuration(d)
    }
}

pub(crate) fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyDuration>()?;
    Ok(())
}
