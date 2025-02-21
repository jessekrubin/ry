use crate::PyDuration;
use pyo3::exceptions::PyOverflowError;
use pyo3::prelude::*;
use pyo3::pyclass::CompareOp;
use pyo3::types::PyType;
use pyo3::IntoPyObjectExt;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::time::Instant;

#[derive(Debug, Clone)]
#[pyclass(name = "Instant", module = "ryo3", frozen)]
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
    pub fn py_new() -> Self {
        PyInstant(Instant::now())
    }

    #[classmethod]
    #[must_use]
    pub fn now(_cls: &Bound<'_, PyType>) -> Self {
        PyInstant(Instant::now())
    }

    #[must_use]
    pub fn __str__(&self) -> String {
        format!("{:?}", self.0)
    }

    #[must_use]
    pub fn __repr__(&self) -> String {
        format!("{:?}", self.0)
    }

    #[must_use]
    pub fn __hash__(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.0.hash(&mut hasher);
        hasher.finish()
    }

    #[must_use]
    pub fn __richcmp__(&self, other: &Self, op: CompareOp) -> bool {
        match op {
            CompareOp::Eq => self.0 == other.0,
            CompareOp::Ne => self.0 != other.0,
            CompareOp::Lt => self.0 < other.0,
            CompareOp::Le => self.0 <= other.0,
            CompareOp::Gt => self.0 > other.0,
            CompareOp::Ge => self.0 >= other.0,
        }
    }

    pub fn __add__(&self, other: &PyDuration) -> Option<Self> {
        self.0.checked_add(other.0).map(Self)
    }

    // fn __iadd__(&mut self, _py: Python<'_>, other: &PyDuration) -> PyResult<()> {
    //     let add_res = self.0.checked_add(other.0);
    //     if let Some(inst) = add_res {
    //         self.0 = inst;
    //         Ok(())
    //     } else {
    //         Err(PyErr::new::<PyOverflowError, _>("instant-overflow-iadd"))
    //     }
    // }

    pub fn __sub__<'py>(
        &self,
        py: Python<'py>,
        other: PyInstantSub,
    ) -> PyResult<Bound<'py, PyAny>> {
        match other {
            PyInstantSub::Instant(other) => {
                let dur = self.0.checked_duration_since(other.0);
                match dur {
                    Some(d) => {
                        let pyduration = PyDuration::from(d);
                        pyduration.into_bound_py_any(py)
                    }
                    None => Err(PyErr::new::<PyOverflowError, _>("instant-sub-overflow")),
                }
            }
            PyInstantSub::Duration(other) => {
                let inst = self.0.checked_sub(other.0);
                match inst {
                    Some(i) => {
                        let pyinstant = PyInstant::from(i);
                        pyinstant.into_bound_py_any(py)
                    }
                    None => Err(PyErr::new::<PyOverflowError, _>("instant-sub-overflow")),
                }
            }
        }
    }

    // pub fn __isub__(&mut self, _py: Python<'_>, other: PyInstantSub) -> PyResult<()> {
    //     match other {
    //         PyInstantSub::Instant(other) => {
    //             let dur = self.0.checked_duration_since(other.0);
    //             match dur {
    //                 Some(d) => {
    //                     let self2assign = self.0.checked_sub(d);
    //                     match self2assign {
    //                         Some(self2assign) => {
    //                             self.0 = self2assign;
    //                             Ok(())
    //                         }
    //                         None => Err(PyErr::new::<PyOverflowError, _>("instant-sub-overflow")),
    //                     }
    //                 }
    //                 None => Err(PyErr::new::<PyOverflowError, _>("instant-sub-overflow")),
    //             }
    //         }
    //         PyInstantSub::Duration(other) => {
    //             let inst = self.0.checked_sub(other.0);
    //             match inst {
    //                 Some(i) => {
    //                     self.0 = i;
    //                     Ok(())
    //                 }
    //                 None => Err(PyErr::new::<PyOverflowError, _>("instant-sub-overflow")),
    //             }
    //         }
    //     }
    // }

    #[must_use]
    pub fn elapsed(&self) -> PyDuration {
        PyDuration(self.0.elapsed())
    }

    pub fn checked_add(&self, other: &PyDuration) -> Option<Self> {
        self.0.checked_add(other.0).map(Self)
    }

    pub fn checked_sub(&self, other: &PyDuration) -> Option<Self> {
        self.0.checked_sub(other.0).map(Self::from)
    }

    pub fn checked_duration_since(&self, earlier: &Self) -> Option<PyDuration> {
        self.0
            .checked_duration_since(earlier.0)
            .map(PyDuration::from)
    }

    #[must_use]
    pub fn saturating_duration_since(&self, earlier: &Self) -> PyDuration {
        PyDuration(self.0.saturating_duration_since(earlier.0))
    }

    #[must_use]
    pub fn duration_since(&self, earlier: &Self) -> PyDuration {
        PyDuration(self.0.duration_since(earlier.0))
    }
}

#[derive(Debug, Clone, FromPyObject)]
pub enum PyInstantSub {
    Instant(PyInstant),
    Duration(PyDuration),
}

#[pyfunction]
#[must_use]
pub fn instant() -> PyInstant {
    PyInstant::from(Instant::now())
}
