use crate::errors::map_py_value_err;
use crate::{JiffWeekday, RyDate};
use jiff::civil::ISOWeekDate;
use jiff::Zoned;
use pyo3::prelude::*;
use pyo3::types::PyType;
use std::hash::{DefaultHasher, Hash, Hasher};

#[derive(Debug, Clone)]
#[pyclass(name = "ISOWeekDate", module = "ryo3", frozen)]
pub struct RyISOWeekDate(pub(crate) ISOWeekDate);

#[pymethods]
impl RyISOWeekDate {
    #[new]
    pub fn py_new(year: i16, week: i8, weekday: JiffWeekday) -> PyResult<Self> {
        ISOWeekDate::new(year, week, weekday.0)
            .map(Self::from)
            .map_err(map_py_value_err)
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

    /// Returns the `ISOWeekDate` for the given `Date`.
    #[classmethod]
    fn from_date(_cls: &Bound<'_, PyType>, date: &RyDate) -> Self {
        Self(ISOWeekDate::from(date.0))
    }

    /// Returns the date today as an `ISOWeekDate`
    #[classmethod]
    fn today(_cls: &Bound<'_, PyType>) -> Self {
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

    fn __repr__(&self) -> String {
        format!(
            "ISOWeekDate({}, {}, '{}')",
            self.year(),
            self.week(),
            self.weekday()
        )
    }

    // ========================================================================
    // DUNDERS/OPERATORS
    // ========================================================================

    fn __eq__(&self, other: &RyISOWeekDate) -> bool {
        self.0 == other.0
    }

    fn __ne__(&self, other: &RyISOWeekDate) -> bool {
        self.0 != other.0
    }

    fn __lt__(&self, other: &RyISOWeekDate) -> bool {
        self.0 < other.0
    }

    fn __le__(&self, other: &RyISOWeekDate) -> bool {
        self.0 <= other.0
    }

    fn __gt__(&self, other: &RyISOWeekDate) -> bool {
        self.0 > other.0
    }

    fn __ge__(&self, other: &RyISOWeekDate) -> bool {
        self.0 >= other.0
    }

    fn __hash__(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.0.hash(&mut hasher);
        hasher.finish()
    }
}

impl From<ISOWeekDate> for RyISOWeekDate {
    fn from(date: ISOWeekDate) -> Self {
        Self(date)
    }
}
