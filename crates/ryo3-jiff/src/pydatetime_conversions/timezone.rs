use jiff::tz::TimeZone;
use pyo3::prelude::*;
use pyo3::pybacked::PyBackedStr;
use pyo3::types::PyTzInfo;
use ryo3_core::map_py_value_err;

use crate::{JiffOffset, JiffSignedDuration, JiffTimeZone, JiffTimeZoneRef};

pub fn timezone_to_pytzinfo<'py>(py: Python<'py>, tz: &TimeZone) -> PyResult<Bound<'py, PyTzInfo>> {
    if tz == &TimeZone::UTC {
        Ok(PyTzInfo::utc(py)?.to_owned())
    } else if let Some(iana_name) = tz.iana_name() {
        Ok(PyTzInfo::timezone(py, iana_name)?)
    } else {
        let off = tz.to_fixed_offset().map_err(map_py_value_err)?;
        let dur = off.duration_since(jiff::tz::Offset::UTC);
        PyTzInfo::fixed_offset(py, JiffSignedDuration::new(dur))
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
        timezone_to_pytzinfo(py, self.inner())
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
        timezone_to_pytzinfo(py, self.inner())
    }
}

impl<'py> FromPyObject<'_, 'py> for JiffTimeZone {
    type Error = PyErr;
    fn extract(obj: Borrowed<'_, 'py, PyAny>) -> PyResult<Self> {
        let ob = obj.cast::<PyTzInfo>()?;
        match ob.getattr_opt(::pyo3::intern!(ob.py(), "key"))? {
            Some(key) => {
                let key = key.extract::<PyBackedStr>()?;
                crate::ry_timezone::get_time_zone(&key)
                    .map(Self::from)
                    .map_err(map_py_value_err)
            }
            None => Ok(ob
                .extract::<JiffOffset>()?
                .into_inner()
                .to_time_zone()
                .into()),
        }
    }
}
