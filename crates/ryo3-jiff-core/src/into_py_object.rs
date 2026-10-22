//! `IntoPyObject` impls (jiff -> python)
//!
//! the conversion logic lives on the `&T` impls; owned new-types forward to
//! them via `into_pyobject_via_ref!`
use jiff::SpanRelativeTo;
use jiff::civil::{Era, Weekday};
use jiff::tz::TimeZone;
use pyo3::IntoPyObjectExt;
use pyo3::prelude::*;
use pyo3::types::{PyDate, PyDateTime, PyDelta, PyInt, PyString, PyTime, PyTuple, PyTzInfo};
use ryo3_core::map_py_value_err;

use crate::{
    JiffDate, JiffDateTime, JiffEra, JiffEraYear, JiffOffset, JiffRoundMode, JiffSignedDuration,
    JiffSignedDurationRef, JiffSpan, JiffTime, JiffTimeZone, JiffTimeZoneRef, JiffTimestamp,
    JiffUnit, JiffWeekday, JiffZoned, JiffZonedRef, date_to_pydate, datetime_to_pydatetime,
    signed_duration_to_pydelta, time_to_pytime, timezone_to_pytzinfo, zoned_to_pydatetime,
};

/// impl `IntoPyObject` for an owned new-type by forwarding to its `&T` impl
///
/// - `T => Target` ~ `Output = Bound<'py, Target>`, `Error = PyErr`
/// - `T => infallible Target` ~ `Output = Bound<'py, Target>`, `Error = Infallible`
/// - `T => borrowed Target` ~ `Output = Borrowed<'py, 'py, Target>`, `Error = Infallible`
macro_rules! into_pyobject_via_ref {
    (@impl $newt:ty, $target:ty, $output:ty, $error:ty) => {
        impl<'py> IntoPyObject<'py> for $newt {
            type Target = $target;
            type Output = $output;
            type Error = $error;

            #[inline]
            fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
                (&self).into_pyobject(py)
            }
        }
    };
    ($newt:ty => $target:ty) => {
        into_pyobject_via_ref!(@impl $newt, $target, Bound<'py, $target>, PyErr);
    };
    ($newt:ty => infallible $target:ty) => {
        into_pyobject_via_ref!(@impl $newt, $target, Bound<'py, $target>, std::convert::Infallible);
    };
    ($newt:ty => borrowed $target:ty) => {
        into_pyobject_via_ref!(@impl $newt, $target, Borrowed<'py, 'py, $target>, std::convert::Infallible);
    };
}

into_pyobject_via_ref!(JiffDate => PyDate);
into_pyobject_via_ref!(JiffTime => PyTime);
into_pyobject_via_ref!(JiffDateTime => PyDateTime);
into_pyobject_via_ref!(JiffTimestamp => PyDateTime);
into_pyobject_via_ref!(JiffZoned => PyDateTime);
into_pyobject_via_ref!(JiffZonedRef<'_> => PyDateTime);
into_pyobject_via_ref!(JiffSpan => PyDelta);
into_pyobject_via_ref!(JiffSignedDuration => PyDelta);
into_pyobject_via_ref!(JiffSignedDurationRef<'_> => PyDelta);
into_pyobject_via_ref!(JiffTimeZone => PyTzInfo);
into_pyobject_via_ref!(JiffTimeZoneRef<'_> => PyTzInfo);
into_pyobject_via_ref!(JiffOffset => PyTzInfo);
into_pyobject_via_ref!(JiffUnit => borrowed PyString);
into_pyobject_via_ref!(JiffRoundMode => borrowed PyString);
into_pyobject_via_ref!(JiffEra => borrowed PyString);
into_pyobject_via_ref!(JiffWeekday => infallible PyInt);
into_pyobject_via_ref!(JiffEraYear => PyTuple);

// ============================================================================
// DATE / TIME / DATETIME
// ============================================================================
impl<'py> IntoPyObject<'py> for &JiffDate {
    type Target = PyDate;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        date_to_pydate(py, self.inner())
    }
}

impl<'py> IntoPyObject<'py> for &JiffTime {
    type Target = PyTime;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    #[inline]
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        time_to_pytime(py, self.inner())
    }
}

impl<'py> IntoPyObject<'py> for &JiffDateTime {
    type Target = PyDateTime;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    #[inline]
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        datetime_to_pydatetime(py, self.inner(), false)
    }
}

// ============================================================================
// TIMESTAMP / ZONED
// ============================================================================
impl<'py> IntoPyObject<'py> for &JiffTimestamp {
    type Target = PyDateTime;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        JiffZoned::new(self.inner().to_zoned(TimeZone::UTC)).into_pyobject(py)
    }
}

impl<'py> IntoPyObject<'py> for &JiffZoned {
    type Target = PyDateTime;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        zoned_to_pydatetime(py, self.inner())
    }
}

impl<'py> IntoPyObject<'py> for &JiffZonedRef<'_> {
    type Target = PyDateTime;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        zoned_to_pydatetime(py, self.inner())
    }
}

