use crate::delta_arithmetic_self::RyDeltaArithmeticSelf;
use crate::errors::map_py_value_err;
use crate::ry_datetime::RyDateTime;
use crate::ry_signed_duration::RySignedDuration;
use crate::ry_span::RySpan;
use crate::ry_time::RyTime;
use crate::ry_timezone::RyTimeZone;
use crate::ry_zoned::RyZoned;
use crate::JiffDate;
use jiff::civil::{Date, Weekday};
use jiff::Zoned;
use pyo3::basic::CompareOp;
use pyo3::types::{PyAnyMethods, PyDate, PyDict, PyDictMethods, PyTuple, PyType};
use pyo3::{
    intern, pyclass, pymethods, Bound, FromPyObject, IntoPyObject, IntoPyObjectExt, PyAny, PyErr,
    PyRef, PyRefMut, PyResult, Python,
};
use ryo3_macros::err_py_not_impl;
use ryo3_std::PyDuration;
use std::fmt::Display;
use std::hash::{DefaultHasher, Hash, Hasher};

#[derive(Debug, Clone)]
#[pyclass(name = "Date", module = "ryo3")]
pub struct RyDate(pub(crate) Date);

#[pymethods]
impl RyDate {
    #[new]
    pub fn new(year: i16, month: i8, day: i8) -> PyResult<Self> {
        Date::new(year, month, day).map(RyDate::from).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                "{e} (year={year}, month={month}, day={day})",
            ))
        })
    }

    #[allow(non_snake_case)]
    #[classattr]
    fn MIN() -> Self {
        Self(Date::MIN)
    }

    #[allow(non_snake_case)]
    #[classattr]
    fn MAX() -> Self {
        Self(Date::MAX)
    }

    #[allow(non_snake_case)]
    #[classattr]
    fn ZERO() -> Self {
        Self(Date::ZERO)
    }

    #[classmethod]
    fn today(_cls: &Bound<'_, PyType>) -> Self {
        let z = jiff::civil::Date::from(Zoned::now());
        Self::from(z)
    }
    fn at(&self, hour: i8, minute: i8, second: i8, subsec_nanosecond: i32) -> RyDateTime {
        RyDateTime::from(self.0.at(hour, minute, second, subsec_nanosecond))
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
    fn to_datetime(&self, time: &RyTime) -> RyDateTime {
        RyDateTime::from(self.0.to_datetime(time.0))
    }

    fn to_zoned(&self, tz: RyTimeZone) -> PyResult<RyZoned> {
        self.0
            .to_zoned(tz.0)
            .map(RyZoned::from)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e}")))
    }

    fn __richcmp__(&self, other: &RyDate, op: CompareOp) -> PyResult<bool> {
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
        self.__str__()
    }

    fn __str__(&self) -> String {
        self.0.to_string()
    }

    fn __repr__(&self) -> String {
        format!(
            "Date(year={}, month={}, day={})",
            self.year(),
            self.month(),
            self.day()
        )
    }

    fn __sub__<'py>(
        &self,
        py: Python<'py>,
        other: RyDateArithmeticSub,
    ) -> PyResult<Bound<'py, PyAny>> {
        match other {
            RyDateArithmeticSub::Time(other) => {
                let span = self.0 - other.0;
                let obj = RySpan::from(span).into_pyobject(py).map(Bound::into_any)?;
                Ok(obj)
            }
            RyDateArithmeticSub::Delta(other) => {
                let t = match other {
                    RyDeltaArithmeticSelf::Span(other) => self.0 - other.0,
                    RyDeltaArithmeticSelf::SignedDuration(other) => self.0 - other.0,
                    RyDeltaArithmeticSelf::Duration(other) => self.0 - other.0,
                };
                Ok(RyDate::from(t).into_pyobject(py)?.into_any())
            }
        }
    }
    fn checked_sub<'py>(
        &self,
        py: Python<'py>,
        other: RyDateArithmeticSub,
    ) -> PyResult<Bound<'py, PyAny>> {
        self.__sub__(py, other)
    }

    fn __isub__(&mut self, _py: Python<'_>, other: RyDeltaArithmeticSelf) -> PyResult<()> {
        let t = match other {
            RyDeltaArithmeticSelf::Span(other) => self.0 - other.0,
            RyDeltaArithmeticSelf::SignedDuration(other) => self.0 - other.0,
            RyDeltaArithmeticSelf::Duration(other) => self.0 - other.0,
        };
        self.0 = t;
        Ok(())
    }

    fn __add__(&self, other: &Bound<'_, PyAny>) -> PyResult<Self> {
        if let Ok(date) = other.downcast::<RySpan>() {
            let other = date.extract::<RySpan>()?;
            let t = self.0 + other.0;
            return Ok(RyDate::from(t));
        }
        if let Ok(signed_dur) = other.downcast::<RySignedDuration>() {
            let other = signed_dur.extract::<RySignedDuration>()?;
            let t = self.0 + other.0;
            return Ok(RyDate::from(t));
        }
        if let Ok(date) = other.downcast::<PyDuration>() {
            let other = date.extract::<PyDuration>()?;
            let t = self.0 + other.0;
            return Ok(RyDate::from(t));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "unsupported operand type(s) for +: 'Date' and 'other'",
        ))
    }
    fn checked_add(&self, other: &Bound<'_, PyAny>) -> PyResult<Self> {
        self.__add__(other)
    }

    fn saturating_add<'py>(&self, _py: Python<'py>, other: &Bound<'py, PyAny>) -> PyResult<Self> {
        if let Ok(date) = other.downcast::<RySpan>() {
            let other = date.extract::<RySpan>()?;
            return Ok(RyDate::from(self.0.saturating_add(other.0)));
        }
        if let Ok(signed_dur) = other.downcast::<RySignedDuration>() {
            let other = signed_dur.extract::<RySignedDuration>()?;
            return Ok(RyDate::from(self.0.saturating_add(other.0)));
        }
        if let Ok(date) = other.downcast::<PyDuration>() {
            let other = date.extract::<PyDuration>()?;
            return Ok(RyDate::from(self.0.saturating_add(other.0)));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "unsupported operand type(s) for +: 'Date' and 'other'",
        ))
    }
    fn saturating_sub<'py>(&self, _py: Python<'py>, other: &Bound<'py, PyAny>) -> PyResult<Self> {
        if let Ok(date) = other.downcast::<RySpan>() {
            let other = date.extract::<RySpan>()?;
            return Ok(RyDate::from(self.0.saturating_sub(other.0)));
        }
        if let Ok(signed_dur) = other.downcast::<RySignedDuration>() {
            let other = signed_dur.extract::<RySignedDuration>()?;
            return Ok(RyDate::from(self.0.saturating_sub(other.0)));
        }
        if let Ok(date) = other.downcast::<PyDuration>() {
            let other = date.extract::<PyDuration>()?;
            return Ok(RyDate::from(self.0.saturating_sub(other.0)));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "unsupported operand type(s) for +: 'Date' and 'other'",
        ))
    }

    // fn __add__(&self, _py: Python<'_>, other: RyDeltaArithmeticSelf) -> PyResult<Self> {
    //     let t = match other {
    //         RyDeltaArithmeticSelf::Span(other) => self.0 + other.0,
    //         RyDeltaArithmeticSelf::SignedDuration(other) => self.0 + other.0,
    //         RyDeltaArithmeticSelf::Duration(other) => self.0 + other.0,
    //     };
    //     Ok(RyDate::from(t))
    // }

    fn __iadd__(&mut self, _py: Python<'_>, other: RyDeltaArithmeticSelf) -> PyResult<()> {
        let t = match other {
            RyDeltaArithmeticSelf::Span(other) => self.0 + other.0,
            RyDeltaArithmeticSelf::SignedDuration(other) => self.0 + other.0,
            RyDeltaArithmeticSelf::Duration(other) => self.0 + other.0,
        };
        self.0 = t;
        Ok(())
    }

    #[classmethod]
    fn from_pydate(_cls: &Bound<'_, PyType>, d: &Bound<'_, PyDate>) -> PyResult<Self> {
        let jiff_date: JiffDate = d.extract()?;
        Ok(Self::from(jiff_date.0))
    }

    fn to_pydate<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDate>> {
        let jiff_date = JiffDate(self.0);
        jiff_date.into_pyobject(py)
    }

    fn astuple<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyTuple>> {
        let year_any = self.0.year().into_pyobject(py)?.into_any();
        let month_any = self.0.month().into_pyobject(py)?.into_any();
        let day_any = self.0.day().into_pyobject(py)?.into_any();
        let parts = vec![year_any, month_any, day_any];

        PyTuple::new(py, parts)
    }

    fn intz(&self, tz: &str) -> PyResult<RyZoned> {
        self.0
            .intz(tz)
            .map(RyZoned::from)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e}")))
    }

    fn asdict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        let dict = PyDict::new(py);
        dict.set_item(intern!(py, "year"), self.0.year())?;
        dict.set_item(intern!(py, "month"), self.0.month())?;
        dict.set_item(intern!(py, "day"), self.0.day())?;
        Ok(dict)
    }

    fn series(&self, period: &RySpan) -> RyDateSeries {
        RyDateSeries {
            series: self.0.series(period.0),
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
    fn first_of_month(&self) -> RyDate {
        Self::from(self.0.first_of_month())
    }
    fn first_of_year(&self) -> RyDate {
        Self::from(self.0.first_of_year())
    }

    fn in_leap_year(&self) -> bool {
        self.0.in_leap_year()
    }
    fn last_of_month(&self) -> RyDate {
        Self::from(self.0.last_of_month())
    }
    fn last_of_year(&self) -> RyDate {
        Self::from(self.0.last_of_year())
    }
    fn tomorrow(&self) -> PyResult<Self> {
        self.0.tomorrow().map(From::from).map_err(map_py_value_err)
    }
    fn yesterday(&self) -> PyResult<Self> {
        self.0.yesterday().map(From::from).map_err(map_py_value_err)
    }
    fn strftime(&self, format: &str) -> PyResult<String> {
        Ok(self.0.strftime(format).to_string())
    }

    #[classmethod]
    fn strptime(_cls: &Bound<'_, PyType>, s: &str, format: &str) -> PyResult<Self> {
        Date::strptime(s, format)
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

    #[classmethod]
    fn from_iso_week_date(_cls: &Bound<'_, PyType>, _iso_week_date: &str) -> PyResult<()> {
        err_py_not_impl!()
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
    fn to_iso_week_date(&self) -> PyResult<()> {
        err_py_not_impl!()
    }
    fn until(&self) -> PyResult<()> {
        err_py_not_impl!()
    }
}

impl Display for RyDate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Date<{}>", self.0)
    }
}

impl From<Date> for RyDate {
    fn from(value: Date) -> Self {
        RyDate(value)
    }
}

#[pyclass]
#[pyo3(name = "DateSeries", module = "ryo3")]
pub struct RyDateSeries {
    pub(crate) series: jiff::civil::DateSeries,
}

#[pymethods]
impl RyDateSeries {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__(mut slf: PyRefMut<'_, Self>) -> Option<RyDate> {
        slf.series.next().map(RyDate::from)
    }
}

#[derive(Debug, Clone, FromPyObject)]
pub(crate) enum RyDateArithmeticSub {
    Time(RyDate),
    Delta(RyDeltaArithmeticSelf),
}
