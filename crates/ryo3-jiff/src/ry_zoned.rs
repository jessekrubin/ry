use crate::delta_arithmetic_self::RyDeltaArithmeticSelf;
use crate::errors::map_py_value_err;
use crate::internal::IntoDateTimeRound;
use crate::ry_datetime::RyDateTime;
use crate::ry_offset::RyOffset;
use crate::ry_signed_duration::RySignedDuration;
use crate::ry_span::RySpan;
use crate::ry_time::RyTime;
use crate::ry_timestamp::RyTimestamp;
use crate::ry_timezone::RyTimeZone;
use crate::{JiffZoned, RyDate};
use jiff::civil::Weekday;
use jiff::Zoned;
use pyo3::basic::CompareOp;
use pyo3::prelude::PyAnyMethods;
use pyo3::types::{PyDate, PyDateTime, PyTuple, PyType};
use pyo3::{
    intern, pyclass, pymethods, Bound, FromPyObject, IntoPyObject, IntoPyObjectExt, PyAny, PyErr,
    PyResult, Python,
};
use ryo3_macros::err_py_not_impl;
use std::fmt::Display;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::str::FromStr;

#[derive(Debug, Clone)]
#[pyclass(name = "ZonedDateTime", module = "ryo3")]
pub struct RyZoned(pub(crate) Zoned);

#[pymethods]
impl RyZoned {
    #[new]
    #[pyo3(signature = (timestamp, time_zone))]
    pub fn new(timestamp: &RyTimestamp, time_zone: RyTimeZone) -> PyResult<Self> {
        let ts = timestamp.0;
        let tz = time_zone.0;
        Ok(RyZoned::from(Zoned::new(ts, tz)))
    }

    #[classmethod]
    #[pyo3(signature = (tz=None))]
    fn now(_cls: &Bound<'_, PyType>, tz: Option<&str>) -> PyResult<Self> {
        if let Some(tz) = tz {
            Zoned::now()
                .intz(tz)
                .map(RyZoned::from)
                .map_err(map_py_value_err)
        } else {
            Ok(Self::from(Zoned::now()))
        }
    }

    #[classmethod]
    fn utcnow(_cls: &Bound<'_, PyType>) -> PyResult<Self> {
        Zoned::now()
            .intz("UTC")
            .map(RyZoned::from)
            .map_err(map_py_value_err)
    }

    #[classmethod]
    fn parse(_cls: &Bound<'_, PyType>, s: &str) -> PyResult<Self> {
        Zoned::from_str(s)
            .map(RyZoned::from)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e}")))
    }

    #[classmethod]
    fn strptime(_cls: &Bound<'_, PyType>, format: &str, input: &str) -> PyResult<Self> {
        Zoned::strptime(format, input)
            .map(RyZoned::from)
            .map_err(map_py_value_err)
    }

    fn strftime(&self, format: &str) -> String {
        self.0.strftime(format).to_string()
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

    fn to_pydatetime<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDateTime>> {
        let new_zoned = JiffZoned(self.0.clone()); // todo: remove clone
        new_zoned.into_pyobject(py)
    }

    #[classmethod]
    fn from_pydatetime(_cls: &Bound<'_, PyType>, d: &Bound<'_, PyDate>) -> PyResult<Self> {
        let jiff_datetime: JiffZoned = d.extract()?;
        Ok(Self::from(jiff_datetime.0))
    }

    fn intz(&self, tz: &str) -> PyResult<Self> {
        self.0
            .intz(tz)
            .map(RyZoned::from)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e}")))
    }

    fn astimezone(&self, tz: &str) -> PyResult<Self> {
        self.intz(tz)
    }

    fn inutc(&self) -> PyResult<Self> {
        self.0
            .intz("UTC")
            .map(RyZoned::from)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e}")))
    }

    fn to_rfc2822(&self) -> PyResult<String> {
        jiff::fmt::rfc2822::to_string(&self.0)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e}")))
    }

    #[staticmethod]
    fn from_rfc2822(s: &str) -> PyResult<Self> {
        jiff::fmt::rfc2822::parse(s)
            .map(RyZoned::from)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e}")))
    }

    fn __sub__<'py>(
        &self,
        py: Python<'py>,
        other: RyZonedArithmeticSub,
    ) -> PyResult<Bound<'py, PyAny>> {
        match other {
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

    fn __isub__(&mut self, _py: Python<'_>, other: RyDeltaArithmeticSelf) -> PyResult<()> {
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
        self.0 = t;
        Ok(())
    }

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

    fn __iadd__(&mut self, _py: Python<'_>, other: RyDeltaArithmeticSelf) -> PyResult<()> {
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
        self.0 = t;
        Ok(())
    }
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

    fn saturating_add(&self, _py: Python<'_>, other: RyDeltaArithmeticSelf) -> PyResult<Self> {
        let t = match other {
            RyDeltaArithmeticSelf::Span(other) => self.0.saturating_add(other.0),
            RyDeltaArithmeticSelf::SignedDuration(other) => self.0.saturating_add(other.0),
            RyDeltaArithmeticSelf::Duration(other) => self.0.saturating_add(other.0),
        };
        Ok(Self::from(t))
    }
    fn saturating_sub(&self, _py: Python<'_>, other: RyDeltaArithmeticSelf) -> PyResult<Self> {
        let t = match other {
            RyDeltaArithmeticSelf::Span(other) => self.0.saturating_sub(other.0),
            RyDeltaArithmeticSelf::SignedDuration(other) => self.0.saturating_sub(other.0),
            RyDeltaArithmeticSelf::Duration(other) => self.0.saturating_sub(other.0),
        };
        Ok(Self::from(t))
    }

    fn timezone(&self) -> RyTimeZone {
        RyTimeZone::from(self.0.time_zone())
    }

    fn round(&self, option: IntoDateTimeRound) -> PyResult<Self> {
        self.0
            .round(option)
            .map(RyZoned::from)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e}")))
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

    fn nth_weekday(&self, _nth: i32, _weekday: u8) -> PyResult<Self> {
        err_py_not_impl!()
    }

    fn nth_weekday_of_month(&self) -> PyResult<()> {
        err_py_not_impl!()
    }
    fn offset(&self) -> RyOffset {
        self.0.offset().into()
    }

    fn since(&self) -> PyResult<()> {
        // self.0.since()
        err_py_not_impl!()
    }
    fn start_of_day(&self) -> PyResult<Self> {
        self.0
            .start_of_day()
            .map(RyZoned::from)
            .map_err(map_py_value_err)
    }

    fn time_zone(&self) -> PyResult<RyTimeZone> {
        Ok(RyTimeZone::from(self.0.time_zone()))
    }

    fn until(&self) -> PyResult<()> {
        err_py_not_impl!()
    }

    fn with_time_zone(&self, tz: &RyTimeZone) -> RyZoned {
        self.0.with_time_zone(tz.0.clone()).into()
    }

    // python doesnt allow for `with` as a method name as it is a reserved keyword
    // fn with(&self) -> PyResult<()> {
    //     err_py_not_impl!()
    // }
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
