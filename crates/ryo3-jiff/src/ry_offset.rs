use crate::round::RyOffsetRound;
use crate::ry_datetime::RyDateTime;
use crate::ry_signed_duration::RySignedDuration;
use crate::ry_span::RySpan;
use crate::ry_timestamp::RyTimestamp;
use crate::ry_timezone::RyTimeZone;
use crate::spanish::Spanish;
use crate::util::SpanKwargs;
use crate::{JiffOffset, JiffRoundMode, JiffSignedDuration, JiffUnit};
use jiff::SignedDuration;
use jiff::tz::{Offset, OffsetRound};
use pyo3::BoundObject;
use pyo3::prelude::*;
use pyo3::pyclass::CompareOp;
use pyo3::types::{PyDict, PyTuple};
use ryo3_core::PyAsciiString;
use ryo3_core::map_py_overflow_err;
use ryo3_core::map_py_value_err;
use ryo3_macro_rules::{any_repr, py_type_err};
use std::hash::{DefaultHasher, Hash, Hasher};
use std::vec;

#[pyclass(name = "Offset", frozen, immutable_type, skip_from_py_object)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
#[derive(Clone, Copy, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct RyOffset(pub(crate) Offset);

impl RyOffset {
    #[expect(clippy::arithmetic_side_effects)]
    fn py_from_hms(hours: i8, minutes: i16, seconds: i32) -> PyResult<Self> {
        let total_seconds = (i32::from(hours) * 3600) + (i32::from(minutes) * 60) + seconds;
        Offset::from_seconds(total_seconds)
            .map(Self::from)
            .map_err(map_py_value_err)
    }

    #[expect(clippy::cast_possible_truncation)]
    fn hms(&self) -> (i8, i16, i32) {
        let total_seconds = self.0.seconds();
        let hours = total_seconds / 3600;
        let minutes = (total_seconds % 3600) / 60;
        let seconds = total_seconds % 60;
        (hours as i8, minutes as i16, seconds)
    }
}

#[expect(clippy::wrong_self_convention)]
#[pymethods]
impl RyOffset {
    #[new]
    #[pyo3(signature = (hours = 0, minutes = 0, seconds = 0))]
    fn py_new(hours: i8, minutes: i16, seconds: i32) -> PyResult<Self> {
        Self::py_from_hms(hours, minutes, seconds)
    }

