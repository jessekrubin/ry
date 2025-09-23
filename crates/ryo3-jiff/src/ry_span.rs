use crate::constants::SPAN_PARSER;
use crate::errors::{map_py_overflow_err, map_py_value_err};
use crate::into_span_arithmetic::IntoSpanArithmetic;
use crate::ry_signed_duration::RySignedDuration;
use crate::span_relative_to::RySpanRelativeTo;
use crate::{JiffRoundMode, JiffSpan, JiffUnit, RyDate, RyDateTime, RyZoned, timespan};
use jiff::{SignedDuration, Span, SpanArithmetic, SpanRelativeTo, SpanRound};
use pyo3::IntoPyObjectExt;
use pyo3::prelude::*;
use pyo3::types::{PyDelta, PyDict, PyFloat, PyInt, PyTuple};
use ryo3_macro_rules::{any_repr, py_overflow_error, py_type_err, py_value_error};
use std::fmt::Display;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::str::FromStr;

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[derive(Debug, Clone, Copy)]
#[pyclass(name = "TimeSpan", frozen)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
pub struct RySpan(pub(crate) Span);

impl RySpan {
    pub(crate) fn assert_non_zero(&self) -> PyResult<()> {
        if self.0.is_zero() {
            Err(py_value_error!("Span cannot be zero",))
        } else {
            Ok(())
        }
    }
}

impl PartialEq for RySpan {
    fn eq(&self, other: &Self) -> bool {
        let self_fieldwise = self.0.fieldwise();
        let other_fieldwise = other.0.fieldwise();
        self_fieldwise == other_fieldwise
    }
}

