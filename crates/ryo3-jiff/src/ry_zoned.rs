use crate::deprecations::deprecation_warning_intz;
use crate::errors::{map_py_overflow_err, map_py_value_err};
use crate::isoformat::{ISOFORMAT_PRINTER, ISOFORMAT_PRINTER_NO_MICROS};
use crate::ry_datetime::RyDateTime;
use crate::ry_iso_week_date::RyISOWeekDate;
use crate::ry_offset::{RyOffset, print_isoformat_offset};
use crate::ry_signed_duration::RySignedDuration;
use crate::ry_span::RySpan;
use crate::ry_time::RyTime;
use crate::ry_timestamp::RyTimestamp;
use crate::ry_timezone::RyTimeZone;
use crate::ry_zoned_round::RyZonedDateTimeRound;
use crate::spanish::Spanish;
use crate::{
    JiffEra, JiffEraYear, JiffRoundMode, JiffTzDisambiguation, JiffTzOffsetConflict, JiffUnit,
    JiffWeekday, JiffZoned, RyDate,
};
use jiff::civil::{Date, Time, Weekday};
use jiff::tz::{Offset, TimeZone};
use jiff::{Zoned, ZonedDifference, ZonedRound};
use pyo3::IntoPyObjectExt;
use pyo3::prelude::*;
use pyo3::pyclass::CompareOp;
use pyo3::types::{PyTuple, PyType};
use std::fmt::Display;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::str::FromStr;

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
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
            Date::new(year, month, day)
                .map_err(map_py_value_err)?
                .at(hour, minute, second, nanosecond)
                .in_tz(tz)
                .map(Self::from)
                .map_err(map_py_value_err)
        } else {
            let tz_system = TimeZone::try_system().map_err(map_py_value_err)?;
            Date::new(year, month, day)
                .map_err(map_py_value_err)?
                .at(hour, minute, second, nanosecond)
                .to_zoned(tz_system)
                .map(Self::from)
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
                .map(Self::from)
                .map_err(map_py_value_err)
        } else {
            Ok(Self::from(Zoned::now()))
        }
    }

    #[classmethod]
    fn utcnow(_cls: &Bound<'_, PyType>) -> PyResult<Self> {
        Zoned::now()
            .in_tz("UTC")
            .map(Self::from)
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
        Self::from(Zoned::new(ts, time_zone.into()))
    }

    #[classmethod]
    fn from_str(_cls: &Bound<'_, PyType>, s: &str) -> PyResult<Self> {
        Zoned::from_str(s).map(Self::from).map_err(map_py_value_err)
    }

    #[classmethod]
    fn parse(cls: &Bound<'_, PyType>, input: &str) -> PyResult<Self> {
        Self::from_str(cls, input)
    }

    #[classmethod]
    fn strptime(_cls: &Bound<'_, PyType>, format: &str, input: &str) -> PyResult<Self> {
        Zoned::strptime(format, input)
            .map(Self::from)
            .map_err(map_py_value_err)
    }

    #[classmethod]
    fn parse_rfc2822(_cls: &Bound<'_, PyType>, input: &str) -> PyResult<Self> {
        ::jiff::fmt::rfc2822::parse(input)
            .map(Self::from)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e}")))
    }

    #[staticmethod]
    fn from_rfc2822(s: &str) -> PyResult<Self> {
        jiff::fmt::rfc2822::parse(s)
            .map(Self::from)
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
    // ISO format mismatch:
    // input datetime: 7639-01-01 00:00:00.395000+00:00 (repr: datetime.datetime(7639, 1, 1, 0, 0, 0, 395000, tzinfo=zoneinfo.ZoneInfo(key='UTC')))
    // py: 7639-01-01T00:00:00.395000+00:00
    // ry: 7639-01-01T00:00:00+00
    // is_eq: False
    // ry_prefix_ok: False
    fn isoformat(&self) -> PyResult<String> {
        let offset: Offset = self.0.offset();
        // let ts = self.0.timestamp();
        let dattie = self.0.datetime();
        let mut s = String::with_capacity(32);
        if self.0.datetime().microsecond() == 0 && self.0.subsec_nanosecond() == 0 {
            ISOFORMAT_PRINTER_NO_MICROS.print_datetime(&dattie, &mut s)
        } else {
            ISOFORMAT_PRINTER.print_datetime(&dattie, &mut s)
        }
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("{e}")))?;
        print_isoformat_offset(&offset, &mut s)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("{e}")))?;
        Ok(s)
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
        format!("{self}")
    }

    fn __hash__(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.0.hash(&mut hasher);
        self.0
            .time_zone()
            .to_offset_info(self.0.timestamp())
            .hash(&mut hasher);
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
    fn from_pydatetime(_cls: &Bound<'_, PyType>, d: JiffZoned) -> Self {
        Self::from(d.0)
    }

    fn in_tz(&self, tz: &str) -> PyResult<Self> {
        self.0
            .in_tz(tz)
            .map(Self::from)
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
            .map(Self::from)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e}")))
    }

    fn __sub__<'py>(
        &self,
        py: Python<'py>,
        other: &Bound<'py, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        #[expect(clippy::arithmetic_side_effects)]
        if let Ok(zoned) = other.downcast::<Self>() {
            // if other is a Zoned, return a Span
            let span = &self.0 - &zoned.get().0;
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

    fn sub<'py>(&self, py: Python<'py>, other: &Bound<'py, PyAny>) -> PyResult<Bound<'py, PyAny>> {
        self.__sub__(py, other)
    }

    fn saturating_add(&self, other: &Bound<'_, PyAny>) -> PyResult<Self> {
        let spanish = Spanish::try_from(other)?;
        Ok(Self::from(self.0.saturating_add(spanish)))
    }

    fn saturating_sub(&self, other: &Bound<'_, PyAny>) -> PyResult<Self> {
        let spanish = Spanish::try_from(other)?;
        Ok(Self::from(self.0.saturating_sub(spanish)))
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
            .map(Self::from)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e}")))
    }

    fn _round(&self, dt_round: &RyZonedDateTimeRound) -> PyResult<Self> {
        self.0
            .round(dt_round.round)
            .map(Self::from)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e}")))
    }

    fn tomorrow(&self) -> PyResult<Self> {
        self.0.tomorrow().map(Self::from).map_err(map_py_value_err)
    }

    fn yesterday(&self) -> PyResult<Self> {
        self.0.yesterday().map(Self::from).map_err(map_py_value_err)
    }

    fn end_of_day(&self) -> PyResult<Self> {
        self.0
            .end_of_day()
            .map(Self::from)
            .map_err(map_py_value_err)
    }

    fn in_leap_year(&self) -> bool {
        self.0.in_leap_year()
    }

    fn last_of_month(&self) -> PyResult<Self> {
        self.0
            .last_of_month()
            .map(Self::from)
            .map_err(map_py_value_err)
    }

    fn last_of_year(&self) -> PyResult<Self> {
        self.0
            .last_of_year()
            .map(Self::from)
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
            .map(Self::from)
            .map_err(map_py_value_err)
    }

    fn first_of_year(&self) -> PyResult<Self> {
        self.0
            .first_of_year()
            .map(Self::from)
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
            .map(Self::from)
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

    fn with_time_zone(&self, tz: &RyTimeZone) -> Self {
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
        builder.build().map(Self::from).map_err(map_py_value_err)
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
        if let Some(tz_name) = self.0.time_zone().iana_name() {
            write!(
                f,
                "ZonedDateTime(year={}, month={}, day={}, hour={}, minute={}, second={}, nanosecond={}, tz=\"{}\")",
                self.0.year(),
                self.0.month(),
                self.0.day(),
                self.0.hour(),
                self.0.minute(),
                self.0.second(),
                self.0.subsec_nanosecond(),
                tz_name
            )
        } else {
            write!(f, "ZonedDateTime.parse(\"{}\")", self.0)
        }
    }
}

impl From<Zoned> for RyZoned {
    fn from(value: Zoned) -> Self {
        Self(value)
    }
}
