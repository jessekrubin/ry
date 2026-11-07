use std::hash::{DefaultHasher, Hash, Hasher};
use std::sync::LazyLock;
use std::time::Instant;

use pyo3::prelude::*;
use ryo3_core::{PyNotImplementedOr, PyOperandArg};
use ryo3_macro_rules::py_overflow_error;

use crate::time::PyDuration;

#[pyclass(name = "Instant", frozen, immutable_type, eq, ord)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PyInstant(Instant);

/// repr anchor
///
/// used for instant repr formatting
pub(crate) static REPR_ANCHOR: LazyLock<Instant> = LazyLock::new(Instant::now);

impl std::fmt::Display for PyInstant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let anchor = *REPR_ANCHOR;
        let (sign, d) = match self.0.checked_duration_since(anchor) {
            Some(d) => ('+', d),
            None => ('-', anchor.duration_since(self.0)),
        };
        write!(f, "Instant<{sign}{}.{:09}s>", d.as_secs(), d.subsec_nanos())
    }
}

impl From<Instant> for PyInstant {
    fn from(i: Instant) -> Self {
        Self(i)
    }
}

type DurationArg<'a, 'py> = PyOperandArg<Borrowed<'a, 'py, PyDuration>>;

impl PyInstant {
    fn try_add(&self, dur: &PyDuration) -> PyResult<Self> {
        self.0
            .checked_add(dur.0)
            .map(Self::from)
            .ok_or_else(|| py_overflow_error!("instant-overflow-add"))
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
        format!("{self}")
    }

    #[must_use]
    fn __hash__(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.0.hash(&mut hasher);
        hasher.finish()
    }

    fn __add__(&self, other: DurationArg<'_, '_>) -> PyResult<PyNotImplementedOr<Self>> {
        other.try_map(|dur| self.try_add(dur.get()))
    }

    fn __radd__(&self, other: DurationArg<'_, '_>) -> PyResult<PyNotImplementedOr<Self>> {
        other.try_map(|dur| self.try_add(dur.get()))
    }

    fn __sub__(
        &self,
        other: PyOperandArg<arithmetic::PyInstantSubtractInput<'_, '_>>,
    ) -> PyNotImplementedOr<arithmetic::PyInstantSubtractOutput> {
        other.map(|rhs| rhs.subtract_from(self))
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
    use std::time::{Duration, Instant};

    use pyo3::IntoPyObjectExt;
    use pyo3::prelude::*;
    use ryo3_core::FromPyOperand;
    use ryo3_macro_rules::py_overflow_err;

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

    impl<'a, 'py> FromPyOperand<'a, 'py> for PyInstantSubtractInput<'a, 'py> {
        fn from_py_operand(obj: Borrowed<'a, 'py, PyAny>) -> Option<Self> {
            use ryo3_core::PyOperandExt;
            obj.operand(Self::Instant).or(Self::Duration).ok()
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
