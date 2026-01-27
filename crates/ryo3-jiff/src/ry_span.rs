use crate::constants::SPAN_PARSER;
use crate::py_temporal_like::PyTermporalTypes;
use crate::ry_signed_duration::RySignedDuration;
use crate::spanish::Spanish;
use crate::{
    JiffRoundMode, JiffSpan, JiffUnit, RyDate, RyDateTime, RyTime, RyTimestamp, RyZoned, timespan,
};
use jiff::{SignedDuration, Span, SpanArithmetic, SpanRelativeTo, SpanRound};
use pyo3::prelude::*;
use pyo3::types::{PyDelta, PyDict, PyFloat, PyInt, PyTuple};
use pyo3::{BoundObject, IntoPyObjectExt};
use ryo3_core::{PyAsciiString, map_py_overflow_err, map_py_value_err};
use ryo3_macro_rules::{any_repr, py_overflow_error, py_type_err, py_value_error};
use std::fmt::Display;
use std::hash::{DefaultHasher, Hash, Hasher};

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[derive(Debug, Clone, Copy)]
#[pyclass(name = "TimeSpan", frozen, immutable_type, skip_from_py_object)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
pub struct RySpan(pub(crate) Span);

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

    #[staticmethod]
    fn from_isoformat(s: &str) -> PyResult<Self> {
        crate::constants::SPAN_PARSER
            .parse_span(s)
            .map(Self::from)
            .map_err(map_py_value_err)
    }

    fn __str__(&self) -> PyAsciiString {
        self.0.to_string().into()
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

    // <UNIFORM>
    #[staticmethod]
    fn from_str(s: &str) -> PyResult<Self> {
        use ryo3_core::PyFromStr;
        Self::py_from_str(s)
    }

    #[staticmethod]
    fn parse(s: &Bound<'_, PyAny>) -> PyResult<Self> {
        use ryo3_core::PyParse;
        Self::py_parse(s)
    }

    fn isoformat(&self) -> PyAsciiString {
        <Self as crate::isoformat::PyIsoFormat>::isoformat(self)
    }
    // </UNIFORM>

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

    #[pyo3(signature = (years, /))]
    fn _years(&self, years: i64) -> PyResult<Self> {
        self.0
            .try_years(years)
            .map(Self::from)
            .map_err(|e| py_overflow_error!("Failed at try_years: {e}"))
    }

    #[pyo3(signature = (months, /))]
    fn _months(&self, months: i64) -> PyResult<Self> {
        self.0
            .try_months(months)
            .map(Self::from)
            .map_err(|e| py_overflow_error!("Failed at try_months: {e}"))
    }

    #[pyo3(signature = (weeks, /))]
    fn _weeks(&self, weeks: i64) -> PyResult<Self> {
        self.0
            .try_weeks(weeks)
            .map(Self::from)
            .map_err(|e| py_overflow_error!("Failed at try_weeks: {e}"))
    }

    #[pyo3(signature = (days, /))]
    fn _days(&self, days: i64) -> PyResult<Self> {
        self.0
            .try_days(days)
            .map(Self::from)
            .map_err(|e| py_overflow_error!("Failed at try_days: {e}"))
    }

    #[pyo3(signature = (hours, /))]
    fn _hours(&self, hours: i64) -> PyResult<Self> {
        self.0
            .try_hours(hours)
            .map(Self::from)
            .map_err(|e| py_overflow_error!("Failed at try_hours: {e}"))
    }

    #[pyo3(signature = (minutes, /))]
    fn _minutes(&self, minutes: i64) -> PyResult<Self> {
        self.0
            .try_minutes(minutes)
            .map(Self::from)
            .map_err(|e| py_overflow_error!("Failed at try_minutes: {e}"))
    }

    #[pyo3(signature = (seconds, /))]
    fn _seconds(&self, seconds: i64) -> PyResult<Self> {
        self.0
            .try_seconds(seconds)
            .map(Self::from)
            .map_err(|e| py_overflow_error!("Failed at try_seconds: {e}"))
    }

    #[pyo3(signature = (milliseconds, /))]
    fn _milliseconds(&self, milliseconds: i64) -> PyResult<Self> {
        self.0
            .try_milliseconds(milliseconds)
            .map(Self::from)
            .map_err(|e| py_overflow_error!("Failed at try_milliseconds: {e}"))
    }

    #[pyo3(signature = (microseconds, /))]
    fn _microseconds(&self, microseconds: i64) -> PyResult<Self> {
        self.0
            .try_microseconds(microseconds)
            .map(Self::from)
            .map_err(|e| py_overflow_error!("Failed at try_microseconds: {e}"))
    }

    #[pyo3(signature = (nanoseconds, /))]
    fn _nanoseconds(&self, nanoseconds: i64) -> PyResult<Self> {
        self.0
            .try_nanoseconds(nanoseconds)
            .map(Self::from)
            .map_err(|e| py_overflow_error!("Failed at try_nanoseconds: {e}"))
    }

    fn __repr__(&self) -> PyAsciiString {
        format!("{self}").into()
    }

    fn repr_full(&self) -> PyAsciiString {
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
        ) .into()
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

    fn fieldwise<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        self.to_dict(py)
    }

    #[pyo3(signature = (relative = None))]
    #[expect(clippy::wrong_self_convention)]
    fn to_signed_duration(&self, relative: Option<RySpanRelativeTo>) -> PyResult<RySignedDuration> {
        if let Some(r) = relative {
            self.0
                .to_duration(&r)
                .map(RySignedDuration)
                .map_err(map_py_value_err)
        } else {
            let now = jiff::Zoned::now();
            self.0
                .to_duration(&now)
                .map(RySignedDuration)
                .map_err(map_py_value_err)
        }
    }

    #[expect(clippy::needless_pass_by_value)]
    fn __add__<'py>(
        &self,
        py: Python<'py>,
        other: SpanAddTarget<'_, 'py>,
    ) -> PyResult<Bound<'py, PyAny>> {
        other.add_span(py, self)
    }

    #[expect(clippy::needless_pass_by_value)]
    fn add<'py>(
        &self,
        py: Python<'py>,
        other: SpanAddTarget<'_, 'py>,
    ) -> PyResult<Bound<'py, PyAny>> {
        other.add_span(py, self)
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

    fn __mul__(&self, other: i64) -> PyResult<Self> {
        self.0
            .checked_mul(other)
            .map(Self::from)
            .map_err(map_py_overflow_err)
    }

    fn mul(&self, other: i64) -> PyResult<Self> {
        self.__mul__(other)
    }

    #[pyo3(signature = (other, relative=None, *, days_are_24_hours=false))]
    fn compare(
        &self,
        other: &Self,
        relative: Option<RySpanRelativeTo>,
        days_are_24_hours: bool,
    ) -> PyResult<i8> {
        if days_are_24_hours && relative.is_some() {
            return Err(py_value_error!(
                "Cannot provide relative with days_are_24_hours=True",
            ));
        }
        if let Some(r) = relative {
            let relative_to: jiff::SpanRelativeTo = (&r).into();
            let r = self
                .0
                .compare((&other.0, relative_to))
                .map_err(map_py_value_err)?;
            match r {
                std::cmp::Ordering::Less => Ok(-1),
                std::cmp::Ordering::Equal => Ok(0),
                std::cmp::Ordering::Greater => Ok(1),
            }
        } else if days_are_24_hours {
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
        signature = (
            smallest=JiffUnit::NANOSECOND,
            increment=1,
            *,
            relative=None,
            largest=None,
            mode=JiffRoundMode::HALF_EXPAND,
            days_are_24_hours=false
        ),
        text_signature = "(self, smallest=\"nanosecond\", increment=1, *, relative=None, largest=None, mode=\"half-expand\", days_are_24_hours=False)"
    )]
    fn round(
        &self,
        smallest: JiffUnit,
        increment: i64,
        // kwarg only
        relative: Option<RySpanRelativeTo>,
        largest: Option<JiffUnit>,
        mode: JiffRoundMode,
        days_are_24_hours: bool,
    ) -> PyResult<Self> {
        // err on both relative and days_are_24_hours provided
        if relative.is_some() && days_are_24_hours {
            return Err(py_value_error!(
                "`relative` and `days_are_24_hours=True` are mutually exclusive",
            ));
        }
        if let Some(relative) = relative {
            let mut span_round: SpanRound = SpanRound::new()
                .increment(increment)
                .smallest(smallest.0)
                .mode(mode.0);
            if let Some(largest) = largest {
                span_round = span_round.largest(largest.0);
            }
            let rel = jiff::SpanRelativeTo::from(&relative);
            span_round = span_round.relative(rel);
            self.0
                .round(span_round)
                .map(Self::from)
                .map_err(map_py_value_err)
        } else if days_are_24_hours {
            let mut span_round: SpanRound = SpanRound::new()
                .increment(increment)
                .smallest(smallest.0)
                .mode(mode.0)
                .relative(SpanRelativeTo::days_are_24_hours());
            if let Some(largest) = largest {
                span_round = span_round.largest(largest.0);
            }
            self.0
                .round(span_round)
                .map(Self::from)
                .map_err(map_py_value_err)
        } else {
            let mut span_round: SpanRound = SpanRound::new()
                .increment(increment)
                .smallest(smallest.0)
                .mode(mode.0);
            if let Some(largest) = largest {
                span_round = span_round.largest(largest.0);
            }
            self.0
                .round(span_round)
                .map(Self::from)
                .map_err(map_py_value_err)
        }
    }

    fn signum(&self) -> i8 {
        self.0.signum()
    }

    #[pyo3(signature = (relative=None, *, days_are_24_hours=false))]
    fn total_seconds(
        &self,
        relative: Option<RySpanRelativeTo>,
        days_are_24_hours: bool,
    ) -> PyResult<f64> {
        self.total(JiffUnit::SECOND, relative, days_are_24_hours)
    }

    #[pyo3(signature = (unit, relative=None, *, days_are_24_hours=false))]
    fn total(
        &self,
        unit: JiffUnit,
        relative: Option<RySpanRelativeTo>,
        days_are_24_hours: bool,
    ) -> PyResult<f64> {
        // err on both relative and days_are_24_hours provided
        if relative.is_some() && days_are_24_hours {
            return Err(py_value_error!(
                "Cannot provide relative with days_are_24_hours=True",
            ));
        }

        if let Some(r) = relative {
            let relative_to: jiff::SpanRelativeTo = (&r).into();
            let r = self
                .0
                .total((unit.0, relative_to))
                .map_err(map_py_value_err)?;
            Ok(r)
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
    fn from_any<'py>(value: &Bound<'py, PyAny>) -> PyResult<Bound<'py, Self>> {
        let py = value.py();
        if let Ok(val) = value.cast_exact::<Self>() {
            Ok(val.as_borrowed().into_bound())
        } else if let Ok(pystr) = value.cast::<pyo3::types::PyString>() {
            let s = pystr.extract::<&str>()?;
            Self::from_str(s).map(|dt| dt.into_pyobject(py))?
        } else if let Ok(pybytes) = value.cast::<pyo3::types::PyBytes>() {
            let s = String::from_utf8_lossy(pybytes.as_bytes());
            Self::from_str(&s).map(|dt| dt.into_pyobject(py))?
        } else if let Ok(v) = value.cast_exact::<PyFloat>() {
            let f = v.extract::<f64>()?;
            let sd = RySignedDuration::py_try_from_secs_f64(f)?;
            let span = jiff::Span::try_from(sd.0).map_err(map_py_overflow_err)?;
            Self::from(span).into_pyobject(py)
        } else if let Ok(v) = value.cast_exact::<PyInt>() {
            let i = v.extract::<i64>()?;
            let sd = SignedDuration::from_secs(i);
            Span::try_from(sd)
                .map(Self::from)
                .map_err(map_py_overflow_err)
                .and_then(|dt| dt.into_pyobject(py))
        } else if let Ok(d) = value.extract::<Span>() {
            Self::from(d).into_pyobject(py)
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
    ) -> PyResult<Bound<'py, Self>> {
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
        // parts that we want are the years, months, weeks, days, hours,
        // minutes, seconds, milliseconds, microseconds, nanoseconds if not
        // zero in the form of kwargs i guess??? tbd
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

#[derive(Debug, Clone)]
pub(crate) enum RySpanRelativeTo<'a, 'py> {
    Zoned(Borrowed<'a, 'py, RyZoned>),
    Date(Borrowed<'a, 'py, RyDate>),
    DateTime(Borrowed<'a, 'py, RyDateTime>),
}

impl<'a, 'py> FromPyObject<'a, 'py> for RySpanRelativeTo<'a, 'py> {
    type Error = PyErr;

    fn extract(obj: Borrowed<'a, 'py, PyAny>) -> Result<Self, Self::Error> {
        if let Ok(z) = obj.cast_exact::<RyZoned>() {
            Ok(Self::Zoned(z))
        } else if let Ok(d) = obj.cast_exact::<RyDate>() {
            Ok(Self::Date(d))
        } else if let Ok(dt) = obj.cast_exact::<RyDateTime>() {
            Ok(Self::DateTime(dt))
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                "Expected ZonedDateTime, DateTime, or Date",
            ))
        }
    }
}

