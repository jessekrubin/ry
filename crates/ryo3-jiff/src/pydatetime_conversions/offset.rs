use crate::{JiffOffset, JiffTimeZone};
use jiff::tz::Offset;
use pyo3::exceptions::{PyTypeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::{PyNone, PyTzInfo};
use std::time::Duration;

impl<'py> IntoPyObject<'py> for JiffOffset {
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        // let tz = self.0.to_time_zone();
        // let tz = JiffTimeZone(tz);
        // tz.into_pyobject(py)
        (&self).into_pyobject(py)
    }
}

impl<'py> IntoPyObject<'py> for &JiffOffset {
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
