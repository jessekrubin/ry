use crate::RyZoned;
use crate::errors::map_py_value_err;
use crate::{RyDate, RyDateTime, RyOffset, RySpan, RyTime};
use jiff::civil::Date;
use jiff::tz::TimeZone;
use jiff::{Span, Zoned};
use pyo3::prelude::*;

#[pyfunction]
#[must_use]
pub fn offset(hours: i8) -> RyOffset {
    RyOffset::from(jiff::tz::offset(hours))
}

#[pyfunction]
pub fn date(year: i16, month: i8, day: i8) -> PyResult<RyDate> {
    RyDate::py_new(year, month, day)
}

#[pyfunction]
#[pyo3(signature = (hour=0, minute=0, second=0, nanosecond=0))]
pub fn time(
    hour: Option<i8>,
    minute: Option<i8>,
    second: Option<i8>,
    nanosecond: Option<i32>,
) -> PyResult<RyTime> {
    RyTime::py_new(hour, minute, second, nanosecond)
}

#[pyfunction]
#[pyo3(signature = ( year, month, day, hour=0, minute=0, second=0, nanosecond=0))]
pub fn datetime(
    year: i16,
    month: i8,
    day: i8,
    hour: i8,
    minute: i8,
    second: i8,
    nanosecond: i32,
) -> PyResult<RyDateTime> {
    RyDateTime::py_new(
        year,
        month,
        day,
        Some(hour),
        Some(minute),
        Some(second),
        Some(nanosecond),
    )
}

#[pyfunction]
#[pyo3(signature = (year, month, day, hour=0, minute=0, second=0, nanosecond=0, tz=None))]
#[expect(clippy::too_many_arguments)]
pub fn zoned(
    year: i16,
    month: i8,
    day: i8,
    hour: i8,
    minute: i8,
    second: i8,
    nanosecond: i32,
    tz: Option<&str>,
) -> PyResult<RyZoned> {
    if let Some(tz) = tz {
        // let a =
        Date::new(year, month, day)
            .map_err(map_py_value_err)?
            .at(hour, minute, second, nanosecond)
            .in_tz(tz)
            .map(RyZoned::from)
            .map_err(map_py_value_err)
    } else {
        let tz_system = jiff::tz::TimeZone::try_system().map_err(map_py_value_err)?;
        Date::new(year, month, day)
            .map_err(map_py_value_err)?
            .at(hour, minute, second, nanosecond)
            .to_zoned(tz_system)
            .map(RyZoned::from)
            .map_err(map_py_value_err)
    }
}

/// Return `ZondeDateTime` for the current time in the system's local timezone.
#[pyfunction]
#[must_use]
pub fn now() -> RyZoned {
    RyZoned(Zoned::now())
}

/// Return `ZonedDateTime` for the current time in UTC.
#[pyfunction]
#[must_use]
pub fn utcnow() -> RyZoned {
    RyZoned::from(Zoned::now().with_time_zone(TimeZone::UTC))
}

#[expect(clippy::too_many_arguments)]
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
