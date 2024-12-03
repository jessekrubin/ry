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

mod delta_arithmetic_self;
mod dev;
mod dev_sandbox;
mod internal;
mod nujiff;
pub use nujiff::*;
mod errors;
mod intz;
pub mod pydatetime_conversions;
mod ry_date;
mod ry_datetime;
mod ry_offset;
mod ry_signed_duration;
mod ry_span;
mod ry_time;
mod ry_timestamp;
mod ry_timezone;
mod ry_zoned;

use crate::dev::RyDateTimeRound;
use crate::ry_date::RyDate;
use crate::ry_datetime::RyDateTime;
use crate::ry_offset::RyOffset;
use crate::ry_signed_duration::RySignedDuration;
use crate::ry_span::RySpan;
use crate::ry_timestamp::RyTimestamp;
use crate::ry_timezone::RyTimeZone;
use crate::ry_zoned::RyZoned;
use jiff::Span;
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

#[allow(clippy::too_many_arguments)]
#[pyfunction]
#[pyo3(signature = (*, years=0, months=0, weeks=0, days=0, hours=0, minutes=0, seconds=0, milliseconds=0, microseconds=0, nanoseconds=0))]
pub fn timespan(
    years: i64,
    months: i64,
    weeks: i64,
    days: i64,
    hours: i64,
    minutes: i64,
    seconds: i64,
    milliseconds: i64,
    microseconds: i64,
    nanoseconds: i64,
) -> PyResult<RySpan> {
    fn apply_if_nonzero(
        span: Span,
        // , PyErr>,
        value: i64,
        method: impl FnOnce(Span, i64) -> Result<Span, jiff::Error>,
        name: &str,
    ) -> Result<Span, PyErr> {
        if value != 0 {
            // span.and_then(|s| {
            method(span, value).map_err(|e| {
                PyErr::new::<pyo3::exceptions::PyOverflowError, _>(format!(
                    "span-overflow: {name}: {e}"
                ))
            })
        } else {
            Ok(span)
        }
    }

    let span = Span::new();
    let span = apply_if_nonzero(span, years, Span::try_years, "years")?;
    let span = apply_if_nonzero(span, months, Span::try_months, "months")?;
    let span = apply_if_nonzero(span, weeks, Span::try_weeks, "weeks")?;
    let span = apply_if_nonzero(span, days, Span::try_days, "days")?;
    let span = apply_if_nonzero(span, hours, Span::try_hours, "hours")?;
    let span = apply_if_nonzero(span, minutes, Span::try_minutes, "minutes")?;
    let span = apply_if_nonzero(span, seconds, Span::try_seconds, "seconds")?;
    let span = apply_if_nonzero(span, milliseconds, Span::try_milliseconds, "milliseconds")?;
    let span = apply_if_nonzero(span, microseconds, Span::try_microseconds, "microseconds")?;
    let span = apply_if_nonzero(span, nanoseconds, Span::try_nanoseconds, "nanoseconds")?;

    Ok(RySpan::from(span))
}

#[allow(clippy::too_many_arguments)]
#[pyfunction]
#[pyo3(signature = (*, years=0, months=0, weeks=0, days=0, hours=0, minutes=0, seconds=0, milliseconds=0, microseconds=0, nanoseconds=0))]
pub fn timespan_unchecked(
    years: i64,
    months: i64,
    weeks: i64,
    days: i64,
    hours: i64,
    minutes: i64,
    seconds: i64,
    milliseconds: i64,
    microseconds: i64,
    nanoseconds: i64,
) -> PyResult<RySpan> {
    fn apply_if_nonzero(span: Span, value: i64, method: impl FnOnce(Span, i64) -> Span) -> Span {
        if value != 0 {
            // span.and_then(|s| {
            method(span, value)
        } else {
            span
        }
    }

    let span = Span::new();
    let span = apply_if_nonzero(span, years, Span::years);
    let span = apply_if_nonzero(span, months, Span::months);
    let span = apply_if_nonzero(span, weeks, Span::weeks);
    let span = apply_if_nonzero(span, days, Span::days);
    let span = apply_if_nonzero(span, hours, Span::hours);
    let span = apply_if_nonzero(span, minutes, Span::minutes);
    let span = apply_if_nonzero(span, seconds, Span::seconds);
    let span = apply_if_nonzero(span, milliseconds, Span::milliseconds);
    let span = apply_if_nonzero(span, microseconds, Span::microseconds);
    let span = apply_if_nonzero(span, nanoseconds, Span::nanoseconds);

    Ok(RySpan::from(span))
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
    m.add_class::<RyOffset>()?;

    // functions
    m.add_function(wrap_pyfunction!(date, m)?)?;
    m.add_function(wrap_pyfunction!(time, m)?)?;
    m.add_function(wrap_pyfunction!(datetime, m)?)?;
    m.add_function(wrap_pyfunction!(ry_offset::offset, m)?)?;
    m.add_function(wrap_pyfunction!(timespan, m)?)?;
    m.add_function(wrap_pyfunction!(timespan_unchecked, m)?)?;

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
