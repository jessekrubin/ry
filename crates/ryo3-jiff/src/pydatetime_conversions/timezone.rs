use crate::JiffTimeZone;
use pyo3::exceptions::PyValueError;
use pyo3::sync::GILOnceCell;
use pyo3::types::{PyAnyMethods, PyType};
use pyo3::{Bound, FromPyObject, IntoPyObject, Py, PyAny, PyErr, PyResult, Python};

impl<'py> IntoPyObject<'py> for JiffTimeZone {
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        static ZONE_INFO: GILOnceCell<Py<PyType>> = GILOnceCell::new();
        ZONE_INFO
            .import(py, "zoneinfo", "ZoneInfo")
            .and_then(|obj| obj.call1((self.0.iana_name(),)))
    }
}
impl FromPyObject<'_> for JiffTimeZone {
    fn extract_bound(ob: &Bound<'_, PyAny>) -> PyResult<JiffTimeZone> {
        let name = ob.to_string();
        if name.ends_with("/etc/localtime") {
            return Ok(JiffTimeZone(jiff::tz::TimeZone::system()));
        }
        let tz = jiff::tz::TimeZone::get(name.as_str())
            .map_err(|e| PyErr::new::<PyValueError, _>(format!("{e}")))?;
        let jtz = JiffTimeZone(tz);
        Ok(jtz)
    }
}