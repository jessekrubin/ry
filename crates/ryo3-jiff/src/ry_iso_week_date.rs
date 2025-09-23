use crate::errors::map_py_value_err;
use crate::isoformat::{iso_weekdate_to_string, parse_iso_week_date};
use crate::{JiffWeekday, RyDate};
use jiff::Zoned;
use jiff::civil::ISOWeekDate;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyTuple};
use std::hash::{DefaultHasher, Hash, Hasher};

#[derive(Debug, Clone, Copy)]
#[pyclass(name = "ISOWeekDate", frozen)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
pub struct RyISOWeekDate(pub(crate) ISOWeekDate);

#[pymethods]
impl RyISOWeekDate {
    #[new]
    fn py_new(year: i16, week: i8, weekday: JiffWeekday) -> PyResult<Self> {
        ISOWeekDate::new(year, week, weekday.0)
            .map(Self::from)
            .map_err(map_py_value_err)
    }

    fn __getnewargs__<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyTuple>> {
        PyTuple::new(
            py,
            vec![
                self.year().into_pyobject(py)?,
                self.week().into_pyobject(py)?,
                self.weekday().into_pyobject(py)?,
            ],
        )
    }

    // ========================================================================
    // CLASSATTR
    // ========================================================================

    /// The minimum representable `ISOWeekDate`.
    #[expect(non_snake_case)]
    #[classattr]
    fn MIN() -> Self {
        Self(ISOWeekDate::MIN)
    }

    /// The maximum representable `ISOWeekDate`.
    #[expect(non_snake_case)]
    #[classattr]
    fn MAX() -> Self {
        Self(ISOWeekDate::MAX)
    }

    /// The zero `ISOWeekDate`.
    #[expect(non_snake_case)]
    #[classattr]
    fn ZERO() -> Self {
        Self(ISOWeekDate::ZERO)
    }

    // ========================================================================
    // CLASSMETHOD
    // ========================================================================
    #[staticmethod]
    fn from_str(s: &str) -> PyResult<Self> {
        parse_iso_week_date(s).map(Self::from)
    }

    #[staticmethod]
    fn parse(s: &str) -> PyResult<Self> {
        parse_iso_week_date(s).map(Self::from)
    }

    /// Returns the `ISOWeekDate` for the given `Date`.
    #[staticmethod]
    fn from_date(date: &RyDate) -> Self {
        Self(ISOWeekDate::from(date.0))
    }

    /// Returns the date today as an `ISOWeekDate`
    #[staticmethod]
    fn today() -> Self {
        let date = jiff::civil::Date::from(Zoned::now());
        Self::from(ISOWeekDate::from(date))
    }

    #[staticmethod]
    fn now() -> Self {
        let date = jiff::civil::Date::from(Zoned::now());
        Self::from(ISOWeekDate::from(date))
    }

    // ========================================================================
    // PROPERTIES
    // ========================================================================

    /// The year of the `ISOWeekDate`.
    #[getter]
    fn year(&self) -> i16 {
        self.0.year()
    }

    /// The week of the `ISOWeekDate`.
    #[getter]
    fn week(&self) -> i8 {
        self.0.week()
    }

    /// The weekday of the `ISOWeekDate`.
    #[getter]
    fn weekday(&self) -> JiffWeekday {
        JiffWeekday(self.0.weekday())
    }

    // ========================================================================
    // METHODS
    // ========================================================================

    fn date(&self) -> RyDate {
        self.0.date().into()
    }

    fn __str__(&self) -> String {
        iso_weekdate_to_string(&self.0)
    }

    #[pyo3(name = "to_string")]
    fn py_to_string(&self) -> String {
        self.__str__()
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

    fn __repr__(&self) -> String {
        format!("{self}")
    }

    #[expect(clippy::wrong_self_convention)]
    fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        use crate::interns;
        let dict = PyDict::new(py);
        dict.set_item(interns::year(py), self.year())?;
        dict.set_item(interns::week(py), self.week())?;
        dict.set_item(interns::weekday(py), self.weekday())?;
        Ok(dict)
    }

    // ========================================================================
    // DUNDERS/OPERATORS
    // ========================================================================

    fn __eq__(&self, other: &Self) -> bool {
        self.0 == other.0
    }

    fn __ne__(&self, other: &Self) -> bool {
        self.0 != other.0
    }

    fn __lt__(&self, other: &Self) -> bool {
        self.0 < other.0
    }

    fn __le__(&self, other: &Self) -> bool {
        self.0 <= other.0
    }

    fn __gt__(&self, other: &Self) -> bool {
        self.0 > other.0
    }

    fn __ge__(&self, other: &Self) -> bool {
        self.0 >= other.0
    }

    fn __hash__(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.0.hash(&mut hasher);
        hasher.finish()
    }
}

impl std::fmt::Display for RyISOWeekDate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ISOWeekDate({}, {}, '{}')",
            self.year(),
            self.week(),
            self.weekday()
        )
    }
}

impl From<ISOWeekDate> for RyISOWeekDate {
    fn from(date: ISOWeekDate) -> Self {
        Self(date)
    }
}
