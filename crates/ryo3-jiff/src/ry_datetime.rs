use crate::delta_arithmetic_self::RyDeltaArithmeticSelf;
use crate::errors::map_py_value_err;
use crate::jiff_types::JiffDateTime;
use crate::ry_signed_duration::RySignedDuration;
use crate::ry_span::RySpan;
use crate::ry_time::RyTime;
use crate::ry_timezone::RyTimeZone;
use crate::ry_zoned::RyZoned;
use crate::RyDate;
use jiff::civil::{DateTime, Weekday};
use jiff::Zoned;
use pyo3::basic::CompareOp;
use pyo3::types::{PyAnyMethods, PyDate, PyDateTime, PyDict, PyDictMethods, PyTuple, PyType};
use pyo3::{
    intern, pyclass, pymethods, Bound, FromPyObject, IntoPyObject, IntoPyObjectExt, PyAny, PyErr,
    PyRef, PyRefMut, PyResult, Python,
};
use ryo3_macros::err_py_not_impl;
use std::fmt::Display;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::str::FromStr;

#[derive(Debug, Clone)]
#[pyclass(name = "DateTime", module = "ryo3")]
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
    pub fn new(
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

    #[allow(non_snake_case)]
    #[classattr]
    fn MIN() -> Self {
        Self(DateTime::MIN)
    }

    #[allow(non_snake_case)]
    #[classattr]
    fn MAX() -> Self {
        Self(DateTime::MAX)
    }
    #[allow(non_snake_case)]
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

    fn __richcmp__(&self, other: &Self, op: CompareOp) -> PyResult<bool> {
        match op {
            CompareOp::Eq => Ok(self.0 == other.0),
            CompareOp::Ne => Ok(self.0 != other.0),
            CompareOp::Lt => Ok(self.0 < other.0),
            CompareOp::Le => Ok(self.0 <= other.0),
            CompareOp::Gt => Ok(self.0 > other.0),
            CompareOp::Ge => Ok(self.0 >= other.0),
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

    fn __iadd__(&mut self, _py: Python<'_>, other: RyDeltaArithmeticSelf) -> PyResult<()> {
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
        self.0 = t;
        Ok(())
    }
    fn __sub__<'py>(
        &self,
        py: Python<'py>,
        other: RyDateTimeArithmeticSub,
    ) -> PyResult<Bound<'py, PyAny>> {
        match other {
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

    fn __isub__(&mut self, _py: Python<'_>, other: RyDeltaArithmeticSelf) -> PyResult<()> {
        let t = match other {
            RyDeltaArithmeticSelf::Span(other) => self.0.checked_sub(other.0),
            RyDeltaArithmeticSelf::SignedDuration(other) => self.0.checked_sub(other.0),
            RyDeltaArithmeticSelf::Duration(other) => self.0.checked_sub(other.0),
        }
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyOverflowError, _>(format!("{e}")))?;
        self.0 = t;
        Ok(())
    }

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

    fn intz(&self, tz: &str) -> PyResult<RyZoned> {
        self.0
            .intz(tz)
            .map(RyZoned::from)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e}")))
    }

    fn to_zoned(&self, tz: RyTimeZone) -> PyResult<RyZoned> {
        self.0
            .to_zoned(tz.0)
            .map(RyZoned::from)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e}")))
    }

    fn first_of_month(&self) -> RyDateTime {
        RyDateTime::from(self.0.first_of_month())
    }
    fn last_of_month(&self) -> RyDateTime {
        RyDateTime::from(self.0.last_of_month())
    }

    fn to_pydatetime<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDateTime>> {
        let jiff_datetime = JiffDateTime(self.0);
        jiff_datetime.into_pyobject(py)
    }

    #[classmethod]
    fn from_pydatetime(_cls: &Bound<'_, PyType>, d: &Bound<'_, PyDate>) -> PyResult<Self> {
        let jiff_datetime: JiffDateTime = d.extract()?;
        Ok(Self::from(jiff_datetime.0))
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

    fn round(&self, option: crate::internal::IntoDateTimeRound) -> PyResult<Self> {
        self.0
            .round(option)
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

    fn end_of_day(&self) -> RyDateTime {
        RyDateTime::from(self.0.end_of_day())
    }
    fn era_year<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyTuple>> {
        let (y, e) = self.0.era_year();

        let era_str_pyobj = match e {
            jiff::civil::Era::BCE => intern!(py, "BCE"),
            jiff::civil::Era::CE => intern!(py, "CE"),
        };
        let year_py = y.into_py_any(py)?;
        let era_str = era_str_pyobj.into_py_any(py)?;

        let pyobjs_vec = vec![year_py, era_str];
        PyTuple::new(py, pyobjs_vec)
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
    fn nth_weekday(&self) -> PyResult<()> {
        err_py_not_impl!()
    }
    fn nth_weekday_of_month(&self) -> PyResult<()> {
        err_py_not_impl!()
    }
    fn since(&self) -> PyResult<()> {
        err_py_not_impl!()
    }
    fn until(&self) -> PyResult<()> {
        err_py_not_impl!()
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

#[pyclass]
#[pyo3(name = "DateTimeSeries", module = "ryo3")]
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
}

#[derive(Debug, Clone, FromPyObject)]
pub(crate) enum RyDateTimeArithmeticSub {
    DateTime(RyDateTime),
    Delta(RyDeltaArithmeticSelf),
}
