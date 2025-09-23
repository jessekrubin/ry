use crate::JiffRoundMode;
use crate::JiffSignedDuration;
use crate::JiffUnit;
use crate::errors::map_py_overflow_err;
use crate::errors::map_py_value_err;
use crate::pydatetime_conversions::signed_duration_from_pyobject;
use crate::round::RySignedDurationRound;
use crate::ry_span::RySpan;
use jiff::SignedDurationRound;
use jiff::{SignedDuration, Span};
use pyo3::prelude::*;

use pyo3::IntoPyObjectExt;
use pyo3::basic::CompareOp;
use pyo3::types::{PyDelta, PyDict, PyFloat, PyInt, PyTuple};
use ryo3_macro_rules::py_overflow_err;
use ryo3_macro_rules::{
    any_repr, py_overflow_error, py_type_err, py_value_err, py_value_error, py_zero_division_err,
};
use ryo3_std::time::PyDuration;
use std::fmt::Display;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::ops::Div;
use std::ops::Mul;
use std::str::FromStr;

const NANOS_PER_SEC: i32 = 1_000_000_000;
// const NANOS_PER_MILLI: i32 = 1_000_000;
// const NANOS_PER_MICRO: i32 = 1_000;
// const MILLIS_PER_SEC: i64 = 1_000;
// const MICROS_PER_SEC: i64 = 1_000_000;
const SECS_PER_MINUTE: i64 = 60;
const MINS_PER_HOUR: i64 = 60;

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[pyclass(name = "SignedDuration", frozen)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
pub struct RySignedDuration(pub(crate) SignedDuration);

impl RySignedDuration {
    pub(crate) fn py_try_from_secs_f32(secs: f32) -> PyResult<Self> {
        if secs.is_nan() {
            py_value_err!("invalid value: nan")
        } else if secs.is_infinite() {
            py_overflow_err!("invalid value: inf")
        } else {
            SignedDuration::try_from_secs_f32(secs)
                .map(Self::from)
                .map_err(|e| py_overflow_error!("{e}"))
        }
    }

    pub(crate) fn py_try_from_secs_f64(secs: f64) -> PyResult<Self> {
        if secs.is_nan() {
            py_value_err!("invalid value: nan")
        } else if secs.is_infinite() {
            py_overflow_err!("invalid value: inf")
        } else {
            SignedDuration::try_from_secs_f64(secs)
                .map(Self::from)
                .map_err(|e| py_overflow_error!("{e}"))
        }
    }
}

#[pymethods]
#[expect(clippy::wrong_self_convention)]
impl RySignedDuration {
    #[new]
    #[pyo3(signature = (secs = 0, nanos = 0))]
    pub fn py_new(secs: i64, nanos: i32) -> PyResult<Self> {
        #[expect(clippy::cast_lossless)]
        if !(-NANOS_PER_SEC < nanos && nanos < NANOS_PER_SEC) {
            let addsecs = nanos / NANOS_PER_SEC;
            secs.checked_add(addsecs as i64).ok_or_else(|| {
                py_overflow_error!("nanoseconds overflowed seconds in PySignedDuration::new")
            })?;
        }
        Ok(Self::from(SignedDuration::new(secs, nanos)))
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
    fn MIN() -> Self {
        Self(SignedDuration::MIN)
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn MAX() -> Self {
        Self(SignedDuration::MAX)
    }

    #[expect(non_snake_case)]
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

    #[getter]
    fn is_positive(&self) -> bool {
        self.0.is_positive()
    }

    #[getter]
    fn is_negative(&self) -> bool {
        self.0.is_negative()
    }

    #[getter]
    fn is_zero(&self) -> bool {
        self.0.is_zero()
    }

    #[getter]
    fn subsec_micros(&self) -> i32 {
        self.0.subsec_micros()
    }

    #[getter]
    fn subsec_millis(&self) -> i32 {
        self.0.subsec_millis()
    }

    #[getter]
    fn subsec_nanos(&self) -> i32 {
        self.0.subsec_nanos()
    }

    #[staticmethod]
    fn from_str(s: &str) -> PyResult<Self> {
        SignedDuration::from_str(s)
            .map(Self::from)
            .map_err(map_py_value_err)
    }

    #[staticmethod]
    fn parse(input: &str) -> PyResult<Self> {
        Self::from_str(input)
    }

    #[staticmethod]
    fn from_pytimedelta(delta: SignedDuration) -> Self {
        Self(delta)
    }

    fn to_py(&self) -> &SignedDuration {
        &self.0
    }

    fn to_pytimedelta(&self) -> &SignedDuration {
        &self.0
    }

    #[expect(clippy::wrong_self_convention)]
    fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        use crate::interns;
        let dict = PyDict::new(py);
        dict.set_item(interns::secs(py), self.0.as_secs())?;
        dict.set_item(interns::nanos(py), self.0.subsec_nanos())?;
        Ok(dict)
    }

    fn to_timespan(&self) -> PyResult<RySpan> {
        Span::try_from(self.0)
            .map(RySpan::from)
            .map_err(map_py_overflow_err)
    }

    fn __abs__(&self) -> Self {
        Self(self.0.abs())
    }

    fn abs(&self) -> Self {
        self.__abs__()
    }

    fn unsigned_abs(&self) -> PyDuration {
        PyDuration::from(self.0.unsigned_abs())
    }

    fn __format__(&self, fmt: &str) -> PyResult<String> {
        if fmt == "#" {
            Ok(format!("{:#}", self.0))
        } else if fmt.is_empty() {
            Ok(self.0.to_string())
        } else {
            py_type_err!("Invalid format specifier '{fmt}' for SignedDuration")
        }
    }

    fn isoformat(&self) -> String {
        crate::constants::SPAN_PRINTER.duration_to_string(&self.0)
    }

    #[staticmethod]
    fn from_isoformat(s: &str) -> PyResult<Self> {
        crate::constants::SPAN_PARSER
            .parse_duration(s)
            .map(Self::from)
            .map_err(map_py_value_err)
    }

    #[pyo3(
        warn(
            message = "obj.string() is deprecated, use `obj.to_string()` or `str(obj)` [remove in 0.0.60]",
            category = pyo3::exceptions::PyDeprecationWarning
        ),
        signature = (*, friendly=false)
    )]
    fn string(&self, friendly: bool) -> String {
        if friendly {
            self.friendly()
        } else {
            self.__str__()
        }
    }

    #[pyo3(signature = (*, friendly=false), name = "to_string")]
    fn py_to_string(&self, friendly: bool) -> String {
        if friendly {
            format!("{:#}", self.0)
        } else {
            self.0.to_string()
        }
    }

    fn friendly(&self) -> String {
        format!("{:#}", self.0)
    }

    fn __float__(&self) -> f64 {
        self.0.as_secs_f64()
    }

    fn __int__(&self) -> i128 {
        self.0.as_nanos()
    }

    fn __bool__(&self) -> bool {
        !self.0.is_zero()
    }

    fn __str__(&self) -> String {
        self.0.to_string()
    }

    fn __repr__(&self) -> String {
        format!("{self}")
    }

    fn __hash__(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.0.hash(&mut hasher);
        hasher.finish()
    }

    fn __add__(&self, other: &Bound<'_, PyAny>) -> PyResult<Self> {
        if let Ok(d) = other.cast_exact::<Self>() {
            let rs_dur = d.get();
            self.0
                .checked_add(rs_dur.0)
                .map(Self::from)
                .ok_or_else(|| py_overflow_error!())
        } else if let Ok(d) = other.cast::<pyo3::types::PyDelta>() {
            let rs_dur: JiffSignedDuration = d.extract()?;
            self.0
                .checked_add(rs_dur.0)
                .map(Self::from)
                .ok_or_else(|| py_overflow_error!())
        } else {
            py_type_err!("unsupported operand type(s); must be SignedDuration | datetime.timedelta")
        }
    }

    fn __radd__(&self, other: &Bound<'_, PyAny>) -> PyResult<Self> {
        self.__add__(other)
    }

    fn __sub__(&self, other: &Bound<'_, PyAny>) -> PyResult<Self> {
        if let Ok(d) = other.cast_exact::<Self>() {
            let rs_dur = d.get();
            self.0
                .checked_sub(rs_dur.0)
                .map(Self::from)
                .ok_or_else(|| py_overflow_error!())
        } else if let Ok(d) = other.cast::<pyo3::types::PyDelta>() {
            let rs_dur: JiffSignedDuration = d.extract()?;
            self.0
                .checked_sub(rs_dur.0)
                .map(Self::from)
                .ok_or_else(|| py_overflow_error!())
        } else {
            py_type_err!("unsupported operand type(s); must be Duration | datetime.timedelta")
        }
    }

    fn __rsub__(&self, other: &Bound<'_, PyAny>) -> PyResult<Self> {
        self.__sub__(other)
    }

    fn __mul__(&self, other: &Bound<'_, PyAny>) -> PyResult<Self> {
        if let Ok(i) = other.extract::<i32>() {
            self.0
                .checked_mul(i)
                .map(Self::from)
                .ok_or_else(|| py_overflow_error!())
        } else if let Ok(f) = other.extract::<f64>() {
            self.mul_f64(f)
        } else {
            py_type_err!("unsupported operand type(s); must be int or float")
        }
    }

    fn __rmul__(&self, other: &Bound<'_, PyAny>) -> PyResult<Self> {
        self.__mul__(other)
    }

    fn __truediv__<'py>(
        &self,
        py: Python<'py>,
        other: &Bound<'py, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        if let Ok(dur) = other.cast_exact::<Self>() {
            self.div_duration_f64(dur.get())?.into_bound_py_any(py)
        } else if let Ok(n) = other.extract::<i32>() {
            if n == 0 {
                py_zero_division_err!("Cannot divide SignedDuration by zero")
            } else {
                self.checked_div(n)
                    .map(|d| d.into_bound_py_any(py))
                    .ok_or_else(|| py_overflow_error!())?
            }
        } else if let Ok(n) = other.extract::<f64>() {
            self.div_f64(n)?.into_bound_py_any(py)
        } else if let Ok(d) = other.cast::<pyo3::types::PyDelta>() {
            let rs_dur: JiffSignedDuration = d.extract()?;
            if rs_dur.0.is_zero() {
                py_zero_division_err!()
            } else {
                self.0.div_duration_f64(rs_dur.0).into_bound_py_any(py)
            }
        } else {
            py_type_err!("Unsupported type for division with SignedDuration")
        }
    }

    fn __neg__(&self) -> PyResult<Self> {
        self.0
            .checked_neg()
            .map(Self::from)
            .ok_or_else(|| py_value_error!("negation does not exist"))
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

    fn __richcmp__(&self, other: RySignedDurationComparable<'_>, op: CompareOp) -> PyResult<bool> {
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
                let other = signed_duration_from_pyobject(&other)?;
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
    #[staticmethod]
    fn from_hours(hours: i64) -> PyResult<Self> {
        const MIN_HOUR: i64 = i64::MIN / (SECS_PER_MINUTE * MINS_PER_HOUR);
        // OK because (SECS_PER_MINUTE*MINS_PER_HOUR)!={-1,0}.
        const MAX_HOUR: i64 = i64::MAX / (SECS_PER_MINUTE * MINS_PER_HOUR);
        if (MIN_HOUR..=MAX_HOUR).contains(&hours) {
            Ok(Self(SignedDuration::from_hours(hours)))
        } else {
            Err(py_overflow_error!(
                "hours value {hours} out of range [{MIN_HOUR}, {MAX_HOUR}]"
            ))
        }
    }

    #[staticmethod]
    fn from_micros(micros: i64) -> Self {
        Self(SignedDuration::from_micros(micros))
    }

    #[staticmethod]
    fn from_millis(millis: i64) -> Self {
        Self(SignedDuration::from_millis(millis))
    }

    #[staticmethod]
    fn from_mins(mins: i64) -> PyResult<Self> {
        const MIN_MINUTE: i64 = i64::MIN / SECS_PER_MINUTE;
        const MAX_MINUTE: i64 = i64::MAX / SECS_PER_MINUTE;
        if (MIN_MINUTE..=MAX_MINUTE).contains(&mins) {
            Ok(Self(SignedDuration::from_mins(mins)))
        } else {
            Err(py_overflow_error!(
                "minutes value {mins} out of range [{MIN_MINUTE}, {MAX_MINUTE}]"
            ))
        }
    }

    #[staticmethod]
    fn from_nanos(nanos: i64) -> Self {
        Self(SignedDuration::from_nanos(nanos))
    }

    #[staticmethod]
    fn from_secs(secs: &Bound<'_, PyAny>) -> PyResult<Self> {
        if let Ok(secs) = secs.extract::<i64>() {
            Ok(Self(SignedDuration::from_secs(secs)))
        } else if let Ok(secs) = secs.extract::<f64>() {
            Self::py_try_from_secs_f64(secs)
        } else {
            py_type_err!("Invalid type for seconds; expected i64 or f64")
        }
    }

    #[staticmethod]
    fn from_secs_f32(secs: f32) -> PyResult<Self> {
        Self::py_try_from_secs_f32(secs)
    }

    #[staticmethod]
    fn from_secs_f64(secs: f64) -> PyResult<Self> {
        Self::py_try_from_secs_f64(secs)
    }

    // =======
    // AS NUMS
    // =======
    fn as_hours(&self) -> i64 {
        self.0.as_hours()
    }

    fn as_micros(&self) -> i128 {
        self.0.as_micros()
    }

    fn as_millis(&self) -> i128 {
        self.0.as_millis()
    }

    fn as_millis_f32(&self) -> f32 {
        self.0.as_millis_f32()
    }

    fn as_millis_f64(&self) -> f64 {
        self.0.as_millis_f64()
    }

    fn as_mins(&self) -> i64 {
        self.0.as_mins()
    }

    fn as_nanos(&self) -> i128 {
        self.0.as_nanos()
    }

    fn as_secs(&self) -> i64 {
        self.0.as_secs()
    }

    fn as_secs_f32(&self) -> f32 {
        self.0.as_secs_f32()
    }

    fn as_secs_f64(&self) -> f64 {
        self.0.as_secs_f64()
    }

    // ------------------------------------------------------------------------
    // CHECKED-ARITHMETIC
    // ------------------------------------------------------------------------
    fn checked_add(&self, other: &Self) -> Option<Self> {
        self.0.checked_add(other.0).map(Self::from)
    }

    fn checked_div(&self, other: i32) -> Option<Self> {
        self.0.checked_div(other).map(Self::from)
    }

    fn checked_mul(&self, other: i32) -> Option<Self> {
        self.0.checked_mul(other).map(Self::from)
    }

    fn checked_neg(&self) -> Option<Self> {
        self.0.checked_neg().map(Self::from)
    }

    fn checked_sub(&self, other: &Self) -> Option<Self> {
        self.0.checked_sub(other.0).map(Self::from)
    }

    fn div_duration_f32(&self, other: &Self) -> PyResult<f32> {
        if other.0.is_zero() {
            py_zero_division_err!("Cannot divide SignedDuration by zero")
        } else {
            Ok(self.0.div_duration_f32(other.0))
        }
    }

    fn div_duration_f64(&self, other: &Self) -> PyResult<f64> {
        if other.0.is_zero() {
            py_zero_division_err!("Cannot divide SignedDuration by zero")
        } else {
            Ok(self.0.div_duration_f64(other.0))
        }
    }

    fn div_f32(&self, n: f32) -> PyResult<Self> {
        if n.abs() == 0.0 {
            py_zero_division_err!()
        } else if n.is_nan() {
            py_value_err!("invalid value: nan")
        } else if n.is_infinite() {
            py_overflow_err!("invalid value: inf")
        } else if n.is_sign_negative() {
            py_type_err!("negative divisor")
        } else {
            let result = self.0.as_secs_f32().div(n);
            Self::py_try_from_secs_f32(result)
        }
    }

    fn div_f64(&self, n: f64) -> PyResult<Self> {
        if n.abs() == 0.0 {
            py_zero_division_err!()
        } else if n.is_nan() {
            py_value_err!("invalid value: nan")
        } else if n.is_infinite() {
            py_overflow_err!("invalid value: inf")
        } else if n.is_sign_negative() {
            py_type_err!("negative divisor")
        } else {
            let result = self.0.as_secs_f64().div(n);
            Self::py_try_from_secs_f64(result)
        }
    }

    fn mul_f32(&self, n: f32) -> PyResult<Self> {
        if n.abs() == 0.0 {
            Ok(Self::from(SignedDuration::ZERO))
        } else if n.is_infinite() {
            py_overflow_err!()
        } else if n.is_nan() {
            py_value_err!("invalid value: nan")
        } else {
            let result = self.0.as_secs_f32().mul(n);
            Self::py_try_from_secs_f32(result)
        }
    }

    fn mul_f64(&self, n: f64) -> PyResult<Self> {
        if n.abs() == 0.0 {
            Ok(Self::from(SignedDuration::ZERO))
        } else if n.is_infinite() {
            py_overflow_err!()
        } else if n.is_nan() {
            py_value_err!("invalid value: nan")
        } else {
            let result = self.0.as_secs_f64().mul(n);
            Self::py_try_from_secs_f64(result)
        }
    }
    // ------------------------------------------------------------------------
    // SATURATING-ARITHMETIC
    // ------------------------------------------------------------------------
    fn saturating_add(&self, other: &Self) -> Self {
        Self::from(self.0.saturating_add(other.0))
    }

    fn saturating_mul(&self, other: i32) -> Self {
        Self::from(self.0.saturating_mul(other))
    }

    fn saturating_sub(&self, other: &Self) -> Self {
        Self::from(self.0.saturating_sub(other.0))
    }

    fn signum(&self) -> i8 {
        self.0.signum()
    }

    // ========================================================================
    // ROUND
    // ========================================================================
    #[pyo3(
       signature = (smallest=None, *, mode = None, increment = None),
    )]
    fn round(
        &self,
        smallest: Option<JiffUnit>,
        mode: Option<JiffRoundMode>,
        increment: Option<i64>,
    ) -> PyResult<Self> {
        let mut dt_round = SignedDurationRound::new();
        if let Some(smallest) = smallest {
            dt_round = dt_round.smallest(smallest.0);
        }
        if let Some(mode) = mode {
            dt_round = dt_round.mode(mode.0);
        }
        if let Some(increment) = increment {
            dt_round = dt_round.increment(increment);
        }
        self.0
            .round(dt_round)
            .map(Self::from)
            .map_err(map_py_value_err)
    }

    fn _round(&self, dt_round: &RySignedDurationRound) -> PyResult<Self> {
        self.0
            .round(dt_round.jiff_round)
            .map(Self::from)
            .map_err(map_py_value_err)
    }

    #[staticmethod]
    fn from_any<'py>(value: &Bound<'py, PyAny>) -> PyResult<Bound<'py, PyAny>> {
        let py = value.py();
        if let Ok(pystr) = value.cast::<pyo3::types::PyString>() {
            let s = pystr.extract::<&str>()?;
            Self::from_str(s).map(|dt| dt.into_bound_py_any(py).map(Bound::into_any))?
        } else if let Ok(pybytes) = value.cast::<pyo3::types::PyBytes>() {
            let s = String::from_utf8_lossy(pybytes.as_bytes());
            Self::from_str(&s).map(|dt| dt.into_bound_py_any(py).map(Bound::into_any))?
        } else if value.is_exact_instance_of::<Self>() {
            value.into_bound_py_any(py)
        } else if let Ok(v) = value.cast_exact::<PyFloat>() {
            let f = v.extract::<f64>()?;
            if f.is_nan() || f.is_infinite() {
                return py_value_err!("Cannot convert NaN or infinite float to SignedDuration");
            }
            Self::py_try_from_secs_f64(f).and_then(|dt| dt.into_bound_py_any(py))
        } else if let Ok(v) = value.cast_exact::<PyInt>() {
            let i = v.extract::<i64>()?;
            Self::from(SignedDuration::new(i, 0)).into_bound_py_any(py)
        } else if let Ok(d) = value.extract::<SignedDuration>() {
            Self::from(d).into_bound_py_any(py)
        } else {
            let valtype = any_repr!(value);
            py_type_err!("SignedDuration conversion error: {valtype}")
        }
    }
    // ========================================================================
    // PYDANTIC
    // ========================================================================

    #[cfg(feature = "pydantic")]
    #[staticmethod]
    fn _pydantic_validate<'py>(
        value: &Bound<'py, PyAny>,
        _handler: &Bound<'py, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        Self::from_any(value).map_err(map_py_value_err)
    }

    #[cfg(feature = "pydantic")]
    #[classmethod]
    fn __get_pydantic_core_schema__<'py>(
        cls: &Bound<'py, ::pyo3::types::PyType>,
        source: &Bound<'py, PyAny>,
        handler: &Bound<'py, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        use ryo3_pydantic::GetPydanticCoreSchemaCls;
        Self::get_pydantic_core_schema(cls, source, handler)
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

impl Display for RySignedDuration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "SignedDuration(secs={}, nanos={})",
            self.0.as_secs(),
            self.0.subsec_nanos()
        )
    }
}
