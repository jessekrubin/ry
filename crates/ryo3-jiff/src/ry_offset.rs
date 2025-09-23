use crate::constants::DATETIME_PARSER;
use crate::errors::{map_py_overflow_err, map_py_value_err};
use crate::round::RyOffsetRound;
use crate::ry_datetime::RyDateTime;
use crate::ry_signed_duration::RySignedDuration;
use crate::ry_span::RySpan;
use crate::ry_timestamp::RyTimestamp;
use crate::ry_timezone::RyTimeZone;
use crate::spanish::Spanish;
use crate::{JiffOffset, JiffRoundMode, JiffUnit};
use jiff::tz::{Offset, OffsetRound};
use pyo3::IntoPyObjectExt;
use pyo3::prelude::*;
use pyo3::pyclass::CompareOp;
use pyo3::types::{PyDict, PyTuple};
use ryo3_macro_rules::py_type_error;
use std::hash::{DefaultHasher, Hash, Hasher};

#[pyclass(name = "Offset", frozen)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
#[derive(Clone, Copy, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct RyOffset(pub(crate) Offset);

#[expect(clippy::wrong_self_convention)]
#[pymethods]
impl RyOffset {
    #[new]
    #[pyo3(signature = (hours = None, seconds = None))]
    fn py_new(hours: Option<i8>, seconds: Option<i32>) -> PyResult<Self> {
        match (hours, seconds) {
            (Some(h), None) => Offset::from_hours(h)
                .map(Self::from)
                .map_err(map_py_value_err),
            (None, Some(s)) => Offset::from_seconds(s)
                .map(Self::from)
                .map_err(map_py_value_err),
            _ => Err(py_type_error!("Offset() takes either hours or seconds")),
        }
    }

    fn __getnewargs__<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyTuple>> {
        PyTuple::new(
            py,
            vec![
                py.None().into_bound_py_any(py)?,
                self.0.seconds().into_bound_py_any(py)?,
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
    fn from_str(s: &str) -> PyResult<Self> {
        let o = DATETIME_PARSER
            .parse_time_zone(s)
            .map_err(map_py_value_err)?;
        o.to_fixed_offset()
            .map(Self::from)
            .map_err(map_py_value_err)
    }

    #[staticmethod]
    fn parse(s: &str) -> PyResult<Self> {
        Self::from_str(s)
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

    fn to_py(&self) -> &Offset {
        &self.0
    }

    fn to_pytzinfo(&self) -> &Offset {
        &self.0
    }

    #[expect(clippy::wrong_self_convention)]
    fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        let dict = PyDict::new(py);
        dict.set_item(crate::interns::seconds(py), self.seconds())?;
        dict.set_item(crate::interns::fmt(py), self.py_to_string())?;
        Ok(dict)
    }

    #[staticmethod]
    fn from_pytzinfo(d: JiffOffset) -> Self {
        Self::from(d.0)
    }

    #[pyo3(
        warn(
            message = "obj.string() is deprecated, use `obj.to_string()` or `str(obj)` [remove in 0.0.60]",
            category = pyo3::exceptions::PyDeprecationWarning
      )
    )]
    #[must_use]
    fn string(&self) -> String {
        self.py_to_string()
    }

    #[pyo3(name = "to_string")]
    #[must_use]
    fn py_to_string(&self) -> String {
        self.__str__()
    }

    #[must_use]
    fn __str__(&self) -> String {
        self.0.to_string()
    }

    #[must_use]
    fn __repr__(&self) -> String {
        format!("{self}")
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
        signature = (smallest = None, *, mode = None, increment = None),
        text_signature = "($self, smallest=\"second\", *, mode=\"half-expand\", increment=1)"
    )]
    fn round(
        &self,
        smallest: Option<JiffUnit>,
        mode: Option<JiffRoundMode>,
        increment: Option<i64>,
    ) -> PyResult<Self> {
        let mut round_ob = OffsetRound::new();
        if let Some(smallest) = smallest {
            round_ob = round_ob.smallest(smallest.0);
        }
        if let Some(mode) = mode {
            round_ob = round_ob.mode(mode.0);
        }
        if let Some(increment) = increment {
            round_ob = round_ob.increment(increment);
        }
        self.0
            .round(round_ob)
            .map(Self::from)
            .map_err(map_py_value_err)
    }

    fn _round(&self, opts: &RyOffsetRound) -> PyResult<Self> {
        opts.round(self)
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

    fn __add__<'py>(&self, other: &'py Bound<'py, PyAny>) -> PyResult<Self> {
        let spanish = Spanish::try_from(other)?;
        self.0
            .checked_add(spanish)
            .map(Self::from)
            .map_err(map_py_overflow_err)
    }

    fn __sub__<'py>(&self, other: &'py Bound<'py, PyAny>) -> PyResult<Self> {
        let spanish = Spanish::try_from(other)?;
        self.0
            .checked_sub(spanish)
            .map(Self::from)
            .map_err(map_py_overflow_err)
    }

    fn add<'py>(&self, other: &'py Bound<'py, PyAny>) -> PyResult<Self> {
        self.__add__(other)
    }

    fn sub<'py>(&self, other: &'py Bound<'py, PyAny>) -> PyResult<Self> {
        self.__sub__(other)
    }

    fn to_timezone(&self) -> RyTimeZone {
        RyTimeZone::from(self.0.to_time_zone())
    }

    fn saturating_add<'py>(&self, other: &'py Bound<'py, PyAny>) -> PyResult<Self> {
        let spanish = Spanish::try_from(other)?;
        Ok(self.0.saturating_add(spanish).into())
    }

    fn saturating_sub<'py>(&self, other: &'py Bound<'py, PyAny>) -> PyResult<Self> {
        let spanish = Spanish::try_from(other)?;
        Ok(self.0.saturating_sub(spanish).into())
    }
}

impl From<Offset> for RyOffset {
    fn from(value: Offset) -> Self {
        Self(value)
    }
}

impl From<JiffOffset> for RyOffset {
    fn from(value: JiffOffset) -> Self {
        Self::from(value.0)
    }
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

pub(crate) fn print_isoformat_offset<W: std::fmt::Write>(
    offset: &Offset,
    w: &mut W,
) -> std::fmt::Result {
    if offset.is_zero() {
        return write!(w, "+00:00");
    }
    // total number of seconds
    let sign = if offset.is_negative() { "-" } else { "+" };
    let total_seconds = offset.seconds();
    // calculate hours and minutes, and seconds
    let hours = total_seconds.abs() / 3600;
    let minutes = (total_seconds.abs() % 3600) / 60;
    let seconds = total_seconds.abs() % 60;

    // write the formatted string
    if seconds == 0 {
        write!(w, "{sign}{hours:02}:{minutes:02}")
    } else {
        write!(w, "{sign}{hours:02}:{minutes:02}:{seconds:02}")
    }
}
