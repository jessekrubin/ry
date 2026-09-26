//! Span-ish ry/python object(s)

use jiff::civil::{DateArithmetic, DateTimeArithmetic, TimeArithmetic};
use jiff::tz::OffsetArithmetic;
use jiff::{SignedDuration, TimestampArithmetic, ZonedArithmetic};
use pyo3::prelude::*;
use pyo3::types::PyDelta;
use ryo3_core::PyCastExactOpt;
use ryo3_macro_rules::py_type_err;
use ryo3_std::time::PyDuration;

use crate::{RySignedDuration, RySpan};

/// `ry.Duration | datetime.timedelta`
///
/// delta/duration types who themselves don't know about ry-temporals:
///
/// `TimeSpan + ry.Date`/`SignedDuration + ry.Date` are handled by `TimeSpan` and `SignedDuration` respectively, while `timedelta + ry.Date`/`ry.Duration + ry.Date` rely on `Date.__radd__`
#[derive(Debug, Clone)]
pub(crate) enum DurationLike<'a, 'py> {
    Duration(Borrowed<'a, 'py, PyDuration>),
    PyTimeDelta(SignedDuration),
}

#[derive(Debug, Clone)]
pub(crate) enum Spanish<'a, 'py> {
    SignedDuration(Borrowed<'a, 'py, RySignedDuration>),
    Span(Borrowed<'a, 'py, RySpan>),
    DurationLike(DurationLike<'a, 'py>),
}

impl<'a, 'py> FromPyObject<'a, 'py> for DurationLike<'a, 'py> {
    type Error = PyErr;

    fn extract(ob: Borrowed<'a, 'py, PyAny>) -> Result<Self, Self::Error> {
        if let Some(duration) = ob.cast_exact_opt::<PyDuration>() {
            Ok(Self::Duration(duration))
        } else if let Some(py_delta) = ob.cast_exact_opt::<PyDelta>() {
            py_delta.extract::<SignedDuration>().map(Self::PyTimeDelta)
        } else {
            py_type_err!("Expected a Duration or datetime.timedelta object")
        }
    }
}

impl<'a, 'py> FromPyObject<'a, 'py> for Spanish<'a, 'py> {
    type Error = PyErr;

    fn extract(ob: Borrowed<'a, 'py, PyAny>) -> Result<Self, Self::Error> {
        if let Some(span) = ob.cast_exact_opt::<RySpan>() {
            Ok(Self::Span(span))
        } else if let Some(duration) = ob.cast_exact_opt::<PyDuration>() {
            Ok(Self::DurationLike(DurationLike::Duration(duration)))
        } else if let Some(signed_duration) = ob.cast_exact_opt::<RySignedDuration>() {
            Ok(Self::SignedDuration(signed_duration))
        } else if let Some(py_delta) = ob.cast_exact_opt::<PyDelta>() {
            py_delta
                .extract::<SignedDuration>()
                .map(|sd| Self::from(DurationLike::PyTimeDelta(sd)))
        } else {
            py_type_err!(
                "Expected a Timespan, Duration, SignedDuration, or datetime.timedelta object"
            )
        }
    }
}

// ============================================================================
// FROM ~ FROM ~ FROM ~ FROM ~ FROM ~ FROM ~ FROM ~ FROM ~ FROM ~ FROM ~ FROM
// ============================================================================
impl<'a, 'py> From<DurationLike<'a, 'py>> for Spanish<'a, 'py> {
    fn from(value: DurationLike<'a, 'py>) -> Self {
        Self::DurationLike(value)
    }
}

impl<'a, 'py> From<&'a DurationLike<'a, 'py>> for jiff::SpanArithmetic<'a> {
    fn from(value: &'a DurationLike<'a, 'py>) -> Self {
        match value {
            DurationLike::Duration(dur) => {
                jiff::SpanArithmetic::from(*(dur.get().inner())).days_are_24_hours()
            }
            DurationLike::PyTimeDelta(sd) => jiff::SpanArithmetic::from(*sd).days_are_24_hours(),
        }
    }
}

impl<'a, 'py> From<&'a Spanish<'a, 'py>> for jiff::SpanArithmetic<'a> {
    fn from(value: &'a Spanish<'a, 'py>) -> Self {
        match value {
            Spanish::Span(sp) => jiff::SpanArithmetic::from(sp.get().0).days_are_24_hours(),
            Spanish::SignedDuration(dur) => {
                jiff::SpanArithmetic::from(dur.get().0).days_are_24_hours()
            }
            Spanish::DurationLike(dl) => dl.into(),
        }
    }
}
// ----------------------------------------------------------------------------
// FROM SPANISH
// ----------------------------------------------------------------------------
impl From<Spanish<'_, '_>> for DateArithmetic {
    fn from(value: Spanish<'_, '_>) -> Self {
        match value {
            Spanish::DurationLike(duration_like) => duration_like.into(),
            Spanish::SignedDuration(signed_duration) => Self::from(signed_duration.get().0),
            Spanish::Span(span) => Self::from(span.get().0),
        }
    }
}

