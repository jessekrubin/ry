use crate::pydatetime_conversions::timezone::timezone2pyobect;
use crate::pydatetime_conversions::{date_from_pyobject, py_time_to_jiff_time};
use crate::JiffZoned;
use jiff::civil::DateTime;
use jiff::tz::TimeZone;
use jiff::Zoned;
use pyo3::exceptions::{PyTypeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::{PyDateTime, PyTzInfo, PyTzInfoAccess};

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

        #[cfg(not(Py_LIMITED_API))]
        let tzinfo = dt.get_tzinfo();
        #[cfg(Py_LIMITED_API)]
        let tzinfo: Option<Bound<'_, PyAny>> = dt.getattr(intern!(dt.py(), "tzinfo"))?.extract()?;

        let Some(tzinfo) = tzinfo else {
            return Err(PyTypeError::new_err(
                "expected a datetime with non-None tzinfo",
            ));
        };
        let tz = tzinfo.to_string();

        let tz_str = if tz.ends_with("/etc/localtime") {
            let systz = TimeZone::system();
            let systz_thing = systz.iana_name().unwrap_or("UTC").to_string();
            systz_thing
        } else {
            tz
        };
        let jiff_date = date_from_pyobject(dt)?;
        let jiff_time = py_time_to_jiff_time(dt)?;
        let dt = DateTime::from_parts(jiff_date, jiff_time);
        let zdt = dt
            .in_tz(&tz_str)
            .map_err(|e| PyErr::new::<PyValueError, _>(format!("{e}")))?;
        Ok(JiffZoned(zdt))
    }
}
