//! Span-ish ry/python object(s)

use crate::{RySignedDuration, RySpan};
use jiff::civil::{DateArithmetic, DateTimeArithmetic, TimeArithmetic};
use jiff::tz::OffsetArithmetic;
use jiff::{SignedDuration, TimestampArithmetic, ZonedArithmetic};
use pyo3::prelude::*;
use pyo3::types::PyDelta;
use ryo3_macro_rules::py_type_err;
use ryo3_std::time::PyDuration;

#[derive(Debug, Clone)]
pub(crate) enum Spanish<'a, 'py> {
    Duration(Borrowed<'a, 'py, PyDuration>),
    SignedDuration(Borrowed<'a, 'py, RySignedDuration>),
    Span(Borrowed<'a, 'py, RySpan>),
    PyTimeDelta(SignedDuration),
}

impl<'a, 'py> FromPyObject<'a, 'py> for Spanish<'a, 'py> {
    type Error = PyErr;

    fn extract(ob: Borrowed<'a, 'py, PyAny>) -> Result<Self, Self::Error> {
        if let Ok(span) = ob.cast_exact::<RySpan>() {
            Ok(Self::Span(span))
        } else if let Ok(duration) = ob.cast_exact::<PyDuration>() {
            Ok(Self::Duration(duration))
        } else if let Ok(signed_duration) = ob.cast_exact::<RySignedDuration>() {
            Ok(Self::SignedDuration(signed_duration))
        } else if let Ok(signed_duration) = ob.cast_exact::<PyDelta>() {
            let signed_duration = signed_duration.extract::<SignedDuration>()?;
            Ok(Self::PyTimeDelta(signed_duration))
        } else {
            py_type_err!(
                "Expected a Timespan, Duration, SignedDuration, or datetime.timedelta object"
            )
        }
    }
}

impl From<Spanish<'_, '_>> for DateArithmetic {
    fn from(value: Spanish<'_, '_>) -> Self {
        match value {
            Spanish::Span(span) => Self::from(span.get().0),
            Spanish::Duration(duration) => Self::from(duration.get().0),
            Spanish::SignedDuration(signed_duration) => Self::from(signed_duration.get().0),
            Spanish::PyTimeDelta(signed_duration) => Self::from(signed_duration),
        }
    }
}

impl From<Spanish<'_, '_>> for DateTimeArithmetic {
    fn from(value: Spanish<'_, '_>) -> Self {
        match value {
            Spanish::Span(span) => Self::from(span.get().0),
            Spanish::Duration(duration) => Self::from(duration.get().0),
            Spanish::SignedDuration(signed_duration) => Self::from(signed_duration.get().0),
            Spanish::PyTimeDelta(signed_duration) => Self::from(signed_duration),
        }
    }
}

impl From<Spanish<'_, '_>> for OffsetArithmetic {
    fn from(value: Spanish<'_, '_>) -> Self {
        match value {
            Spanish::Span(span) => Self::from(span.get().0),
            Spanish::Duration(duration) => Self::from(duration.get().0),
            Spanish::SignedDuration(signed_duration) => Self::from(signed_duration.get().0),
            Spanish::PyTimeDelta(signed_duration) => Self::from(signed_duration),
        }
    }
}

impl From<Spanish<'_, '_>> for TimeArithmetic {
    fn from(value: Spanish<'_, '_>) -> Self {
        match value {
            Spanish::Span(span) => Self::from(span.get().0),
            Spanish::Duration(duration) => Self::from(duration.get().0),
            Spanish::SignedDuration(signed_duration) => Self::from(signed_duration.get().0),
            Spanish::PyTimeDelta(signed_duration) => Self::from(signed_duration),
        }
    }
}

impl From<Spanish<'_, '_>> for TimestampArithmetic {
    fn from(value: Spanish<'_, '_>) -> Self {
        match value {
            Spanish::Span(span) => Self::from(span.get().0),
            Spanish::Duration(duration) => Self::from(duration.get().0),
            Spanish::SignedDuration(signed_duration) => Self::from(signed_duration.get().0),
            Spanish::PyTimeDelta(signed_duration) => Self::from(signed_duration),
        }
    }
}

impl From<Spanish<'_, '_>> for ZonedArithmetic {
    fn from(value: Spanish<'_, '_>) -> Self {
        match value {
            Spanish::Span(span) => Self::from(span.get().0),
            Spanish::Duration(duration) => Self::from(duration.get().0),
            Spanish::SignedDuration(signed_duration) => Self::from(signed_duration.get().0),
            Spanish::PyTimeDelta(signed_duration) => Self::from(signed_duration),
        }
    }
}
