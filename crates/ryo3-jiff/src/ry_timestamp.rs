use crate::deprecations::deprecation_warning_intz;
use crate::errors::{map_py_overflow_err, map_py_value_err};
use crate::ry_signed_duration::RySignedDuration;
use crate::ry_span::RySpan;
use crate::ry_timestamp_difference::{RyTimestampDifference, TimestampDifferenceArg};
use crate::ry_timestamp_round::RyTimestampRound;
use crate::ry_timezone::RyTimeZone;
use crate::ry_zoned::RyZoned;
use crate::series::RyTimestampSeries;
use crate::spanish::Spanish;
use crate::{JiffRoundMode, JiffUnit, RyOffset};
use jiff::tz::TimeZone;
use jiff::{Timestamp, TimestampRound, Zoned};
use pyo3::IntoPyObjectExt;
use pyo3::basic::CompareOp;
use pyo3::prelude::*;
use pyo3::types::{PyTuple, PyType};
use std::fmt::Display;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::ops::Sub;
use std::str::FromStr;

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[pyclass(name = "Timestamp", module = "ry.ryo3", frozen)]
pub struct RyTimestamp(pub(crate) Timestamp);

#[pymethods]
impl RyTimestamp {
    #[new]
    #[pyo3(signature = (second = None, nanosecond = None))]
    pub fn py_new(second: Option<i64>, nanosecond: Option<i32>) -> PyResult<Self> {
        let s = second.unwrap_or(0);
        let ns = nanosecond.unwrap_or(0);
        Timestamp::new(s, ns)
            .map(Self::from)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e}")))
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

    #[classmethod]
    fn now(_cls: &Bound<'_, PyType>) -> Self {
        Self::from(Timestamp::now())
    }

