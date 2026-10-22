//! raw python <-> jiff conversion functions
//!
//! these hold the actual conversion logic; the `IntoPyObject`/`FromPyObject`
//! impls in `to_py.rs`/`from_py.rs` are thin wrappers around them
use std::sync::LazyLock;

use jiff::civil::{Date, DateTime, Time};
use jiff::tz::{AmbiguousOffset, Offset, TimeZone, TimeZoneDatabase};
use jiff::{SignedDuration, Zoned};
use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;
use pyo3::types::{
    PyDate, PyDateAccess, PyDateTime, PyDelta, PyDeltaAccess, PyTime, PyTimeAccess, PyTzInfo,
    PyTzInfoAccess,
};
use ryo3_core::map_py_value_err;
use ryo3_macro_rules::{py_overflow_error, py_type_err};

use crate::{JiffSignedDuration, JiffTimeZone};

// ============================================================================
// TZ DATABASE
// ============================================================================
static BUNDLED_TZDB: LazyLock<TimeZoneDatabase> = LazyLock::new(TimeZoneDatabase::bundled);

/// jiff's bundled tz database
#[must_use]
pub fn bundled_tzdb() -> &'static TimeZoneDatabase {
    &BUNDLED_TZDB
}

/// get a time zone from the system tz database, falling back to the bundled
/// database (for systems without one, ie windows)
///
/// # Errors
///
/// Returns an error if `time_zone_name` is not found.
pub fn get_time_zone(time_zone_name: &str) -> Result<TimeZone, jiff::Error> {
    TimeZone::get(time_zone_name).or_else(|_| bundled_tzdb().get(time_zone_name))
}

// ============================================================================
// DATE
// ============================================================================
/// Convert a `jiff::civil::Date` to a Python `datetime.date`.
///
/// # Errors
///
/// Returns a `ValueError` if the year is outside of the range supported by
/// `datetime.date` (`1..=9999`); jiff supports years `-9999..=9999`.
pub fn date_to_pydate<'py>(py: Python<'py>, date: &Date) -> PyResult<Bound<'py, PyDate>> {
    PyDate::new(
        py,
        date.year().into(),
        date.month().cast_unsigned(),
        date.day().cast_unsigned(),
    )
}

/// Convert a Python `datetime.date` (or anything implementing `PyDateAccess`,
/// eg `datetime.datetime`) to a `jiff::civil::Date`.
///
/// # Errors
///
/// Returns a `ValueError` if a component does not fit in the jiff integer
/// type or the date is invalid for jiff; neither happens for a valid
/// `datetime.date`.
pub fn pydate_to_date(py_date: &impl PyDateAccess) -> PyResult<Date> {
    let y = py_date.get_year();
    let y_i16 = i16::try_from(y).map_err(map_py_value_err)?;
    let m = py_date.get_month();
    let m_i8 = i8::try_from(m).map_err(map_py_value_err)?;
    let d = py_date.get_day();
    let d_i8 = i8::try_from(d).map_err(map_py_value_err)?;
    let date = Date::new(y_i16, m_i8, d_i8).map_err(map_py_value_err)?;
    Ok(date)
}

// ============================================================================
// TIME
// ============================================================================
/// Convert a `jiff::civil::Time` to a naive Python `datetime.time`.
///
/// Sub-microsecond precision is truncated (`datetime.time` only has
/// microseconds).
///
/// # Errors
///
/// Returns an error if a component fails to convert or Python fails to
/// create the `datetime.time`; shouldnt happen for a valid `jiff::civil::Time`.
pub fn time_to_pytime<'py>(py: Python<'py>, time: &Time) -> PyResult<Bound<'py, PyTime>> {
    PyTime::new(
        py,
        time.hour().try_into()?,
        time.minute().try_into()?,
        time.second().try_into()?,
        (time.subsec_nanosecond() / 1000).try_into()?,
        None,
    )
}

