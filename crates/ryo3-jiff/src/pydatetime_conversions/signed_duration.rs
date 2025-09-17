use crate::JiffSignedDuration;
use jiff::SignedDuration;
use pyo3::prelude::*;
use pyo3::types::PyDelta;

#[cfg(not(Py_LIMITED_API))]
use pyo3::types::PyDeltaAccess;

use pyo3::exceptions::PyOverflowError;
const SECONDS_PER_DAY: i64 = 86_400;

pub fn signed_duration_to_pyobject<'py>(
    py: Python<'py>,
    duration: &SignedDuration,
) -> PyResult<Bound<'py, PyDelta>> {
    duration.into_pyobject(py)
}

pub fn signed_duration_from_pyobject(obj: &Bound<'_, PyAny>) -> PyResult<SignedDuration> {
    let delta = obj.cast::<PyDelta>()?;
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
        use pyo3::intern;

        let py = delta.py();
        let days = delta.getattr(crate::interns::days(py))?.extract::<i64>()?;
        let seconds = delta
            .getattr(crate::interns::seconds(py))?
            .extract::<i64>()?;
        let microseconds = obj
            .getattr(crate::interns::microseconds(py))?
            .extract::<i32>()?;
        (days, seconds, microseconds)
    };

    // Calculate total seconds
    let total_seconds = days
        .checked_mul(SECONDS_PER_DAY)
        .and_then(|d| d.checked_add(seconds))
        .ok_or_else(|| PyErr::new::<PyOverflowError, _>("Overflow in total_seconds calculation"))?;

    // Convert microseconds to nanoseconds
    let nanoseconds = microseconds
        .checked_mul(1_000)
        .ok_or_else(|| PyErr::new::<PyOverflowError, _>("Overflow in nanoseconds calculation"))?;
    Ok(SignedDuration::new(total_seconds, nanoseconds))
}

impl<'py> IntoPyObject<'py> for JiffSignedDuration {
    type Target = PyDelta;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        signed_duration_to_pyobject(py, &self.0)
    }
}

impl<'py> IntoPyObject<'py> for &JiffSignedDuration {
    type Target = PyDelta;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    #[inline]
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        signed_duration_to_pyobject(py, &self.0)
    }
}

impl FromPyObject<'_> for JiffSignedDuration {
    fn extract_bound(ob: &Bound<'_, PyAny>) -> PyResult<Self> {
        let sdur: SignedDuration = signed_duration_from_pyobject(ob)?;
        Ok(Self(sdur))
    }
}
