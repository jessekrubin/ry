//! `FromPyObject` impls (python -> jiff)
use jiff::Span;
use jiff::tz::Offset;
use pyo3::prelude::*;
use pyo3::pybacked::PyBackedStr;
use pyo3::types::{PyDate, PyDateTime, PyDelta, PyNone, PyTime, PyTzInfo, PyTzInfoAccess};
use ryo3_core::map_py_value_err;
use ryo3_macro_rules::{py_type_err, py_value_err};

use crate::{
    JiffDate, JiffDateTime, JiffDateTimeOrZoned, JiffEra, JiffFriendlyDesignator, JiffOffset,
    JiffRoundMode, JiffSignedDuration, JiffSpan, JiffTime, JiffTimeZone, JiffTimestamp,
    JiffTzDisambiguation, JiffTzOffsetConflict, JiffUnit, JiffWeekday, JiffZoned, get_time_zone,
    pydate_to_date, pydatetime_to_datetime, pydatetime_to_zoned, pydelta_to_signed_duration,
    pytime_to_time,
};

// ============================================================================
// DATE / TIME / DATETIME
// ============================================================================
impl<'py> FromPyObject<'_, 'py> for JiffDate {
    type Error = PyErr;
    fn extract(ob: Borrowed<'_, 'py, PyAny>) -> Result<Self, Self::Error> {
        let py_date = ob.cast::<PyDate>()?;
        pydate_to_date(&*py_date).map(Self::from)
    }
}

impl<'py> FromPyObject<'_, 'py> for JiffTime {
    type Error = PyErr;
    fn extract(ob: Borrowed<'_, 'py, PyAny>) -> PyResult<Self> {
        let time = ob.cast::<PyTime>()?;
        pytime_to_time(&*time).map(Self::from)
    }
}

impl<'py> FromPyObject<'_, 'py> for JiffDateTime {
    type Error = PyErr;
    fn extract(ob: Borrowed<'_, 'py, PyAny>) -> Result<Self, Self::Error> {
        let dt = ob.cast::<PyDateTime>()?;
        pydatetime_to_datetime(&dt).map(Self::from)
    }
}

impl<'py> FromPyObject<'_, 'py> for JiffDateTimeOrZoned {
    type Error = PyErr;
    fn extract(ob: Borrowed<'_, 'py, PyAny>) -> PyResult<Self> {
        let dt = ob.cast::<PyDateTime>()?;
        if dt.get_tzinfo().is_some() {
            pydatetime_to_zoned(&dt).map(Self::Aware)
        } else {
            pydatetime_to_datetime(&dt).map(Self::Naive)
        }
    }
}

// ============================================================================
// TIMESTAMP / ZONED
// ============================================================================
impl<'py> FromPyObject<'_, 'py> for JiffTimestamp {
    type Error = PyErr;
    fn extract(ob: Borrowed<'_, 'py, PyAny>) -> PyResult<Self> {
        ob.extract::<JiffZoned>()
            .map(|e| e.into_inner().timestamp().into())
    }
}

impl<'py> FromPyObject<'_, 'py> for JiffZoned {
    type Error = PyErr;
    fn extract(dt: Borrowed<'_, 'py, PyAny>) -> PyResult<Self> {
        let dt = dt.cast::<PyDateTime>()?;
        pydatetime_to_zoned(&dt).map(Self::from)
    }
}

// ============================================================================
// SPAN / SIGNED DURATION
// ============================================================================
impl<'py> FromPyObject<'_, 'py> for JiffSpan {
    type Error = PyErr;
    fn extract(obj: Borrowed<'_, 'py, PyAny>) -> PyResult<Self> {
        let signed_duration = obj.extract::<JiffSignedDuration>()?;
        let span: Span = signed_duration
            .into_inner()
            .try_into()
            .map_err(map_py_value_err)?;
        Ok(Self::from(span))
    }
}

