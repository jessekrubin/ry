use crate::errors::map_py_value_err;
use crate::pydatetime_conversions::{pydate_to_date, pytime_to_time};
use crate::{JiffTimeZone, JiffTimeZoneRef, JiffZoned, JiffZonedRef};
use jiff::Zoned;
use jiff::civil::DateTime;
use jiff::tz::TimeZone;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
#[cfg(not(Py_LIMITED_API))]
use pyo3::types::PyTimeAccess;
use pyo3::types::{PyDateTime, PyTzInfoAccess};

fn datetime_to_pydatetime<'py>(
    py: Python<'py>,
    datetime: &DateTime,
    fold: bool,
    timezone: Option<&TimeZone>,
) -> PyResult<Bound<'py, PyDateTime>> {
    let tz = timezone
        .map(|tz| JiffTimeZoneRef(tz).into_pyobject(py))
        .transpose()?;
    PyDateTime::new_with_fold(
        py,
        datetime.year().into(),
        datetime.month().try_into()?,
        datetime.day().try_into()?,
        datetime.hour().try_into()?,
        datetime.minute().try_into()?,
        datetime.second().try_into()?,
        (datetime.subsec_nanosecond() / 1000).try_into()?,
        tz.as_ref(),
        fold,
    )
}

#[expect(clippy::arithmetic_side_effects)]
fn fold(zoned: &Zoned) -> Option<bool> {
    let prev = zoned.time_zone().preceding(zoned.timestamp()).next()?;
    let next = zoned.time_zone().following(prev.timestamp()).next()?;
    let start_of_current_offset = if next.timestamp() == zoned.timestamp() {
        next.timestamp()
    } else {
        prev.timestamp()
    };
    Some(zoned.timestamp() + (zoned.offset() - prev.offset()) <= start_of_current_offset)
}

pub fn zoned2pyobject<'py>(py: Python<'py>, z: &Zoned) -> PyResult<Bound<'py, PyDateTime>> {
    datetime_to_pydatetime(
        py,
        &z.datetime(),
        fold(z).unwrap_or(false),
        Some(z.time_zone()),
    )
}

impl<'py> IntoPyObject<'py> for &JiffZonedRef<'_> {
    type Target = PyDateTime;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        let z = &self.0;
        zoned2pyobject(py, z)
    }
}

impl<'py> IntoPyObject<'py> for JiffZonedRef<'_> {
    type Target = PyDateTime;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    #[inline]
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (&self).into_pyobject(py)
    }
}

impl<'py> IntoPyObject<'py> for &JiffZoned {
    type Target = PyDateTime;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        let z = &self.0;
        zoned2pyobject(py, z)
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

impl<'py> FromPyObject<'_, 'py> for JiffZoned {
    type Error = PyErr;
    fn extract(dt: Borrowed<'_, 'py, PyAny>) -> PyResult<Self> {
        let dt = dt.cast::<PyDateTime>()?;
        let tzinfo = dt.get_tzinfo().map_or_else(
            || {
                Err(PyErr::new::<PyValueError, _>(
                    "expected a datetime with non-None tzinfo",
                ))
            },
            |tz| tz.extract::<JiffTimeZone>(),
        )?;
        // #[expect(clippy::explicit_auto_deref)]
        let jiff_time = pytime_to_time(&*dt)?;
        // #[expect(clippy::explicit_auto_deref)]
        let jiff_date = pydate_to_date(&*dt)?;
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
            Ok(Self::from(zoned.later().map_err(map_py_value_err)?))
        } else {
            Ok(Self::from(zoned.earlier().map_err(map_py_value_err)?))
        }
    }
}