// ============================================================================
// SPAN / SIGNED DURATION
// ============================================================================
impl<'py> IntoPyObject<'py> for &JiffSpan {
    type Target = PyDelta;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    #[inline]
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        let signed_duration = self
            .into_inner()
            .to_duration(SpanRelativeTo::days_are_24_hours())
            .map(JiffSignedDuration::from)
            .map_err(map_py_value_err)?;
        signed_duration.into_pyobject(py)
    }
}

impl<'py> IntoPyObject<'py> for &JiffSignedDuration {
    type Target = PyDelta;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    #[inline]
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        signed_duration_to_pydelta(py, self.inner())
    }
}

impl<'py> IntoPyObject<'py> for &JiffSignedDurationRef<'_> {
    type Target = PyDelta;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    #[inline]
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        signed_duration_to_pydelta(py, self.inner())
    }
}

// ============================================================================
// TIMEZONE / OFFSET
// ============================================================================
impl<'py> IntoPyObject<'py> for &JiffTimeZone {
    type Target = PyTzInfo;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        timezone_to_pytzinfo(py, self.inner())
    }
}

impl<'py> IntoPyObject<'py> for &JiffTimeZoneRef<'_> {
    type Target = PyTzInfo;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        timezone_to_pytzinfo(py, self.inner())
    }
}

impl<'py> IntoPyObject<'py> for &JiffOffset {
    type Target = PyTzInfo;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        JiffTimeZone::new(self.inner().to_time_zone()).into_pyobject(py)
    }
}

// ============================================================================
// UNIT / ROUND MODE
// ============================================================================
impl<'py> IntoPyObject<'py> for &JiffUnit {
    type Target = PyString;
    type Output = Borrowed<'py, 'py, Self::Target>;
    type Error = std::convert::Infallible;

    #[inline]
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        let s = match self.into_inner() {
            jiff::Unit::Year => pyo3::intern!(py, "year"),
            jiff::Unit::Month => pyo3::intern!(py, "month"),
            jiff::Unit::Week => pyo3::intern!(py, "week"),
            jiff::Unit::Day => pyo3::intern!(py, "day"),
            jiff::Unit::Hour => pyo3::intern!(py, "hour"),
            jiff::Unit::Minute => pyo3::intern!(py, "minute"),
            jiff::Unit::Second => pyo3::intern!(py, "second"),
            jiff::Unit::Millisecond => pyo3::intern!(py, "millisecond"),
            jiff::Unit::Microsecond => pyo3::intern!(py, "microsecond"),
            jiff::Unit::Nanosecond => pyo3::intern!(py, "nanosecond"),
        };
        Ok(s.as_borrowed())
    }
}

impl<'py> IntoPyObject<'py> for &JiffRoundMode {
    type Target = PyString;
    type Output = Borrowed<'py, 'py, Self::Target>;
    type Error = std::convert::Infallible;

    #[inline]
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        let s = match self.into_inner() {
            jiff::RoundMode::Ceil => pyo3::intern!(py, "ceil"),
            jiff::RoundMode::Floor => pyo3::intern!(py, "floor"),
            jiff::RoundMode::Expand => pyo3::intern!(py, "expand"),
            jiff::RoundMode::Trunc => pyo3::intern!(py, "trunc"),
            jiff::RoundMode::HalfCeil => pyo3::intern!(py, "half-ceil"),
            jiff::RoundMode::HalfFloor => pyo3::intern!(py, "half-floor"),
            jiff::RoundMode::HalfExpand => pyo3::intern!(py, "half-expand"),
            jiff::RoundMode::HalfTrunc => pyo3::intern!(py, "half-trunc"),
            jiff::RoundMode::HalfEven => pyo3::intern!(py, "half-even"),
            _ => pyo3::intern!(py, "unknown"),
        };
        Ok(s.as_borrowed())
    }
}

// ============================================================================
// WEEKDAY / ERA
// ============================================================================
impl<'py> IntoPyObject<'py> for &JiffWeekday {
    type Target = PyInt;
    type Output = Bound<'py, Self::Target>;
    type Error = std::convert::Infallible;

    #[inline]
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        let num: u8 = match self.into_inner() {
            Weekday::Monday => 1,
            Weekday::Tuesday => 2,
            Weekday::Wednesday => 3,
            Weekday::Thursday => 4,
            Weekday::Friday => 5,
            Weekday::Saturday => 6,
            Weekday::Sunday => 7,
        };
        num.into_pyobject(py)
    }
}

fn era_pystr(py: Python<'_>, era: Era) -> &Bound<'_, PyString> {
    match era {
        Era::BCE => pyo3::intern!(py, "BCE"),
        Era::CE => pyo3::intern!(py, "CE"),
    }
}

impl<'py> IntoPyObject<'py> for &JiffEra {
    type Target = PyString;
    type Output = Borrowed<'py, 'py, Self::Target>;
    type Error = std::convert::Infallible;

    #[inline]
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        Ok(era_pystr(py, self.into_inner()).as_borrowed())
    }
}

impl<'py> IntoPyObject<'py> for &JiffEraYear {
    type Target = PyTuple;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        let (y, e) = self.into_inner();
        let year_py = y.into_py_any(py)?;
        let era_str = era_pystr(py, e).into_py_any(py)?;

        let pyobjs_vec = vec![year_py, era_str];
        PyTuple::new(py, pyobjs_vec)
    }
}