impl<'py> FromPyObject<'_, 'py> for JiffSignedDuration {
    type Error = PyErr;
    fn extract(obj: Borrowed<'_, 'py, PyAny>) -> PyResult<Self> {
        let delta = obj.cast::<PyDelta>()?;
        pydelta_to_signed_duration(&delta).map(Self::from)
    }
}

// ============================================================================
// TIMEZONE / OFFSET
// ============================================================================
impl<'py> FromPyObject<'_, 'py> for JiffTimeZone {
    type Error = PyErr;
    fn extract(obj: Borrowed<'_, 'py, PyAny>) -> PyResult<Self> {
        let ob = obj.cast::<PyTzInfo>()?;
        match ob.getattr_opt(pyo3::intern!(ob.py(), "key"))? {
            Some(key) => {
                let key = key.extract::<PyBackedStr>()?;
                get_time_zone(&key)
                    .map(Self::from)
                    .map_err(map_py_value_err)
            }
            None => Ok(ob
                .extract::<JiffOffset>()?
                .into_inner()
                .to_time_zone()
                .into()),
        }
    }
}

impl<'py> FromPyObject<'_, 'py> for JiffOffset {
    type Error = PyErr;

    fn extract(obj: Borrowed<'_, 'py, PyAny>) -> PyResult<Self> {
        let ob = obj.cast::<PyTzInfo>()?;

        let py_timedelta =
            ob.call_method1(pyo3::intern!(ob.py(), "utcoffset"), (PyNone::get(ob.py()),))?;
        if py_timedelta.is_none() {
            return py_type_err!("{ob:?} is not a fixed offset timezone");
        }
        let total_seconds: JiffSignedDuration = py_timedelta.extract()?;

        let total_seconds =
            i32::try_from(total_seconds.into_inner().as_secs()).map_err(map_py_value_err)?;
        Offset::from_seconds(total_seconds)
            .map(Self::from)
            .map_err(map_py_value_err)
    }
}

const JIFF_TZ_DISAMBIGUATION_STRINGS: &str = "'compatible', 'earlier', 'later', 'reject'";

impl<'py> FromPyObject<'_, 'py> for JiffTzDisambiguation {
    type Error = PyErr;
    fn extract(ob: Borrowed<'_, 'py, PyAny>) -> PyResult<Self> {
        if let Ok(s) = ob.extract::<&str>() {
            match s {
                "compatible" => Ok(Self::COMPATIBLE),
                "earlier" => Ok(Self::EARLIER),
                "later" => Ok(Self::LATER),
                "reject" => Ok(Self::REJECT),
                _ => py_value_err!(
                    "Invalid tz disambiguation: {s} (options: {JIFF_TZ_DISAMBIGUATION_STRINGS})"
                ),
            }
        } else {
            py_type_err!(
                "Invalid type for tz disambiguation, expected a string (options: {JIFF_TZ_DISAMBIGUATION_STRINGS})"
            )
        }
    }
}

const JIFF_TZ_OFFSET_CONFLICTS: &str = "'always-offset', 'always-timezone', 'prefer-offset', 'reject' (underscores and hyphens are interchangeable)";

impl<'py> FromPyObject<'_, 'py> for JiffTzOffsetConflict {
    type Error = PyErr;
    fn extract(ob: Borrowed<'_, 'py, PyAny>) -> PyResult<Self> {
        if let Ok(s) = ob.extract::<&str>() {
            match s {
                "always-offset" | "always_offset" => Ok(Self::ALWAYS_OFFSET),
                "always-timezone" | "always_timezone" => Ok(Self::ALWAYS_TIME_ZONE),
                "prefer-offset" | "prefer_offset" => Ok(Self::PREFER_OFFSET),
                "reject" => Ok(Self::REJECT),
                _ => py_value_err!(
                    "Invalid tz offset conflict: {s} (options: {JIFF_TZ_OFFSET_CONFLICTS})"
                ),
            }
        } else {
            py_type_err!(
                "Invalid type for tz offset conflict, expected a string (options: {JIFF_TZ_OFFSET_CONFLICTS})"
            )
        }
    }
}

