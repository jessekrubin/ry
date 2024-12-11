use crate::errors::map_py_value_err;
use crate::pydatetime_conversions::signed_duration_from_pyobject;
use crate::ry_span::RySpan;
use crate::JiffSignedDuration;
use jiff::{SignedDuration, Span};
use pyo3::basic::CompareOp;
use pyo3::types::{PyAnyMethods, PyDelta, PyType};
use pyo3::{pyclass, pymethods, Bound, FromPyObject, IntoPyObject, PyAny, PyErr, PyResult, Python};
use ryo3_macros::err_py_not_impl;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::str::FromStr;

#[derive(Debug, Clone)]
#[pyclass(name = "SignedDuration", module = "ryo3")]
pub struct RySignedDuration(pub(crate) SignedDuration);

#[pymethods]
impl RySignedDuration {
    #[new]
    fn new(secs: i64, nanos: i32) -> Self {
        Self(SignedDuration::new(secs, nanos))
    }

    #[allow(non_snake_case)]
    #[classattr]
    fn MIN() -> Self {
        Self(SignedDuration::MIN)
    }

    #[allow(non_snake_case)]
    #[classattr]
    fn MAX() -> Self {
        Self(SignedDuration::MAX)
    }

    #[allow(non_snake_case)]
    #[classattr]
    fn ZERO() -> Self {
        Self(SignedDuration::ZERO)
    }

    #[getter]
    fn secs(&self) -> i64 {
        self.0.as_secs()
    }

    #[getter]
    fn nanos(&self) -> i32 {
        self.0.subsec_nanos()
    }

