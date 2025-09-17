use crate::pydatetime_conversions::{py_date_to_date, py_time_to_jiff_time};
use crate::{JiffTimeZone, JiffZoned};
use jiff::Zoned;
use jiff::civil::DateTime;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
#[cfg(not(Py_LIMITED_API))]
use pyo3::types::PyTimeAccess;
use pyo3::types::{PyDateTime, PyTzInfoAccess};

pub fn zoned2pyobect<'py>(py: Python<'py>, z: &Zoned) -> PyResult<Bound<'py, PyDateTime>> {
    z.into_pyobject(py)
}

impl<'py> IntoPyObject<'py> for &JiffZoned {
    type Target = PyDateTime;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        let z = &self.0;
        zoned2pyobect(py, z)
    }
}

impl<'py> IntoPyObject<'py> for JiffZoned {
    type Target = PyDateTime;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    #[inline]
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (&self).into_pyobject(py)
    }
}

impl FromPyObject<'_> for JiffZoned {
    fn extract_bound(dt: &Bound<'_, PyAny>) -> PyResult<Self> {
        let dt = dt.cast::<PyDateTime>()?;
        let tzinfo = dt.get_tzinfo().map_or_else(
            || {
                Err(PyErr::new::<PyValueError, _>(
                    "expected a datetime with non-None tzinfo",
                ))
            },
            |tz| tz.extract::<JiffTimeZone>(),
        )?;
        let jiff_time = py_time_to_jiff_time(dt)?;
        let jiff_date = py_date_to_date(dt)?;
        let datetime = DateTime::from_parts(jiff_date, jiff_time);
        let zoned = tzinfo.0.into_ambiguous_zoned(datetime);

        #[cfg(not(Py_LIMITED_API))]
        let fold = dt.get_fold();

        #[cfg(Py_LIMITED_API)]
        let fold = dt
            .getattr(pyo3::intern!(dt.py(), "fold"))?
            .extract::<usize>()?
            > 0;
        if fold {
            Ok(Self::from(zoned.later()?))
        } else {
            Ok(Self::from(zoned.earlier()?))
        }
    }
}
