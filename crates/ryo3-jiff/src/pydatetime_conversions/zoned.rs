use crate::pydatetime_conversions::timezone::timezone2pyobect;
use crate::pydatetime_conversions::{date_from_pyobject, py_time_to_jiff_time};
use crate::{JiffTimeZone, JiffZoned};
use jiff::Zoned;
use jiff::civil::DateTime;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::{PyDateTime, PyTimeAccess, PyTzInfo, PyTzInfoAccess};

pub fn zoned2pyobect<'py>(py: Python<'py>, z: &Zoned) -> PyResult<Bound<'py, PyDateTime>> {
    // let tz = self.offset().fix().into_pyobject(py)?;
    let tz = z.time_zone();
    let pytz = timezone2pyobect(py, tz)?;
    // let pytz = JiffTimeZone(tz.clone()).into_pyobject(py)?;
    // downcast to tz
    let tz = pytz.downcast::<PyTzInfo>()?;

    let year = i32::from(z.year());
    let m_u8 =
        u8::try_from(z.month()).map_err(|e| PyErr::new::<PyValueError, _>(format!("{e}")))?;
    let d_u8 = u8::try_from(z.day()).map_err(|e| PyErr::new::<PyValueError, _>(format!("{e}")))?;
    let hour_u8 =
        u8::try_from(z.hour()).map_err(|e| PyErr::new::<PyValueError, _>(format!("hour: {e}")))?;
    let minute_u8 = u8::try_from(z.minute())
        .map_err(|e| PyErr::new::<PyValueError, _>(format!("minute: {e}")))?;
    let second_u8 = u8::try_from(z.second())
        .map_err(|e| PyErr::new::<PyValueError, _>(format!("second: {e}")))?;
    let microsecond_u32 = u32::try_from(z.microsecond())
        .map_err(|e| PyErr::new::<PyValueError, _>(format!("microsecond: {e}")))?;

    #[cfg(not(Py_LIMITED_API))]
    let datetime = PyDateTime::new(
        py,
        year,
        m_u8,
        d_u8,
        hour_u8,
        minute_u8,
        second_u8,
        microsecond_u32,
        Some(tz),
    )?;

    #[cfg(Py_LIMITED_API)]
    let datetime = PyDateTime::new(
        py,
        year,
        m_u8,
        d_u8,
        hour_u8,
        minute_u8,
        second_u8,
        microsecond_u32,
        Some(tz),
    );
    Ok(datetime)
}

impl<'py> IntoPyObject<'py> for &JiffZoned {
    #[cfg(Py_LIMITED_API)]
    type Target = PyAny;
    #[cfg(not(Py_LIMITED_API))]
    type Target = PyDateTime;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        let z = &self.0;
        zoned2pyobect(py, z)
    }
}

impl<'py> IntoPyObject<'py> for JiffZoned {
    #[cfg(Py_LIMITED_API)]
    type Target = PyAny;
    #[cfg(not(Py_LIMITED_API))]
    type Target = PyDateTime;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    #[inline]
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (&self).into_pyobject(py)
    }
}

impl FromPyObject<'_> for JiffZoned {
    fn extract_bound(dt: &Bound<'_, PyAny>) -> PyResult<JiffZoned> {
        #[cfg(not(Py_LIMITED_API))]
        let dt = dt.downcast::<PyDateTime>()?;
        #[cfg(Py_LIMITED_API)]
        check_type(dt, &DatetimeTypes::get(dt.py()).datetime, "PyDateTime")?;
        let tzinfo = dt.get_tzinfo().map_or_else(
            || {
                Err(PyErr::new::<PyValueError, _>(
                    "expected a datetime with non-None tzinfo",
                ))
            },
            |tz| tz.extract::<JiffTimeZone>(),
        )?;
        let jiff_time = py_time_to_jiff_time(dt)?;
        let jiff_date = date_from_pyobject(dt)?;
        let datetime = DateTime::from_parts(jiff_date, jiff_time);
        let zoned = tzinfo.0.into_ambiguous_zoned(datetime);

        #[cfg(not(Py_LIMITED_API))]
        let fold = dt.get_fold();

        #[cfg(Py_LIMITED_API)]
        let fold = dt.getattr(intern!(dt.py(), "fold"))?.extract::<usize>()? > 0;
        if fold {
            Ok(JiffZoned::from(zoned.later()?))
        } else {
            Ok(JiffZoned::from(zoned.earlier()?))
        }
    }
}