/// Convert a Python `datetime.time` (or anything implementing `PyTimeAccess`,
/// eg `datetime.datetime`) to a `jiff::civil::Time`.
///
/// # Errors
///
/// Errors if a component does not fit into the jiff integer type; which, (afaict) should not happen for a valid `datetime.time`.
#[expect(clippy::arithmetic_side_effects)]
pub fn pytime_to_time(time: &impl PyTimeAccess) -> PyResult<Time> {
    Time::new(
        time.get_hour().try_into()?,
        time.get_minute().try_into()?,
        time.get_second().try_into()?,
        (time.get_microsecond() * 1000).try_into()?,
    )
    .map_err(map_py_value_err)
}

// ============================================================================
// DATETIME (NAIVE ~ NO TZINFO)
// ============================================================================
/// Convert a `jiff::civil::DateTime` to a naive Python `datetime.datetime`
/// (no `tzinfo`) with the given `fold`.
///
/// NOTE: sub-microsecond precision is truncated
///
/// # Errors
///
/// Returns a `ValueError` if the year is outside of the range supported by
/// `datetime.datetime` (`1..=9999`).
pub fn datetime_to_pydatetime<'py>(
    py: Python<'py>,
    datetime: &DateTime,
    fold: bool,
) -> PyResult<Bound<'py, PyDateTime>> {
    PyDateTime::new_with_fold(
        py,
        datetime.year().into(),
        datetime.month().try_into()?,
        datetime.day().try_into()?,
        datetime.hour().try_into()?,
        datetime.minute().try_into()?,
        datetime.second().try_into()?,
        (datetime.subsec_nanosecond() / 1000).try_into()?,
        None,
        fold,
    )
}

/// Convert a naive Python `datetime.datetime` to a `jiff::civil::DateTime`.
///
/// # Errors
///
/// - `TypeError` if `dt` has a `tzinfo` (use [`pydatetime_to_zoned`])
/// - see [`pydate_to_date`] and [`pytime_to_time`]
pub fn pydatetime_to_datetime(dt: &Bound<'_, PyDateTime>) -> PyResult<DateTime> {
    if dt.get_tzinfo().is_some() {
        return Err(PyTypeError::new_err("expected a datetime without tzinfo"));
    }
    Ok(DateTime::from_parts(
        pydate_to_date(dt)?,
        pytime_to_time(dt)?,
    ))
}

// ============================================================================
// ZONED (AWARE ~ WITH TZINFO)
// ============================================================================
/// PEP 495 `fold` for a zoned datetime: `true` only if its civil datetime is
/// ambiguous (a fold) and the zoned uses the later of the two offsets
fn fold(zoned: &Zoned) -> bool {
    match zoned
        .time_zone()
        .to_ambiguous_timestamp(zoned.datetime())
        .offset()
    {
        AmbiguousOffset::Fold { after, .. } => zoned.offset() == after,
        _ => false,
    }
}

/// Convert a `jiff::Zoned` to an aware Python `datetime.datetime`.
///
/// The `tzinfo` comes from [`timezone_to_pytzinfo`], and `fold` is set per
/// PEP 495 for ambiguous datetimes. Sub-microsecond precision is truncated.
///
/// # Errors
///
/// - `ValueError` if the year is outside of the range supported by
///   `datetime.datetime` (`1..=9999`)
/// - see [`timezone_to_pytzinfo`]
pub fn zoned_to_pydatetime<'py>(py: Python<'py>, z: &Zoned) -> PyResult<Bound<'py, PyDateTime>> {
    let datetime = z.datetime();
    let tz = timezone_to_pytzinfo(py, z.time_zone())?;
    PyDateTime::new_with_fold(
        py,
        datetime.year().into(),
        datetime.month().try_into()?,
        datetime.day().try_into()?,
        datetime.hour().try_into()?,
        datetime.minute().try_into()?,
        datetime.second().try_into()?,
        (datetime.subsec_nanosecond() / 1000).try_into()?,
        Some(&tz),
        fold(z),
    )
}

