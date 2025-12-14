use crate::errors::map_py_value_err;
use crate::isoformat::iso_weekdate_to_string;
use crate::{JiffWeekday, RyDate, RyDateTime, RyTimestamp, RyZoned};
use jiff::Zoned;
use jiff::civil::ISOWeekDate;
use pyo3::basic::CompareOp;
use pyo3::types::{PyDict, PyTuple};
use pyo3::{BoundObject, prelude::*};
use ryo3_macro_rules::{any_repr, py_type_err};
use std::hash::{DefaultHasher, Hash, Hasher};

#[derive(Debug, Clone, Copy)]
#[pyclass(name = "ISOWeekDate", frozen, immutable_type, from_py_object)]
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
        use ryo3_core::PyFromStr;
        Self::py_from_str(s)
    }

    #[staticmethod]
    fn parse(s: &Bound<'_, PyAny>) -> PyResult<Self> {
        use ryo3_core::PyParse;
        Self::py_parse(s)
    }

    /// Returns the `ISOWeekDate` for the given `Date`.
    #[staticmethod]
    fn from_pydate(date: jiff::civil::Date) -> Self {
        Self::from(date)
    }

    /// Returns the `ISOWeekDate` for the given `Date`.
    #[staticmethod]
    fn from_date(date: &RyDate) -> Self {
        Self(ISOWeekDate::from(date.0))
    }

    /// Convert to `datetime.date`
    #[expect(clippy::wrong_self_convention)]
    fn to_py(&self) -> jiff::civil::Date {
        self.to_pydate()
    }

    /// Convert to `datetime.date`
    #[expect(clippy::wrong_self_convention)]
    fn to_pydate(&self) -> jiff::civil::Date {
        self.0.date()
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

    #[staticmethod]
    fn from_any<'py>(value: &Bound<'py, PyAny>) -> PyResult<Bound<'py, Self>> {
        let py = value.py();
        if let Ok(val) = value.cast_exact::<Self>() {
            Ok(val.as_borrowed().into_bound())
        } else if let Ok(pystr) = value.cast::<pyo3::types::PyString>() {
            let s = pystr.extract::<&str>()?;
            Self::from_str(s).map(|dt| dt.into_pyobject(py))?
        } else if let Ok(pybytes) = value.cast::<pyo3::types::PyBytes>() {
            let s = String::from_utf8_lossy(pybytes.as_bytes());
            Self::from_str(&s).map(|dt| dt.into_pyobject(py))?
        } else if let Ok(d) = value.cast_exact::<RyDate>() {
            d.get().iso_week_date().into_pyobject(py)
        } else if let Ok(d) = value.cast_exact::<RyDateTime>() {
            d.get().iso_week_date().into_pyobject(py)
        } else if let Ok(d) = value.cast_exact::<RyZoned>() {
            d.get().iso_week_date().into_pyobject(py)
        } else if let Ok(d) = value.cast_exact::<RyTimestamp>() {
            d.get().iso_week_date().into_pyobject(py)
        } else if let Ok(d) = value.extract::<jiff::civil::Date>() {
            Self::from(d).into_pyobject(py)
        } else {
            let valtype = any_repr!(value);
            py_type_err!("Date conversion error: {valtype}",)
        }
    }

    // ========================================================================
    // PYDANTIC
    // ========================================================================

    /// Try to create a Date from a variety of python objects
    #[cfg(feature = "pydantic")]
    #[staticmethod]
    fn _pydantic_validate<'py>(
        value: &Bound<'py, PyAny>,
        _handler: &Bound<'py, PyAny>,
    ) -> PyResult<Bound<'py, Self>> {
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
