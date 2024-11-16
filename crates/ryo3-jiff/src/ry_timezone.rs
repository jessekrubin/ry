use jiff::tz::{Offset, TimeZone};
use pyo3::types::PyType;
use pyo3::{pyclass, pymethods, Bound, PyErr, PyResult};

#[derive(Debug, Clone)]
#[pyclass(name = "TimeZone", module = "ryo3")]
pub struct RyTimeZone(pub(crate) TimeZone);

impl From<TimeZone> for RyTimeZone {
    fn from(value: TimeZone) -> Self {
        RyTimeZone(value)
    }
}

#[pymethods]
impl RyTimeZone {
    #[new]
    pub fn new(time_zone_name: &str) -> PyResult<Self> {
        TimeZone::get(time_zone_name)
            .map(|tz| RyTimeZone::from(tz))
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{}", e)))
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

    fn __str__(&self) -> String {
        // TODO; figure out good repr
        let iana_name = self.0.iana_name();
        match iana_name {
            Some(name) => format!("TimeZone<{}>", name),
            None => "TimeZone<None>".to_string(),
        }
    }

    fn __eq__(&self, other: &RyTimeZone) -> bool {
        self.0 == other.0
    }
}
