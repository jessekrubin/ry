use crate::{JiffTimestamp, JiffZoned};
use pyo3::prelude::*;

impl<'py> IntoPyObject<'py> for JiffTimestamp {
    type Target = pyo3::types::PyDateTime;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (&self).into_pyobject(py)
    }
}

impl<'py> IntoPyObject<'py> for &JiffTimestamp {
    type Target = pyo3::types::PyDateTime;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        JiffZoned(self.0.to_zoned(jiff::tz::TimeZone::UTC)).into_pyobject(py)
    }
}

impl<'py> FromPyObject<'_, 'py> for JiffTimestamp {
    type Error = PyErr;
    fn extract(ob: Borrowed<'_, 'py, PyAny>) -> PyResult<Self> {
        let zoned = ob.extract::<JiffZoned>()?;
        Ok(Self(zoned.0.timestamp()))
    }
}
