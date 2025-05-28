use crate::delta_arithmetic_self::RyDeltaArithmeticSelf;
use crate::deprecations::deprecation_warning_intz;
use crate::errors::map_py_value_err;
use crate::ry_datetime::RyDateTime;
use crate::ry_iso_week_date::RyISOWeekDate;
use crate::ry_offset::RyOffset;
use crate::ry_signed_duration::RySignedDuration;
use crate::ry_span::RySpan;
use crate::ry_time::RyTime;
use crate::ry_timestamp::RyTimestamp;
use crate::ry_timezone::RyTimeZone;
use crate::ry_zoned_round::RyZonedDateTimeRound;
use crate::{
    JiffEra, JiffEraYear, JiffRoundMode, JiffTzDisambiguation, JiffTzOffsetConflict, JiffUnit,
    JiffWeekday, RyDate,
};
use jiff::civil::{Date, Time, Weekday};
use jiff::tz::TimeZone;
use jiff::{Zoned, ZonedDifference, ZonedRound};
use pyo3::prelude::*;
use pyo3::pyclass::CompareOp;
use pyo3::types::{PyTuple, PyType};
use pyo3::IntoPyObjectExt;
use std::fmt::Display;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::str::FromStr;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[derive(Debug, Clone, PartialEq)]
#[pyclass(name = "ZonedDateTime", module = "ry.ryo3", frozen)]
pub struct RyZoned(pub(crate) Zoned);

#[pymethods]
impl RyZoned {
    #[new]
    #[pyo3(signature = (year, month, day, hour=0, minute=0, second=0, nanosecond=0, tz=None))]
    #[expect(clippy::too_many_arguments)]
    fn py_new(
        year: i16,
        month: i8,
        day: i8,
        hour: i8,
        minute: i8,
        second: i8,
        nanosecond: i32,
        tz: Option<&str>,
    ) -> PyResult<Self> {
        if let Some(tz) = tz {
            // let a =
            Date::new(year, month, day)
                .map_err(map_py_value_err)?
                .at(hour, minute, second, nanosecond)
                .in_tz(tz)
                .map(RyZoned::from)
                .map_err(map_py_value_err)
        } else {
            let tz_system = jiff::tz::TimeZone::try_system().map_err(map_py_value_err)?;
            Date::new(year, month, day)
                .map_err(map_py_value_err)?
                .at(hour, minute, second, nanosecond)
                .to_zoned(tz_system)
                .map(RyZoned::from)
                .map_err(map_py_value_err)
        }
    }

    fn __getnewargs__<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyTuple>> {
        PyTuple::new(
            py,
            vec![
                self.year().into_bound_py_any(py)?,
                self.month().into_bound_py_any(py)?,
                self.day().into_bound_py_any(py)?,
                self.hour().into_bound_py_any(py)?,
                self.minute().into_bound_py_any(py)?,
                self.second().into_bound_py_any(py)?,
                self.subsec_nanosecond().into_bound_py_any(py)?,
                self.0.time_zone().iana_name().into_bound_py_any(py)?,
            ],
        )
    }

    #[classmethod]
    #[pyo3(signature = (tz=None))]
    fn now(_cls: &Bound<'_, PyType>, tz: Option<&str>) -> PyResult<Self> {
        if let Some(tz) = tz {
            Zoned::now()
                .in_tz(tz)
                .map(RyZoned::from)
                .map_err(map_py_value_err)
        } else {
            Ok(Self::from(Zoned::now()))
        }
    }

    #[classmethod]
    fn utcnow(_cls: &Bound<'_, PyType>) -> PyResult<Self> {
        Zoned::now()
            .in_tz("UTC")
            .map(RyZoned::from)
            .map_err(map_py_value_err)
    }

    #[classmethod]
    #[pyo3(signature = (timestamp, time_zone))]
    fn from_parts(
        _cls: &Bound<'_, PyType>,
        timestamp: &RyTimestamp,
        time_zone: &RyTimeZone,
    ) -> Self {
        let ts = timestamp.0;
        RyZoned::from(Zoned::new(ts, time_zone.into()))
    }

    #[classmethod]
    fn parse(_cls: &Bound<'_, PyType>, s: &str) -> PyResult<Self> {
        Zoned::from_str(s)
            .map(RyZoned::from)
            .map_err(map_py_value_err)
    }

    #[classmethod]
    fn strptime(_cls: &Bound<'_, PyType>, format: &str, input: &str) -> PyResult<Self> {
        Zoned::strptime(format, input)
            .map(RyZoned::from)
            .map_err(map_py_value_err)
    }

