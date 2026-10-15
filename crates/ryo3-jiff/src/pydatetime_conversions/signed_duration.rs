use jiff::SignedDuration;
use pyo3::prelude::*;
use pyo3::types::PyDelta;
#[cfg(not(Py_LIMITED_API))]
use pyo3::types::PyDeltaAccess;
use ryo3_macro_rules::py_overflow_error;

use crate::JiffSignedDuration;
const SECONDS_PER_DAY: i64 = 86_400;

pub fn signed_duration_to_pydelta<'py>(
    py: Python<'py>,
    duration: &SignedDuration,
) -> PyResult<Bound<'py, PyDelta>> {
    let total_seconds = duration.as_secs();
    let days: i32 = (total_seconds / (24 * 60 * 60)).try_into()?;
    let seconds: i32 = (total_seconds % (24 * 60 * 60)).try_into()?;
    let microseconds = duration.subsec_micros();

    PyDelta::new(py, days, seconds, microseconds, true)
}

pub fn pydelta_to_signed_duration(delta: &Bound<'_, PyDelta>) -> PyResult<SignedDuration> {
    #[cfg(not(Py_LIMITED_API))]
    let (days, seconds, microseconds) = {
        (
            i64::from(delta.get_days()),
            i64::from(delta.get_seconds()),
            delta.get_microseconds(),
        )
    };

    #[cfg(Py_LIMITED_API)]
    let (days, seconds, microseconds) = {
        let py = delta.py();
        let days = delta.getattr(crate::interns::days(py))?.extract::<i64>()?;
        let seconds = delta
            .getattr(crate::interns::seconds(py))?
            .extract::<i64>()?;
        let microseconds = delta
            .getattr(crate::interns::microseconds(py))?
            .extract::<i32>()?;
        (days, seconds, microseconds)
    };

    // Calculate total seconds
    let total_seconds = days
        .checked_mul(SECONDS_PER_DAY)
        .and_then(|d| d.checked_add(seconds))
        .ok_or_else(|| py_overflow_error!("Overflow in total_seconds calculation"))?;

    // Convert microseconds to nanoseconds
    let nanoseconds = microseconds
        .checked_mul(1_000)
        .ok_or_else(|| py_overflow_error!("Overflow in nanoseconds calculation"))?;
    Ok(SignedDuration::new(total_seconds, nanoseconds))
}

impl<'py> IntoPyObject<'py> for JiffSignedDuration {
    type Target = PyDelta;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        signed_duration_to_pydelta(py, self.inner())
    }
}

impl<'py> IntoPyObject<'py> for &JiffSignedDuration {
    type Target = PyDelta;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    #[inline]
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        signed_duration_to_pydelta(py, self.inner())
    }
}

impl<'py> FromPyObject<'_, 'py> for JiffSignedDuration {
    type Error = PyErr;
    fn extract(obj: Borrowed<'_, 'py, PyAny>) -> PyResult<Self> {
        let delta = obj.cast::<PyDelta>()?;
        pydelta_to_signed_duration(&delta).map(Self::from)
    }
}
