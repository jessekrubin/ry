use crate::pydatetime_conversions::{date_from_pyobject, py_time_to_jiff_time};
use crate::{JiffOffset, JiffTimeZone, JiffZoned};
use jiff::civil::DateTime;
use jiff::tz::Offset;
use pyo3::exceptions::{PyTypeError, PyValueError};
use pyo3::sync::GILOnceCell;
use pyo3::types::{PyAnyMethods, PyDateTime, PyTzInfo, PyTzInfoAccess};
use pyo3::types::{PyNone, PyType};
use pyo3::{Bound, FromPyObject, IntoPyObject, Py, PyAny, PyErr, PyResult, Python};
use std::time::Duration;

pub fn zoned_to_pyobject<'a>(
    py: Python<'a>,
    datetime: &jiff::Zoned,
) -> PyResult<Bound<'a, PyDateTime>> {
    let year = i32::from(datetime.year());
    let m_u8 = u8::try_from(datetime.month())
        .map_err(|e| PyErr::new::<PyValueError, _>(format!("{e}")))?;
    let d_u8 =
        u8::try_from(datetime.day()).map_err(|e| PyErr::new::<PyValueError, _>(format!("{e}")))?;
    let hour_u8 = u8::try_from(datetime.hour())
        .map_err(|e| PyErr::new::<PyValueError, _>(format!("hour: {e}")))?;
    let minute_u8 = u8::try_from(datetime.minute())
        .map_err(|e| PyErr::new::<PyValueError, _>(format!("minute: {e}")))?;
    let second_u8 = u8::try_from(datetime.second())
        .map_err(|e| PyErr::new::<PyValueError, _>(format!("second: {e}")))?;
    let microsecond_u32 = u32::try_from(datetime.microsecond())
        .map_err(|e| PyErr::new::<PyValueError, _>(format!("microsecond: {e}")))?;
    // let tz = datetime.time_zone();
    // let pytzinfo  =  TODO: implement pytzinfo
    PyDateTime::new(
        py,
        year,
        m_u8,
        d_u8,
        hour_u8,
        minute_u8,
        second_u8,
        microsecond_u32,
        None,
    )
}

// impl<'py> IntoPyObject<'py> for JiffOffset {
//     #[cfg(Py_LIMITED_API)]
//     type Target = PyAny;
//     #[cfg(not(Py_LIMITED_API))]
//     type Target = PyTzInfo;
//     type Output = Bound<'py, Self::Target>;
//     type Error = PyErr;
//
//     fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
//         // let seconds_offset = self.local_minus_utc();
//         let seconds_offset = self.0.seconds();
//         // #[cfg(not(Py_LIMITED_API))]
//         // {
//         //     let td = PyDelta::new(py, 0, seconds_offset, 0, true)?;
//         // //     make the timezone object
//         //     let tz = PyTzInfo::new(py, td, None)?;
//         //     Ok(tz)
//         //
//         // }
//
//         {
//             // let td = Duration::seconds(seconds_offset.into()).into_pyobject(py)?;
//             // DatetimeTypes::try_get(py).and_then(|dt| dt.timezone.bind(py).call1((td,)))
//         }
//     }
// }
impl<'py> IntoPyObject<'py> for JiffTimeZone {
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        static ZONE_INFO: GILOnceCell<Py<PyType>> = GILOnceCell::new();
        ZONE_INFO
            .import(py, "zoneinfo", "ZoneInfo")
            .and_then(|obj| obj.call1((self.0.iana_name(),)))
    }
}

impl<'py> IntoPyObject<'py> for JiffOffset {
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        // static ZONE_INFO: GILOnceCell<Py<PyType>> = GILOnceCell::new();
        // ZONE_INFO
        //     .import(py, "zoneinfo", "ZoneInfo")
        //     .and_then(|obj| obj.call1((self.name(),)))
        //
        let tz = self.0.to_time_zone();
        let tz = JiffTimeZone(tz);
        tz.into_pyobject(py)
    }
}
// impl<'py> IntoPyObject<'py> for &JiffOffset {
//     #[cfg(Py_LIMITED_API)]
//     type Target = PyAny;
//     #[cfg(not(Py_LIMITED_API))]
//     type Target = PyTzInfo;
//     type Output = Bound<'py, Self::Target>;
//     type Error = PyErr;
//
//     #[inline]
//     fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
//         (*self).into_pyobject(py)
//     }
// }

