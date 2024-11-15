use jiff::tz::{Offset, TimeZone};
use pyo3::types::PyType;
use pyo3::{pyclass, pymethods, Bound, PyErr, PyResult};

#[derive(Debug, Clone)]
#[pyclass(name = "TimeZone", module = "ryo3")]
pub struct PyTimeZone(pub(crate) TimeZone);

impl From<jiff::tz::TimeZone> for PyTimeZone {
    fn from(value: jiff::tz::TimeZone) -> Self {
        PyTimeZone(value)
    }
}

#[pymethods]
impl PyTimeZone {
    #[new]
    pub fn new(time_zone_name: &str) -> PyResult<Self> {
        jiff::tz::TimeZone::get(time_zone_name)
            .map(|tz| PyTimeZone::from(tz))
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{}", e)))
    }

    #[classmethod]
    fn utc(_cls: &Bound<'_, PyType>) -> Self {
        Self::from(jiff::tz::TimeZone::fixed(Offset::UTC))
    }

    #[classmethod]
    fn system(_cls: &Bound<'_, PyType>) -> Self {
        Self::from(jiff::tz::TimeZone::system())
    }

    fn iana_name(&self) -> Option<&str> {
        self.0.iana_name()
    }

    fn __str__(&self) -> String {
        // TODO; figure out good repr
        let iana_name = self.0.iana_name();
        match iana_name {
            Some(name) => format!("TimeZone<{}>", name),
            None => "TimeZone<None>".to_string(),
        }
    }
}
