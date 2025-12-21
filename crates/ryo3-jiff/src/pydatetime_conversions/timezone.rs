use crate::{JiffOffset, JiffTimeZone};
use crate::{JiffSignedDuration, JiffTimeZoneRef};
use jiff::tz::TimeZone;
use pyo3::intern;
use pyo3::prelude::*;
use pyo3::pybacked::PyBackedStr;
use pyo3::types::PyTzInfo;
use ryo3_core::map_py_value_err;

pub fn timezone2pyobject<'py>(py: Python<'py>, tz: &TimeZone) -> PyResult<Bound<'py, PyTzInfo>> {
    if tz == &TimeZone::UTC {
        Ok(PyTzInfo::utc(py)?.to_owned())
    } else if let Some(iana_name) = tz.iana_name() {
        Ok(PyTzInfo::timezone(py, iana_name)?)
    } else {
        let off = tz.to_fixed_offset().map_err(map_py_value_err)?;
        let dur = off.duration_since(jiff::tz::Offset::UTC);
        PyTzInfo::fixed_offset(py, JiffSignedDuration(dur))
    }
}

impl<'py> IntoPyObject<'py> for JiffTimeZoneRef<'_> {
    type Target = PyTzInfo;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (&self).into_pyobject(py)
    }
}

impl<'py> IntoPyObject<'py> for &JiffTimeZoneRef<'_> {
    type Target = PyTzInfo;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        timezone2pyobject(py, self.0)
    }
}

impl<'py> IntoPyObject<'py> for JiffTimeZone {
    type Target = PyTzInfo;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (&self).into_pyobject(py)
    }
}

impl<'py> IntoPyObject<'py> for &JiffTimeZone {
    type Target = PyTzInfo;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        timezone2pyobject(py, &self.0)
    }
}

impl<'py> FromPyObject<'_, 'py> for JiffTimeZone {
    type Error = PyErr;
    fn extract(obj: Borrowed<'_, 'py, PyAny>) -> PyResult<Self> {
        let ob = obj.cast::<PyTzInfo>()?;
        let attr = intern!(ob.py(), "key");
        if ob.hasattr(attr)? {
            let tz_pystr = ob.getattr(attr)?.extract::<PyBackedStr>()?;
            let tz_str = tz_pystr.to_string();
            let tz = TimeZone::get(&tz_str).map_err(map_py_value_err)?;
            Ok(Self(tz))
        } else {
            Ok(ob.extract::<JiffOffset>()?.0.to_time_zone().into())
        }
    }
}