    #[classmethod]
    fn parse_rfc2822(_cls: &Bound<'_, PyType>, input: &str) -> PyResult<Self> {
        ::jiff::fmt::rfc2822::parse(input)
            .map(RyZoned::from)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e}")))
    }

    #[staticmethod]
    fn from_rfc2822(s: &str) -> PyResult<Self> {
        jiff::fmt::rfc2822::parse(s)
            .map(RyZoned::from)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e}")))
    }

    fn strftime(&self, format: &str) -> String {
        self.0.strftime(format).to_string()
    }

    fn format_rfc2822(&self) -> PyResult<String> {
        jiff::fmt::rfc2822::to_string(&self.0)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e}")))
    }

    fn to_rfc2822(&self) -> PyResult<String> {
        jiff::fmt::rfc2822::to_string(&self.0)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e}")))
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

    fn __str__(&self) -> String {
        self.0.to_string()
    }

    fn __repr__(&self) -> String {
        // representable format
        format!("ZonedDateTime.parse(\"{}\")", self.0)
    }

    fn __hash__(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.0.datetime().hash(&mut hasher);
        self.0.time_zone().iana_name().hash(&mut hasher);
        hasher.finish()
    }

    fn timestamp(&self) -> RyTimestamp {
        RyTimestamp::from(self.0.timestamp())
    }

    fn date(&self) -> RyDate {
        RyDate::from(self.0.date())
    }

    fn time(&self) -> RyTime {
        RyTime::from(self.0.time())
    }

    fn datetime(&self) -> RyDateTime {
        RyDateTime::from(self.0.datetime())
    }

    fn to_py(&self) -> &Zoned {
        self.to_pydatetime()
    }

    fn to_pydatetime(&self) -> &Zoned {
        &self.0
    }

    fn to_pydate(&self) -> Date {
        self.0.date()
    }

    fn to_pytime(&self) -> Time {
        self.0.time()
    }

    fn to_pytzinfo(&self) -> &TimeZone {
        self.0.time_zone()
    }

    #[classmethod]
    fn from_pydatetime(_cls: &Bound<'_, PyType>, d: Zoned) -> Self {
        Self::from(d)
    }

    fn in_tz(&self, tz: &str) -> PyResult<Self> {
        self.0
            .in_tz(tz)
            .map(RyZoned::from)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e}")))
    }

    fn intz(&self, py: Python, tz: &str) -> PyResult<Self> {
        deprecation_warning_intz(py)?;
        self.in_tz(tz)
    }

    fn astimezone(&self, tz: &str) -> PyResult<Self> {
        self.in_tz(tz)
    }

    fn inutc(&self) -> PyResult<Self> {
        self.0
            .in_tz("UTC")
            .map(RyZoned::from)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e}")))
    }

    fn __sub__<'py>(
        &self,
        py: Python<'py>,
        other: RyZonedArithmeticSub,
    ) -> PyResult<Bound<'py, PyAny>> {
        match other {
            #[expect(clippy::arithmetic_side_effects)]
            RyZonedArithmeticSub::Zoned(other) => {
                let span = &self.0 - &other.0;
                let obj = RySpan::from(span).into_pyobject(py).map(Bound::into_any)?;
                Ok(obj)
            }
            RyZonedArithmeticSub::Delta(other) => {
                let t = match other {
                    RyDeltaArithmeticSelf::Span(other) => {
                        self.0.checked_sub(other.0).map_err(map_py_value_err)?
                    }
                    RyDeltaArithmeticSelf::SignedDuration(other) => {
                        self.0.checked_sub(other.0).map_err(map_py_value_err)?
                    }
                    RyDeltaArithmeticSelf::Duration(other) => {
                        self.0.checked_sub(other.0).map_err(map_py_value_err)?
                    }
                };
                Ok(Self::from(t).into_pyobject(py)?.into_any())
            }
        }
    }

    // ----------------------------
    // incompatible with `frozen`
    // ----------------------------
    // fn __isub__(&mut self, _py: Python<'_>, other: RyDeltaArithmeticSelf) -> PyResult<()> {
    //     let t = match other {
    //         RyDeltaArithmeticSelf::Span(other) => {
    //             self.0.checked_sub(other.0).map_err(map_py_value_err)?
    //         }
    //         RyDeltaArithmeticSelf::SignedDuration(other) => {
    //             self.0.checked_sub(other.0).map_err(map_py_value_err)?
    //         }
    //         RyDeltaArithmeticSelf::Duration(other) => {
    //             self.0.checked_sub(other.0).map_err(map_py_value_err)?
    //         }
    //     };
    //     self.0 = t;
    //     Ok(())
    // }

    fn __add__(&self, _py: Python<'_>, other: RyDeltaArithmeticSelf) -> PyResult<Self> {
        let t = match other {
            RyDeltaArithmeticSelf::Span(other) => {
                self.0.checked_add(other.0).map_err(map_py_value_err)?
            }
            RyDeltaArithmeticSelf::SignedDuration(other) => {
                self.0.checked_add(other.0).map_err(map_py_value_err)?
            }
            RyDeltaArithmeticSelf::Duration(other) => {
                self.0.checked_add(other.0).map_err(map_py_value_err)?
            }
        };
        Ok(Self::from(t))
    }

    // ----------------------------
    // incompatible with `frozen`
    // ----------------------------
    // fn __iadd__(&mut self, _py: Python<'_>, other: RyDeltaArithmeticSelf) -> PyResult<()> {
    //     let t = match other {
    //         RyDeltaArithmeticSelf::Span(other) => {
    //             self.0.checked_add(other.0).map_err(map_py_value_err)?
    //         }
    //         RyDeltaArithmeticSelf::SignedDuration(other) => {
    //             self.0.checked_add(other.0).map_err(map_py_value_err)?
    //         }
    //         RyDeltaArithmeticSelf::Duration(other) => {
    //             self.0.checked_add(other.0).map_err(map_py_value_err)?
    //         }
    //     };
    //     self.0 = t;
    //     Ok(())
    // }

    fn checked_add(&self, py: Python<'_>, other: RyDeltaArithmeticSelf) -> PyResult<Self> {
        self.__add__(py, other)
    }

    fn checked_sub<'py>(
        &self,
        py: Python<'py>,
        other: RyZonedArithmeticSub,
    ) -> PyResult<Bound<'py, PyAny>> {
        self.__sub__(py, other)
    }

    fn saturating_add(&self, _py: Python<'_>, other: RyDeltaArithmeticSelf) -> Self {
        let t = match other {
            RyDeltaArithmeticSelf::Span(other) => self.0.saturating_add(other.0),
            RyDeltaArithmeticSelf::SignedDuration(other) => self.0.saturating_add(other.0),
            RyDeltaArithmeticSelf::Duration(other) => self.0.saturating_add(other.0),
        };
        Self::from(t)
    }

    fn saturating_sub(&self, _py: Python<'_>, other: RyDeltaArithmeticSelf) -> Self {
        let t = match other {
            RyDeltaArithmeticSelf::Span(other) => self.0.saturating_sub(other.0),
            RyDeltaArithmeticSelf::SignedDuration(other) => self.0.saturating_sub(other.0),
            RyDeltaArithmeticSelf::Duration(other) => self.0.saturating_sub(other.0),
        };
        Self::from(t)
    }

    #[pyo3(
       signature = (smallest=None, *, mode = None, increment = None),
    )]
    fn round(
        &self,
        smallest: Option<JiffUnit>,
        mode: Option<JiffRoundMode>,
        increment: Option<i64>,
    ) -> PyResult<Self> {
        let mut zdt_round = ZonedRound::new();
        if let Some(smallest) = smallest {
            zdt_round = zdt_round.smallest(smallest.0);
        }
        if let Some(mode) = mode {
            zdt_round = zdt_round.mode(mode.0);
        }
        if let Some(increment) = increment {
            zdt_round = zdt_round.increment(increment);
        }
        self.0
            .round(zdt_round)
            .map(RyZoned::from)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e}")))
    }

    fn _round(&self, dt_round: &RyZonedDateTimeRound) -> PyResult<Self> {
        self.0
            .round(dt_round.round)
            .map(RyZoned::from)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e}")))
    }

    fn tomorrow(&self) -> PyResult<Self> {
        self.0
            .tomorrow()
            .map(RyZoned::from)
            .map_err(map_py_value_err)
    }

    fn yesterday(&self) -> PyResult<Self> {
        self.0
            .yesterday()
            .map(RyZoned::from)
            .map_err(map_py_value_err)
    }

    fn end_of_day(&self) -> PyResult<Self> {
        self.0
            .end_of_day()
            .map(RyZoned::from)
            .map_err(map_py_value_err)
    }

    fn in_leap_year(&self) -> bool {
        self.0.in_leap_year()
    }

    fn last_of_month(&self) -> PyResult<Self> {
        self.0
            .last_of_month()
            .map(RyZoned::from)
            .map_err(map_py_value_err)
    }

    fn last_of_year(&self) -> PyResult<Self> {
        self.0
            .last_of_year()
            .map(RyZoned::from)
            .map_err(map_py_value_err)
    }

    fn days_in_month(&self) -> i8 {
        self.0.days_in_month()
    }

    fn days_in_year(&self) -> i16 {
        self.0.days_in_year()
    }

    fn day_of_year(&self) -> i16 {
        self.0.day_of_year()
    }

    fn day_of_year_no_leap(&self) -> Option<i16> {
        self.0.day_of_year_no_leap()
    }

    fn duration_since(&self, other: &Self) -> RySignedDuration {
        RySignedDuration::from(self.0.duration_since(&other.0))
    }

    fn duration_until(&self, other: &Self) -> RySignedDuration {
        RySignedDuration::from(self.0.duration_until(&other.0))
    }

    fn era_year<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let era_year = JiffEraYear(self.0.era_year());
        let obj = era_year.into_pyobject(py)?;
        Ok(obj.into_any())
    }

    fn first_of_month(&self) -> PyResult<Self> {
        self.0
            .first_of_month()
            .map(RyZoned::from)
            .map_err(map_py_value_err)
    }

    fn first_of_year(&self) -> PyResult<Self> {
        self.0
            .first_of_year()
            .map(RyZoned::from)
            .map_err(map_py_value_err)
    }

    fn nth_weekday(&self, nth: i32, weekday: JiffWeekday) -> PyResult<Self> {
        self.0
            .nth_weekday(nth, weekday.0)
            .map(Self::from)
            .map_err(map_py_value_err)
    }

    fn nth_weekday_of_month(&self, nth: i8, weekday: JiffWeekday) -> PyResult<Self> {
        self.0
            .nth_weekday_of_month(nth, weekday.0)
            .map(Self::from)
            .map_err(map_py_value_err)
    }

    fn offset(&self) -> RyOffset {
        self.0.offset().into()
    }

    fn start_of_day(&self) -> PyResult<Self> {
        self.0
            .start_of_day()
            .map(RyZoned::from)
            .map_err(map_py_value_err)
    }

    #[pyo3(
       signature = (zdt, *, smallest=None, largest = None, mode = None, increment = None),
    )]
    fn since(
        &self,
        zdt: &Self,
        smallest: Option<JiffUnit>,
        largest: Option<JiffUnit>,
        mode: Option<JiffRoundMode>,
        increment: Option<i64>,
    ) -> PyResult<RySpan> {
        let mut zdt_diff = ZonedDifference::from(&zdt.0);
        if let Some(smallest) = smallest {
            zdt_diff = zdt_diff.smallest(smallest.0);
        }
        if let Some(largest) = largest {
            zdt_diff = zdt_diff.largest(largest.0);
        }
        if let Some(mode) = mode {
            zdt_diff = zdt_diff.mode(mode.0);
        }
        if let Some(increment) = increment {
            zdt_diff = zdt_diff.increment(increment);
        }
        self.0
            .since(zdt_diff)
            .map(RySpan::from)
            .map_err(map_py_value_err)
    }

    #[pyo3(
       signature = (zdt, *, smallest=None, largest = None, mode = None, increment = None),
    )]
    fn until(
        &self,
        zdt: &Self,
        smallest: Option<JiffUnit>,
        largest: Option<JiffUnit>,
        mode: Option<JiffRoundMode>,
        increment: Option<i64>,
    ) -> PyResult<RySpan> {
        let mut zdt_diff = ZonedDifference::from(&zdt.0);
        if let Some(smallest) = smallest {
            zdt_diff = zdt_diff.smallest(smallest.0);
        }
        if let Some(largest) = largest {
            zdt_diff = zdt_diff.largest(largest.0);
        }
        if let Some(mode) = mode {
            zdt_diff = zdt_diff.mode(mode.0);
        }
        if let Some(increment) = increment {
            zdt_diff = zdt_diff.increment(increment);
        }
        self.0
            .until(zdt_diff)
            .map(RySpan::from)
            .map_err(map_py_value_err)
    }

    fn with_time_zone(&self, tz: &RyTimeZone) -> RyZoned {
        self.0.with_time_zone(tz.into()).into()
    }

    fn iso_week_date(&self) -> RyISOWeekDate {
        let d = self.0.date();
        d.iso_week_date().into()
    }

    #[pyo3(
        signature = (
            obj=None,
            *,
            date=None,
            time=None,
            year=None,
            era_year=None,
            month=None,
            day=None,
            day_of_year=None,
            day_of_year_no_leap=None,
            hour=None,
            minute=None,
            second=None,
            millisecond=None,
            microsecond=None,
            nanosecond=None,
            subsec_nanosecond=None,
            offset=None,
            offset_conflict=None,
            disambiguation=None,
        )
    )]
    #[expect(clippy::too_many_arguments)]
    fn replace(
        &self,
        obj: Option<Bound<'_, PyAny>>,
        date: Option<RyDate>,
        time: Option<RyTime>,
        year: Option<i16>,
        era_year: Option<(i16, JiffEra)>,
        month: Option<i8>,
        day: Option<i8>,
        day_of_year: Option<i16>,
        day_of_year_no_leap: Option<i16>,
        hour: Option<i8>,
        minute: Option<i8>,
        second: Option<i8>,
        millisecond: Option<i16>,
        microsecond: Option<i16>,
        nanosecond: Option<i16>,
        subsec_nanosecond: Option<i32>,
        offset: Option<RyOffset>,
        offset_conflict: Option<JiffTzOffsetConflict>,
        disambiguation: Option<JiffTzDisambiguation>,
    ) -> PyResult<Self> {
        // start the builder
        let mut builder = self.0.with();
        if let Some(obj) = obj {
            // if obj is a Zoned, use it as the base
            if let Ok(zoned) = obj.downcast::<RyDate>() {
                // if obj is a Zoned, use it as the base
                let date = zoned.extract::<RyDate>()?;
                builder = builder.date(date.0);
            } else if let Ok(time) = obj.downcast::<RyTime>() {
                // if obj is a Time, use it as the base
                let time = time.extract::<RyTime>()?;
                builder = builder.time(time.0);
            } else if let Ok(date) = obj.downcast::<RyOffset>() {
                let offset = date.extract::<RyOffset>()?;
                builder = builder.offset(offset.0);
            } else {
                return Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(format!(
                    "obj must be a Date, Time or Offset; given: {obj}",
                )));
            }
        }

        // only override if the Option is Some
        if let Some(date) = date {
            builder = builder.date(date.0);
        }
        if let Some(time) = time {
            builder = builder.time(time.0);
        }
        if let Some(y) = year {
            builder = builder.year(y);
        }
        if let Some(ey) = era_year {
            builder = builder.era_year(ey.0, (ey.1).0);
        }
        if let Some(m) = month {
            builder = builder.month(m);
        }
        if let Some(d) = day {
            builder = builder.day(d);
        }
        if let Some(doy) = day_of_year {
            builder = builder.day_of_year(doy);
        }
        if let Some(doy) = day_of_year_no_leap {
            builder = builder.day_of_year_no_leap(doy);
        }
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
        if let Some(offset) = offset {
            builder = builder.offset(offset.0);
        }
        if let Some(offset_conflict) = offset_conflict {
            builder = builder.offset_conflict(offset_conflict.0);
        }
        if let Some(disambiguation) = disambiguation {
            builder = builder.disambiguation(disambiguation.0);
        }
        // finally build, mapping any error back to Python
        builder.build().map(RyZoned::from).map_err(map_py_value_err)
    }

    // -----------------------------------------------------------------------
    // getters
    // -----------------------------------------------------------------------
    #[getter]
    fn timezone(&self) -> RyTimeZone {
        RyTimeZone::from(self.0.time_zone())
    }

    #[getter]
    fn tz(&self) -> RyTimeZone {
        RyTimeZone::from(self.0.time_zone())
    }

    #[getter]
    fn year(&self) -> i16 {
        self.0.year()
    }

    #[getter]
    fn month(&self) -> i8 {
        self.0.month()
    }

    #[getter]
    fn day(&self) -> i8 {
        self.0.day()
    }

    #[getter]
    fn weekday(&self) -> i8 {
        match self.0.weekday() {
            Weekday::Monday => 1,
            Weekday::Tuesday => 2,
            Weekday::Wednesday => 3,
            Weekday::Thursday => 4,
            Weekday::Friday => 5,
            Weekday::Saturday => 6,
            Weekday::Sunday => 7,
        }
    }

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
    fn microsecond(&self) -> i16 {
        self.0.microsecond()
    }

    #[getter]
    fn millisecond(&self) -> i16 {
        self.0.millisecond()
    }

    #[getter]
    fn nanosecond(&self) -> i16 {
        self.0.nanosecond()
    }

    #[getter]
    fn subsec_nanosecond(&self) -> i32 {
        self.0.subsec_nanosecond()
    }
}

impl Display for RyZoned {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl From<Zoned> for RyZoned {
    fn from(value: Zoned) -> Self {
        RyZoned(value)
    }
}

#[derive(Debug, Clone, FromPyObject)]
pub(crate) enum RyZonedArithmeticSub {
    Zoned(RyZoned),
    Delta(RyDeltaArithmeticSelf),
}
