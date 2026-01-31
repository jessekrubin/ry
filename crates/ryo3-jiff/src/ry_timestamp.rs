use crate::difference::{RyTimestampDifference, TimestampDifferenceArg};
use crate::round::RyTimestampRound;
use crate::ry_signed_duration::RySignedDuration;
use crate::ry_span::{RySpan, SpanKwargs, SpanKwargs2};
use crate::ry_timezone::RyTimeZone;
use crate::ry_zoned::RyZoned;
use crate::series::RyTimestampSeries;
use crate::spanish::Spanish;
use crate::{
    JiffRoundMode, JiffUnit, RyDate, RyDateTime, RyISOWeekDate, RyOffset, RyTime, timespan,
};
use jiff::tz::TimeZone;
use jiff::{Timestamp, TimestampRound, Zoned};
use pyo3::basic::CompareOp;
use pyo3::prelude::*;
use pyo3::types::PyTuple;
use pyo3::{BoundObject, IntoPyObjectExt};
use ryo3_core::{PyAsciiString, map_py_overflow_err, map_py_value_err};
use ryo3_macro_rules::{any_repr, py_type_err};
use std::hash::{DefaultHasher, Hash, Hasher};
use std::ops::Sub;

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[pyclass(name = "Timestamp", frozen, immutable_type, skip_from_py_object)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
pub struct RyTimestamp(pub(crate) Timestamp);

#[pymethods]
impl RyTimestamp {
    #[new]
    #[pyo3(signature = (second = 0, nanosecond = 0))]
    pub fn py_new(second: i64, nanosecond: i32) -> PyResult<Self> {
        Timestamp::new(second, nanosecond)
            .map(Self::from)
            .map_err(map_py_value_err)
    }

    fn __getnewargs__<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyTuple>> {
        PyTuple::new(
            py,
            vec![
                self.as_second().into_pyobject(py)?,
                self.subsec_nanosecond().into_pyobject(py)?,
            ],
        )
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn MIN() -> Self {
        Self(Timestamp::MIN)
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn MAX() -> Self {
        Self(Timestamp::MAX)
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn UNIX_EPOCH() -> Self {
        Self(Timestamp::UNIX_EPOCH)
    }

    #[staticmethod]
    fn now() -> Self {
        Self::from(Timestamp::now())
    }

    #[staticmethod]
    fn from_millisecond(millisecond: i64) -> PyResult<Self> {
        Timestamp::from_millisecond(millisecond)
            .map(Self::from)
            .map_err(map_py_value_err)
    }

    fn date(&self) -> RyDate {
        RyDate::from(self)
    }

    fn time(&self) -> RyTime {
        RyTime::from(self)
    }

    fn datetime(&self) -> RyDateTime {
        RyDateTime::from(self)
    }

    fn iso_week_date(&self) -> RyISOWeekDate {
        RyISOWeekDate::from(self)
    }

    #[expect(clippy::wrong_self_convention)]
    fn to_zoned(&self, time_zone: &RyTimeZone) -> RyZoned {
        RyZoned::from(Zoned::new(self.0, time_zone.into()))
    }

    #[expect(clippy::wrong_self_convention)]
    fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, ::pyo3::types::PyDict>> {
        use crate::interns;
        let dict = ::pyo3::types::PyDict::new(py);
        dict.set_item(interns::second(py), self.0.as_second())?;
        dict.set_item(interns::nanosecond(py), self.0.subsec_nanosecond())?;
        Ok(dict)
    }

    #[staticmethod]
    fn from_pydatetime(datetime: &Bound<'_, PyAny>) -> PyResult<Self> {
        let ts = datetime.extract::<Timestamp>()?;
        Ok(Self(ts))
    }

    #[expect(clippy::wrong_self_convention)]
    fn to_py(&self) -> Timestamp {
        self.0
    }

    #[expect(clippy::wrong_self_convention)]
    fn to_pydatetime(&self) -> Timestamp {
        self.0
    }

    #[expect(clippy::wrong_self_convention)]
    fn to_pydate(&self) -> jiff::civil::Date {
        self.0.to_zoned(TimeZone::UTC).date()
    }

    #[expect(clippy::wrong_self_convention)]
    fn to_pytime(&self) -> jiff::civil::Time {
        self.0.to_zoned(TimeZone::UTC).time()
    }

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

    #[pyo3(name = "to_string")]
    fn py_to_string(&self) -> PyAsciiString {
        self.0.to_string().into()
    }

    fn __str__(&self) -> PyAsciiString {
        self.0.to_string().into()
    }

    fn __repr__(&self) -> PyAsciiString {
        format!("{self}").into()
    }

    fn __hash__(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        // use nanosecond as hash as it is lossless
        self.0.as_nanosecond().hash(&mut hasher);
        hasher.finish()
    }

    fn __sub__<'py>(
        &self,
        py: Python<'py>,
        other: &Bound<'py, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        if let Ok(ob) = other.cast_exact::<Self>() {
            let span = self.0.sub(ob.get().0);
            let obj = RySpan::from(span).into_pyobject(py).map(Bound::into_any)?;
            Ok(obj)
        } else {
            let spanish = other.extract::<Spanish>()?;
            let z = self.0.checked_sub(spanish).map_err(map_py_overflow_err)?;
            Self::from(z).into_bound_py_any(py)
        }
    }

    fn __add__(&self, other: Spanish) -> PyResult<Self> {
        self.0
            .checked_add(other)
            .map(Self::from)
            .map_err(map_py_overflow_err)
    }

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
    fn add(
        &self,
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
        let span = timespan(
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
        )?;
        self.0
            .checked_add(span.0)
            .map(Self::from)
            .map_err(map_py_overflow_err)
    }

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
    fn sub(
        &self,
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
        let span = timespan(
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
        )?;
        self.0
            .checked_sub(span.0)
            .map(Self::from)
            .map_err(map_py_overflow_err)
    }

    fn as_second(&self) -> i64 {
        self.0.as_second()
    }

    fn as_microsecond(&self) -> i64 {
        self.0.as_microsecond()
    }

    fn as_millisecond(&self) -> i64 {
        self.0.as_millisecond()
    }

    fn as_nanosecond(&self) -> i128 {
        self.0.as_nanosecond()
    }

    #[getter]
    fn second(&self) -> i64 {
        self.0.as_second()
    }

    #[getter]
    fn nanosecond(&self) -> i32 {
        self.0.subsec_nanosecond()
    }

    #[getter]
    fn subsec_nanosecond(&self) -> i32 {
        self.0.subsec_nanosecond()
    }

    #[getter]
    fn subsec_microsecond(&self) -> i32 {
        self.0.subsec_microsecond()
    }

    #[getter]
    fn subsec_millisecond(&self) -> i32 {
        self.0.subsec_millisecond()
    }

    fn series(&self, period: &RySpan) -> PyResult<RyTimestampSeries> {
        (self, period).try_into()
    }

    #[getter]
    fn is_zero(&self) -> bool {
        self.0.is_zero()
    }

    fn in_tz(&self, tz: &str) -> PyResult<RyZoned> {
        self.0
            .in_tz(tz)
            .map(RyZoned::from)
            .map_err(map_py_value_err)
    }

    #[pyo3(
        warn(
            message = "`intz` is deprecated, use `in_tz` instead",
            category = pyo3::exceptions::PyDeprecationWarning
        )
    )]
    fn intz(&self, tz: &str) -> PyResult<RyZoned> {
        self.in_tz(tz)
    }

    #[staticmethod]
    fn from_microsecond(microsecond: i64) -> PyResult<Self> {
        Timestamp::from_microsecond(microsecond)
            .map(Self::from)
            .map_err(map_py_value_err)
    }

    #[staticmethod]
    fn from_nanosecond(nanosecond: i128) -> PyResult<Self> {
        Timestamp::from_nanosecond(nanosecond)
            .map(Self::from)
            .map_err(map_py_value_err)
    }

    #[staticmethod]
    fn from_second(second: i64) -> PyResult<Self> {
        Timestamp::from_second(second)
            .map(Self::from)
            .map_err(map_py_value_err)
    }

    fn signum(&self) -> i8 {
        self.0.signum()
    }

    // ========================================================================
    // STRPTIME/STRFTIME
    // ========================================================================
    fn __format__(&self, fmt: &str) -> PyResult<String> {
        if fmt.is_empty() {
            Ok(self.0.to_string())
        } else {
            self.strftime(fmt)
        }
    }

    fn strftime(&self, fmt: &str) -> PyResult<String> {
        let bdt: jiff::fmt::strtime::BrokenDownTime = self.0.into();
        bdt.to_string(fmt).map_err(map_py_value_err)
    }

    #[staticmethod]
    #[pyo3(signature = (s, /, fmt))]
    fn strptime(s: &str, fmt: &str) -> PyResult<Self> {
        Timestamp::strptime(fmt, s)
            .map(Self::from)
            .map_err(map_py_value_err)
    }

    #[pyo3(
        signature = (ts, *, smallest=JiffUnit::NANOSECOND, largest=None, mode=JiffRoundMode::TRUNC, increment=1),
        text_signature = "(self, other, *, smallest=\"nanosecond\", largest=None, mode=\"trunc\", increment=1)"
    )]
    fn since(
        &self,
        ts: TimestampDifferenceArg,
        smallest: JiffUnit,
        largest: Option<JiffUnit>,
        mode: JiffRoundMode,
        increment: i64,
    ) -> PyResult<RySpan> {
        let dt_diff = ts.build(smallest, largest, mode, increment);
        self.0
            .since(dt_diff)
            .map(RySpan::from)
            .map_err(map_py_value_err)
    }

    #[pyo3(
        signature = (ts, *, smallest=JiffUnit::NANOSECOND, largest=None, mode=JiffRoundMode::TRUNC, increment=1),
        text_signature = "(self, other, *, smallest=\"nanosecond\", largest=None, mode=\"trunc\", increment=1)"
    )]
    fn until(
        &self,
        ts: TimestampDifferenceArg,
        smallest: JiffUnit,
        largest: Option<JiffUnit>,
        mode: JiffRoundMode,
        increment: i64,
    ) -> PyResult<RySpan> {
        let dt_diff = ts.build(smallest, largest, mode, increment);
        self.0
            .until(dt_diff)
            .map(RySpan::from)
            .map_err(map_py_value_err)
    }

    fn _since(&self, other: &RyTimestampDifference) -> PyResult<RySpan> {
        self.0
            .since(other.diff)
            .map(RySpan::from)
            .map_err(map_py_value_err)
    }

    fn _until(&self, other: &RyTimestampDifference) -> PyResult<RySpan> {
        self.0
            .until(other.diff)
            .map(RySpan::from)
            .map_err(map_py_value_err)
    }

    fn display_with_offset(&self, offset: &RyOffset) -> PyAsciiString {
        let dwo = self.0.display_with_offset(offset.0);
        dwo.to_string().into()
    }

    fn duration_since(&self, other: &Self) -> RySignedDuration {
        RySignedDuration::from(self.0.duration_since(other.0))
    }

    fn duration_until(&self, other: &Self) -> RySignedDuration {
        RySignedDuration::from(self.0.duration_until(other.0))
    }

    #[pyo3(
        signature = (
            smallest=JiffUnit::NANOSECOND,
            *,
            mode=JiffRoundMode::HALF_EXPAND,
            increment=1
        ),
        text_signature = "(self, smallest=\"nanosecond\", *, mode=\"half-expand\", increment=1)"
    )]
    fn round(&self, smallest: JiffUnit, mode: JiffRoundMode, increment: i64) -> PyResult<Self> {
        let ts_round = TimestampRound::new()
            .smallest(smallest.0)
            .increment(increment)
            .mode(mode.0);
        self.0
            .round(ts_round)
            .map(Self::from)
            .map_err(map_py_value_err)
    }

    fn _round(&self, options: &RyTimestampRound) -> PyResult<Self> {
        options.round(self)
    }

    fn saturating_add(&self, other: Spanish) -> PyResult<Self> {
        self.0
            .saturating_add(other)
            .map(Self::from)
            .map_err(map_py_value_err)
    }

    fn saturating_sub(&self, other: Spanish) -> PyResult<Self> {
        self.0
            .saturating_sub(other)
            .map(Self::from)
            .map_err(map_py_value_err)
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
        } else if let Ok(d) = value.cast_exact::<RyZoned>() {
            Self::from(d.get()).into_pyobject(py)
        } else if let Ok(dt) = value.cast_exact::<RyDateTime>() {
            let zdt = dt.get().0.to_zoned(TimeZone::UTC)?;
            let ts = zdt.timestamp();
            Self::from(ts).into_pyobject(py)
        } else if let Ok(ts) = value.extract::<Timestamp>() {
            Self::from(ts).into_pyobject(py)
        } else {
            let valtype = any_repr!(value);
            py_type_err!("Timestamp conversion error: {valtype}")
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

    // ========================================================================
    // STANDARD METHODS
    // ========================================================================
    // <STD-METHODS>
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
    // </STD-METHODS>
}

impl std::fmt::Display for RyTimestamp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Timestamp(second={:?}, nanosecond={:?})",
            self.0.as_second(),
            self.0.subsec_nanosecond()
        )
    }
}