#[pymethods]
impl RySpan {
    #[expect(clippy::too_many_arguments)]
    #[new]
    #[pyo3(
        signature = (
            *,
            years=0,
            months=0,
            weeks=0,
            days=0,
            hours=0,
            minutes=0,
            seconds=0,
            milliseconds=0,
            microseconds=0,
            nanoseconds=0
        )
    )]
    fn py_new(
        years: i64,
        months: i64,
        weeks: i64,
        days: i64,
        hours: i64,
        minutes: i64,
        seconds: i64,
        milliseconds: i64,
        microseconds: i64,
        nanoseconds: i64,
    ) -> PyResult<Self> {
        timespan(
            years,
            months,
            weeks,
            days,
            hours,
            minutes,
            seconds,
            milliseconds,
            microseconds,
            nanoseconds,
        )
    }

    fn isoformat(&self) -> String {
        crate::constants::SPAN_PRINTER.span_to_string(&self.0)
    }

    #[staticmethod]
    fn from_isoformat(s: &str) -> PyResult<Self> {
        crate::constants::SPAN_PARSER
            .parse_span(s)
            .map(Self::from)
            .map_err(map_py_value_err)
    }

    fn __str__(&self) -> String {
        self.0.to_string()
    }

    fn __getnewargs_ex__<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyTuple>> {
        let args = PyTuple::empty(py).into_bound_py_any(py)?;
        let kwargs = self.to_dict(py)?.into_bound_py_any(py)?;
        PyTuple::new(py, vec![args, kwargs])
    }

    fn __format__(&self, fmt: &str) -> PyResult<String> {
        if fmt == "#" {
            Ok(format!("{:#}", self.0))
        } else if fmt.is_empty() {
            Ok(self.0.to_string())
        } else {
            py_type_err!("Invalid format specifier '{fmt}' for TimeSpan")
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

    #[pyo3(
        warn(
            message = "obj.string() is deprecated, use `obj.to_string()` or `str(obj)` [remove in 0.0.60]",
            category = pyo3::exceptions::PyDeprecationWarning
        ),
        signature = (*, friendly=false)
    )]
    fn string(&self, friendly: bool) -> String {
        if friendly {
            format!("{:#}", self.0)
        } else {
            self.0.to_string()
        }
    }

    fn friendly(&self) -> String {
        format!("{:#}", self.0)
    }

    fn __eq__(&self, other: &Self) -> bool {
        let self_fieldwise = self.0.fieldwise();
        let other_fieldwise = other.0.fieldwise();
        self_fieldwise == other_fieldwise
    }

    fn __ne__(&self, other: &Self) -> bool {
        let self_fieldwise = self.0.fieldwise();
        let other_fieldwise = other.0.fieldwise();
        self_fieldwise != other_fieldwise
    }

    fn negate(&self) -> Self {
        Self(self.0.negate())
    }

    fn __neg__(&self) -> Self {
        Self(self.0.negate())
    }

    #[inline]
    fn __abs__(&self) -> Self {
        Self(self.0.abs())
    }

    fn abs(&self) -> Self {
        self.__abs__()
    }

    fn __invert__(&self) -> Self {
        Self(self.0.negate())
    }

    #[staticmethod]
    fn from_pytimedelta(delta: Span) -> Self {
        Self(delta)
    }

    #[expect(clippy::wrong_self_convention)]
    fn to_py<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDelta>> {
        self.to_pytimedelta(py)
    }

    #[expect(clippy::wrong_self_convention)]
    fn to_pytimedelta<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDelta>> {
        let jiff_span = JiffSpan(self.0);
        jiff_span.into_pyobject(py)
    }

    #[staticmethod]
    fn parse_common_iso(s: &str) -> PyResult<Self> {
        SPAN_PARSER
            .parse_span(s)
            .map(Self::from)
            .map_err(map_py_value_err)
    }

    #[staticmethod]
    fn from_str(s: &str) -> PyResult<Self> {
        Span::from_str(s).map(Self::from).map_err(map_py_value_err)
    }

    #[staticmethod]
    fn parse(input: &str) -> PyResult<Self> {
        Self::from_str(input)
    }

    #[expect(clippy::too_many_arguments)]
    #[pyo3(signature = (years=None, months=None, weeks=None, days=None, hours=None, minutes=None, seconds=None, milliseconds=None, microseconds=None, nanoseconds=None))]
    fn replace(
        &self,
        years: Option<i64>,
        months: Option<i64>,
        weeks: Option<i64>,
        days: Option<i64>,
        hours: Option<i64>,
        minutes: Option<i64>,
        seconds: Option<i64>,
        milliseconds: Option<i64>,
        microseconds: Option<i64>,
        nanoseconds: Option<i64>,
    ) -> PyResult<Self> {
        let years = years.unwrap_or_else(|| i64::from(self.0.get_years()));
        let months = months.unwrap_or_else(|| i64::from(self.0.get_months()));
        let weeks = weeks.unwrap_or_else(|| i64::from(self.0.get_weeks()));
        let days = days.unwrap_or_else(|| i64::from(self.0.get_days()));
        let hours = hours.unwrap_or_else(|| i64::from(self.0.get_hours()));
        let minutes = minutes.unwrap_or_else(|| self.0.get_minutes());
        let seconds = seconds.unwrap_or_else(|| self.0.get_seconds());
        let milliseconds = milliseconds.unwrap_or_else(|| self.0.get_milliseconds());
        let microseconds = microseconds.unwrap_or_else(|| self.0.get_microseconds());
        let nanoseconds = nanoseconds.unwrap_or_else(|| self.0.get_nanoseconds());
        Self::py_new(
            years,
            months,
            weeks,
            days,
            hours,
            minutes,
            seconds,
            milliseconds,
            microseconds,
            nanoseconds,
        )
    }

    fn _years(&self, n: i64) -> PyResult<Self> {
        self.0
            .try_years(n)
            .map(Self::from)
            .map_err(|e| py_overflow_error!("Failed at try_years: {e}"))
    }

    fn _months(&self, n: i64) -> PyResult<Self> {
        self.0
            .try_months(n)
            .map(Self::from)
            .map_err(|e| py_overflow_error!("Failed at try_months: {e}"))
    }

    fn _weeks(&self, n: i64) -> PyResult<Self> {
        self.0
            .try_weeks(n)
            .map(Self::from)
            .map_err(|e| py_overflow_error!("Failed at try_weeks: {e}"))
    }

    fn _days(&self, n: i64) -> PyResult<Self> {
        self.0
            .try_days(n)
            .map(Self::from)
            .map_err(|e| py_overflow_error!("Failed at try_days: {e}"))
    }

    fn _hours(&self, n: i64) -> PyResult<Self> {
        self.0
            .try_hours(n)
            .map(Self::from)
            .map_err(|e| py_overflow_error!("Failed at try_hours: {e}"))
    }

    fn _minutes(&self, n: i64) -> PyResult<Self> {
        self.0
            .try_minutes(n)
            .map(Self::from)
            .map_err(|e| py_overflow_error!("Failed at try_minutes: {e}"))
    }

    fn _seconds(&self, n: i64) -> PyResult<Self> {
        self.0
            .try_seconds(n)
            .map(Self::from)
            .map_err(|e| py_overflow_error!("Failed at try_seconds: {e}"))
    }

    fn _milliseconds(&self, n: i64) -> PyResult<Self> {
        self.0
            .try_milliseconds(n)
            .map(Self::from)
            .map_err(|e| py_overflow_error!("Failed at try_milliseconds: {e}"))
    }

    fn _microseconds(&self, n: i64) -> PyResult<Self> {
        self.0
            .try_microseconds(n)
            .map(Self::from)
            .map_err(|e| py_overflow_error!("Failed at try_microseconds: {e}"))
    }

    fn _nanoseconds(&self, n: i64) -> PyResult<Self> {
        self.0
            .try_nanoseconds(n)
            .map(Self::from)
            .map_err(|e| py_overflow_error!("Failed at try_nanoseconds: {e}"))
    }

    fn __repr__(&self) -> String {
        // parts that we want are the years, months, weeks, days, hours,
        // minutes, seconds, milliseconds, microseconds, nanoseconds if not
        // zero in the form of kwargs i guess??? tbd
        format!("{self}")
    }

    fn repr_full(&self) -> String {
        format!(
            "TimeSpan(years={}, months={}, weeks={}, days={}, hours={}, minutes={}, seconds={}, milliseconds={}, microseconds={}, nanoseconds={})",
            self.0.get_years(),
            self.0.get_months(),
            self.0.get_weeks(),
            self.0.get_days(),
            self.0.get_hours(),
            self.0.get_minutes(),
            self.0.get_seconds(),
            self.0.get_milliseconds(),
            self.0.get_microseconds(),
            self.0.get_nanoseconds()
        )
    }

    fn __hash__(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.0.fieldwise().hash(&mut hasher);
        hasher.finish()
    }

    #[expect(clippy::wrong_self_convention)]
    fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        use crate::interns;
        let dict = PyDict::new(py);
        dict.set_item(interns::years(py), self.0.get_years())?;
        dict.set_item(interns::months(py), self.0.get_months())?;
        dict.set_item(interns::weeks(py), self.0.get_weeks())?;
        dict.set_item(interns::days(py), self.0.get_days())?;
        dict.set_item(interns::hours(py), self.0.get_hours())?;
        dict.set_item(interns::minutes(py), self.0.get_minutes())?;
        dict.set_item(interns::seconds(py), self.0.get_seconds())?;
        dict.set_item(interns::milliseconds(py), self.0.get_milliseconds())?;
        dict.set_item(interns::microseconds(py), self.0.get_microseconds())?;
        dict.set_item(interns::nanoseconds(py), self.0.get_nanoseconds())?;
        Ok(dict)
    }

    // TODO fix and allow relative
    fn total_seconds(&self) -> PyResult<f64> {
        self.0
            .total(jiff::SpanTotal::from(jiff::Unit::Second).days_are_24_hours())
            .map_err(map_py_value_err)
    }

    #[pyo3(signature = (relative = None))]
    #[expect(clippy::wrong_self_convention)]
    fn to_signed_duration(&self, relative: Option<RySpanRelativeTo>) -> PyResult<RySignedDuration> {
        if let Some(r) = relative {
            match r {
                RySpanRelativeTo::Zoned(z) => self
                    .0
                    .to_duration(&z.0)
                    .map(RySignedDuration)
                    .map_err(map_py_value_err),
                RySpanRelativeTo::Date(d) => self
                    .0
                    .to_duration(d.0)
                    .map(RySignedDuration)
                    .map_err(map_py_value_err),
                RySpanRelativeTo::DateTime(dt) => self
                    .0
                    .to_duration(dt.0)
                    .map(RySignedDuration)
                    .map_err(map_py_value_err),
            }
        } else {
            let now = jiff::Zoned::now();
            self.0
                .to_duration(&now)
                .map(RySignedDuration)
                .map_err(map_py_value_err)
        }
    }

    #[expect(clippy::needless_pass_by_value)]
    fn __add__(&self, other: IntoSpanArithmetic) -> PyResult<Self> {
        let span_arithmetic: SpanArithmetic = (&other).into();
        self.0
            .checked_add(span_arithmetic)
            .map(Self::from)
            .map_err(map_py_overflow_err)
    }

    #[expect(clippy::needless_pass_by_value)]
    fn add(&self, other: IntoSpanArithmetic) -> PyResult<Self> {
        let span_arithmetic: SpanArithmetic = (&other).into();

        self.0
            .checked_add(span_arithmetic)
            .map(Self::from)
            .map_err(map_py_overflow_err)
    }

    #[expect(clippy::needless_pass_by_value)]
    fn __sub__(&self, other: IntoSpanArithmetic) -> PyResult<Self> {
        let span_arithmetic: SpanArithmetic = (&other).into();
        self.0
            .checked_sub(span_arithmetic)
            .map(Self::from)
            .map_err(map_py_overflow_err)
    }

    #[expect(clippy::needless_pass_by_value)]
    fn sub(&self, other: IntoSpanArithmetic) -> PyResult<Self> {
        let span_arithmetic: SpanArithmetic = (&other).into();
        self.0
            .checked_sub(span_arithmetic)
            .map(Self::from)
            .map_err(map_py_overflow_err)
    }

    fn __mul__(&self, rhs: i64) -> PyResult<Self> {
        self.0
            .checked_mul(rhs)
            .map(Self::from)
            .map_err(map_py_overflow_err)
    }

    fn mul(&self, rhs: i64) -> PyResult<Self> {
        self.__mul__(rhs)
    }

    #[pyo3(signature = (other, relative=None, *, days_are_24_hours=None))]
    fn compare(
        &self,
        other: &Self,
        relative: Option<SpanCompareRelative>,
        days_are_24_hours: Option<bool>,
    ) -> PyResult<i8> {
        if days_are_24_hours.is_some() && relative.is_some() {
            return Err(py_value_error!(
                "Cannot provide relative with days_are_24_hours=True",
            ));
        }
        if let Some(r) = relative {
            match r {
                SpanCompareRelative::Zoned(z) => {
                    let r = self.0.compare((&other.0, &z.0)).map_err(map_py_value_err)?;
                    match r {
                        std::cmp::Ordering::Less => Ok(-1),
                        std::cmp::Ordering::Equal => Ok(0),
                        std::cmp::Ordering::Greater => Ok(1),
                    }
                }
                SpanCompareRelative::Date(d) => {
                    let r = self.0.compare((&other.0, d.0)).map_err(map_py_value_err)?;
                    match r {
                        std::cmp::Ordering::Less => Ok(-1),
                        std::cmp::Ordering::Equal => Ok(0),
                        std::cmp::Ordering::Greater => Ok(1),
                    }
                }
                SpanCompareRelative::DateTime(dt) => {
                    let r = self.0.compare((&other.0, dt.0)).map_err(map_py_value_err)?;
                    match r {
                        std::cmp::Ordering::Less => Ok(-1),
                        std::cmp::Ordering::Equal => Ok(0),
                        std::cmp::Ordering::Greater => Ok(1),
                    }
                }
            }
        } else {
            let days_are_24_hours = days_are_24_hours.unwrap_or(false);

            if days_are_24_hours {
                let span_total = SpanRelativeTo::days_are_24_hours();
                let r = self
                    .0
                    .compare((&other.0, span_total))
                    .map_err(map_py_value_err)?;
                match r {
                    std::cmp::Ordering::Less => Ok(-1),
                    std::cmp::Ordering::Equal => Ok(0),
                    std::cmp::Ordering::Greater => Ok(1),
                }
            } else {
                let r = self.0.compare(other.0).map_err(map_py_value_err)?;
                match r {
                    std::cmp::Ordering::Less => Ok(-1),
                    std::cmp::Ordering::Equal => Ok(0),
                    std::cmp::Ordering::Greater => Ok(1),
                }
            }
        }
    }
    // ========================================================================
    // PROPERTIES
    // ========================================================================
    #[getter]
    fn is_negative(&self) -> bool {
        self.0.is_negative()
    }

    #[getter]
    fn is_positive(&self) -> bool {
        self.0.is_positive()
    }

    #[getter]
    fn is_zero(&self) -> bool {
        self.0.is_zero()
    }

    #[getter]
    fn years(&self) -> i16 {
        self.0.get_years()
    }

    #[getter]
    fn months(&self) -> i32 {
        self.0.get_months()
    }

    #[getter]
    fn weeks(&self) -> i32 {
        self.0.get_weeks()
    }

    #[getter]
    fn days(&self) -> i32 {
        self.0.get_days()
    }

    #[getter]
    fn hours(&self) -> i32 {
        self.0.get_hours()
    }

    #[getter]
    fn minutes(&self) -> i64 {
        self.0.get_minutes()
    }

    #[getter]
    fn seconds(&self) -> i64 {
        self.0.get_seconds()
    }

    #[getter]
    fn milliseconds(&self) -> i64 {
        self.0.get_milliseconds()
    }

    #[getter]
    fn microseconds(&self) -> i64 {
        self.0.get_microseconds()
    }

    #[getter]
    fn nanoseconds(&self) -> i64 {
        self.0.get_nanoseconds()
    }

    // ========================================================================
    // INSTANCE METHODS
    // ========================================================================
    #[pyo3(
       signature = (smallest = None, increment = None, *, relative=None, largest = None, mode = None),
    )]
    fn round(
        &self,
        smallest: Option<JiffUnit>,
        increment: Option<i64>,
        // kwarg only
        relative: Option<SpanCompareRelative>,
        largest: Option<JiffUnit>,
        mode: Option<JiffRoundMode>,
    ) -> PyResult<Self> {
        if let Some(SpanCompareRelative::Zoned(z)) = relative {
            let mut span_round: SpanRound = SpanRound::new();
            if let Some(smallest) = smallest {
                span_round = span_round.smallest(smallest.0);
            }
            if let Some(largest) = largest {
                span_round = span_round.largest(largest.0);
            }
            if let Some(mode) = mode {
                span_round = span_round.mode(mode.0);
            }
            if let Some(increment) = increment {
                span_round = span_round.increment(increment);
            }
            span_round = span_round.relative(&z.0);
            return self
                .0
                .round(span_round)
                .map(Self::from)
                .map_err(map_py_value_err);
        }
        let mut span_round: SpanRound = SpanRound::new();
        if let Some(smallest) = smallest {
            span_round = span_round.smallest(smallest.0);
        }
        if let Some(largest) = largest {
            span_round = span_round.largest(largest.0);
        }
        if let Some(mode) = mode {
            span_round = span_round.mode(mode.0);
        }
        if let Some(increment) = increment {
            span_round = span_round.increment(increment);
        }
        if let Some(relative) = relative {
            match relative {
                SpanCompareRelative::Zoned(_z) => {
                    unreachable!("This should not happen")
                }
                SpanCompareRelative::Date(d) => {
                    span_round = span_round.relative(d.0);
                }
                SpanCompareRelative::DateTime(dt) => {
                    span_round = span_round.relative(dt.0);
                }
            }
        }

        self.0
            .round(span_round)
            .map(Self::from)
            .map_err(map_py_value_err)
    }

    fn signum(&self) -> i8 {
        self.0.signum()
    }

    #[pyo3(signature = (unit, relative=None, *, days_are_24_hours=false))]
    fn total(
        &self,
        unit: JiffUnit,
        relative: Option<SpanCompareRelative>,
        days_are_24_hours: bool,
    ) -> PyResult<f64> {
        // err on both relative and days_are_24_hours provided
        if relative.is_some() && days_are_24_hours {
            return Err(py_value_error!(
                "Cannot provide relative with days_are_24_hours=True",
            ));
        }
        if let Some(r) = relative {
            match r {
                SpanCompareRelative::Zoned(z) => {
                    Ok(self.0.total((unit.0, &z.0)).map_err(map_py_value_err)?)
                }
                SpanCompareRelative::Date(d) => {
                    Ok(self.0.total((unit.0, d.0)).map_err(map_py_value_err)?)
                }
                SpanCompareRelative::DateTime(dt) => {
                    Ok(self.0.total((unit.0, dt.0)).map_err(map_py_value_err)?)
                }
            }
        } else if days_are_24_hours {
            let span_total = SpanRelativeTo::days_are_24_hours();
            let a = self
                .0
                .total((unit.0, span_total))
                .map_err(map_py_value_err)?;
            Ok(a)
        } else {
            Ok(self.0.total(unit.0).map_err(map_py_value_err)?)
        }
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
            let sd = RySignedDuration::py_try_from_secs_f64(f)?;
            let span = jiff::Span::try_from(sd.0).map_err(map_py_overflow_err)?;
            Self::from(span).into_bound_py_any(py)
        } else if let Ok(v) = value.cast_exact::<PyInt>() {
            let i = v.extract::<i64>()?;
            let sd = SignedDuration::from_secs(i);
            Span::try_from(sd)
                .map(Self::from)
                .map_err(map_py_overflow_err)
                .and_then(|dt| dt.into_bound_py_any(py))
        } else if let Ok(d) = value.extract::<Span>() {
            Self::from(d).into_bound_py_any(py)
        } else {
            let valtype = any_repr!(value);
            py_type_err!("TimeSpan conversion error: {valtype}")
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

impl Display for RySpan {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("TimeSpan(")?;
        let mut write_sep = false;

        let years = self.0.get_years();
        if years != 0 {
            write!(f, "years={years}")?;
            write_sep = true;
        }

        let months = self.0.get_months();
        if months != 0 {
            if write_sep {
                f.write_str(", ")?;
            }
            write!(f, "months={months}")?;
            write_sep = true;
        }

        let weeks = self.0.get_weeks();
        if weeks != 0 {
            if write_sep {
                f.write_str(", ")?;
            }
            write!(f, "weeks={weeks}")?;
            write_sep = true;
        }

        let days = self.0.get_days();
        if days != 0 {
            if write_sep {
                f.write_str(", ")?;
            }
            write!(f, "days={days}")?;
            write_sep = true;
        }

        let hours = self.0.get_hours();
        if hours != 0 {
            if write_sep {
                f.write_str(", ")?;
            }
            write!(f, "hours={hours}")?;
            write_sep = true;
        }

        let minutes = self.0.get_minutes();
        if minutes != 0 {
            if write_sep {
                f.write_str(", ")?;
            }
            write!(f, "minutes={minutes}")?;
            write_sep = true;
        }

        let seconds = self.0.get_seconds();
        if seconds != 0 {
            if write_sep {
                f.write_str(", ")?;
            }
            write!(f, "seconds={seconds}")?;
            write_sep = true;
        }

        let milliseconds = self.0.get_milliseconds();
        if milliseconds != 0 {
            if write_sep {
                f.write_str(", ")?;
            }
            write!(f, "milliseconds={milliseconds}")?;
            write_sep = true;
        }

        let microseconds = self.0.get_microseconds();
        if microseconds != 0 {
            if write_sep {
                f.write_str(", ")?;
            }
            write!(f, "microseconds={microseconds}")?;
            write_sep = true;
        }

        let nanoseconds = self.0.get_nanoseconds();
        if nanoseconds != 0 {
            if write_sep {
                f.write_str(", ")?;
            }
            write!(f, "nanoseconds={nanoseconds}")?;
        }

        f.write_str(")")
    }
}

impl From<Span> for RySpan {
    fn from(span: Span) -> Self {
        Self(span)
    }
}

impl From<JiffSpan> for RySpan {
    fn from(span: JiffSpan) -> Self {
        Self(span.0)
    }
}

impl TryFrom<SignedDuration> for RySpan {
    type Error = PyErr;

    fn try_from(value: SignedDuration) -> Result<Self, Self::Error> {
        Span::try_from(value)
            .map(Self::from)
            .map_err(map_py_overflow_err)
    }
}

#[derive(Debug, Clone, FromPyObject)]
pub(crate) enum SpanCompareRelative {
    Zoned(RyZoned),
    Date(RyDate),
    DateTime(RyDateTime),
}