    fn __getnewargs__<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyTuple>> {
        let (hours, minutes, seconds) = self.hms();
        PyTuple::new(
            py,
            vec![
                hours.into_pyobject(py)?,
                minutes.into_pyobject(py)?,
                seconds.into_pyobject(py)?,
            ],
        )
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn MIN() -> Self {
        Self::from(Offset::MIN)
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn MAX() -> Self {
        Self::from(Offset::MAX)
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn ZERO() -> Self {
        Self::from(Offset::ZERO)
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn UTC() -> Self {
        Self::from(Offset::UTC)
    }

    #[staticmethod]
    fn utc() -> Self {
        Self::from(Offset::UTC)
    }

    #[staticmethod]
    fn from_hours(hours: i8) -> PyResult<Self> {
        Offset::from_hours(hours)
            .map(Self::from)
            .map_err(map_py_value_err)
    }

    #[staticmethod]
    fn from_seconds(seconds: i32) -> PyResult<Self> {
        Offset::from_seconds(seconds)
            .map(Self::from)
            .map_err(map_py_value_err)
    }

    fn to_py(&self) -> JiffSignedDuration {
        self.to_pytimedelta()
    }

    fn to_pytzinfo(&self) -> &Offset {
        &self.0
    }

    fn to_pytimedelta(&self) -> JiffSignedDuration {
        SignedDuration::from_secs(self.0.seconds().into()).into()
    }

    #[expect(clippy::wrong_self_convention)]
    pub(crate) fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        let dict = PyDict::new(py);
        dict.set_item(crate::interns::seconds(py), self.seconds())?;
        dict.set_item(crate::interns::fmt(py), self.py_to_string())?;
        Ok(dict)
    }

    #[staticmethod]
    fn from_pytimedelta(delta: JiffSignedDuration) -> PyResult<Self> {
        Offset::try_from(delta.0)
            .map(Self::from)
            .map_err(map_py_value_err)
    }

    #[staticmethod]
    fn from_pytzinfo(tz: JiffOffset) -> Self {
        Self::from(tz.0)
    }

    #[pyo3(name = "to_string")]
    #[must_use]
    fn py_to_string(&self) -> PyAsciiString {
        self.__str__()
    }

    #[must_use]
    fn __str__(&self) -> PyAsciiString {
        self.0.to_string().into()
    }

    #[must_use]
    fn __repr__(&self) -> PyAsciiString {
        format!("{self}").into()
    }

    #[getter]
    #[must_use]
    pub(crate) fn seconds(&self) -> i32 {
        self.0.seconds()
    }

    #[must_use]
    fn negate(&self) -> Self {
        Self::from(self.0.negate())
    }

    fn __neg__(&self) -> Self {
        self.negate()
    }

    #[getter]
    #[must_use]
    fn is_negative(&self) -> bool {
        self.0.is_negative()
    }

    #[getter]
    #[must_use]
    fn is_positive(&self) -> bool {
        !self.0.is_negative()
    }

    #[must_use]
    fn to_datetime(&self, timestamp: &RyTimestamp) -> RyDateTime {
        RyDateTime::from(self.0.to_datetime(timestamp.0))
    }

    fn to_timestamp(&self, datetime: &RyDateTime) -> PyResult<RyTimestamp> {
        self.0
            .to_timestamp(datetime.0)
            .map(RyTimestamp::from)
            .map_err(map_py_value_err)
    }

    #[must_use]
    fn until(&self, other: &Self) -> RySpan {
        let s = self.0.until(other.0);
        RySpan::from(s)
    }

    #[must_use]
    fn since(&self, other: &Self) -> RySpan {
        let s = self.0.since(other.0);
        RySpan::from(s)
    }

    #[must_use]
    fn duration_until(&self, other: &Self) -> RySignedDuration {
        let s = self.0.duration_until(other.0);
        RySignedDuration::from(s)
    }

    #[must_use]
    fn duration_since(&self, other: &Self) -> RySignedDuration {
        let s = self.0.duration_since(other.0);
        RySignedDuration::from(s)
    }

    #[pyo3(
        signature=(
            smallest=JiffUnit::SECOND,
            *,
            mode=JiffRoundMode::HALF_EXPAND,
            increment=1
        ),
        text_signature = "($self, smallest=\"second\", *, mode=\"half-expand\", increment=1)"
    )]
    fn round(&self, smallest: JiffUnit, mode: JiffRoundMode, increment: i64) -> PyResult<Self> {
        let round_ob = OffsetRound::new()
            .increment(increment)
            .mode(mode.into())
            .smallest(smallest.into());
        self.0
            .round(round_ob)
            .map(Self::from)
            .map_err(map_py_value_err)
    }

    fn _round(&self, options: &RyOffsetRound) -> PyResult<Self> {
        options.round(self)
    }

    fn __hash__(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.0.hash(&mut hasher);
        hasher.finish()
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

    fn __add__(&self, other: Spanish) -> PyResult<Self> {
        self.0
            .checked_add(other)
            .map(Self::from)
            .map_err(map_py_overflow_err)
    }

    fn __sub__(&self, other: Spanish) -> PyResult<Self> {
        self.0
            .checked_sub(other)
            .map(Self::from)
            .map_err(map_py_overflow_err)
    }

    #[expect(clippy::too_many_arguments)]
    #[pyo3(
        signature=(
            other=None,
            *,
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
        other: Option<Spanish>,
        hours: i64,
        minutes: i64,
        seconds: i64,
        milliseconds: i64,
        microseconds: i64,
        nanoseconds: i64,
    ) -> PyResult<Self> {
        let spkw = SpanKwargs::new()
            .hours(hours)
            .minutes(minutes)
            .seconds(seconds)
            .milliseconds(milliseconds)
            .microseconds(microseconds)
            .nanoseconds(nanoseconds);

        match (other, !spkw.is_zero()) {
            (Some(o), false) => self.__add__(o),
            (None, true) => {
                let span = spkw.build()?;
                self.0
                    .checked_add(span)
                    .map(Self::from)
                    .map_err(map_py_overflow_err)
            }
            (Some(_), true) => {
                py_type_err!("add accepts either a span-like object or keyword units, not both")
            }
            (None, false) => Ok(*self),
        }
    }

    #[expect(clippy::too_many_arguments)]
    #[pyo3(
        signature=(
            other=None,
            *,
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
        other: Option<Spanish>,
        hours: i64,
        minutes: i64,
        seconds: i64,
        milliseconds: i64,
        microseconds: i64,
        nanoseconds: i64,
    ) -> PyResult<Self> {
        let spkw = SpanKwargs::new()
            .hours(hours)
            .minutes(minutes)
            .seconds(seconds)
            .milliseconds(milliseconds)
            .microseconds(microseconds)
            .nanoseconds(nanoseconds);

        match (other, !spkw.is_zero()) {
            (Some(o), false) => self.__sub__(o),
            (None, true) => {
                let span = spkw.build()?;
                self.0
                    .checked_sub(span)
                    .map(Self::from)
                    .map_err(map_py_overflow_err)
            }
            (Some(_), true) => {
                py_type_err!("add accepts either a span-like object or keyword units, not both")
            }
            (None, false) => Ok(*self),
        }
    }

    fn to_timezone(&self) -> RyTimeZone {
        RyTimeZone::from(self.0.to_time_zone())
    }

    fn saturating_add(&self, other: Spanish) -> Self {
        Self::from(self.0.saturating_add(other))
    }

    fn saturating_sub(&self, other: Spanish) -> Self {
        Self::from(self.0.saturating_sub(other))
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
        } else if let Ok(val) = value.cast_exact::<RySignedDuration>() {
            // let sd = val.get().0;
            Offset::try_from(val.get().0)
                .map(Self::from)
                .map_err(map_py_value_err)
                .map(|dt| dt.into_pyobject(py))?
        } else if let Ok(d) = value.cast_exact::<pyo3::types::PyDelta>() {
            let signed_dur = d.extract::<JiffSignedDuration>()?;
            Self::from_pytimedelta(signed_dur).map(|dt| dt.into_pyobject(py))?
        } else if let Ok(d) = value.cast::<pyo3::types::PyTzInfo>() {
            let wrapped_offset = d.extract::<JiffOffset>()?;
            Self::from(wrapped_offset.0).into_pyobject(py)
        } else {
            let valtype = any_repr!(value);
            py_type_err!("Offset conversion error: {valtype}")
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

impl std::fmt::Display for RyOffset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self.0.seconds();
        // if it is hours then use hours for repr
        write!(f, "Offset(")?;
        if s % 3600 == 0 {
            write!(f, "hours={}", s / 3600)?;
        } else {
            write!(f, "seconds={s}")?;
        }
        write!(f, ")")
    }
}