/// Convert an aware Python `datetime.datetime` to a `jiff::Zoned`.
///
/// UTC is special-cased (no tz lookup). Otherwise ambiguous (fold) and
/// nonexistent (gap) datetimes are resolved per PEP 495 using `fold`.
///
/// # Errors
///
/// - `TypeError` if `dt` has no `tzinfo` (use [`pydatetime_to_datetime`])
/// - an error if the `tzinfo` cannot be extracted as a jiff `TimeZone`
/// - `ValueError` if the resulting zoned datetime is out of jiff's range
/// - see [`pydate_to_date`] and [`pytime_to_time`]
pub fn pydatetime_to_zoned(dt: &Bound<'_, PyDateTime>) -> PyResult<Zoned> {
    let Some(tzinfo) = dt.get_tzinfo() else {
        return py_type_err!("expected a datetime with non-None tzinfo");
    };
    let datetime = DateTime::from_parts(pydate_to_date(dt)?, pytime_to_time(dt)?);
    if tzinfo.is(PyTzInfo::utc(dt.py())?) {
        // utc aint ambiguous... skip tz lookup + fold
        let ts = Offset::UTC
            .to_timestamp(datetime)
            .map_err(map_py_value_err)?;
        return Ok(ts.to_zoned(TimeZone::UTC));
    }
    let zoned = tzinfo
        .extract::<JiffTimeZone>()?
        .into_inner()
        .into_ambiguous_zoned(datetime);
    // REF: https://peps.python.org/pep-0495/
    // REF: https://github.com/PyO3/pyo3/pull/6450
    if dt.get_fold() ^ matches!(zoned.offset(), AmbiguousOffset::Gap { .. }) {
        zoned.later()
    } else {
        zoned.earlier()
    }
    .map_err(map_py_value_err)
}

// ============================================================================
// TIMEZONE
// ============================================================================
/// Convert a `jiff::tz::TimeZone` to a Python `tzinfo`.
///
/// - UTC -> `datetime.timezone.utc`
/// - time zones with an IANA name -> `zoneinfo.ZoneInfo`
/// - anything else -> fixed offset `datetime.timezone`
///
/// # Errors
///
/// - an error if `zoneinfo.ZoneInfo` cannot load the IANA name (eg
///   `ZoneInfoNotFoundError`)
/// - `ValueError` if the time zone has no IANA name and is not a fixed
///   offset (eg a POSIX TZ string with DST rules)
pub fn timezone_to_pytzinfo<'py>(py: Python<'py>, tz: &TimeZone) -> PyResult<Bound<'py, PyTzInfo>> {
    if tz == &TimeZone::UTC {
        Ok(PyTzInfo::utc(py)?.to_owned())
    } else if let Some(iana_name) = tz.iana_name() {
        Ok(PyTzInfo::timezone(py, iana_name)?)
    } else {
        let off = tz.to_fixed_offset().map_err(map_py_value_err)?;
        let dur = off.duration_since(Offset::UTC);
        PyTzInfo::fixed_offset(py, JiffSignedDuration::new(dur))
    }
}

// ============================================================================
// SIGNED DURATION
// ============================================================================
const SECONDS_PER_DAY: i64 = 86_400;

/// Convert a `SignedDuration` to a Python `datetime.timedelta`.
///
/// Sub-microsecond precision is truncated.
///
/// # Errors
///
/// Returns an error if the conversion overflows the range of `datetime.timedelta`.
pub fn signed_duration_to_pydelta<'py>(
    py: Python<'py>,
    duration: &SignedDuration,
) -> PyResult<Bound<'py, PyDelta>> {
    let total_seconds = duration.as_secs();
    let days: i32 = (total_seconds / SECONDS_PER_DAY)
        .try_into()
        .map_err(|_| py_overflow_error!("SignedDuration -> datetime.timedelta"))?;
    let seconds = (total_seconds % SECONDS_PER_DAY) as i32;
    let microseconds = duration.subsec_micros();

    PyDelta::new(py, days, seconds, microseconds, true)
}

/// Convert a Python `datetime.timedelta` to a `jiff::SignedDuration`.
///
/// # Errors
///
/// Returns an error if the conversion overflows the range of `SignedDuration`.
pub fn pydelta_to_signed_duration(delta: &Bound<'_, PyDelta>) -> PyResult<SignedDuration> {
    let (days, seconds, microseconds) = (
        i64::from(delta.get_days()),
        i64::from(delta.get_seconds()),
        delta.get_microseconds(),
    );

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
