//! Span-ish ry/python object(s)

use crate::{RySignedDuration, RySpan};
use jiff::civil::{DateArithmetic, DateTimeArithmetic, TimeArithmetic};
use jiff::{SignedDuration, TimestampArithmetic, ZonedArithmetic};
use pyo3::prelude::*;
use pyo3::types::PyDelta;
use ryo3_std::PyDuration;

enum RySpanishObject<'py> {
    Span(&'py Bound<'py, RySpan>),
    Duration(&'py Bound<'py, PyDuration>),
    SignedDuration(&'py Bound<'py, RySignedDuration>),
    PyTimeDelta(SignedDuration),
}

pub(crate) struct Spanish<'py> {
    inner: RySpanishObject<'py>,
}
impl<'py> TryFrom<&'py Bound<'py, PyAny>> for Spanish<'py> {
    type Error = PyErr;

    fn try_from(ob: &'py Bound<'py, PyAny>) -> Result<Self, Self::Error> {
        let inner = if let Ok(span) = ob.downcast::<RySpan>() {
            RySpanishObject::Span(span)
        } else if let Ok(duration) = ob.downcast::<PyDuration>() {
            RySpanishObject::Duration(duration)
        } else if let Ok(signed_duration) = ob.downcast::<RySignedDuration>() {
            RySpanishObject::SignedDuration(signed_duration)
        } else if let Ok(signed_duration) = ob.downcast::<PyDelta>() {
            let signed_duration = signed_duration.extract::<SignedDuration>()?;
            RySpanishObject::PyTimeDelta(signed_duration)
        } else {
            return Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                "Expected a RySpan, PyDuration or Signed PyDuration",
            ));
        };
        Ok(Spanish { inner })
    }
}

impl From<Spanish<'_>> for TimestampArithmetic {
    fn from(val: Spanish<'_>) -> Self {
        match val.inner {
            RySpanishObject::Span(span) => Self::from(span.get().0),
            RySpanishObject::Duration(duration) => Self::from(duration.get().0),
            RySpanishObject::SignedDuration(signed_duration) => Self::from(signed_duration.get().0),
            RySpanishObject::PyTimeDelta(signed_duration) => Self::from(signed_duration),
        }
    }
}

impl From<Spanish<'_>> for ZonedArithmetic {
    fn from(val: Spanish<'_>) -> Self {
        match val.inner {
            RySpanishObject::Span(span) => Self::from(span.get().0),
            RySpanishObject::Duration(duration) => Self::from(duration.get().0),
            RySpanishObject::SignedDuration(signed_duration) => Self::from(signed_duration.get().0),
            RySpanishObject::PyTimeDelta(signed_duration) => Self::from(signed_duration),
        }
    }
}

impl From<Spanish<'_>> for DateArithmetic {
    fn from(val: Spanish<'_>) -> Self {
        match val.inner {
            RySpanishObject::Span(span) => Self::from(span.get().0),
            RySpanishObject::Duration(duration) => Self::from(duration.get().0),
            RySpanishObject::SignedDuration(signed_duration) => Self::from(signed_duration.get().0),
            RySpanishObject::PyTimeDelta(signed_duration) => Self::from(signed_duration),
        }
    }
}

impl From<Spanish<'_>> for DateTimeArithmetic {
    fn from(val: Spanish<'_>) -> Self {
        match val.inner {
            RySpanishObject::Span(span) => Self::from(span.get().0),
            RySpanishObject::Duration(duration) => Self::from(duration.get().0),
            RySpanishObject::SignedDuration(signed_duration) => Self::from(signed_duration.get().0),
            RySpanishObject::PyTimeDelta(signed_duration) => Self::from(signed_duration),
        }
    }
}

impl From<Spanish<'_>> for TimeArithmetic {
    fn from(val: Spanish<'_>) -> Self {
        match val.inner {
            RySpanishObject::Span(span) => Self::from(span.get().0),
            RySpanishObject::Duration(duration) => Self::from(duration.get().0),
            RySpanishObject::SignedDuration(signed_duration) => Self::from(signed_duration.get().0),
            RySpanishObject::PyTimeDelta(signed_duration) => Self::from(signed_duration),
        }
    }
}
