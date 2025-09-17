use crate::jiff_types::JiffDateTime;
use crate::pydatetime_conversions::{py_date_to_date, py_time_to_jiff_time};
use jiff::civil::DateTime;
use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;
use pyo3::types::PyDateTime;
#[cfg(not(Py_LIMITED_API))]
use pyo3::types::PyTzInfoAccess;

impl<'py> IntoPyObject<'py> for JiffDateTime {
    type Target = PyDateTime;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (&self).into_pyobject(py)
    }
}

impl<'py> IntoPyObject<'py> for &JiffDateTime {
    type Target = PyDateTime;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    #[inline]
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        self.0.into_pyobject(py)
    }
}

impl FromPyObject<'_> for JiffDateTime {
    fn extract_bound(dt: &Bound<'_, PyAny>) -> PyResult<Self> {
        let dt = dt.cast::<PyDateTime>()?;

        // If the user tries to convert a timezone aware datetime into a naive one,
        // we return a hard error. We could silently remove tzinfo, or assume local timezone
        // and do a conversion, but better leave this decision to the user of the library.
        #[cfg(not(Py_LIMITED_API))]
        let has_tzinfo = dt.get_tzinfo().is_some();
        #[cfg(Py_LIMITED_API)]
        let has_tzinfo = !dt.getattr(pyo3::intern!(dt.py(), "tzinfo"))?.is_none();
        if has_tzinfo {
            return Err(PyTypeError::new_err("expected a datetime without tzinfo"));
        }
        let jiff_date = py_date_to_date(dt)?;
        let jiff_time = py_time_to_jiff_time(dt)?;
        let dt = DateTime::from_parts(jiff_date, jiff_time);
        // ::new(date_from_pyobject(dt)?, py_time_to_jiff_time(dt)?);
        Ok(dt.into())
    }
}