// ============================================================================
// UNIT / ROUND MODE
// ============================================================================
const JIFF_UNIT_STRINGS: &str = "'year', 'month', 'week', 'day', 'hour', 'minute', 'second', 'millisecond', 'microsecond', 'nanosecond'";
const JIFF_UNIT_OPTIONS: &str = "0='year', 1='month', 2='week', 3='day', 4='hour', 5='minute', 6='second', 7='millisecond', 8='microsecond', 9='nanosecond'";

impl<'py> FromPyObject<'_, 'py> for JiffUnit {
    type Error = PyErr;
    fn extract(ob: Borrowed<'_, 'py, PyAny>) -> PyResult<Self> {
        // downcast to string...
        if let Ok(s) = ob.extract::<&str>() {
            match s.to_ascii_lowercase().as_str() {
                "year" => Ok(Self::YEAR),
                "month" => Ok(Self::MONTH),
                "week" => Ok(Self::WEEK),
                "day" => Ok(Self::DAY),
                "hour" => Ok(Self::HOUR),
                "minute" => Ok(Self::MINUTE),
                "second" => Ok(Self::SECOND),
                "millisecond" => Ok(Self::MILLISECOND),
                "microsecond" => Ok(Self::MICROSECOND),
                "nanosecond" => Ok(Self::NANOSECOND),
                _ => py_value_err!("Invalid unit: {s} (options: {JIFF_UNIT_STRINGS})"),
            }
        } else if let Ok(i) = ob.extract::<i64>() {
            match i {
                0 => Ok(Self::YEAR),
                1 => Ok(Self::MONTH),
                2 => Ok(Self::WEEK),
                3 => Ok(Self::DAY),
                4 => Ok(Self::HOUR),
                5 => Ok(Self::MINUTE),
                6 => Ok(Self::SECOND),
                7 => Ok(Self::MILLISECOND),
                8 => Ok(Self::MICROSECOND),
                9 => Ok(Self::NANOSECOND),
                _ => py_value_err!("Invalid unit: {i} (options: {JIFF_UNIT_OPTIONS})"),
            }
        } else {
            py_value_err!(
                "Invalid type for unit, expected a string or integer (options: {JIFF_UNIT_STRINGS})"
            )
        }
    }
}

const JIFF_ROUND_MODE_ACCEPTED: &str = "'ceil', 'floor', 'expand', 'trunc', 'half-ceil', 'half-floor', 'half-expand', 'half-trunc', 'half-even' (underscores and hyphens are interchangeable)";

impl<'py> FromPyObject<'_, 'py> for JiffRoundMode {
    type Error = PyErr;
    fn extract(ob: Borrowed<'_, 'py, PyAny>) -> PyResult<Self> {
        if let Ok(str_mode) = ob.extract::<&str>() {
            match str_mode.to_ascii_lowercase().as_str() {
                "ceil" => Ok(Self::CEIL),
                "floor" => Ok(Self::FLOOR),
                "expand" => Ok(Self::EXPAND),
                "trunc" => Ok(Self::TRUNC),
                "half-ceil" | "half_ceil" => Ok(Self::HALF_CEIL),
                "half-floor" | "half_floor" => Ok(Self::HALF_FLOOR),
                "half-expand" | "half_expand" => Ok(Self::HALF_EXPAND),
                "half-trunc" | "half_trunc" => Ok(Self::HALF_TRUNC),
                "half-even" | "half_even" => Ok(Self::HALF_EVEN),
                _ => py_value_err!(
                    "Invalid type for round mode, expected a string (options: {JIFF_ROUND_MODE_ACCEPTED})"
                ),
            }
        } else {
            py_type_err!(
                "Invalid type for round mode, expected a string (options: {JIFF_ROUND_MODE_ACCEPTED})"
            )
        }
    }
}

