use jiff::SignedDuration;
use pyo3::exceptions::PyOverflowError;
use pyo3::prelude::PyAnyMethods;
use pyo3::types::{PyDelta, PyDeltaAccess};
use pyo3::{Bound, PyErr, PyResult, Python};

const SECONDS_PER_DAY: i64 = 86_400;

pub fn signed_duration_from_pyobject<'py>(
    _py: Python<'py>,
    obj: &Bound<'py, PyDelta>,
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

// impl<'py> IntoPyObject<'py> for RySignedDuration {
//     #[cfg(not(Py_LIMITED_API))]
//     type Target = PyDelta;
//     #[cfg(Py_LIMITED_API)]
//     type Target = PyAny;
//     type Output = Bound<'py, Self::Target>;
//     type Error = PyErr;
//
//     fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
//         let days = self.0.as_secs() / SECONDS_PER_DAY;
//         let seconds = self.0.as_secs() % SECONDS_PER_DAY;
//         let microseconds = self.0.subsec_micros();
//
//         #[cfg(not(Py_LIMITED_API))]
//         {
//             PyDelta::new(
//                 py,
//                 days.try_into()?,
//                 seconds.try_into().unwrap(),
//                 microseconds.try_into().unwrap(),
//                 false,
//             )
//         }
//         #[cfg(Py_LIMITED_API)]
//         {
//             static TIMEDELTA: GILOnceCell<Py<PyType>> = GILOnceCell::new();
//             TIMEDELTA
//                 .import(py, "datetime", "timedelta")?
//                 .call1((days, seconds, microseconds))
//         }
//     }
// }
//
// impl<'py> IntoPyObject<'py> for &RySignedDuration {
//     #[cfg(not(Py_LIMITED_API))]
//     type Target = PyDelta;
//     #[cfg(Py_LIMITED_API)]
//     type Target = PyAny;
//     type Output = Bound<'py, Self::Target>;
//     type Error = PyErr;
//
//     #[inline]
//     fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
//         self.into_pyobject(py)
//     }
// }

pub fn signed_duration_to_pyobject<'py>(
    py: Python<'py>,
    duration: &SignedDuration,
) -> PyResult<Bound<'py, PyDelta>> {
    let days = duration.as_secs() / SECONDS_PER_DAY;
    let days_i32 = days
        .try_into()
        .map_err(|_| PyErr::new::<PyOverflowError, _>("Overflow in days conversion"))?;
    let seconds = duration.as_secs() % SECONDS_PER_DAY;
    let seconds_i32 = seconds
        .try_into()
        .map_err(|_| PyErr::new::<PyOverflowError, _>("Overflow in seconds conversion"))?;
    let microseconds = duration.subsec_micros();

    #[cfg(not(Py_LIMITED_API))]
    {
        PyDelta::new(py, days_i32, seconds_i32, microseconds, false)
    }
    #[cfg(Py_LIMITED_API)]
    {
        static TIMEDELTA: GILOnceCell<Py<PyType>> = GILOnceCell::new();
        TIMEDELTA
            .import(py, "datetime", "timedelta")?
            .call1((days, seconds, microseconds))
    }
}
