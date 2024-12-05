use crate::JiffSignedDuration;
use jiff::SignedDuration;
use pyo3::prelude::*;
use pyo3::types::{PyDelta, PyDeltaAccess};

use pyo3::exceptions::{PyOverflowError, PyValueError};
use pyo3::prelude::*;
use std::convert::TryInto;
const SECONDS_PER_DAY: i64 = 86_400;
const MICROS_PER_DAY: i128 = 86_400_000_000;
const MICROS_PER_SECOND: i64 = 1_000_000;

/// Convert a SignedDuration into a total count of microseconds as an i64.
///
/// This step depends on your SignedDuration implementation. We assume:
/// - `duration.num_seconds()` returns an i64 representing the total whole seconds.
/// - `duration.subsec_micros()` returns a u32 with the fractional microseconds part (0 to <1_000_000).
///
/// For negative durations, we must be careful to ensure that the final result
/// accurately represents the total number of microseconds. For example, if the
/// duration is `-1 day + 1 microsecond`, total_micros should be `-86399999999`.
fn total_microseconds(duration: &SignedDuration) -> PyResult<i64> {
    let secs = duration.as_secs();
    let frac_micros = duration.subsec_micros() as i64;

    // If duration is negative and has a fractional component, we need to subtract
    // that fractional part from the total. For example:
    //   duration = -1 day + 1µs
    //   num_seconds() might be -86400
    //   subsec_micros() = 1
    // The total should be: (-86400 * 1_000_000) + 1 = -86399999999.
    // If the SignedDuration implementation already ensures this behavior correctly
    // (i.e. `num_seconds()` includes the sign and `subsec_micros()` is always the "extra" part),
    // then we can just do a direct calculation:
    let total_micros = secs
        .checked_mul(MICROS_PER_SECOND)
        .and_then(|s| s.checked_add(frac_micros))
        .ok_or_else(|| PyErr::new::<PyValueError, _>("Overflow in microseconds conversion"))?;

    Ok(total_micros)
}

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

pub fn signed_duration_to_pyobject2<'py>(
    py: Python<'py>,
    duration: &SignedDuration,
) -> PyResult<Bound<'py, PyDelta>> {
    // Convert the entire SignedDuration into microseconds (i64).
    // This step depends on your SignedDuration implementation.
    // If num_microseconds is not available, compute manually:
    // total_micros = duration.num_seconds() * 1_000_000 + duration.subsec_micros() as i64
    let total_micros = duration.as_micros();

    // .ok_or_else(|| PyErr::new::<PyValueError, _>("Overflow in microseconds conversion"))?;

    // Constants for normalization
    const MICROS_PER_DAY: i128 = 86_400_000_000; // 24 * 3600 * 1_000_000

    // Normalize into days, seconds, and microseconds in Python's canonical form
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
        // The `false` for `normalize` indicates we've already normalized values.
        PyDelta::new(py, days_i32, seconds_i32, microseconds_i32, false)
    }
    #[cfg(Py_LIMITED_API)]
    {
        Err(PyErr::new::<PyNotImplementedError, _>(
            "not implemented for Py_LIMITED_API",
        ))
    }
}
pub fn signed_duration_to_pyobject_branching<'py>(
    py: Python<'py>,
    duration: &SignedDuration,
) -> PyResult<Bound<'py, PyDelta>> {
    // if duration.is_negative() {
    // }
    let (days_i32, seconds_i32, microseconds) = if duration.is_negative() {
        let total_micros = duration.as_micros();
        // .ok_or_else(|| PyErr::new::<PyValueError, _>("Overflow in microseconds conversion"))?;

        // Constants for normalization
        const MICROS_PER_DAY: i128 = 86_400_000_000; // 24 * 3600 * 1_000_000

        // Normalize into days, seconds, and microseconds in Python's canonical form
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
        let microseconds: i32 = microseconds
            .try_into()
            .map_err(|_| PyErr::new::<PyOverflowError, _>("Overflow in microseconds conversion"))?;

        // let days = duration.as_secs() / SECONDS_PER_DAY;
        // let days_i32 = days
        //     .try_into()
        //     .map_err(|_| PyErr::new::<PyOverflowError, _>("Overflow in days conversion"))?;
        // let seconds = duration.as_secs() % SECONDS_PER_DAY;
        // let seconds_i32 = seconds
        //     .try_into()
        //     .map_err(|_| PyErr::new::<PyOverflowError, _>("Overflow in seconds conversion"))?;
        // let microseconds = duration.subsec_micros();
        (days_i32, seconds_i32, microseconds)
    } else {
        let days = duration.as_secs() / SECONDS_PER_DAY;
        let days_i32 = days
            .try_into()
            .map_err(|_| PyErr::new::<PyOverflowError, _>("Overflow in days conversion"))?;
        let seconds = duration.as_secs() % SECONDS_PER_DAY;
        let seconds_i32 = seconds
            .try_into()
            .map_err(|_| PyErr::new::<PyOverflowError, _>("Overflow in seconds conversion"))?;
        let microseconds = duration.subsec_micros();
        (days_i32, seconds_i32, microseconds)
    };

    #[cfg(not(Py_LIMITED_API))]
    {
        PyDelta::new(
            py,
            days_i32,
            seconds_i32,
            microseconds,
            false,
            // duration.is_negative(),
        )
    }
    #[cfg(Py_LIMITED_API)]
    {
        Err(PyErr::new::<PyNotImplementedError, _>(
            "not implemented for Py_LIMITED_API",
        ))
        // static TIMEDELTA: GILOnceCell<Py<PyType>> = GILOnceCell::new();
        // TIMEDELTA
        //     .import(py, "datetime", "timedelta")?
        //     .call1((days, seconds, microseconds))
    }
}