    #[classmethod]
    fn parse(_cls: &Bound<'_, PyType>, s: &str) -> PyResult<Self> {
        SignedDuration::from_str(s)
            .map(RySignedDuration::from)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e}")))
    }

    fn is_negative(&self) -> bool {
        self.0.is_negative()
    }

    fn is_zero(&self) -> bool {
        self.0.is_zero()
    }

    #[classmethod]
    fn from_pytimedelta<'py>(
        _cls: &Bound<'py, PyType>,
        // py: Python<'py>,
        delta: &Bound<'py, PyAny>,
    ) -> PyResult<Self> {
        delta.extract::<JiffSignedDuration>().map(Self::from)
    }

    fn to_pytimedelta<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDelta>> {
        JiffSignedDuration(self.0).into_pyobject(py)
    }

    fn to_timespan(&self) -> PyResult<RySpan> {
        Span::try_from(self.0)
            .map(RySpan::from)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyOverflowError, _>(format!("{e}")))
    }

    fn __abs__(&self) -> Self {
        Self(self.0.abs())
    }

    fn string(&self) -> String {
        self.0.to_string()
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
    fn __hash__(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.0.hash(&mut hasher);
        hasher.finish()
    }

    fn __add__(&self, other: &RySignedDuration) -> PyResult<Self> {
        let maybe_dur = self.0.checked_add(other.0);
        match maybe_dur {
            Some(dur) => Ok(RySignedDuration(dur)),
            None => Err(PyErr::new::<pyo3::exceptions::PyOverflowError, _>(
                "overflow",
            )),
        }
    }

    fn __sub__(&self, other: &RySignedDuration) -> PyResult<Self> {
        let dur = self.0.checked_sub(other.0);
        match dur {
            Some(dur) => Ok(RySignedDuration(dur)),
            None => Err(PyErr::new::<pyo3::exceptions::PyOverflowError, _>(
                "overflow",
            )),
        }
    }

    fn __mul__(&self, other: i32) -> PyResult<Self> {
        let dur = self.0.checked_mul(other);
        match dur {
            Some(dur) => Ok(RySignedDuration(dur)),
            None => Err(PyErr::new::<pyo3::exceptions::PyOverflowError, _>(
                "overflow",
            )),
        }
    }

    fn __div__(&self, other: i32) -> PyResult<Self> {
        let dur = self.0.checked_div(other);
        match dur {
            Some(dur) => Ok(RySignedDuration(dur)),
            None => Err(PyErr::new::<pyo3::exceptions::PyOverflowError, _>(
                "overflow",
            )),
        }
    }

    fn __neg__(&self) -> PyResult<Self> {
        self.0
            .checked_neg()
            .map(RySignedDuration::from)
            .ok_or_else(|| {
                PyErr::new::<pyo3::exceptions::PyValueError, _>("negation does not exist")
            })
    }

    #[getter]
    fn days(&self) -> i64 {
        self.0.as_secs() / 86400
    }

    #[getter]
    fn seconds(&self) -> i64 {
        self.0.as_secs() % 86400
    }

    #[getter]
    fn microseconds(&self) -> i32 {
        self.0.subsec_micros()
    }

    fn __richcmp__<'py>(
        &self,
        py: Python<'py>,
        other: RySignedDurationComparable<'py>,
        op: CompareOp,
    ) -> PyResult<bool> {
        match other {
            RySignedDurationComparable::RySignedDuration(other) => match op {
                CompareOp::Eq => Ok(self.0 == other.0),
                CompareOp::Ne => Ok(self.0 != other.0),
                CompareOp::Lt => Ok(self.0 < other.0),
                CompareOp::Le => Ok(self.0 <= other.0),
                CompareOp::Gt => Ok(self.0 > other.0),
                CompareOp::Ge => Ok(self.0 >= other.0),
            },
            RySignedDurationComparable::PyDelta(other) => {
                let other = signed_duration_from_pyobject(py, &other)?;
                match op {
                    CompareOp::Eq => Ok(self.0 == other),
                    CompareOp::Ne => Ok(self.0 != other),
                    CompareOp::Lt => Ok(self.0 < other),
                    CompareOp::Le => Ok(self.0 <= other),
                    CompareOp::Gt => Ok(self.0 > other),
                    CompareOp::Ge => Ok(self.0 >= other),
                }
            }
        }
    }

    // =========
    // FROM NUMS
    // =========
    #[classmethod]
    fn from_hours(_cls: &Bound<'_, PyType>, hours: i64) -> Self {
        Self(SignedDuration::from_hours(hours))
    }

    #[classmethod]
    fn from_micros(_cls: &Bound<'_, PyType>, micros: i64) -> Self {
        Self(SignedDuration::from_micros(micros))
    }

    #[classmethod]
    fn from_millis(_cls: &Bound<'_, PyType>, millis: i64) -> Self {
        Self(SignedDuration::from_millis(millis))
    }

    #[classmethod]
    fn from_mins(_cls: &Bound<'_, PyType>, mins: i64) -> Self {
        Self(SignedDuration::from_mins(mins))
    }

    #[classmethod]
    fn from_nanos(_cls: &Bound<'_, PyType>, nanos: i64) -> Self {
        Self(SignedDuration::from_nanos(nanos))
    }

    #[classmethod]
    fn from_secs(_cls: &Bound<'_, PyType>, secs: i64) -> Self {
        Self(SignedDuration::from_secs(secs))
    }

    #[classmethod]
    fn from_secs_f32(_cls: &Bound<'_, PyType>, secs: f32) -> PyResult<Self> {
        SignedDuration::try_from_secs_f32(secs)
            .map(Self::from)
            .map_err(map_py_value_err)
    }

    #[classmethod]
    fn from_secs_f64(_cls: &Bound<'_, PyType>, secs: f64) -> PyResult<Self> {
        SignedDuration::try_from_secs_f64(secs)
            .map(Self::from)
            .map_err(map_py_value_err)
    }

    fn abs(&self) -> PyResult<()> {
        err_py_not_impl!()
    }
    fn as_hours(&self) -> PyResult<()> {
        err_py_not_impl!()
    }
    fn as_micros(&self) -> PyResult<()> {
        err_py_not_impl!()
    }
    fn as_millis(&self) -> PyResult<()> {
        err_py_not_impl!()
    }
    fn as_millis_f32(&self) -> PyResult<()> {
        err_py_not_impl!()
    }
    fn as_millis_f64(&self) -> PyResult<()> {
        err_py_not_impl!()
    }
    fn as_mins(&self) -> PyResult<()> {
        err_py_not_impl!()
    }
    fn as_nanos(&self) -> PyResult<()> {
        err_py_not_impl!()
    }
    fn as_secs(&self) -> PyResult<()> {
        err_py_not_impl!()
    }
    fn as_secs_f32(&self) -> PyResult<()> {
        err_py_not_impl!()
    }
    fn as_secs_f64(&self) -> PyResult<()> {
        err_py_not_impl!()
    }
    fn checked_add(&self) -> PyResult<()> {
        err_py_not_impl!()
    }
    fn checked_div(&self) -> PyResult<()> {
        err_py_not_impl!()
    }
    fn checked_mul(&self) -> PyResult<()> {
        err_py_not_impl!()
    }
    fn checked_neg(&self) -> PyResult<()> {
        err_py_not_impl!()
    }
    fn checked_sub(&self) -> PyResult<()> {
        err_py_not_impl!()
    }
    fn div_duration_f32(&self) -> PyResult<()> {
        err_py_not_impl!()
    }
    fn div_duration_f64(&self) -> PyResult<()> {
        err_py_not_impl!()
    }
    fn div_f32(&self) -> PyResult<()> {
        err_py_not_impl!()
    }
    fn div_f64(&self) -> PyResult<()> {
        err_py_not_impl!()
    }

    fn is_positive(&self) -> PyResult<()> {
        err_py_not_impl!()
    }
    fn mul_f32(&self) -> PyResult<()> {
        err_py_not_impl!()
    }
    fn mul_f64(&self) -> PyResult<()> {
        err_py_not_impl!()
    }
    fn saturating_add(&self) -> PyResult<()> {
        err_py_not_impl!()
    }
    fn saturating_mul(&self) -> PyResult<()> {
        err_py_not_impl!()
    }
    fn saturating_sub(&self) -> PyResult<()> {
        err_py_not_impl!()
    }
    fn signum(&self) -> PyResult<()> {
        err_py_not_impl!()
    }
    fn subsec_micros(&self) -> PyResult<()> {
        err_py_not_impl!()
    }
    fn subsec_millis(&self) -> PyResult<()> {
        err_py_not_impl!()
    }
    fn subsec_nanos(&self) -> PyResult<()> {
        err_py_not_impl!()
    }
    fn system_until(&self) -> PyResult<()> {
        err_py_not_impl!()
    }
    fn try_from_secs_f32(&self) -> PyResult<()> {
        err_py_not_impl!()
    }
    fn try_from_secs_f64(&self) -> PyResult<()> {
        err_py_not_impl!()
    }
    fn unsigned_abs(&self) -> PyResult<()> {
        err_py_not_impl!()
    }
}

impl From<SignedDuration> for RySignedDuration {
    fn from(d: SignedDuration) -> Self {
        Self(d)
    }
}

impl From<JiffSignedDuration> for RySignedDuration {
    fn from(d: JiffSignedDuration) -> Self {
        Self(d.0)
    }
}

#[derive(Debug, Clone, FromPyObject)]
enum RySignedDurationComparable<'py> {
    RySignedDuration(RySignedDuration),
    PyDelta(Bound<'py, PyDelta>),
}
