use crate::RySpan;
use crate::RyTimeRound;
use crate::difference::{RyTimeDifference, TimeDifferenceArg};
use crate::errors::{map_py_overflow_err, map_py_value_err};
use crate::isoformat::{ISOFORMAT_PRINTER, ISOFORMAT_PRINTER_NO_MICROS};
use crate::series::RyTimeSeries;
use crate::spanish::Spanish;
use crate::{JiffRoundMode, JiffTime, JiffUnit};
use crate::{RyDate, RyDateTime};
use crate::{RySignedDuration, RyTimestamp, RyZoned};
use jiff::Zoned;
use jiff::civil::{Time, TimeRound};
use pyo3::IntoPyObjectExt;
use pyo3::basic::CompareOp;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyTuple};
use ryo3_macro_rules::any_repr;
use ryo3_macro_rules::py_type_err;
use std::fmt::Display;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::ops::Sub;
use std::str::FromStr;

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[pyclass(name = "Time", frozen)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
pub struct RyTime(pub(crate) Time);

#[pymethods]
impl RyTime {
    #[new]
    #[pyo3(signature = (hour=0, minute=0, second=0, nanosecond=0))]
    pub(crate) fn py_new(
        hour: Option<i8>,
        minute: Option<i8>,
        second: Option<i8>,
        nanosecond: Option<i32>,
    ) -> PyResult<Self> {
        Time::new(
            hour.unwrap_or(0),
            minute.unwrap_or(0),
            second.unwrap_or(0),
            nanosecond.unwrap_or(0),
        )
        .map(Self::from)
        .map_err(map_py_value_err)
    }

    fn __getnewargs__<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyTuple>> {
        PyTuple::new(
            py,
            vec![
                self.hour().into_pyobject(py)?,
                self.minute().into_pyobject(py)?,
                self.second().into_pyobject(py)?,
                self.subsec_nanosecond().into_pyobject(py)?,
            ],
        )
    }
    // ========================================================================
    // CLASS ATTRS
    // ========================================================================
    #[expect(non_snake_case)]
    #[classattr]
    fn MIN() -> Self {
        Self(Time::MIN)
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn MAX() -> Self {
        Self(Time::MAX)
    }

    // ========================================================================
    // CLASS METHODS
    // ========================================================================
    #[staticmethod]
    fn now() -> Self {
        jiff::civil::Time::from(Zoned::now()).into()
    }

    #[staticmethod]
    fn utcnow() -> Self {
        jiff::civil::Time::from(Zoned::now().with_time_zone(jiff::tz::TimeZone::UTC)).into()
    }

    #[staticmethod]
    fn midnight() -> Self {
        Self(Time::midnight())
    }

    #[staticmethod]
    fn from_str(s: &str) -> PyResult<Self> {
        Time::from_str(s).map(Self::from).map_err(map_py_value_err)
    }

    #[staticmethod]
    fn parse(input: &str) -> PyResult<Self> {
        Self::from_str(input)
    }

    // ========================================================================
    // STRPTIME/STRFTIME
    // ========================================================================
    fn __format__(&self, fmt: &str) -> String {
        self.0.strftime(fmt).to_string()
    }

    fn strftime(&self, fmt: &str) -> String {
        self.0.strftime(fmt).to_string()
    }

    #[staticmethod]
    #[pyo3(signature = (s, /, fmt))]
    fn strptime(s: &str, fmt: &str) -> PyResult<Self> {
        Time::strptime(fmt, s)
            .map(Self::from)
            .map_err(map_py_value_err)
    }

    // ========================================================================
    // STRING
    // ========================================================================
    #[pyo3(
        warn(
            message = "obj.string() is deprecated, use `obj.to_string()` or `str(obj)` [remove in 0.0.60]",
            category = pyo3::exceptions::PyDeprecationWarning
      )
    )]
    fn string(&self) -> String {
        self.0.to_string()
    }

    #[pyo3(name = "to_string")]
    fn py_to_string(&self) -> String {
        self.__str__()
    }

    fn __str__(&self) -> String {
        self.0.to_string()
    }

    fn __repr__(&self) -> String {
        format!("{self}")
    }

    fn isoformat(&self) -> String {
        if self.0.subsec_nanosecond() == 0 {
            ISOFORMAT_PRINTER_NO_MICROS.time_to_string(&self.0)
        } else {
            ISOFORMAT_PRINTER.time_to_string(&self.0)
        }
    }

    // ========================================================================
    // OPERATORS/DUNDERS
    // ========================================================================
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

    fn __hash__(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.0.hash(&mut hasher);
        hasher.finish()
    }

    fn __sub__<'py>(
        &self,
        py: Python<'py>,
        other: &Bound<'py, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        if let Ok(ob) = other.cast::<Self>() {
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

    fn add(&self, other: &Bound<'_, PyAny>) -> PyResult<Self> {
        self.__add__(other)
    }

    fn sub<'py>(&self, py: Python<'py>, other: &Bound<'py, PyAny>) -> PyResult<Bound<'py, PyAny>> {
        self.__sub__(py, other)
    }

    // ========================================================================
    // PROPERTIES
    // ========================================================================
    #[getter]
    fn hour(&self) -> i8 {
        self.0.hour()
    }

    #[getter]
    fn minute(&self) -> i8 {
        self.0.minute()
    }
    #[getter]
    fn second(&self) -> i8 {
        self.0.second()
    }

    #[getter]
    fn millisecond(&self) -> i16 {
        self.0.millisecond()
    }

    #[getter]
    fn microsecond(&self) -> i16 {
        self.0.microsecond()
    }

    #[getter]
    fn nanosecond(&self) -> i16 {
        self.0.nanosecond()
    }

    #[getter]
    fn subsec_nanosecond(&self) -> i32 {
        self.0.subsec_nanosecond()
    }

    // =====================================================================
    // PYTHON CONVERSIONS
    // =====================================================================
    #[expect(clippy::wrong_self_convention)]
    fn to_py(&self) -> Time {
        self.to_pytime()
    }

    #[expect(clippy::wrong_self_convention)]
    fn to_pytime(&self) -> Time {
        self.0
    }

    #[staticmethod]
    fn from_pytime(py_time: JiffTime) -> Self {
        Self::from(py_time.0)
    }

    // =====================================================================
    // INSTANCE METHODS
    // =====================================================================

    fn on(&self, year: i16, month: i8, day: i8) -> RyDateTime {
        RyDateTime::from(self.0.on(year, month, day))
    }

    #[pyo3(
        signature = (
            *,
            hour=None,
            minute=None,
            second=None,
            millisecond=None,
            microsecond=None,
            nanosecond=None,
            subsec_nanosecond=None,
        )
    )]
    #[expect(clippy::too_many_arguments)]
    fn replace(
        &self,
        hour: Option<i8>,
        minute: Option<i8>,
        second: Option<i8>,
        millisecond: Option<i16>,
        microsecond: Option<i16>,
        nanosecond: Option<i16>,
        subsec_nanosecond: Option<i32>,
    ) -> PyResult<Self> {
        if hour.is_none()
            && minute.is_none()
            && second.is_none()
            && millisecond.is_none()
            && microsecond.is_none()
            && nanosecond.is_none()
            && subsec_nanosecond.is_none()
        {
            // nothing to replace, return self
            return Ok(*self);
        }
        if subsec_nanosecond.is_some()
            && (millisecond.is_some() || microsecond.is_some() || nanosecond.is_some())
        {
            return py_type_err!(
                "Cannot specify both subsec_nanosecond and millisecond/microsecond/nanosecond",
            );
        }

        // start the builder
        let mut builder = self.0.with();
        if let Some(h) = hour {
            builder = builder.hour(h);
        }
        if let Some(min) = minute {
            builder = builder.minute(min);
        }
        if let Some(sec) = second {
            builder = builder.second(sec);
        }
        if let Some(us) = microsecond {
            builder = builder.microsecond(us);
        }
        if let Some(ms) = millisecond {
            builder = builder.millisecond(ms);
        }
        if let Some(ns) = nanosecond {
            builder = builder.nanosecond(ns);
        }
        if let Some(subns) = subsec_nanosecond {
            builder = builder.subsec_nanosecond(subns);
        }
        // finally build, mapping any error back to Python
        builder.build().map(Self::from).map_err(map_py_value_err)
    }

    fn astuple<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyTuple>> {
        PyTuple::new(
            py,
            vec![
                i32::from(self.0.hour()),
                i32::from(self.0.minute()),
                i32::from(self.0.second()),
                self.0.subsec_nanosecond(),
            ],
        )
    }

    #[expect(clippy::wrong_self_convention)]
    fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        use crate::interns;
        let dict = PyDict::new(py);
        dict.set_item(interns::hour(py), self.0.hour())?;
        dict.set_item(interns::minute(py), self.0.minute())?;
        dict.set_item(interns::second(py), self.0.second())?;
        dict.set_item(interns::nanosecond(py), self.0.subsec_nanosecond())?;
        Ok(dict)
    }

    fn series(&self, period: &RySpan) -> PyResult<RyTimeSeries> {
        period.assert_non_zero()?;
        let s = self.0.series(period.0);
        Ok(RyTimeSeries::from(s))
    }

    fn duration_since(&self, other: &Self) -> RySignedDuration {
        RySignedDuration::from(self.0.duration_since(other.0))
    }

    fn duration_until(&self, other: &Self) -> RySignedDuration {
        RySignedDuration::from(self.0.duration_until(other.0))
    }

    fn saturating_add(&self, other: &Bound<'_, PyAny>) -> PyResult<Self> {
        let spanish = Spanish::try_from(other)?;
        Ok(Self::from(self.0.saturating_add(spanish)))
    }

    fn saturating_sub(&self, other: &Bound<'_, PyAny>) -> PyResult<Self> {
        let spanish = Spanish::try_from(other)?;
        Ok(Self::from(self.0.saturating_sub(spanish)))
    }

    fn wrapping_add<'py>(&self, other: &'py Bound<'py, PyAny>) -> PyResult<Self> {
        let spanish = Spanish::try_from(other)?;
        Ok(Self::from(self.0.wrapping_add(spanish)))
    }

    fn wrapping_sub<'py>(&self, other: &'py Bound<'py, PyAny>) -> PyResult<Self> {
        let spanish = Spanish::try_from(other)?;
        Ok(Self::from(self.0.wrapping_sub(spanish)))
    }

    #[expect(clippy::wrong_self_convention)]
    fn to_datetime(&self, date: &RyDate) -> RyDateTime {
        RyDateTime::from(self.0.to_datetime(date.0))
    }

    #[pyo3(signature = (smallest = None, mode = None, increment = None))]
    fn round(
        &self,
        smallest: Option<JiffUnit>,
        mode: Option<JiffRoundMode>,
        increment: Option<i64>,
    ) -> PyResult<Self> {
        let mut timeround = TimeRound::new();
        if let Some(smallest) = smallest {
            timeround = timeround.smallest(smallest.0);
        }
        if let Some(mode) = mode {
            timeround = timeround.mode(mode.0);
        }
        if let Some(increment) = increment {
            timeround = timeround.increment(increment);
        }
        self.0
            .round(timeround)
            .map(Self::from)
            .map_err(map_py_value_err)
    }

    fn _round(&self, dt_round: &RyTimeRound) -> PyResult<Self> {
        dt_round.round(self)
    }
    // ------------------------------------------------------------------------
    // SINCE/UNTIL
    // ------------------------------------------------------------------------
    #[pyo3(
       signature = (t, *, smallest=None, largest = None, mode = None, increment = None),
    )]
    fn since(
        &self,
        t: TimeDifferenceArg,
        smallest: Option<JiffUnit>,
        largest: Option<JiffUnit>,
        mode: Option<JiffRoundMode>,
        increment: Option<i64>,
    ) -> PyResult<RySpan> {
        let t_diff = t.build(smallest, largest, mode, increment);
        self.0
            .since(t_diff)
            .map(RySpan::from)
            .map_err(map_py_value_err)
    }

    #[pyo3(
       signature = (t, *, smallest=None, largest = None, mode = None, increment = None),
    )]
    fn until(
        &self,
        t: TimeDifferenceArg,
        smallest: Option<JiffUnit>,
        largest: Option<JiffUnit>,
        mode: Option<JiffRoundMode>,
        increment: Option<i64>,
    ) -> PyResult<RySpan> {
        let t_diff = t.build(smallest, largest, mode, increment);
        self.0
            .until(t_diff)
            .map(RySpan::from)
            .map_err(map_py_value_err)
    }

    fn _since(&self, other: &RyTimeDifference) -> PyResult<RySpan> {
        self.0
            .since(other.diff)
            .map(RySpan::from)
            .map_err(map_py_value_err)
    }

    fn _until(&self, other: &RyTimeDifference) -> PyResult<RySpan> {
        self.0
            .until(other.diff)
            .map(RySpan::from)
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
        } else if let Ok(d) = value.cast_exact::<RyDateTime>() {
            let dt = d.get().time();
            dt.into_bound_py_any(py)
        } else if let Ok(d) = value.cast_exact::<RyZoned>() {
            let dt = d.get().time();
            dt.into_bound_py_any(py)
        } else if let Ok(d) = value.cast_exact::<RyTimestamp>() {
            let dt = d.get().time();
            dt.into_bound_py_any(py)
        } else if let Ok(d) = value.extract::<JiffTime>() {
            Self::from_pytime(d).into_bound_py_any(py)
        } else {
            let valtype = any_repr!(value);
            py_type_err!("Time conversion error: {valtype}")
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

impl Display for RyTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Time(hour={}, minute={}, second={}, nanosecond={})",
            self.0.hour(),
            self.0.minute(),
            self.0.second(),
            self.0.subsec_nanosecond()
        )
    }
}

impl From<Time> for RyTime {
    fn from(value: Time) -> Self {
        Self(value)
    }
}

impl From<JiffTime> for RyTime {
    fn from(value: JiffTime) -> Self {
        Self(value.0)
    }
}
