use crate::delta_arithmetic_self::RyDeltaArithmeticSelf;
use crate::deprecations::deprecation_warning_intz;
use crate::errors::map_py_value_err;
use crate::ry_datetime_difference::{DateTimeDifferenceArg, RyDateTimeDifference};
use crate::ry_iso_week_date::RyISOWeekDate;
use crate::ry_signed_duration::RySignedDuration;
use crate::ry_span::RySpan;
use crate::ry_time::RyTime;
use crate::ry_timezone::RyTimeZone;
use crate::ry_zoned::RyZoned;
use crate::{JiffEraYear, JiffRoundMode, JiffUnit, JiffWeekday, RyDate, RyDateTimeRound};
use jiff::civil::{Date, DateTime, DateTimeRound, Time, Weekday};
use jiff::Zoned;
use pyo3::basic::CompareOp;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyTuple, PyType};
use pyo3::{intern, IntoPyObjectExt};
use std::borrow::BorrowMut;
use std::fmt::Display;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::str::FromStr;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[pyclass(name = "DateTime", module = "ry.ryo3", frozen)]
pub struct RyDateTime(pub(crate) DateTime);

impl From<DateTime> for RyDateTime {
    fn from(value: DateTime) -> Self {
        RyDateTime(value)
    }
}

#[pymethods]
impl RyDateTime {
    #[new]
    #[pyo3(signature = ( year, month, day, hour=0, minute=0, second=0, subsec_nanosecond=0))]
    pub(crate) fn py_new(
        year: i16,
        month: i8,
        day: i8,
        hour: Option<i8>,
        minute: Option<i8>,
        second: Option<i8>,
        subsec_nanosecond: Option<i32>,
    ) -> PyResult<Self> {
        DateTime::new(
            year,
            month,
            day,
            hour.unwrap_or(0),
            minute.unwrap_or(0),
            second.unwrap_or(0),
            subsec_nanosecond.unwrap_or(0),
        )
        .map(RyDateTime::from)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e}")))
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
            ],
        )
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn MIN() -> Self {
        Self(DateTime::MIN)
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn MAX() -> Self {
        Self(DateTime::MAX)
    }
    #[expect(non_snake_case)]
    #[classattr]
    fn ZERO() -> Self {
        Self(DateTime::ZERO)
    }

    #[classmethod]
    fn now(_cls: &Bound<'_, PyType>) -> Self {
        Self::from(DateTime::from(Zoned::now()))
    }
    #[classmethod]
    fn parse(_cls: &Bound<'_, PyType>, s: &str) -> PyResult<Self> {
        DateTime::from_str(s)
            .map(RyDateTime::from)
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

    fn __str__(&self) -> String {
        self.to_string()
    }

    fn string(&self) -> String {
        self.to_string()
    }

    fn __repr__(&self) -> String {
        format!(
            "DateTime(year={}, month={}, day={}, hour={}, minute={}, second={}, subsec_nanosecond={})",
            self.year(),
            self.month(),
            self.day(),
            self.hour(),
            self.minute(),
            self.second(),
            self.subsec_nanosecond()
        )
    }

    fn __hash__(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.0.hash(&mut hasher);
        hasher.finish()
    }

    fn __add__(&self, _py: Python<'_>, other: RyDeltaArithmeticSelf) -> PyResult<Self> {
        let t = match other {
            RyDeltaArithmeticSelf::Span(other) => self
                .0
                .checked_add(other.0)
                .map_err(|e| PyErr::new::<pyo3::exceptions::PyOverflowError, _>(format!("{e}"))),
            RyDeltaArithmeticSelf::SignedDuration(other) => self
                .0
                .checked_add(other.0)
                .map_err(|e| PyErr::new::<pyo3::exceptions::PyOverflowError, _>(format!("{e}"))),
            RyDeltaArithmeticSelf::Duration(other) => self
                .0
                .checked_add(other.0)
                .map_err(|e| PyErr::new::<pyo3::exceptions::PyOverflowError, _>(format!("{e}"))),
        }?;
        Ok(Self::from(t))
    }

    // ----------------------------
    // incompatible with `frozen`
    // ----------------------------
    // fn __iadd__(&mut self, _py: Python<'_>, other: RyDeltaArithmeticSelf) -> PyResult<()> {
    //     let t = match other {
    //         RyDeltaArithmeticSelf::Span(other) => self
    //             .0
    //             .checked_add(other.0)
    //             .map_err(|e| PyErr::new::<pyo3::exceptions::PyOverflowError, _>(format!("{e}"))),
    //         RyDeltaArithmeticSelf::SignedDuration(other) => self
    //             .0
    //             .checked_add(other.0)
    //             .map_err(|e| PyErr::new::<pyo3::exceptions::PyOverflowError, _>(format!("{e}"))),
    //         RyDeltaArithmeticSelf::Duration(other) => self
    //             .0
    //             .checked_add(other.0)
    //             .map_err(|e| PyErr::new::<pyo3::exceptions::PyOverflowError, _>(format!("{e}"))),
    //     }?;
    //     self.0 = t;
    //     Ok(())
    // }

    fn __sub__<'py>(
        &self,
        py: Python<'py>,
        other: RyDateTimeArithmeticSub,
    ) -> PyResult<Bound<'py, PyAny>> {
        match other {
            #[expect(clippy::arithmetic_side_effects)]
            RyDateTimeArithmeticSub::DateTime(other) => {
                let span = self.0 - other.0;
                let obj = RySpan::from(span).into_pyobject(py).map(Bound::into_any)?;
                Ok(obj)
            }
            RyDateTimeArithmeticSub::Delta(other) => {
                let t = match other {
                    RyDeltaArithmeticSelf::Span(other) => self.0.checked_sub(other.0),
                    RyDeltaArithmeticSelf::SignedDuration(other) => self.0.checked_sub(other.0),
                    RyDeltaArithmeticSelf::Duration(other) => self.0.checked_sub(other.0),
                }
                .map_err(|e| PyErr::new::<pyo3::exceptions::PyOverflowError, _>(format!("{e}")))?;
                Ok(Self::from(t).into_pyobject(py)?.into_any())
            }
        }
    }

    // ----------------------------
    // incompatible with `frozen`
    // ----------------------------
    // fn __isub__(&mut self, _py: Python<'_>, other: RyDeltaArithmeticSelf) -> PyResult<()> {
    //     let t = match other {
    //         RyDeltaArithmeticSelf::Span(other) => self.0.checked_sub(other.0),
    //         RyDeltaArithmeticSelf::SignedDuration(other) => self.0.checked_sub(other.0),
    //         RyDeltaArithmeticSelf::Duration(other) => self.0.checked_sub(other.0),
    //     }
    //     .map_err(|e| PyErr::new::<pyo3::exceptions::PyOverflowError, _>(format!("{e}")))?;
    //     self.0 = t;
    //     Ok(())
    // }

    fn checked_add(&self, py: Python<'_>, other: RyDeltaArithmeticSelf) -> PyResult<Self> {
        self.__add__(py, other)
    }
    fn checked_sub<'py>(
        &self,
        py: Python<'py>,
        other: RyDateTimeArithmeticSub,
    ) -> PyResult<Bound<'py, PyAny>> {
        self.__sub__(py, other)
    }

    fn saturating_add<'py>(
        &self,
        py: Python<'py>,
        other: RyDeltaArithmeticSelf,
    ) -> PyResult<Bound<'py, PyAny>> {
        let t = match other {
            RyDeltaArithmeticSelf::Span(other) => self.0.saturating_add(other.0),
            RyDeltaArithmeticSelf::SignedDuration(other) => self.0.saturating_add(other.0),
            RyDeltaArithmeticSelf::Duration(other) => self.0.saturating_add(other.0),
        };
        Ok(Self::from(t).into_pyobject(py)?.into_any())
    }

    fn saturating_sub<'py>(
        &self,
        py: Python<'py>,
        other: RyDeltaArithmeticSelf,
    ) -> PyResult<Bound<'py, PyAny>> {
        let t = match other {
            RyDeltaArithmeticSelf::Span(other) => self.0.saturating_sub(other.0),
            RyDeltaArithmeticSelf::SignedDuration(other) => self.0.saturating_sub(other.0),
            RyDeltaArithmeticSelf::Duration(other) => self.0.saturating_sub(other.0),
        };
        Ok(Self::from(t).into_pyobject(py)?.into_any())
    }

    fn time(&self) -> RyTime {
        RyTime::from(self.0.time())
    }

    fn date(&self) -> RyDate {
        RyDate::from(self.0.date())
    }

    pub(crate) fn in_tz(&self, tz: &str) -> PyResult<RyZoned> {
        self.0
            .in_tz(tz)
            .map(RyZoned::from)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e}")))
    }

    fn intz(&self, py: Python, tz: &str) -> PyResult<RyZoned> {
        deprecation_warning_intz(py)?;
        self.in_tz(tz)
    }

    fn iso_week_date(&self) -> RyISOWeekDate {
        RyISOWeekDate::from(self.0.iso_week_date())
    }

    fn to_zoned(&self, tz: RyTimeZone) -> PyResult<RyZoned> {
        self.0
            .to_zoned(tz.into())
            .map(RyZoned::from)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e}")))
    }

    fn first_of_month(&self) -> RyDateTime {
        RyDateTime::from(self.0.first_of_month())
    }
    fn last_of_month(&self) -> RyDateTime {
        RyDateTime::from(self.0.last_of_month())
    }

    fn to_py(&self) -> DateTime {
        self.to_pydatetime()
    }

    fn to_pydatetime(&self) -> DateTime {
        self.0
    }

    fn to_pydate(&self) -> Date {
        self.0.date()
    }

    fn to_pytime(&self) -> Time {
        self.0.time()
    }

    #[classmethod]
    fn from_pydatetime(_cls: &Bound<'_, PyType>, d: DateTime) -> Self {
        Self::from(d)
    }

    fn series(&self, period: &RySpan) -> RyDateTimeSeries {
        RyDateTimeSeries {
            series: self.0.series(period.0),
        }
    }

    fn asdict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        let dict = PyDict::new(py);
        dict.set_item(intern!(py, "year"), self.0.year())?;
        dict.set_item(intern!(py, "month"), self.0.month())?;
        dict.set_item(intern!(py, "day"), self.0.day())?;
        dict.set_item(intern!(py, "hour"), self.0.hour())?;
        dict.set_item(intern!(py, "minute"), self.0.minute())?;
        dict.set_item(intern!(py, "second"), self.0.second())?;
        dict.set_item(intern!(py, "subsec_nanosecond"), self.0.subsec_nanosecond())?;

        Ok(dict)
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
        let mut dt_round = DateTimeRound::new();
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
            .map(RyDateTime::from)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e}")))
    }

    fn _round(&self, dt_round: &RyDateTimeRound) -> PyResult<Self> {
        self.0
            .round(dt_round.round)
            .map(RyDateTime::from)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e}")))
    }

    fn day_of_year(&self) -> i16 {
        self.0.day_of_year()
    }

    fn day_of_year_no_leap(&self) -> Option<i16> {
        self.0.day_of_year_no_leap()
    }

    fn days_in_month(&self) -> i8 {
        self.0.days_in_month()
    }

    fn days_in_year(&self) -> i16 {
        self.0.days_in_year()
    }

    fn duration_since(&self, dt: &Self) -> RySignedDuration {
        self.0.duration_since(dt.0).into()
    }

    fn duration_until(&self, dt: &Self) -> RySignedDuration {
        self.0.duration_until(dt.0).into()
    }

    /// Returns the end of the day DateTime
    fn end_of_day(&self) -> RyDateTime {
        RyDateTime::from(self.0.end_of_day())
    }

    /// Return the era year as a tuple (era, year)
    fn era_year<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let era_year = JiffEraYear(self.0.era_year());
        let obj = era_year.into_pyobject(py)?;
        Ok(obj.into_any())
    }

    fn first_of_year(&self) -> RyDateTime {
        RyDateTime::from(self.0.first_of_year())
    }

    #[classmethod]
    fn from_parts(_cls: &Bound<'_, PyType>, date: &RyDate, time: &RyTime) -> Self {
        Self::from(DateTime::from_parts(date.0, time.0))
    }
    fn in_leap_year(&self) -> bool {
        self.0.in_leap_year()
    }
    fn last_of_year(&self) -> RyDateTime {
        RyDateTime::from(self.0.last_of_year())
    }
    fn start_of_day(&self) -> RyDateTime {
        Self::from(self.0.start_of_day())
    }
    fn strftime(&self, format: &str) -> String {
        self.0.strftime(format).to_string()
    }

    #[classmethod]
    fn strptime(_cls: &Bound<'_, PyType>, s: &str, format: &str) -> PyResult<Self> {
        DateTime::strptime(s, format)
            .map(RyDateTime::from)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e}")))
    }

    #[pyo3(
       signature = (datetime, *, smallest=None, largest = None, mode = None, increment = None),
    )]
    fn since(
        &self,
        datetime: DateTimeDifferenceArg,
        smallest: Option<JiffUnit>,
        largest: Option<JiffUnit>,
        mode: Option<JiffRoundMode>,
        increment: Option<i64>,
    ) -> PyResult<RySpan> {
        let dt_diff = datetime.build(smallest, largest, mode, increment);
        self.0
            .since(dt_diff)
            .map(RySpan::from)
            .map_err(map_py_value_err)
    }
    #[pyo3(
       signature = (datetime, *, smallest=None, largest = None, mode = None, increment = None),
    )]
    fn until(
        &self,
        datetime: DateTimeDifferenceArg,
        smallest: Option<JiffUnit>,
        largest: Option<JiffUnit>,
        mode: Option<JiffRoundMode>,
        increment: Option<i64>,
    ) -> PyResult<RySpan> {
        let dt_diff = datetime.build(smallest, largest, mode, increment);
        self.0
            .until(dt_diff)
            .map(RySpan::from)
            .map_err(map_py_value_err)
    }

    fn _since(&self, other: &RyDateTimeDifference) -> PyResult<RySpan> {
        self.0
            .since(other.0)
            .map(RySpan::from)
            .map_err(map_py_value_err)
    }

    fn _until(&self, other: &RyDateTimeDifference) -> PyResult<RySpan> {
        self.0
            .until(other.0)
            .map(RySpan::from)
            .map_err(map_py_value_err)
    }

    fn tomorrow(&self) -> PyResult<Self> {
        self.0
            .tomorrow()
            .map(RyDateTime::from)
            .map_err(map_py_value_err)
    }

    fn yesterday(&self) -> PyResult<Self> {
        self.0
            .yesterday()
            .map(RyDateTime::from)
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
}

impl Display for RyDateTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[pyclass(name = "DateTimeSeries", module = "ry.ryo3")]
pub struct RyDateTimeSeries {
    pub(crate) series: jiff::civil::DateTimeSeries,
}

#[pymethods]
impl RyDateTimeSeries {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__(mut slf: PyRefMut<'_, Self>) -> Option<RyDateTime> {
        slf.series.next().map(RyDateTime::from)
    }

    fn take(mut slf: PyRefMut<'_, Self>, n: usize) -> Vec<RyDateTime> {
        slf.series
            .borrow_mut()
            .take(n)
            .map(RyDateTime::from)
            .collect()
    }
}

#[derive(Debug, Clone, FromPyObject)]
pub(crate) enum RyDateTimeArithmeticSub {
    DateTime(RyDateTime),
    Delta(RyDeltaArithmeticSelf),
}
