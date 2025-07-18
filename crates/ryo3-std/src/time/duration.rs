use pyo3::basic::CompareOp;
use pyo3::exceptions::{PyOverflowError, PyTypeError, PyValueError, PyZeroDivisionError};
use pyo3::prelude::PyAnyMethods;
use pyo3::types::{PyDelta, PyTuple, PyType};
use pyo3::{
    Bound, FromPyObject, IntoPyObject, IntoPyObjectExt, PyAny, PyResult, Python, pyclass, pymethods,
};
use std::hash::{DefaultHasher, Hash, Hasher};
use std::ops::{Div, Mul};
use std::time::Duration;

const SECS_PER_MINUTE: u64 = 60;
const MINS_PER_HOUR: u64 = 60;
const HOURS_PER_DAY: u64 = 24;
const DAYS_PER_WEEK: u64 = 7;

const MAX_DAYS: u64 = u64::MAX / (SECS_PER_MINUTE * MINS_PER_HOUR * HOURS_PER_DAY);
const MAX_WEEKS: u64 = u64::MAX / (SECS_PER_MINUTE * MINS_PER_HOUR * HOURS_PER_DAY * DAYS_PER_WEEK);

#[derive(Debug, Clone, PartialEq)]
#[pyclass(name = "Duration", module = "ry.ryo3", frozen)]
pub struct PyDuration(pub Duration);

impl From<Duration> for PyDuration {
    fn from(d: Duration) -> Self {
        Self(d)
    }
}

impl PyDuration {
    fn try_from_secs_f32(secs: f32) -> PyResult<Self> {
        Duration::try_from_secs_f32(secs)
            .map(Self::from)
            .map_err(|e| PyValueError::new_err(format!("{e}")))
    }

    fn try_from_secs_f64(secs: f64) -> PyResult<Self> {
        Duration::try_from_secs_f64(secs)
            .map(Self::from)
            .map_err(|e| PyValueError::new_err(format!("{e}")))
    }
}

#[pymethods]
impl PyDuration {
    #[new]
    #[pyo3(signature = (secs = 0, nanos = 0))]
    fn py_new(secs: u64, nanos: u32) -> PyResult<Self> {
        let carry = nanos / 1_000_000_000;
        let nanos = nanos % 1_000_000_000;

        let secs = secs
            .checked_add(u64::from(carry))
            .ok_or_else(|| PyOverflowError::new_err("overflow in Duration::new"))?;
        Ok(Self(Duration::new(secs, nanos)))
    }

