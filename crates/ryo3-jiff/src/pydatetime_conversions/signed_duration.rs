use crate::JiffSignedDuration;
use jiff::SignedDuration;
use pyo3::prelude::*;
use pyo3::types::{PyDelta, PyDeltaAccess};

use pyo3::exceptions::PyOverflowError;
use std::convert::TryInto;
const SECONDS_PER_DAY: i64 = 86_400;
const MICROS_PER_DAY: i128 = 86_400_000_000;

pub fn signed_duration_to_pyobject<'py>(
    py: Python<'py>,
    duration: &SignedDuration,
) -> PyResult<Bound<'py, PyDelta>> {
    let total_micros = duration.as_micros();
    // total_microseconds(duration)?;

    let days = total_micros.div_euclid(MICROS_PER_DAY);
    let remainder = total_micros.rem_euclid(MICROS_PER_DAY);
    let seconds = remainder.div_euclid(1_000_000);
    let microseconds = remainder.rem_euclid(1_000_000);

    let days_i32: i32 = days
        .try_into()
        .map_err(|_| PyErr::new::<PyOverflowError, _>("Overflow in days conversion"))?;
    let seconds_i32: i32 = seconds
        .try_into()
        .map_err(|_| PyErr::new::<PyOverflowError, _>("Overflow in seconds conversion"))?;
    let microseconds_i32: i32 = microseconds
        .try_into()
        .map_err(|_| PyErr::new::<PyOverflowError, _>("Overflow in microseconds conversion"))?;

    #[cfg(not(Py_LIMITED_API))]
    {
        // `normalize = false` because we've already normalized the values.
        PyDelta::new(py, days_i32, seconds_i32, microseconds_i32, false)
    }
    #[cfg(Py_LIMITED_API)]
    {
        Err(PyErr::new::<PyNotImplementedError, _>(
            "not implemented for Py_LIMITED_API",
        ))
    }
}

pub fn signed_duration_from_pyobject<'py>(
    _py: Python<'py>,
    obj: &Bound<'py, PyAny>,
) -> PyResult<SignedDuration> {
    #[cfg(not(Py_LIMITED_API))]
    let (days, seconds, microseconds) = {
        let delta = obj.downcast::<PyDelta>()?;
        (
            delta.get_days(),
            delta.get_seconds(),
            delta.get_microseconds(),
        )
    };
    #[cfg(Py_LIMITED_API)]
    let (days, seconds, microseconds): (i32, i32, i32) = {
        (
            obj.getattr("days")?.extract()?,
            obj.getattr("seconds")?.extract()?,
            obj.getattr("microseconds")?.extract()?,
        )
    };

    // Convert to i64 to handle negative durations
    let days = i64::from(days);
    let seconds = i64::from(seconds);
    let microseconds = i64::from(microseconds);

    // Calculate total seconds
    let total_seconds = days
        .checked_mul(SECONDS_PER_DAY)
        .and_then(|d| d.checked_add(seconds))
        .ok_or_else(|| PyErr::new::<PyOverflowError, _>("Overflow in total_seconds calculation"))?;

    // Convert microseconds to nanoseconds
    let nanoseconds = microseconds
        .checked_mul(1_000)
        .ok_or_else(|| PyErr::new::<PyOverflowError, _>("Overflow in nanoseconds calculation"))?;

    // Check if total_seconds fits in i64
    if !(i64::MIN..=i64::MAX).contains(&total_seconds) {
        return Err(PyErr::new::<PyOverflowError, _>(
            "Duration too large to represent",
        ));
    }

    // Ensure nanoseconds is within i32 range
    let nanoseconds = i32::try_from(nanoseconds)
        .map_err(|_| PyErr::new::<PyOverflowError, _>("Nanoseconds out of range"))?;

    Ok(SignedDuration::new(total_seconds, nanoseconds))
}

impl<'py> IntoPyObject<'py> for JiffSignedDuration {
    #[cfg(Py_LIMITED_API)]
    type Target = PyAny;
    #[cfg(not(Py_LIMITED_API))]
    type Target = PyDelta;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        signed_duration_to_pyobject(py, &self.0)
    }
}

impl<'py> IntoPyObject<'py> for &JiffSignedDuration {
    #[cfg(Py_LIMITED_API)]
    type Target = PyAny;
    #[cfg(not(Py_LIMITED_API))]
    type Target = PyDelta;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    #[inline]
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        signed_duration_to_pyobject(py, &self.0)
    }
}

impl FromPyObject<'_> for JiffSignedDuration {
    fn extract_bound(ob: &Bound<'_, PyAny>) -> PyResult<JiffSignedDuration> {
        let sdur: SignedDuration = signed_duration_from_pyobject(ob.py(), ob)?;
        Ok(JiffSignedDuration(sdur))
    }
}
