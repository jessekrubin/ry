use jiff::civil::Date;
use jiff::tz::TimeZone;
use jiff::{Span, Zoned};
use pyo3::prelude::*;
use ryo3_core::map_py_value_err;
use ryo3_macro_rules::py_overflow_error;

use crate::{RyDate, RyDateTime, RyOffset, RySpan, RyTime, RyZoned};

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
#[pyo3(signature = (hour = 0, minute = 0, second = 0, nanosecond = 0))]
pub fn time(hour: i8, minute: i8, second: i8, nanosecond: i32) -> PyResult<RyTime> {
    RyTime::py_new(hour, minute, second, nanosecond)
}

#[pyfunction]
#[pyo3(signature = ( year, month, day, hour = 0, minute = 0, second = 0, nanosecond = 0))]
pub fn datetime(
    year: i16,
    month: i8,
    day: i8,
    hour: i8,
    minute: i8,
    second: i8,
    nanosecond: i32,
) -> PyResult<RyDateTime> {
    RyDateTime::py_new(year, month, day, hour, minute, second, nanosecond)
}

#[pyfunction]
#[pyo3(signature = (year, month, day, hour = 0, minute = 0, second = 0, nanosecond = 0, tz = None))]
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
        let tz = crate::ry_timezone::get_time_zone(tz).map_err(map_py_value_err)?;
        Date::new(year, month, day)
            .map_err(map_py_value_err)?
            .at(hour, minute, second, nanosecond)
            .to_zoned(tz)
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
#[inline]
pub(crate) fn span(
    years: i16,
    months: i32,
    weeks: i32,
    days: i32,
    hours: i32,
    minutes: i64,
    seconds: i64,
    milliseconds: i64,
    microseconds: i64,
    nanoseconds: i64,
) -> PyResult<jiff::Span> {
    macro_rules! apply_if_nonzero {
        ($span:ident, $value:ident, $method:ident, $name:literal) => {
            if $value != 0 {
                $span = $span
                    .$method($value)
                    .map_err(|e| py_overflow_error!("span-overflow ({}): {e}", $name))?;
            }
        };
    }
    let mut span = Span::new();
    apply_if_nonzero!(span, years, try_years, "years");
    apply_if_nonzero!(span, months, try_months, "months");
    apply_if_nonzero!(span, weeks, try_weeks, "weeks");
    apply_if_nonzero!(span, days, try_days, "days");
    apply_if_nonzero!(span, hours, try_hours, "hours");
    apply_if_nonzero!(span, minutes, try_minutes, "minutes");
    apply_if_nonzero!(span, seconds, try_seconds, "seconds");
    apply_if_nonzero!(span, milliseconds, try_milliseconds, "milliseconds");
    apply_if_nonzero!(span, microseconds, try_microseconds, "microseconds");
    apply_if_nonzero!(span, nanoseconds, try_nanoseconds, "nanoseconds");
    Ok(span)
}

#[expect(clippy::too_many_arguments)]
#[pyfunction]
#[pyo3(
    signature = (
        *,
        years = 0,
        months = 0,
        weeks = 0,
        days = 0,
        hours = 0,
        minutes = 0,
        seconds = 0,
        milliseconds = 0,
        microseconds = 0,
        nanoseconds = 0
    )
)]
pub fn timespan(
    years: i16,
    months: i32,
    weeks: i32,
    days: i32,
    hours: i32,
    minutes: i64,
    seconds: i64,
    milliseconds: i64,
    microseconds: i64,
    nanoseconds: i64,
) -> PyResult<RySpan> {
    span(
        years,
        months,
        weeks,
        days,
        hours,
        minutes,
        seconds,
        milliseconds,
        microseconds,
        nanoseconds,
    )
    .map(RySpan::from)
}
