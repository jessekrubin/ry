use crate::ry_date::RyDate;
use crate::ry_datetime::RyDateTime;
use crate::ry_offset::RyOffset;
use crate::ry_span::RySpan;
use crate::ry_time::RyTime;
use jiff::Span;
use pyo3::prelude::*;

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
    RyDateTime::py_new(
        year,
        month,
        day,
        Some(hour),
        Some(minute),
        Some(second),
        Some(subsec_nanosecond),
    )
}

#[expect(clippy::too_many_arguments)]
#[pyfunction]
#[pyo3(signature = (*, years=0, months=0, weeks=0, days=0, hours=0, minutes=0, seconds=0, milliseconds=0, microseconds=0, nanoseconds=0, unchecked=false))]
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
    unchecked: bool,
) -> PyResult<RySpan> {
    if unchecked {
        fn apply_if_nonzero(
            span: Span,
            value: i64,
            method: impl FnOnce(Span, i64) -> Span,
        ) -> Span {
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
    } else {
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
}

#[pyfunction]
#[must_use]
pub fn offset(hours: i8) -> RyOffset {
    RyOffset::from(jiff::tz::offset(hours))
}
