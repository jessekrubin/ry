use pyo3::basic::CompareOp;
use pyo3::prelude::{PyModule, PyModuleMethods};
use pyo3::types::PyType;
use pyo3::{pyclass, pymethods, Bound, PyResult};

#[derive(Debug, Clone)]
#[pyclass(name = "Duration", module = "ryo3")]
pub struct PyDuration(std::time::Duration);

#[pymethods]
impl PyDuration {
    #[new]
    fn new(secs: u64, nanos: u32) -> Self {
        let dur = std::time::Duration::new(secs, nanos);
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
        PyDuration(std::time::Duration::new(0, 0))
    }

    #[classmethod]
    fn from_secs(_cls: &Bound<'_, PyType>, secs: u64) -> Self {
        PyDuration(std::time::Duration::from_secs(secs))
    }

    #[classmethod]
    fn from_millis(_cls: &Bound<'_, PyType>, millis: u64) -> Self {
        PyDuration(std::time::Duration::from_millis(millis))
    }

    #[classmethod]
    fn from_micros(_cls: &Bound<'_, PyType>, micros: u64) -> Self {
        PyDuration(std::time::Duration::from_micros(micros))
    }

    #[classmethod]
    fn from_nanos(_cls: &Bound<'_, PyType>, nanos: u64) -> Self {
        PyDuration(std::time::Duration::from_nanos(nanos))
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
}

impl From<std::time::Duration> for PyDuration {
    fn from(d: std::time::Duration) -> Self {
        PyDuration(d)
    }
}

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyDuration>()?;
    Ok(())
}
