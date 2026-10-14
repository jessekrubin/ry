use jiff::Zoned;
use jiff::civil::DateTime;
use jiff::tz::AmbiguousOffset;
use pyo3::prelude::*;
#[cfg(not(Py_LIMITED_API))]
use pyo3::types::PyTimeAccess;
use pyo3::types::{PyDateTime, PyTzInfoAccess};
use ryo3_core::map_py_value_err;
use ryo3_macro_rules::py_value_err;

use crate::pydatetime_conversions::{pydate_to_date, pytime_to_time, timezone_to_pytzinfo};
use crate::{JiffTimeZone, JiffZoned, JiffZonedRef};

fn fold(zoned: &Zoned) -> bool {
    match zoned
        .time_zone()
        .to_ambiguous_timestamp(zoned.datetime())
        .offset()
    {
        AmbiguousOffset::Fold { after, .. } => zoned.offset() == after,
        _ => false,
    }
}

pub fn zoned_to_pydatetime<'py>(py: Python<'py>, z: &Zoned) -> PyResult<Bound<'py, PyDateTime>> {
    let datetime = z.datetime();
    let tz = timezone_to_pytzinfo(py, z.time_zone())?;
    PyDateTime::new_with_fold(
        py,
        datetime.year().into(),
        datetime.month().try_into()?,
        datetime.day().try_into()?,
        datetime.hour().try_into()?,
        datetime.minute().try_into()?,
        datetime.second().try_into()?,
        (datetime.subsec_nanosecond() / 1000).try_into()?,
        Some(&tz),
        fold(z),
    )
}

impl<'py> IntoPyObject<'py> for &JiffZonedRef<'_> {
    type Target = PyDateTime;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        let z = &self.0;
        zoned_to_pydatetime(py, z)
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
        zoned_to_pydatetime(py, z)
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
            || py_value_err!("expected a datetime with non-None tzinfo"),
            |tz| tz.extract::<JiffTimeZone>(),
        )?;
        let jiff_time = pytime_to_time(&*dt)?;
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
        // REF: https://peps.python.org/pep-0495/
        // REF: https://github.com/PyO3/pyo3/pull/6450
        if fold ^ matches!(zoned.offset(), AmbiguousOffset::Gap { .. }) {
            Ok(Self::from(zoned.later().map_err(map_py_value_err)?))
        } else {
            Ok(Self::from(zoned.earlier().map_err(map_py_value_err)?))
        }
    }
}
