use crate::time::PyDuration;
use pyo3::IntoPyObjectExt;
use pyo3::prelude::*;
use pyo3::pyclass::CompareOp;
use ryo3_macro_rules::{py_overflow_err, py_overflow_error};
use std::hash::{DefaultHasher, Hash, Hasher};
use std::time::Instant;

#[pyclass(name = "Instant", frozen)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PyInstant(pub Instant);

impl From<Instant> for PyInstant {
    fn from(i: Instant) -> Self {
        Self(i)
    }
}

#[pymethods]
impl PyInstant {
    #[new]
    #[must_use]
    fn py_new() -> Self {
        Self(Instant::now())
    }

    #[staticmethod]
    #[must_use]
    fn now() -> Self {
        Self(Instant::now())
    }

    #[must_use]
    fn __str__(&self) -> String {
        format!("{:?}", self.0)
    }

    #[must_use]
    fn __repr__(&self) -> String {
        format!("{:?}", self.0)
    }

    #[must_use]
    fn __hash__(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.0.hash(&mut hasher);
        hasher.finish()
    }

    #[must_use]
    fn __richcmp__(&self, other: &Self, op: CompareOp) -> bool {
        match op {
            CompareOp::Eq => self.0 == other.0,
            CompareOp::Ne => self.0 != other.0,
            CompareOp::Lt => self.0 < other.0,
            CompareOp::Le => self.0 <= other.0,
            CompareOp::Gt => self.0 > other.0,
            CompareOp::Ge => self.0 >= other.0,
        }
    }

    fn __add__(&self, other: &PyDuration) -> PyResult<Self> {
        self.0
            .checked_add(other.0)
            .map(Self::from)
            .ok_or_else(|| py_overflow_error!("instant-overflow-add"))
    }

    fn __sub__<'py>(&self, py: Python<'py>, other: PyInstantSub) -> PyResult<Bound<'py, PyAny>> {
        match other {
            PyInstantSub::Instant(other) => {
                let dur = self.0.checked_duration_since(other.0);
                match dur {
                    Some(d) => {
                        let pyduration = PyDuration::from(d);
                        pyduration.into_bound_py_any(py)
                    }
                    None => py_overflow_err!(),
                }
            }
            PyInstantSub::Duration(other) => {
                let inst = self.0.checked_sub(other.0);
                match inst {
                    Some(i) => {
                        let pyinstant = Self::from(i);
                        pyinstant.into_bound_py_any(py)
                    }
                    None => py_overflow_err!(),
                }
            }
        }
    }

    #[must_use]
    fn elapsed(&self) -> PyDuration {
        PyDuration(self.0.elapsed())
    }

    fn checked_add(&self, other: &PyDuration) -> Option<Self> {
        self.0.checked_add(other.0).map(Self)
    }

    fn checked_sub(&self, other: &PyDuration) -> Option<Self> {
        self.0.checked_sub(other.0).map(Self::from)
    }

    fn checked_duration_since(&self, earlier: &Self) -> Option<PyDuration> {
        self.0
            .checked_duration_since(earlier.0)
            .map(PyDuration::from)
    }

    #[must_use]
    fn saturating_duration_since(&self, earlier: &Self) -> PyDuration {
        PyDuration(self.0.saturating_duration_since(earlier.0))
    }

    #[must_use]
    fn duration_since(&self, earlier: &Self) -> PyDuration {
        PyDuration(self.0.duration_since(earlier.0))
    }
}

#[derive(Debug, Clone, Copy, FromPyObject)]
enum PyInstantSub {
    Instant(PyInstant),
    Duration(PyDuration),
}