    #[classmethod]
    fn from_str(_cls: &Bound<'_, PyType>, s: &str) -> PyResult<Self> {
        Timestamp::from_str(s)
            .map(Self::from)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e}")))
    }

    #[classmethod]
    fn parse(cls: &Bound<'_, PyType>, input: &str) -> PyResult<Self> {
        Self::from_str(cls, input)
    }

    #[classmethod]
    fn from_millisecond(_cls: &Bound<'_, PyType>, millisecond: i64) -> PyResult<Self> {
        Timestamp::from_millisecond(millisecond)
            .map(Self::from)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e}")))
    }

    fn to_zoned(&self, time_zone: &RyTimeZone) -> RyZoned {
        RyZoned::from(Zoned::new(self.0, time_zone.into()))
    }

    #[classmethod]
    fn from_pydatetime<'py>(_cls: &Bound<'py, PyType>, dt: &Bound<'py, PyAny>) -> PyResult<Self> {
        let ts = dt.extract::<Timestamp>()?;
        Ok(Self(ts))
    }

    fn to_py(&self) -> Timestamp {
        self.0
    }

    fn to_pydatetime(&self) -> Timestamp {
        self.0
    }

    fn to_pydate(&self) -> jiff::civil::Date {
        self.0.to_zoned(TimeZone::UTC).date()
    }

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

    fn string(&self) -> String {
        self.0.to_string()
    }

    fn __repr__(&self) -> String {
        format!(
            "Timestamp({:?}, {:?})",
            self.0.as_second(),
            self.0.subsec_nanosecond()
        )
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
        if let Ok(ob) = other.downcast::<Self>() {
            let span = self.0.sub(ob.get().0);
            let obj = RySpan::from(span).into_pyobject(py).map(Bound::into_any)?;
            Ok(obj)
        } else {
            let spanish = Spanish::try_from(other)?;
            let z = self.0.checked_sub(spanish).map_err(map_py_overflow_err)?;
            Self::from(z).into_bound_py_any(py)
        }
    }

    fn __add__<'py>(&self, other: &'py Bound<'py, PyAny>) -> PyResult<Self> {
        let spanish = Spanish::try_from(other)?;
        self.0
            .checked_add(spanish)
            .map(Self::from)
            .map_err(map_py_overflow_err)
    }

    fn add<'py>(&self, other: &'py Bound<'py, PyAny>) -> PyResult<Self> {
        self.__add__(other)
    }

    fn sub<'py>(
        &self,
        py: Python<'py>,
        other: &'py Bound<'py, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        self.__sub__(py, other)
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

    fn subsec_nanosecond(&self) -> i32 {
        self.0.subsec_nanosecond()
    }

    fn subsec_microsecond(&self) -> i32 {
        self.0.subsec_microsecond()
    }

    fn subsec_millisecond(&self) -> i32 {
        self.0.subsec_millisecond()
    }

    fn series(&self, period: &RySpan) -> PyResult<RyTimestampSeries> {
        period.assert_non_zero()?;
        Ok(self.0.series(period.0).into())
    }

    #[getter]
    fn is_zero(&self) -> bool {
        self.0.is_zero()
    }

    fn in_tz(&self, time_zone_name: &str) -> PyResult<RyZoned> {
        self.0
            .in_tz(time_zone_name)
            .map(RyZoned::from)
            .map_err(map_py_value_err)
    }

    fn intz(&self, py: Python, time_zone_name: &str) -> PyResult<RyZoned> {
        deprecation_warning_intz(py)?;
        self.in_tz(time_zone_name)
    }

    #[classmethod]
    fn from_microsecond(_cls: &Bound<'_, PyType>, microsecond: i64) -> PyResult<Self> {
        Timestamp::from_microsecond(microsecond)
            .map(Self::from)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e}")))
    }

    #[classmethod]
    fn from_nanosecond(_cls: &Bound<'_, PyType>, nanosecond: i128) -> PyResult<Self> {
        Timestamp::from_nanosecond(nanosecond)
            .map(Self::from)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e}")))
    }

    #[classmethod]
    fn from_second(_cls: &Bound<'_, PyType>, second: i64) -> PyResult<Self> {
        Timestamp::from_second(second)
            .map(Self::from)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e}")))
    }
    fn signum(&self) -> i8 {
        self.0.signum()
    }
    fn strftime(&self, format: &str) -> String {
        self.0.strftime(format).to_string()
    }

    #[classmethod]
    fn strptime(_cls: &Bound<'_, PyType>, s: &str, format: &str) -> PyResult<Self> {
        Timestamp::strptime(s, format)
            .map(Self::from)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e}")))
    }

    #[pyo3(
       signature = (ts, *, smallest=None, largest = None, mode = None, increment = None),
    )]
    fn since(
        &self,
        ts: TimestampDifferenceArg,
        smallest: Option<JiffUnit>,
        largest: Option<JiffUnit>,
        mode: Option<JiffRoundMode>,
        increment: Option<i64>,
    ) -> PyResult<RySpan> {
        let dt_diff = ts.build(smallest, largest, mode, increment);
        self.0
            .since(dt_diff)
            .map(RySpan::from)
            .map_err(map_py_value_err)
    }

    #[pyo3(
       signature = (ts, *, smallest=None, largest = None, mode = None, increment = None),
    )]
    fn until(
        &self,
        ts: TimestampDifferenceArg,
        smallest: Option<JiffUnit>,
        largest: Option<JiffUnit>,
        mode: Option<JiffRoundMode>,
        increment: Option<i64>,
    ) -> PyResult<RySpan> {
        let dt_diff = ts.build(smallest, largest, mode, increment);
        self.0
            .until(dt_diff)
            .map(RySpan::from)
            .map_err(map_py_value_err)
    }

    fn _since(&self, other: &RyTimestampDifference) -> PyResult<RySpan> {
        self.0
            .since(other.0)
            .map(RySpan::from)
            .map_err(map_py_value_err)
    }

    fn _until(&self, other: &RyTimestampDifference) -> PyResult<RySpan> {
        self.0
            .until(other.0)
            .map(RySpan::from)
            .map_err(map_py_value_err)
    }

    fn display_with_offset(&self, offset: &RyOffset) -> String {
        let dwo = self.0.display_with_offset(offset.0);
        dwo.to_string()
    }

    fn duration_since(&self, other: &Self) -> RySignedDuration {
        RySignedDuration::from(self.0.duration_since(other.0))
    }

    fn duration_until(&self, other: &Self) -> RySignedDuration {
        RySignedDuration::from(self.0.duration_until(other.0))
    }

    #[pyo3(
       signature = (smallest=None, mode = None, increment = None),
    )]
    fn round(
        &self,
        smallest: Option<JiffUnit>,
        mode: Option<JiffRoundMode>,
        increment: Option<i64>,
    ) -> PyResult<Self> {
        let mut ts_round = TimestampRound::new();
        if let Some(smallest) = smallest {
            ts_round = ts_round.smallest(smallest.0);
        }
        if let Some(mode) = mode {
            ts_round = ts_round.mode(mode.0);
        }
        if let Some(increment) = increment {
            ts_round = ts_round.increment(increment);
        }
        self.0
            .round(ts_round)
            .map(Self::from)
            .map_err(map_py_value_err)
    }

    fn _round(&self, opts: &RyTimestampRound) -> PyResult<Self> {
        self.0
            .round(opts.round)
            .map(Self::from)
            .map_err(map_py_value_err)
    }

    fn saturating_add(&self, other: &Bound<'_, PyAny>) -> PyResult<Self> {
        let spanish = Spanish::try_from(other)?;
        let t = self.0.saturating_add(spanish).map_err(map_py_value_err)?;
        Ok(Self::from(t))
    }

    fn saturating_sub(&self, other: &Bound<'_, PyAny>) -> PyResult<Self> {
        let spanish = Spanish::try_from(other)?;
        let t = self.0.saturating_sub(spanish).map_err(map_py_value_err)?;
        Ok(Self::from(t))
    }
}

impl Display for RyTimestamp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl From<Timestamp> for RyTimestamp {
    fn from(value: Timestamp) -> Self {
        Self(value)
    }
}
