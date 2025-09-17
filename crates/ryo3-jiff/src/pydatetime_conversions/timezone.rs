use crate::{JiffOffset, JiffTimeZone};
use jiff::tz::TimeZone;
use pyo3::intern;
use pyo3::prelude::*;
use pyo3::pybacked::PyBackedStr;
use pyo3::sync::PyOnceLock;
use pyo3::types::{PyType, PyTzInfo};

pub fn timezone2pyobect<'py>(py: Python<'py>, tz: &TimeZone) -> PyResult<Bound<'py, PyAny>> {
    static ZONE_INFO: PyOnceLock<Py<PyType>> = PyOnceLock::new();
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

impl<'py> FromPyObject<'py> for JiffTimeZone {
    fn extract_bound(ob: &Bound<'py, PyAny>) -> PyResult<Self> {
        let ob = ob.cast::<PyTzInfo>()?;
        let attr = intern!(ob.py(), "key");
        if ob.hasattr(attr)? {
            let tz_pystr = ob.getattr(attr)?.extract::<PyBackedStr>()?;
            let tz_str = tz_pystr.to_string();
            Ok(Self(TimeZone::get(&tz_str)?))
        } else {
            Ok(ob.extract::<JiffOffset>()?.0.to_time_zone().into())
        }
    }
}
