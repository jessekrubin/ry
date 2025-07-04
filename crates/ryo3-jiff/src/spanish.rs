//! Span-ish ry/python object(s)

use crate::{RySignedDuration, RySpan};
use jiff::civil::{DateArithmetic, DateTimeArithmetic, TimeArithmetic};
use jiff::{TimestampArithmetic, ZonedArithmetic};
use pyo3::prelude::*;
use ryo3_std::PyDuration;

enum RySpanishObject<'py> {
    Span(&'py Bound<'py, RySpan>),
    Duration(&'py Bound<'py, PyDuration>),
    SignedDuration(&'py Bound<'py, RySignedDuration>),
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
            RySpanishObject::Span(span) => TimestampArithmetic::from(span.get().0),
            RySpanishObject::Duration(duration) => TimestampArithmetic::from(duration.get().0),
            RySpanishObject::SignedDuration(signed_duration) => {
                TimestampArithmetic::from(signed_duration.get().0)
            }
        }
    }
}
impl From<Spanish<'_>> for ZonedArithmetic {
    fn from(val: Spanish<'_>) -> Self {
        match val.inner {
            RySpanishObject::Span(span) => ZonedArithmetic::from(span.get().0),
            RySpanishObject::Duration(duration) => ZonedArithmetic::from(duration.get().0),
            RySpanishObject::SignedDuration(signed_duration) => {
                ZonedArithmetic::from(signed_duration.get().0)
            }
        }
    }
}

impl From<Spanish<'_>> for DateArithmetic {
    fn from(val: Spanish<'_>) -> Self {
        match val.inner {
            RySpanishObject::Span(span) => DateArithmetic::from(span.get().0),
            RySpanishObject::Duration(duration) => DateArithmetic::from(duration.get().0),
            RySpanishObject::SignedDuration(signed_duration) => {
                DateArithmetic::from(signed_duration.get().0)
            }
        }
    }
}

impl From<Spanish<'_>> for DateTimeArithmetic {
    fn from(val: Spanish<'_>) -> Self {
        match val.inner {
            RySpanishObject::Span(span) => DateTimeArithmetic::from(span.get().0),
            RySpanishObject::Duration(duration) => DateTimeArithmetic::from(duration.get().0),
            RySpanishObject::SignedDuration(signed_duration) => {
                DateTimeArithmetic::from(signed_duration.get().0)
            }
        }
    }
}

impl From<Spanish<'_>> for TimeArithmetic {
    fn from(val: Spanish<'_>) -> Self {
        match val.inner {
            RySpanishObject::Span(span) => TimeArithmetic::from(span.get().0),
            RySpanishObject::Duration(duration) => TimeArithmetic::from(duration.get().0),
            RySpanishObject::SignedDuration(signed_duration) => {
                TimeArithmetic::from(signed_duration.get().0)
            }
        }
    }
}
