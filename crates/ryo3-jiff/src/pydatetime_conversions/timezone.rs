use crate::JiffSignedDuration;
use crate::{JiffOffset, JiffTimeZone};
use jiff::tz::TimeZone;
use pyo3::intern;
use pyo3::prelude::*;
use pyo3::pybacked::PyBackedStr;
use pyo3::types::PyTzInfo;

pub fn timezone2pyobject<'py>(py: Python<'py>, tz: &TimeZone) -> PyResult<Bound<'py, PyTzInfo>> {
    if tz == &TimeZone::UTC {
        Ok(PyTzInfo::utc(py)?.to_owned())
    } else if let Some(iana_name) = tz.iana_name() {
        Ok(PyTzInfo::timezone(py, iana_name)?)
    } else {
        let off = tz.to_fixed_offset()?;
        let dur = off.duration_since(jiff::tz::Offset::UTC);
        PyTzInfo::fixed_offset(py, JiffSignedDuration(dur))
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
