#![deny(clippy::all)]
#![deny(clippy::correctness)]
#![deny(clippy::panic)]
#![deny(clippy::perf)]
#![deny(clippy::pedantic)]
#![deny(clippy::style)]
#![deny(clippy::unwrap_used)]
#![warn(clippy::must_use_candidate)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::unnecessary_wraps)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::unused_self)]

mod dev;
mod internal;
mod pydatetime_conversions;
mod ry_date;
mod ry_datetime;
mod ry_signed_duration;
mod ry_span;
mod ry_time;
mod ry_timestamp;
mod ry_timezone;
mod ry_zoned;

use crate::dev::RyDateTimeRound;
use crate::ry_date::RyDate;
use crate::ry_datetime::RyDateTime;
use crate::ry_signed_duration::RySignedDuration;
use crate::ry_span::RySpan;
use crate::ry_timestamp::RyTimestamp;
use crate::ry_timezone::RyTimeZone;
use crate::ry_zoned::RyZoned;
use pyo3::prelude::PyModule;
use pyo3::prelude::*;
use ry_time::RyTime;

#[pyfunction]
pub fn date(year: i16, month: i8, day: i8) -> PyResult<RyDate> {
    RyDate::new(year, month, day)
}

#[pyfunction]
#[pyo3(signature = (hour=0, minute=0, second=0, nanosecond=0))]
pub fn time(
    hour: Option<i8>,
    minute: Option<i8>,
    second: Option<i8>,
    nanosecond: Option<i32>,
) -> PyResult<RyTime> {
    RyTime::new(hour, minute, second, nanosecond)
}

#[pyfunction]
#[pyo3(signature = ( year, month, day, hour=0, minute=0, second=0, subsec_nanosecond=0))]
pub fn datetime(
    year: i16,
    month: i8,
    day: i8,
    hour: i8,
    minute: i8,
    second: i8,
    subsec_nanosecond: i32,
) -> PyResult<RyDateTime> {
    RyDateTime::new(
        year,
        month,
        day,
        Some(hour),
        Some(minute),
        Some(second),
        Some(subsec_nanosecond),
    )
}
pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // classes
    m.add_class::<RyDate>()?;
    m.add_class::<RyDateTime>()?;
    m.add_class::<RySignedDuration>()?;
    m.add_class::<RySpan>()?;
    m.add_class::<RyTime>()?;
    m.add_class::<RyTimeZone>()?;
    m.add_class::<RyTimestamp>()?;
    m.add_class::<RyZoned>()?;
    m.add_class::<RyDateTimeRound>()?;

    // functions
    m.add_function(wrap_pyfunction!(date, m)?)?;
    m.add_function(wrap_pyfunction!(time, m)?)?;
    m.add_function(wrap_pyfunction!(datetime, m)?)?;

    // okee-dokey
    Ok(())
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]
    #[test]
    fn test_dev() {
        assert_eq!(1 + 1, 2);
    }
}
