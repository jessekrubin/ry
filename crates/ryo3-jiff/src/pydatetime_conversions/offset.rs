use crate::{JiffOffset, JiffTimeZone};
use jiff::SignedDuration;
use jiff::tz::Offset;
use pyo3::exceptions::{PyTypeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::{PyNone, PyTzInfo};

impl<'py> IntoPyObject<'py> for JiffOffset {
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (&self).into_pyobject(py)
    }
}

impl<'py> IntoPyObject<'py> for &JiffOffset {
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        let tz = self.0.to_time_zone();
        let tz = JiffTimeZone(tz);
        tz.into_pyobject(py)
    }
}

impl FromPyObject<'_> for JiffOffset {
    /// Convert python tzinfo to rust [`FixedOffset`].
    ///
    /// Note that the conversion will result in precision lost in microseconds as chrono offset
    /// does not supports microseconds.
    fn extract_bound(ob: &Bound<'_, PyAny>) -> PyResult<Self> {
        let ob = ob.cast::<PyTzInfo>()?;

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
        let total_seconds: SignedDuration = py_timedelta.extract()?;
        // This cast is safe since the timedelta is limited to -24 hours and 24 hours.

        let total_seconds = i32::try_from(total_seconds.as_secs())
            .map_err(|e| PyErr::new::<PyValueError, _>(format!("{e}")))?;
        let o = Offset::from_seconds(total_seconds)
            .map_err(|e| PyErr::new::<PyValueError, _>(format!("{e}")))?;
        Ok(Self::from(o))
    }
}