impl From<Spanish<'_, '_>> for DateTimeArithmetic {
    fn from(value: Spanish<'_, '_>) -> Self {
        match value {
            Spanish::DurationLike(duration_like) => duration_like.into(),
            Spanish::SignedDuration(signed_duration) => Self::from(signed_duration.get().0),
            Spanish::Span(span) => Self::from(span.get().0),
        }
    }
}

impl From<Spanish<'_, '_>> for OffsetArithmetic {
    fn from(value: Spanish<'_, '_>) -> Self {
        match value {
            Spanish::DurationLike(duration_like) => duration_like.into(),
            Spanish::Span(span) => Self::from(span.get().0),
            Spanish::SignedDuration(signed_duration) => Self::from(signed_duration.get().0),
        }
    }
}

impl From<Spanish<'_, '_>> for TimeArithmetic {
    fn from(value: Spanish<'_, '_>) -> Self {
        match value {
            Spanish::DurationLike(duration_like) => duration_like.into(),
            Spanish::SignedDuration(signed_duration) => Self::from(signed_duration.get().0),
            Spanish::Span(span) => Self::from(span.get().0),
        }
    }
}

impl From<Spanish<'_, '_>> for TimestampArithmetic {
    fn from(value: Spanish<'_, '_>) -> Self {
        match value {
            Spanish::DurationLike(duration_like) => duration_like.into(),
            Spanish::Span(span) => Self::from(span.get().0),
            Spanish::SignedDuration(signed_duration) => Self::from(signed_duration.get().0),
        }
    }
}

impl From<Spanish<'_, '_>> for ZonedArithmetic {
    fn from(value: Spanish<'_, '_>) -> Self {
        match value {
            Spanish::DurationLike(duration_like) => duration_like.into(),
            Spanish::SignedDuration(signed_duration) => Self::from(signed_duration.get().0),
            Spanish::Span(span) => Self::from(span.get().0),
        }
    }
}

// ----------------------------------------------------------------------------
// FROM DURATIONLIKE
// ----------------------------------------------------------------------------
impl From<DurationLike<'_, '_>> for DateArithmetic {
    fn from(value: DurationLike<'_, '_>) -> Self {
        match value {
            DurationLike::Duration(duration) => Self::from(duration.get().inner()),
            DurationLike::PyTimeDelta(signed_duration) => Self::from(signed_duration),
        }
    }
}

impl From<DurationLike<'_, '_>> for DateTimeArithmetic {
    fn from(value: DurationLike<'_, '_>) -> Self {
        match value {
            DurationLike::Duration(duration) => Self::from(duration.get().inner()),
            DurationLike::PyTimeDelta(signed_duration) => Self::from(signed_duration),
        }
    }
}

impl From<DurationLike<'_, '_>> for OffsetArithmetic {
    fn from(value: DurationLike<'_, '_>) -> Self {
        match value {
            DurationLike::Duration(duration) => Self::from(duration.get().inner()),
            DurationLike::PyTimeDelta(signed_duration) => Self::from(signed_duration),
        }
    }
}

impl From<DurationLike<'_, '_>> for TimeArithmetic {
    fn from(value: DurationLike<'_, '_>) -> Self {
        match value {
            DurationLike::Duration(duration) => Self::from(duration.get().inner()),
            DurationLike::PyTimeDelta(signed_duration) => Self::from(signed_duration),
        }
    }
}

impl From<DurationLike<'_, '_>> for TimestampArithmetic {
    fn from(value: DurationLike<'_, '_>) -> Self {
        match value {
            DurationLike::Duration(duration) => Self::from(duration.get().inner()),
            DurationLike::PyTimeDelta(signed_duration) => Self::from(signed_duration),
        }
    }
}

impl From<DurationLike<'_, '_>> for ZonedArithmetic {
    fn from(value: DurationLike<'_, '_>) -> Self {
        match value {
            DurationLike::Duration(duration) => Self::from(duration.get().inner()),
            DurationLike::PyTimeDelta(signed_duration) => Self::from(signed_duration),
        }
    }
}
