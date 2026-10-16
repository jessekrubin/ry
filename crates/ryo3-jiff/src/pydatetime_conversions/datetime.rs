use jiff::civil::DateTime;
use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;
use pyo3::types::{PyDateTime, PyTzInfoAccess};

use crate::jiff_types::JiffDateTime;
use crate::pydatetime_conversions::{pydate_to_date, pytime_to_time};

pub fn datetime_to_pydatetime<'py>(
    py: Python<'py>,
    datetime: &DateTime,
    fold: bool,
) -> PyResult<Bound<'py, PyDateTime>> {
    PyDateTime::new_with_fold(
        py,
        datetime.year().into(),
        datetime.month().try_into()?,
        datetime.day().try_into()?,
        datetime.hour().try_into()?,
        datetime.minute().try_into()?,
        datetime.second().try_into()?,
        (datetime.subsec_nanosecond() / 1000).try_into()?,
        None,
        fold,
    )
}

pub fn pydatetime_to_datetime(dt: &Bound<'_, PyDateTime>) -> PyResult<DateTime> {
    if dt.get_tzinfo().is_some() {
        return Err(PyTypeError::new_err("expected a datetime without tzinfo"));
    }
    Ok(DateTime::from_parts(
        pydate_to_date(dt)?,
        pytime_to_time(dt)?,
    ))
}

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
        datetime_to_pydatetime(py, self.inner(), false)
    }
}

impl<'py> FromPyObject<'_, 'py> for JiffDateTime {
    type Error = PyErr;
    fn extract(ob: Borrowed<'_, 'py, PyAny>) -> Result<Self, Self::Error> {
        let dt = ob.cast::<PyDateTime>()?;
        pydatetime_to_datetime(&dt).map(Self::from)
    }
}