    fn __getnewargs__<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyTuple>> {
        PyTuple::new(
            py,
            vec![
                self.0.as_secs().into_bound_py_any(py)?,
                self.0.subsec_nanos().into_bound_py_any(py)?,
            ],
        )
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn ZERO() -> Self {
        Self(Duration::ZERO)
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn MIN() -> Self {
        Self(Duration::ZERO)
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn MAX() -> Self {
        Self(Duration::MAX)
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn SECOND() -> Self {
        Self(Duration::from_secs(1))
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn MILLISECOND() -> Self {
        Self(Duration::from_millis(1))
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn MICROSECOND() -> Self {
        Self(Duration::from_micros(1))
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn NANOSECOND() -> Self {
        Self(Duration::from_nanos(1))
    }

    fn __repr__(&self) -> String {
        format!(
            "Duration(secs={}, nanos={})",
            self.0.as_secs(),
            self.0.subsec_nanos()
        )
    }

    fn __hash__(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.0.hash(&mut hasher);
        hasher.finish()
    }

    fn __bool__(&self) -> bool {
        !self.0.is_zero()
    }

    fn __float__(&self) -> f64 {
        self.0.as_secs_f64()
    }

    fn __int__(&self) -> u128 {
        self.0.as_nanos()
    }

    fn dbg(&self) -> String {
        format!("Duration<{:?}>", self.0)
    }

    // ========================================================================
    // MATHS/OPERATORS
    // ========================================================================
    fn __richcmp__(&self, other: PyDurationComparable, op: CompareOp) -> bool {
        match other {
            PyDurationComparable::PyDuration(other) => match op {
                CompareOp::Eq => self.0 == other.0,
                CompareOp::Ne => self.0 != other.0,
                CompareOp::Lt => self.0 < other.0,
                CompareOp::Le => self.0 <= other.0,
                CompareOp::Gt => self.0 > other.0,
                CompareOp::Ge => self.0 >= other.0,
            },
            PyDurationComparable::Duration(other) => match op {
                CompareOp::Eq => self.0 == other,
                CompareOp::Ne => self.0 != other,
                CompareOp::Lt => self.0 < other,
                CompareOp::Le => self.0 <= other,
                CompareOp::Gt => self.0 > other,
                CompareOp::Ge => self.0 >= other,
            },
        }
    }

    fn __add__(&self, other: PyDurationComparable) -> PyResult<Self> {
        let maybe_duration = match other {
            PyDurationComparable::PyDuration(other) => self.0.checked_add(other.0),
            PyDurationComparable::Duration(other) => self.0.checked_add(other),
        };
        if let Some(duration) = maybe_duration {
            Ok(Self(duration))
        } else {
            Err(PyOverflowError::new_err("overflow in Duration addition"))
        }
    }

    fn __sub__(&self, other: PyDurationComparable) -> PyResult<Self> {
        let maybe_duration = match other {
            PyDurationComparable::PyDuration(other) => self.0.checked_sub(other.0),
            PyDurationComparable::Duration(other) => self.0.checked_sub(other),
        };
        if let Some(duration) = maybe_duration {
            Ok(Self(duration))
        } else {
            Err(PyOverflowError::new_err("overflow in Duration subtraction"))
        }
    }

    fn __truediv__<'py>(
        &self,
        py: Python<'py>,
        other: PyDurationArithmeticDiv,
    ) -> PyResult<Bound<'py, PyAny>> {
        match other {
            PyDurationArithmeticDiv::Int(other) => {
                if other == 0 {
                    return Err(PyZeroDivisionError::new_err("division by zero"));
                }
                self.0
                    .checked_div(other)
                    .map(Self::from)
                    .ok_or_else(|| PyOverflowError::new_err("overflow in Duration division"))?
                    .into_bound_py_any(py)
            }
            PyDurationArithmeticDiv::Float(other) => self.div_f64(other)?.into_bound_py_any(py),
            PyDurationArithmeticDiv::PyDuration(other) => {
                let result = self.0.div_duration_f64(other.0);
                Self::try_from_secs_f64(result)?.into_bound_py_any(py)
            }
            PyDurationArithmeticDiv::Duration(other) => {
                let result = self.0.div_duration_f64(other);
                Self::try_from_secs_f64(result)?.into_bound_py_any(py)
            }
        }
    }

    fn __mul__(&self, other: &Bound<'_, PyAny>) -> PyResult<Self> {
        if let Ok(i) = other.extract::<u32>() {
            self.0
                .checked_mul(i)
                .map(Self::from)
                .ok_or_else(|| PyOverflowError::new_err("overflow in Duration multiplication"))
        } else if let Ok(f) = other.extract::<f64>() {
            self.mul_f64(f)
        } else {
            return Err(PyTypeError::new_err(
                "unsupported operand type(s); must be int | float",
            ));
        }
    }

    // ========================================================================
    // PROPERTIES/GETTERS
    // ========================================================================
    #[getter]
    fn is_zero(&self) -> bool {
        self.0.is_zero()
    }

    #[getter]
    fn secs(&self) -> u64 {
        self.0.as_secs()
    }

    #[getter]
    fn nanos(&self) -> u32 {
        self.0.subsec_nanos()
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

    #[getter]
    fn subsec_millis(&self) -> u32 {
        self.0.subsec_millis()
    }

    #[getter]
    fn subsec_micros(&self) -> u32 {
        self.0.subsec_micros()
    }

    #[getter]
    fn subsec_nanos(&self) -> u32 {
        self.0.subsec_nanos()
    }

    // ========================================================================
    // TO-CONVERSIONS
    // ========================================================================
    fn as_secs(&self) -> f64 {
        self.0.as_secs_f64()
    }

    fn as_secs_f32(&self) -> f32 {
        self.0.as_secs_f32()
    }

    fn as_secs_f64(&self) -> f64 {
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

    // ========================================================================
    // PYTHON CONVERSIONS
    // ========================================================================

    /// Convert to python `datetime.timedelta`
    fn to_py(&self) -> Duration {
        self.0
    }

    /// Convert to python `datetime.timedelta`
    fn to_pytimedelta<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDelta>> {
        self.0.into_pyobject(py)
    }

    /// Convert from python `datetime.timedelta`
    #[classmethod]
    fn from_pytimedelta(_cls: &Bound<'_, PyType>, delta: Duration) -> Self {
        Self(delta)
    }

    // ========================================================================
    // FROM NUMBERS
    // ========================================================================

    /// Create a new `Duration` from the specified number of seconds.
    #[classmethod]
    fn from_secs(_cls: &Bound<'_, PyType>, secs: u64) -> Self {
        Self(Duration::from_secs(secs))
    }

    #[classmethod]
    fn from_millis(_cls: &Bound<'_, PyType>, millis: u64) -> Self {
        Self(Duration::from_millis(millis))
    }

    #[classmethod]
    fn from_micros(_cls: &Bound<'_, PyType>, micros: u64) -> Self {
        Self(Duration::from_micros(micros))
    }

    #[classmethod]
    fn from_nanos(_cls: &Bound<'_, PyType>, nanos: u64) -> Self {
        Self(Duration::from_nanos(nanos))
    }

    #[classmethod]
    fn from_hours(_cls: &Bound<'_, PyType>, hours: u64) -> PyResult<Self> {
        if hours > u64::MAX / (60 * 60) {
            Err(PyOverflowError::new_err("overflow in Duration::from_hours"))
        } else {
            Ok(Self(Duration::from_secs(hours * 60 * 60)))
        }
    }

    #[classmethod]
    fn from_mins(_cls: &Bound<'_, PyType>, mins: u64) -> PyResult<Self> {
        if mins > u64::MAX / 60 {
            Err(PyOverflowError::new_err("overflow in Duration::from_mins"))
        } else {
            Ok(Self(Duration::from_secs(mins * 60)))
        }
    }

    #[classmethod]
    fn from_days(_cls: &Bound<'_, PyType>, days: u64) -> PyResult<Self> {
        if days > u64::MAX / MAX_DAYS {
            Err(PyOverflowError::new_err(format!(
                "overflow in Duration::from_days: {days} > {MAX_DAYS}"
            )))
        } else {
            Ok(Self(Duration::from_secs(days * 60 * 60 * 24)))
        }
    }

    #[classmethod]
    fn from_weeks(_cls: &Bound<'_, PyType>, weeks: u64) -> PyResult<Self> {
        if weeks > u64::MAX / (MAX_WEEKS) {
            Err(PyOverflowError::new_err(format!(
                "overflow in Duration::from_weeks: {weeks} > {MAX_WEEKS}"
            )))
        } else {
            Ok(Self(Duration::from_secs(weeks * 60 * 60 * 24 * 7)))
        }
    }

    #[classmethod]
    fn from_secs_f32(_cls: &Bound<'_, PyType>, secs: f32) -> PyResult<Self> {
        Self::try_from_secs_f32(secs)
    }

    #[classmethod]
    fn from_secs_f64(_cls: &Bound<'_, PyType>, secs: f64) -> PyResult<Self> {
        Self::try_from_secs_f64(secs)
    }

    // ========================================================================
    // METHODS
    // ========================================================================
    #[pyo3(signature = (interval = None))]
    /// Sleep for the duration
    pub fn sleep(&self, py: Python<'_>, interval: Option<u64>) -> PyResult<()> {
        let interval = match interval {
            Some(interval) => {
                if interval > 1000 {
                    return Err(PyValueError::new_err(
                        "interval must be less than or equal to 1000",
                    ));
                }
                interval
            }
            None => 10,
        };
        let sleep_duration = self.0;
        let check_interval = Duration::from_millis(interval);
        let mut remaining = sleep_duration;
        while remaining > check_interval {
            py.check_signals()?; // This ensures signals are handled
            std::thread::sleep(check_interval);
            remaining -= check_interval;
        }
        if remaining > Duration::ZERO {
            py.check_signals()?; // One last signal check before sleeping
            std::thread::sleep(remaining);
        }

        Ok(())
    }

    // ========================================================================
    // SLEEP CROSSBEAM IMPL
    // ========================================================================
    // pub fn sleep<'py>(&self, py: Python<'py>) -> PyResult<()> {
    //     // channel for sleepytime to signal done
    //     let (tx, rx) = crossbeam_channel::bounded::<()>(1);
    //     let duration = self.0;
    //     std::thread::spawn(move || {
    //         std::thread::sleep(duration);
    //         let _ = tx.send(());
    //     });
    //     loop {
    //         py.check_signals()?;
    //         crossbeam_channel::select! {
    //             recv(rx) -> _ => {
    //                 break;
    //             }
    //             default(Duration::from_millis(10)) => {
    //             }
    //         }
    //     }
    //     Ok(())
    // }

    fn abs_diff(&self, other: PyDurationComparable) -> Self {
        match other {
            PyDurationComparable::PyDuration(other) => Self(self.0.abs_diff(other.0)),
            PyDurationComparable::Duration(other) => Self(self.0.abs_diff(other)),
        }
    }

    fn checked_add(&self, other: &Self) -> Option<Self> {
        self.0.checked_add(other.0).map(Self::from)
    }

    fn checked_div(&self, other: u32) -> Option<Self> {
        self.0.checked_div(other).map(Self::from)
    }

    fn checked_mul(&self, other: u32) -> Option<Self> {
        self.0.checked_mul(other).map(Self::from)
    }

    fn checked_sub(&self, other: &Self) -> Option<Self> {
        self.0.checked_sub(other.0).map(Self::from)
    }

    fn div_duration_f32(&self, other: &Self) -> f32 {
        self.0.div_duration_f32(other.0)
    }

    fn div_duration_f64(&self, other: &Self) -> f64 {
        self.0.div_duration_f64(other.0)
    }

    fn div_f32(&self, n: f32) -> PyResult<Self> {
        if n == 0.0 {
            return Err(PyZeroDivisionError::new_err("division by zero"));
        }
        if n.is_sign_negative() || n.is_infinite() || n.is_nan() {
            return Err(PyValueError::new_err("invalid value"));
        }
        let result = self.0.as_secs_f32().div(n);
        Self::try_from_secs_f32(result)
    }

    fn div_f64(&self, n: f64) -> PyResult<Self> {
        if n == 0.0 {
            return Err(PyZeroDivisionError::new_err("division by zero"));
        }
        if n.is_sign_negative() || n.is_infinite() || n.is_nan() {
            return Err(PyValueError::new_err("invalid value"));
        }
        let result = self.0.as_secs_f64().div(n);
        Self::try_from_secs_f64(result)
    }

    fn mul_f32(&self, n: f32) -> PyResult<Self> {
        let result = self.0.as_secs_f32().mul(n);
        Self::try_from_secs_f32(result)
    }

    fn mul_f64(&self, n: f64) -> PyResult<Self> {
        let result = self.0.as_secs_f64().mul(n);
        Self::try_from_secs_f64(result)
    }

    fn saturating_add(&self, other: &Self) -> Self {
        Self::from(self.0.saturating_add(other.0))
    }

    fn saturating_mul(&self, other: u32) -> Self {
        Self::from(self.0.saturating_mul(other))
    }

    fn saturating_sub(&self, other: &Self) -> Self {
        Self::from(self.0.saturating_sub(other.0))
    }
}

#[derive(Debug, Clone, FromPyObject)]
enum PyDurationComparable {
    PyDuration(PyDuration),
    Duration(Duration),
}

#[derive(Debug, Clone, FromPyObject)]
enum PyDurationArithmeticDiv {
    Int(u32), // matches Duration::checked_mul
    Float(f64),
    PyDuration(PyDuration),
    Duration(Duration),
}