impl FromPyObject<'_> for JiffOffset {
    /// Convert python tzinfo to rust [`FixedOffset`].
    ///
    /// Note that the conversion will result in precision lost in microseconds as chrono offset
    /// does not supports microseconds.
    fn extract_bound(ob: &Bound<'_, PyAny>) -> PyResult<JiffOffset> {
        #[cfg(not(Py_LIMITED_API))]
        let ob = ob.downcast::<PyTzInfo>()?;
        #[cfg(Py_LIMITED_API)]
        check_type(ob, &DatetimeTypes::get(ob.py()).tzinfo, "PyTzInfo")?;

        // Passing Python's None to the `utcoffset` function will only
        // work for timezones defined as fixed offsets in Python.
        // Any other timezone would require a datetime as the parameter, and return
        // None if the datetime is not provided.
        // Trying to convert None to a PyDelta in the next line will then fail.
        let py_timedelta = ob.call_method1("utcoffset", (PyNone::get(ob.py()),))?;
        if py_timedelta.is_none() {
            return Err(PyTypeError::new_err(format!(
                "{ob:?} is not a fixed offset timezone"
            )));
        }
        let total_seconds: Duration = py_timedelta.extract()?;
        // This cast is safe since the timedelta is limited to -24 hours and 24 hours.

        let total_seconds = i32::try_from(total_seconds.as_secs())
            .map_err(|e| PyErr::new::<PyValueError, _>(format!("{e}")))?;
        let o = Offset::from_seconds(total_seconds)
            .map_err(|e| PyErr::new::<PyValueError, _>(format!("{e}")))?;
        Ok(JiffOffset::from(o))
    }
}

impl<'py> IntoPyObject<'py> for &JiffZoned {
    #[cfg(Py_LIMITED_API)]
    type Target = PyAny;
    #[cfg(not(Py_LIMITED_API))]
    type Target = PyDateTime;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        // let tz = self.offset().fix().into_pyobject(py)?;
        let tz = self.0.time_zone();
        let pytz = JiffTimeZone(tz.clone()).into_pyobject(py)?;
        // downcast to tz
        let tz = pytz.downcast::<PyTzInfo>()?;

        let year = i32::from(self.0.year());
        let m_u8 = u8::try_from(self.0.month())
            .map_err(|e| PyErr::new::<PyValueError, _>(format!("{e}")))?;
        let d_u8 = u8::try_from(self.0.day())
            .map_err(|e| PyErr::new::<PyValueError, _>(format!("{e}")))?;
        let hour_u8 = u8::try_from(self.0.hour())
            .map_err(|e| PyErr::new::<PyValueError, _>(format!("hour: {e}")))?;
        let minute_u8 = u8::try_from(self.0.minute())
            .map_err(|e| PyErr::new::<PyValueError, _>(format!("minute: {e}")))?;
        let second_u8 = u8::try_from(self.0.second())
            .map_err(|e| PyErr::new::<PyValueError, _>(format!("second: {e}")))?;
        let microsecond_u32 = u32::try_from(self.0.microsecond())
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
            None,
        );

        // if truncated_leap_second {
        //     warn_truncated_leap_second(&datetime);
        // }

        Ok(datetime)
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
        let jiff_date = date_from_pyobject(dt)?;
        let jiff_time = py_time_to_jiff_time(dt)?;
        let dt = DateTime::from_parts(jiff_date, jiff_time);
        let zdt = dt
            .intz(tz.as_str())
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e}")))?;
        Ok(JiffZoned(zdt))
        // Ok(JiffZoned::from(zdt))
    }
}
