use crate::jiff_types::JiffDateTime;
use crate::pydatetime_conversions::{py_date_to_date, py_time_to_jiff_time};
use jiff::civil::DateTime;
use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;
use pyo3::types::PyDateTime;
#[cfg(not(Py_LIMITED_API))]
use pyo3::types::PyTzInfoAccess;

// fn datetime_to_pyobject<'a>(
//     py: Python<'a>,
//     datetime: &DateTime,
// ) -> PyResult<Bound<'a, PyDateTime>> {
//     let year = i32::from(datetime.year());
//     let m_u8 = u8::try_from(datetime.month())
//         .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e}")))?;
//     let d_u8 = u8::try_from(datetime.day())
//         .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e}")))?;
//     let hour_u8 = u8::try_from(datetime.hour())
//         .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("hour: {e}")))?;
//     let minute_u8 = u8::try_from(datetime.minute())
//         .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("minute: {e}")))?;
//     let second_u8 = u8::try_from(datetime.second())
//         .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("second: {e}")))?;
//     let microsecond_u32 = u32::try_from(datetime.microsecond()).map_err(|e| {
//         PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("microsecond: {e}"))
//     })?;
//     #[cfg(not(Py_LIMITED_API))]
//     let py_datetime = PyDateTime::new(
//         py,
//         year,
//         m_u8,
//         d_u8,
//         hour_u8,
//         minute_u8,
//         second_u8,
//         microsecond_u32,
//         None,
//     )?;

//     #[cfg(Py_LIMITED_API)]
//     let py_datetime = DatetimeTypes::try_get(py).and_then(|dt| {
//         dt.datetime.bind(py).call1((
//             year,
//             m_u8,
//             d_u8,
//             hour_u8,
//             minute_u8,
//             second_u8,
//             microsecond_u32,
//         ))
//     })?;
//     Ok(py_datetime)
// }

impl<'py> IntoPyObject<'py> for JiffDateTime {
    // #[cfg(Py_LIMITED_API)]
    // type Target = PyAny;
    // #[cfg(not(Py_LIMITED_API))]
    type Target = PyDateTime;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (&self).into_pyobject(py)
    }
}

impl<'py> IntoPyObject<'py> for &JiffDateTime {
    // #[cfg(Py_LIMITED_API)]
    // type Target = PyAny;
    // #[cfg(not(Py_LIMITED_API))]
    type Target = PyDateTime;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    #[inline]
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        self.0.into_pyobject(py)
    }
}

impl FromPyObject<'_> for JiffDateTime {
    fn extract_bound(dt: &Bound<'_, PyAny>) -> PyResult<JiffDateTime> {
        let dt = dt.downcast::<PyDateTime>()?;

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
