use jiff::Zoned;
use jiff::civil::DateTime;
use jiff::tz::{AmbiguousOffset, Offset, TimeZone};
use pyo3::prelude::*;
use pyo3::types::{PyDateTime, PyTimeAccess, PyTzInfo, PyTzInfoAccess};
use ryo3_core::map_py_value_err;
use ryo3_macro_rules::py_type_err;

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

pub fn pydatetime_to_zoned(dt: &Bound<'_, PyDateTime>) -> PyResult<Zoned> {
    let Some(tzinfo) = dt.get_tzinfo() else {
        return py_type_err!("expected a datetime with non-None tzinfo");
    };
    let datetime = DateTime::from_parts(pydate_to_date(dt)?, pytime_to_time(dt)?);
    if tzinfo.is(PyTzInfo::utc(dt.py())?) {
        // utc aint ambiguous... skip tz lookup + fold
        let ts = Offset::UTC
            .to_timestamp(datetime)
            .map_err(map_py_value_err)?;
        return Ok(ts.to_zoned(TimeZone::UTC));
    }
    let zoned = tzinfo
        .extract::<JiffTimeZone>()?
        .into_inner()
        .into_ambiguous_zoned(datetime);
    // REF: https://peps.python.org/pep-0495/
    // REF: https://github.com/PyO3/pyo3/pull/6450
    if dt.get_fold() ^ matches!(zoned.offset(), AmbiguousOffset::Gap { .. }) {
        zoned.later()
    } else {
        zoned.earlier()
    }
    .map_err(map_py_value_err)
}

impl<'py> IntoPyObject<'py> for &JiffZonedRef<'_> {
    type Target = PyDateTime;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        zoned_to_pydatetime(py, self.inner())
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
        zoned_to_pydatetime(py, self.inner())
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
        pydatetime_to_zoned(&dt).map(Self::from)
    }
}