// ============================================================================
// WEEKDAY / ERA
// ============================================================================
const JIFF_WEEKDAY_STRING: &str =
    "1='monday', 2='tuesday', 3='wednesday', 4='thursday', 5='friday', 6='saturday', 7='sunday'";

impl<'py> FromPyObject<'_, 'py> for JiffWeekday {
    type Error = PyErr;

    fn extract(ob: Borrowed<'_, 'py, PyAny>) -> PyResult<Self> {
        if let Ok(s) = ob.extract::<&str>() {
            match s {
                "monday" | "MONDAY" => Ok(Self::MONDAY),
                "tuesday" | "TUESDAY" => Ok(Self::TUESDAY),
                "wednesday" | "WEDNESDAY" => Ok(Self::WEDNESDAY),
                "thursday" | "THURSDAY" => Ok(Self::THURSDAY),
                "friday" | "FRIDAY" => Ok(Self::FRIDAY),
                "saturday" | "SATURDAY" => Ok(Self::SATURDAY),
                "sunday" | "SUNDAY" => Ok(Self::SUNDAY),
                _ => py_value_err!("Invalid weekday: {s} (options: {JIFF_WEEKDAY_STRING})"),
            }
        } else if let Ok(i) = ob.extract::<u8>() {
            match i {
                1 => Ok(Self::MONDAY),
                2 => Ok(Self::TUESDAY),
                3 => Ok(Self::WEDNESDAY),
                4 => Ok(Self::THURSDAY),
                5 => Ok(Self::FRIDAY),
                6 => Ok(Self::SATURDAY),
                7 => Ok(Self::SUNDAY),
                _ => py_value_err!("Invalid weekday: {i} (options: {JIFF_WEEKDAY_STRING})"),
            }
        } else {
            py_type_err!(
                "Invalid type for weekday, expected a string or integer (options: {JIFF_WEEKDAY_STRING})"
            )
        }
    }
}

const JIFF_ERA_STRINGS: &str = "'BCE'/'BC', 'CE'/'AD' (case insensitive)";

impl<'py> FromPyObject<'_, 'py> for JiffEra {
    type Error = PyErr;
    fn extract(obj: Borrowed<'_, 'py, PyAny>) -> PyResult<Self> {
        if let Ok(s) = obj.extract::<&str>() {
            match s {
                "bce" | "bc" | "BCE" | "BC" => Ok(Self::BCE),
                "ce" | "ad" | "CE" | "AD" => Ok(Self::CE),
                _ => py_value_err!("Invalid era: {s} (options: {JIFF_ERA_STRINGS})"),
            }
        } else {
            py_type_err!("Invalid type for era, expected a string (options: {JIFF_ERA_STRINGS})")
        }
    }
}

// ============================================================================
// FRIENDLY DESIGNATOR
// ============================================================================
const JIFF_FRIENDLY_DESIGNATOR_ACCEPTED: &str =
    "'human'/'human-time', 'short', 'compact', or 'verbose'";

impl<'py> FromPyObject<'_, 'py> for JiffFriendlyDesignator {
    type Error = PyErr;
    fn extract(ob: Borrowed<'_, 'py, PyAny>) -> PyResult<Self> {
        if let Ok(s) = ob.extract::<&str>() {
            match s {
                "human-time" | "human" => Ok(Self::HUMAN_TIME),
                "short" => Ok(Self::SHORT),
                "compact" => Ok(Self::COMPACT),
                "verbose" => Ok(Self::VERBOSE),
                _ => py_value_err!(
                    "Invalid designator: {s} (options: {JIFF_FRIENDLY_DESIGNATOR_ACCEPTED})"
                ),
            }
        } else {
            py_type_err!(
                "Invalid type for designator, expected a string (options: {JIFF_FRIENDLY_DESIGNATOR_ACCEPTED})"
            )
        }
    }
}
