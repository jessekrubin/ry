use crate::constants::DATETIME_PARSER;
use crate::difference::{DateDifferenceArg, RyDateDifference};
use crate::errors::{map_py_overflow_err, map_py_value_err};
use crate::ry_datetime::RyDateTime;
use crate::ry_iso_week_date::RyISOWeekDate;
use crate::ry_signed_duration::RySignedDuration;
use crate::ry_span::RySpan;
use crate::ry_time::RyTime;
use crate::ry_timezone::RyTimeZone;
use crate::ry_zoned::RyZoned;
use crate::series::RyDateSeries;
use crate::spanish::Spanish;
use crate::{JiffEra, JiffEraYear, JiffRoundMode, JiffUnit, JiffWeekday};
use jiff::Zoned;
use jiff::civil::{Date, Weekday};
use pyo3::prelude::*;
use pyo3::pyclass::CompareOp;
use pyo3::types::{PyDict, PyTuple};
use pyo3::{IntoPyObject, IntoPyObjectExt};
use ryo3_macro_rules::{any_repr, py_type_err, py_value_error};
use std::fmt::Display;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::ops::Sub;

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[pyclass(name = "Date", frozen)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
pub struct RyDate(pub(crate) Date);

#[pymethods]
impl RyDate {
    #[new]
    pub(crate) fn py_new(year: i16, month: i8, day: i8) -> PyResult<Self> {
        Date::new(year, month, day)
            .map(Self::from)
            .map_err(|e| py_value_error!("{e} (year={year}, month={month}, day={day})",))
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn MIN() -> Self {
        Self(Date::MIN)
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn MAX() -> Self {
        Self(Date::MAX)
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn ZERO() -> Self {
        Self(Date::ZERO)
    }

    #[staticmethod]
    fn today() -> Self {
        let z = jiff::civil::Date::from(Zoned::now());
        Self::from(z)
    }

    #[staticmethod]
    fn from_str(input: &str) -> PyResult<Self> {
        DATETIME_PARSER
            .parse_date(input)
            .map(Self::from)
            .map_err(map_py_value_err)
    }

    #[staticmethod]
    fn parse(input: &str) -> PyResult<Self> {
        Self::from_str(input)
    }

    #[pyo3(signature = (hour, minute, second, nanosecond=0))]
    pub(crate) fn at(&self, hour: i8, minute: i8, second: i8, nanosecond: i32) -> RyDateTime {
        RyDateTime::from(self.0.at(hour, minute, second, nanosecond))
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

    fn __hash__(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.0.hash(&mut hasher);
        hasher.finish()
    }

    #[expect(clippy::wrong_self_convention)]
    fn to_datetime(&self, time: &RyTime) -> RyDateTime {
        RyDateTime::from(self.0.to_datetime(time.0))
    }

    #[expect(clippy::wrong_self_convention)]
    fn to_zoned(&self, tz: RyTimeZone) -> PyResult<RyZoned> {
        self.0
            .to_zoned(tz.into())
            .map(RyZoned::from)
            .map_err(map_py_value_err)
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

    fn isoformat(&self) -> String {
        self.0.to_string()
    }

    #[pyo3(
        warn(
            message = "obj.string() is deprecated, use `obj.to_string()` or `str(obj)` [remove in 0.0.60]",
            category = pyo3::exceptions::PyDeprecationWarning
      )
    )]
    fn string(&self) -> String {
        self.__str__()
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

    fn __getnewargs__<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyTuple>> {
        PyTuple::new(
            py,
            vec![
                self.year().into_pyobject(py)?,
                self.month().into_pyobject(py)?,
                self.day().into_pyobject(py)?,
            ],
        )
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
        signature = (
            *,
            year=None,
            era_year=None,
            month=None,
            day=None,
            day_of_year=None,
            day_of_year_no_leap=None,
        )
    )]
    fn replace(
        &self,
        year: Option<i16>,
        era_year: Option<(i16, JiffEra)>,
        month: Option<i8>,
        day: Option<i8>,
        day_of_year: Option<i16>,
        day_of_year_no_leap: Option<i16>,
    ) -> PyResult<Self> {
        let mut builder = self.0.with();
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
        // finally build, mapping any error back to Python
        builder.build().map(Self::from).map_err(map_py_value_err)
    }

    #[staticmethod]
    fn from_pydate(d: Date) -> Self {
        Self(d)
    }

    #[expect(clippy::wrong_self_convention)]
    fn to_py(&self) -> Date {
        self.to_pydate()
    }

    #[expect(clippy::wrong_self_convention)]
    fn to_pydate(&self) -> Date {
        self.0
    }

    fn astuple<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyTuple>> {
        let year_any = self.0.year().into_pyobject(py)?.into_any();
        let month_any = self.0.month().into_pyobject(py)?.into_any();
        let day_any = self.0.day().into_pyobject(py)?.into_any();
        let parts = vec![year_any, month_any, day_any];
        PyTuple::new(py, parts)
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

    #[expect(clippy::wrong_self_convention)]
    fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        use crate::interns;
        let dict = PyDict::new(py);
        dict.set_item(interns::year(py), self.0.year())?;
        dict.set_item(interns::month(py), self.0.month())?;
        dict.set_item(interns::day(py), self.0.day())?;
        Ok(dict)
    }

    fn series(&self, period: &RySpan) -> PyResult<RyDateSeries> {
        if period.0.is_zero() {
            Err(py_value_error!("period cannot be zero"))
        } else {
            Ok(RyDateSeries::from(self.0.series(period.0)))
        }
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

    fn duration_since(&self, other: &Self) -> RySignedDuration {
        RySignedDuration::from(self.0.duration_since(other.0))
    }
    fn duration_until(&self, other: &Self) -> RySignedDuration {
        RySignedDuration::from(self.0.duration_until(other.0))
    }

    fn era_year(&self) -> JiffEraYear {
        JiffEraYear(self.0.era_year())
    }

    fn first_of_month(&self) -> Self {
        Self::from(self.0.first_of_month())
    }

    fn first_of_year(&self) -> Self {
        Self::from(self.0.first_of_year())
    }

    fn in_leap_year(&self) -> bool {
        self.0.in_leap_year()
    }

    fn last_of_month(&self) -> Self {
        Self::from(self.0.last_of_month())
    }

    fn last_of_year(&self) -> Self {
        Self::from(self.0.last_of_year())
    }

    fn tomorrow(&self) -> PyResult<Self> {
        self.0.tomorrow().map(From::from).map_err(map_py_value_err)
    }

    fn yesterday(&self) -> PyResult<Self> {
        self.0.yesterday().map(From::from).map_err(map_py_value_err)
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
        Date::strptime(fmt, s)
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

    #[pyo3(
       signature = (d, *, smallest=None, largest = None, mode = None, increment = None),
    )]
    fn since(
        &self,
        d: DateDifferenceArg,
        smallest: Option<JiffUnit>,
        largest: Option<JiffUnit>,
        mode: Option<JiffRoundMode>,
        increment: Option<i64>,
    ) -> PyResult<RySpan> {
        let dt_diff = d.build(smallest, largest, mode, increment);
        self.0
            .since(dt_diff)
            .map(RySpan::from)
            .map_err(map_py_value_err)
    }

    #[pyo3(
       signature = (d, *, smallest=None, largest = None, mode = None, increment = None),
    )]
    fn until(
        &self,
        d: DateDifferenceArg,
        smallest: Option<JiffUnit>,
        largest: Option<JiffUnit>,
        mode: Option<JiffRoundMode>,
        increment: Option<i64>,
    ) -> PyResult<RySpan> {
        let dt_diff = d.build(smallest, largest, mode, increment);
        self.0
            .until(dt_diff)
            .map(RySpan::from)
            .map_err(map_py_value_err)
    }

    fn _since(&self, other: &RyDateDifference) -> PyResult<RySpan> {
        self.0
            .since(other.diff)
            .map(RySpan::from)
            .map_err(map_py_value_err)
    }

    fn _until(&self, other: &RyDateDifference) -> PyResult<RySpan> {
        self.0
            .until(other.diff)
            .map(RySpan::from)
            .map_err(map_py_value_err)
    }

    #[staticmethod]
    fn from_iso_week_date(iso_week_date: &RyISOWeekDate) -> Self {
        Self::from(iso_week_date.0.date())
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

    fn iso_week_date(&self) -> RyISOWeekDate {
        self.0.iso_week_date().into()
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
        // } else if let Ok(v) = value.cast::<PyInt>() {
        //     let i = v.extract::<i64>()?;
        //     let ts = if (-20_000_000_000..=20_000_000_000).contains(&i) {
        //         jiff::Timestamp::from_second(i)
        //     } else {
        //         jiff::Timestamp::from_millisecond(i)
        //     }
        //     .map_err(map_py_value_err)?;
        //     let zdt = ts.to_zoned(TimeZone::UTC);
        //     let date = zdt.date();
        //     Self::from(date).into_bound_py_any(py)
        } else if let Ok(d) = value.extract::<RyDateTime>() {
            let dt = d.date();
            dt.into_bound_py_any(py)
        } else if let Ok(d) = value.extract::<RyZoned>() {
            let dt = d.date();
            dt.into_bound_py_any(py)
        } else if let Ok(d) = value.extract::<Date>() {
            Self::from_pydate(d).into_bound_py_any(py)
        } else {
            let valtype = any_repr!(value);
            py_type_err!("Date conversion error: {valtype}",)
        }
    }

    /// Try to create a Date from a variety of python objects
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

impl Display for RyDate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Date(year={}, month={}, day={})",
            self.year(),
            self.month(),
            self.day()
        )
    }
}

impl From<Date> for RyDate {
    fn from(value: Date) -> Self {
        Self(value)
    }
}
