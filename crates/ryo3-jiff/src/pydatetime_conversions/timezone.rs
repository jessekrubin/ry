use crate::JiffTimeZone;
use jiff::tz::TimeZone;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::sync::GILOnceCell;
use pyo3::types::{PyString, PyType};

pub fn timezone2pyobect<'py>(py: Python<'py>, tz: &TimeZone) -> PyResult<Bound<'py, PyAny>> {
    static ZONE_INFO: GILOnceCell<Py<PyType>> = GILOnceCell::new();
    ZONE_INFO
        .import(py, "zoneinfo", "ZoneInfo")
        .and_then(|obj| obj.call1((tz.iana_name(),)))
}
impl<'py> IntoPyObject<'py> for JiffTimeZone {
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (&self).into_pyobject(py)
    }
}
impl<'py> IntoPyObject<'py> for &JiffTimeZone {
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        timezone2pyobect(py, &self.0)
    }
}

impl FromPyObject<'_> for JiffTimeZone {
    fn extract_bound(ob: &Bound<'_, PyAny>) -> PyResult<JiffTimeZone> {
        // if it is a string we go w/ that
        if let Ok(s) = ob.downcast::<PyString>() {
            let str = s.to_string();
            if str.ends_with("/etc/localtime") {
                return Ok(JiffTimeZone(jiff::tz::TimeZone::system()));
            }
            let tz = jiff::tz::TimeZone::get(str.as_str())
                .map_err(|e| PyErr::new::<PyValueError, _>(format!("{e}")))?;
            let jtz = JiffTimeZone(tz);
            Ok(jtz)
        } else {
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
}
