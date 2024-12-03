use crate::JiffTimeZone;
use jiff::tz::{Offset, TimeZone};
use pyo3::types::{PyAnyMethods, PyType, PyTzInfo};
use pyo3::{pyclass, pymethods, Bound, FromPyObject, IntoPyObject, PyAny, PyErr, PyResult, Python};
use std::hash::{DefaultHasher, Hash, Hasher};

#[derive(Debug, Clone)]
#[pyclass(name = "TimeZone", module = "ryo3")]
pub struct RyTimeZone(pub(crate) TimeZone);

impl From<TimeZone> for RyTimeZone {
    fn from(value: TimeZone) -> Self {
        RyTimeZone(value)
    }
}

impl From<&TimeZone> for RyTimeZone {
    fn from(value: &TimeZone) -> Self {
        RyTimeZone(value.clone())
    }
}

#[pymethods]
impl RyTimeZone {
    #[new]
    pub fn new(time_zone_name: &str) -> PyResult<Self> {
        TimeZone::get(time_zone_name)
            .map(RyTimeZone::from)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e}")))
    }

    #[classmethod]
    fn utc(_cls: &Bound<'_, PyType>) -> Self {
        Self::from(TimeZone::fixed(Offset::UTC))
    }

    #[classmethod]
    fn system(_cls: &Bound<'_, PyType>) -> Self {
        Self::from(TimeZone::system())
    }

    fn iana_name(&self) -> Option<&str> {
        self.0.iana_name()
    }

    #[getter]
    fn name(&self) -> Option<&str> {
        self.iana_name()
    }

    fn __repr__(&self) -> String {
        let iana_name = self.0.iana_name();
        match iana_name {
            Some(name) => format!("TimeZone(\"{name}\")"),
            None => "TimeZone(None)".to_string(),
        }
    }

    fn __hash__(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.__str__().hash(&mut hasher);
        hasher.finish()
    }

    fn __str__(&self) -> String {
        self.iana_name().unwrap_or("Unknown").to_string()
    }

    fn __eq__(&self, other: TimeZoneEquality) -> bool {
        match other {
            TimeZoneEquality::TimeZone(other) => {
                self.0.eq(&other.0) || self.0.iana_name() == other.0.iana_name()
            }
            TimeZoneEquality::Str(other) => self.0.iana_name() == Some(other.as_str()),
        }
    }

    fn to_pytzinfo<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let jiff_tz = JiffTimeZone::from(self.0.clone()); // TODO: figure out no clone
        jiff_tz.into_pyobject(py)
    }

    #[classmethod]
    fn from_pytzinfo(_cls: &Bound<'_, PyType>, d: &Bound<'_, PyTzInfo>) -> PyResult<Self> {
        let jiff_tz: JiffTimeZone = d.extract()?;
        Ok(Self::from(jiff_tz.0))
    }
}

#[derive(Debug, Clone, FromPyObject)]
enum TimeZoneEquality {
    TimeZone(RyTimeZone),
    Str(String),
}
