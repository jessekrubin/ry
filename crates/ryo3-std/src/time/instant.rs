use crate::time::PyDuration;
use pyo3::prelude::*;
use pyo3::pyclass::CompareOp;
use ryo3_macro_rules::py_overflow_error;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::time::Instant;

#[pyclass(name = "Instant", frozen, immutable_type, skip_from_py_object)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PyInstant(Instant);

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
    fn __repr__(&self) -> String {
        // inner string without 'Instant {' from debug...
        let mut s = format!("{:?}", self.0);
        // replace char after 'Instant' with '<' which will be the 8th char
        s.replace_range(7..8, "<");

        // append closing '}>'
        s.push('>');
        s
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

    fn __sub__(
        &self,
        other: arithmetic::PyInstantSubtractInput<'_, '_>,
    ) -> arithmetic::PyInstantSubtractOutput {
        other.subtract_from(self)
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

mod arithmetic {
    use pyo3::{IntoPyObjectExt, prelude::*};
    use ryo3_core::py_type_err;
    use ryo3_macro_rules::py_overflow_err;
    use std::time::{Duration, Instant};

    use crate::time::{PyDuration, PyInstant};

    pub(super) enum PyInstantSubtractInput<'a, 'py> {
        Instant(Borrowed<'a, 'py, PyInstant>),
        Duration(Borrowed<'a, 'py, PyDuration>),
    }

    impl PyInstantSubtractInput<'_, '_> {
        pub(super) fn subtract_from(self, instant: &PyInstant) -> PyInstantSubtractOutput {
            match self {
                Self::Instant(other_inst) => {
                    instant.0.checked_duration_since(other_inst.get().0).into()
                }
                Self::Duration(other_dur) => instant.0.checked_sub(other_dur.get().0).into(),
            }
        }
    }

    impl<'a, 'py> FromPyObject<'a, 'py> for PyInstantSubtractInput<'a, 'py> {
        type Error = PyErr;

        fn extract(obj: Borrowed<'a, 'py, PyAny>) -> Result<Self, Self::Error> {
            if let Ok(inst) = obj.cast_exact::<PyInstant>() {
                Ok(Self::Instant(inst))
            } else if let Ok(dur) = obj.cast_exact::<PyDuration>() {
                Ok(Self::Duration(dur))
            } else {
                py_type_err!("unsupported operand type(s) for Instant.__sub__")
            }
        }
    }

    impl<'a, 'py> FromPyObject<'a, 'py> for &PyInstantSubtractInput<'a, 'py> {
        type Error = PyErr;

        fn extract(obj: Borrowed<'a, 'py, PyAny>) -> Result<Self, Self::Error> {
            let input = PyInstantSubtractInput::extract(obj)?;
            Ok(Box::leak(Box::new(input)))
        }
    }

    pub(super) enum PyInstantSubtractOutput {
        Duration(PyDuration),
        Instant(PyInstant),
        Overflow,
    }

    impl From<Duration> for PyInstantSubtractOutput {
        fn from(dur: Duration) -> Self {
            Self::Duration(PyDuration::from(dur))
        }
    }

    impl From<Instant> for PyInstantSubtractOutput {
        fn from(inst: Instant) -> Self {
            Self::Instant(PyInstant::from(inst))
        }
    }

    impl From<Option<Duration>> for PyInstantSubtractOutput {
        fn from(opt: Option<Duration>) -> Self {
            // why the fuck is `map_or` backwards? idk
            opt.map_or(Self::Overflow, Self::from)
        }
    }

    impl From<Option<Instant>> for PyInstantSubtractOutput {
        fn from(opt: Option<Instant>) -> Self {
            opt.map_or(Self::Overflow, Self::from)
        }
    }

    impl<'py> IntoPyObject<'py> for PyInstantSubtractOutput {
        type Target = PyAny;

        type Output = Bound<'py, Self::Target>;

        type Error = PyErr;

        fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
            match self {
                Self::Duration(dur) => dur.into_bound_py_any(py),
                Self::Instant(inst) => inst.into_bound_py_any(py),
                Self::Overflow => py_overflow_err!(),
            }
        }
    }
}