impl<'py> IntoPyObject<'py> for JiffSignedDuration {
    #[cfg(Py_LIMITED_API)]
    type Target = PyAny;
    #[cfg(not(Py_LIMITED_API))]
    type Target = PyDelta;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        // // Total number of days
        // let days = self.num_days();
        // // Remainder of seconds
        // let secs_dur = self - Duration::days(days);
        // let secs = secs_dur.num_seconds();
        // // Fractional part of the microseconds
        // let micros = (secs_dur - Duration::seconds(secs_dur.num_seconds()))
        //     .num_microseconds()
        //     // This should never panic since we are just getting the fractional
        //     // part of the total microseconds, which should never overflow.
        //     .unwrap();
        //
        // #[cfg(not(Py_LIMITED_API))]
        // {
        //     // We do not need to check the days i64 to i32 cast from rust because
        //     // python will panic with OverflowError.
        //     // We pass true as the `normalize` parameter since we'd need to do several checks here to
        //     // avoid that, and it shouldn't have a big performance impact.
        //     // The seconds and microseconds cast should never overflow since it's at most the number of seconds per day
        //     PyDelta::new(
        //         py,
        //         days.try_into().unwrap_or(i32::MAX),
        //         secs.try_into()?,
        //         micros.try_into()?,
        //         true,
        //     )
        // }
        //
        // #[cfg(Py_LIMITED_API)]
        // {
        //     DatetimeTypes::try_get(py)
        //         .and_then(|dt| dt.timedelta.bind(py).call1((days, secs, micros)))
        // }
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

//
// impl<'py> IntoPyObject<'py> for JiffSignedDuration{
//     type Target = PyAny;
//     type Output = Bound<'py, Self::Target>;
//     type Error = PyErr;
//
//     fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
//
//         // let tz = self.0.to_time_zone();
//         // let tz = JiffTimeZone(tz);
//         // tz.into_pyobject(py)
//         (&self).into_pyobject(py)
//     }
// }
//
// impl<'py> IntoPyObject<'py> for &JiffSignedDuration {
//     type Target = PyAny;
//     type Output = Bound<'py, Self::Target>;
//     type Error = PyErr;
//
//     fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
//         // static ZONE_INFO: GILOnceCell<Py<PyType>> = GILOnceCell::new();
//         // ZONE_INFO
//         //     .import(py, "zoneinfo", "ZoneInfo")
//         //     .and_then(|obj| obj.call1((self.name(),)))
//         //
//         signed_duration_to_pyobject(py, &self.0)
//     }
// }
impl FromPyObject<'_> for JiffSignedDuration {
    fn extract_bound(ob: &Bound<'_, PyAny>) -> PyResult<JiffSignedDuration> {
        let sdur: SignedDuration = signed_duration_from_pyobject(ob.py(), ob)?;
        Ok(JiffSignedDuration(sdur))
    }
}