impl<'a, 'py> From<&'a RySpanRelativeTo<'a, 'py>> for jiff::SpanRelativeTo<'a> {
    fn from(val: &'a RySpanRelativeTo<'a, 'py>) -> Self {
        match val {
            RySpanRelativeTo::Zoned(z) => jiff::SpanRelativeTo::from(&z.get().0),
            RySpanRelativeTo::Date(d) => jiff::SpanRelativeTo::from(d.get().0),
            RySpanRelativeTo::DateTime(dt) => jiff::SpanRelativeTo::from(dt.get().0),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) enum IntoSpanArithmetic<'a, 'py> {
    Uno(Spanish<'a, 'py>),
    Dos((Spanish<'a, 'py>, RySpanRelativeTo<'a, 'py>)),
}

impl<'a, 'py> FromPyObject<'a, 'py> for IntoSpanArithmetic<'a, 'py> {
    type Error = PyErr;

    fn extract(obj: Borrowed<'a, 'py, PyAny>) -> Result<Self, Self::Error> {
        if let Ok(tup) = obj.cast_exact::<PyTuple>() {
            if tup.len() == 2 {
                let both = tup.extract::<(Spanish<'a, 'py>, RySpanRelativeTo<'a, 'py>)>()?;
                Ok(IntoSpanArithmetic::Dos(both))
            } else {
                py_type_err!("Expected a tuple of length 2 for Span arithmetic with relative")
            }
        } else if let Ok(spanish) = obj.extract::<Spanish<'a, 'py>>() {
            Ok(IntoSpanArithmetic::Uno(spanish))
        } else {
            py_type_err!(
                "Expected TimeSpan, SignedDuration, Duration, datetime.timedelta, or a tuple of length 2 for Span arithmetic with relative",
            )
        }
    }
}

impl<'a, 'py> From<&'a IntoSpanArithmetic<'a, 'py>> for SpanArithmetic<'a> {
    fn from(value: &'a IntoSpanArithmetic<'a, 'py>) -> Self {
        // HERE WE HAVE A TOTAL CLUSTER-FUCK OF MATCHING...
        // BUT I AM NOT SURE HOW TO GET THIS TO PLAY NICE WITH PYTHON + LIFETIMES
        // -- update --
        // SO this is A BIT LESS of a clusterfuck but still pretty cluster-fucky
        match value {
            IntoSpanArithmetic::Uno(s) => match s {
                Spanish::Span(sp) => SpanArithmetic::from(sp.get().0).days_are_24_hours(),
                Spanish::Duration(dur) => SpanArithmetic::from(dur.get().0).days_are_24_hours(),
                Spanish::SignedDuration(dur) => {
                    SpanArithmetic::from(dur.get().0).days_are_24_hours()
                }
                // delta
                Spanish::PyTimeDelta(sd) => SpanArithmetic::from(*sd).days_are_24_hours(),
            },
            IntoSpanArithmetic::Dos((s, r)) => match s {
                Spanish::Span(sp) => match r {
                    RySpanRelativeTo::Zoned(z) => SpanArithmetic::from((sp.get().0, &z.get().0)),
                    RySpanRelativeTo::Date(d) => SpanArithmetic::from((sp.get().0, d.get().0)),
                    RySpanRelativeTo::DateTime(dt) => {
                        SpanArithmetic::from((sp.get().0, dt.get().0))
                    }
                },
                Spanish::Duration(dur) => match r {
                    RySpanRelativeTo::Zoned(z) => SpanArithmetic::from((dur.get().0, &z.get().0)),
                    RySpanRelativeTo::Date(d) => SpanArithmetic::from((dur.get().0, d.get().0)),
                    RySpanRelativeTo::DateTime(dt) => {
                        SpanArithmetic::from((dur.get().0, dt.get().0))
                    }
                },
                Spanish::SignedDuration(dur) => match r {
                    RySpanRelativeTo::Zoned(z) => SpanArithmetic::from((dur.get().0, &z.get().0)),
                    RySpanRelativeTo::Date(d) => SpanArithmetic::from((dur.get().0, d.get().0)),
                    RySpanRelativeTo::DateTime(dt) => {
                        SpanArithmetic::from((dur.get().0, dt.get().0))
                    }
                },
                // delta
                Spanish::PyTimeDelta(sd) => match r {
                    RySpanRelativeTo::Zoned(z) => SpanArithmetic::from((*sd, &z.get().0)),
                    RySpanRelativeTo::Date(d) => SpanArithmetic::from((*sd, d.get().0)),
                    RySpanRelativeTo::DateTime(dt) => SpanArithmetic::from((*sd, dt.get().0)),
                },
            },
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) enum SpanAddTarget<'a, 'py> {
    Span(IntoSpanArithmetic<'a, 'py>),
    TemporalType(PyTermporalTypes<'a, 'py>),
}

impl<'a, 'py> FromPyObject<'a, 'py> for SpanAddTarget<'a, 'py> {
    type Error = PyErr;

    fn extract(obj: Borrowed<'a, 'py, PyAny>) -> Result<Self, Self::Error> {
        if let Ok(temporal_ish) = obj.extract::<PyTermporalTypes<'a, 'py>>() {
            Ok(Self::TemporalType(temporal_ish))
        } else if let Ok(span_arith) = obj.extract::<IntoSpanArithmetic<'a, 'py>>() {
            Ok(Self::Span(span_arith))
        } else {
            py_type_err!(
                "Expected TimeSpan, SignedDuration, datetime.timedelta, a tuple of length 2 for Span arithmetic with relative, or a date/time type",
            )
        }
    }
}

trait SpanAdd<'a, 'py> {
    type Target;
    type Output;
    fn add_span(&self, py: Python<'py>, span: &RySpan) -> PyResult<Self::Output>;
}

macro_rules! impl_span_add_for_borrowed {
    ($ty:ty) => {
        impl<'a, 'py> SpanAdd<'a, 'py> for Borrowed<'a, 'py, $ty> {
            type Target = $ty;
            type Output = Bound<'py, Self::Target>;
            fn add_span(&self, py: Python<'py>, span: &RySpan) -> PyResult<Self::Output> {
                self.get()
                    .0
                    .checked_add(span.0)
                    .map(Self::Target::from)
                    .map_err(map_py_overflow_err)
                    .map(|r| r.into_pyobject(py))?
            }
        }
    };
}

impl_span_add_for_borrowed!(RyDate);
impl_span_add_for_borrowed!(RyDateTime);
impl_span_add_for_borrowed!(RyTime);
impl_span_add_for_borrowed!(RyZoned);
impl_span_add_for_borrowed!(RyTimestamp);

impl<'a, 'py> SpanAdd<'a, 'py> for PyTermporalTypes<'a, 'py> {
    type Target = PyAny;
    type Output = Bound<'py, PyAny>;
    fn add_span(&self, py: Python<'py>, span: &RySpan) -> PyResult<Self::Output> {
        match self {
            Self::Date(date) => date.add_span(py, span).map(Bound::into_any),
            Self::DateTime(datetime) => datetime.add_span(py, span).map(Bound::into_any),
            Self::Time(time) => time.add_span(py, span).map(Bound::into_any),
            Self::Zoned(zoned) => zoned.add_span(py, span).map(Bound::into_any),
            Self::Timestamp(timestamp) => timestamp.add_span(py, span).map(Bound::into_any),
        }
    }
}

impl<'a, 'py> SpanAdd<'a, 'py> for IntoSpanArithmetic<'a, 'py> {
    type Target = RySpan;
    type Output = Bound<'py, Self::Target>;
    fn add_span(&self, py: Python<'py>, span: &RySpan) -> PyResult<Self::Output> {
        let span_arithmetic: SpanArithmetic = self.into();
        span.0
            .checked_add(span_arithmetic)
            .map(RySpan::from)
            .map_err(map_py_overflow_err)?
            .into_pyobject(py)
    }
}

impl<'a, 'py> SpanAdd<'a, 'py> for SpanAddTarget<'a, 'py> {
    type Target = PyAny;
    type Output = Bound<'py, PyAny>;

    fn add_span(&self, py: Python<'py>, span: &RySpan) -> PyResult<Self::Output> {
        match self {
            Self::Span(span_arithmetic) => {
                let span_arithmetic: SpanArithmetic = span_arithmetic.into();
                span.0
                    .checked_add(span_arithmetic)
                    .map(RySpan::from)
                    .map_err(map_py_overflow_err)?
                    .into_pyobject(py)
                    .map(Bound::into_any)
            }
            Self::TemporalType(temporal_type) => temporal_type.add_span(py, span),
        }
    }
}
