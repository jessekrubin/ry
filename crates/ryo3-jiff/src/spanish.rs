//! Span-ish ry/python object(s)

use crate::{RySignedDuration, RySpan};
use jiff::civil::{DateArithmetic, DateTimeArithmetic, TimeArithmetic};
use jiff::tz::OffsetArithmetic;
use jiff::{SignedDuration, Span, TimestampArithmetic, ZonedArithmetic};
use pyo3::prelude::*;
use pyo3::types::PyDelta;
use ryo3_std::time::PyDuration;

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
        let inner = if let Ok(span) = ob.cast::<RySpan>() {
            RySpanishObject::Span(span)
        } else if let Ok(duration) = ob.cast::<PyDuration>() {
            RySpanishObject::Duration(duration)
        } else if let Ok(signed_duration) = ob.cast::<RySignedDuration>() {
            RySpanishObject::SignedDuration(signed_duration)
        } else if let Ok(signed_duration) = ob.cast::<PyDelta>() {
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

impl TryFrom<Spanish<'_>> for Span {
    type Error = PyErr;
    fn try_from(val: Spanish<'_>) -> Result<Self, Self::Error> {
        match val.inner {
            RySpanishObject::Span(span) => Ok(span.get().0),
            RySpanishObject::Duration(duration) => Self::try_from(duration.get().0)
                .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e}"))),
            RySpanishObject::SignedDuration(signed_duration) => {
                let sd = signed_duration.get().0;
                Self::try_from(sd)
                    .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e}")))
            }
            RySpanishObject::PyTimeDelta(signed_duration) => Self::try_from(signed_duration)
                .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e}"))),
        }
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

impl From<Spanish<'_>> for OffsetArithmetic {
    fn from(val: Spanish<'_>) -> Self {
        match val.inner {
            RySpanishObject::Span(span) => Self::from(span.get().0),
            RySpanishObject::Duration(duration) => Self::from(duration.get().0),
            RySpanishObject::SignedDuration(signed_duration) => Self::from(signed_duration.get().0),
            RySpanishObject::PyTimeDelta(signed_duration) => Self::from(signed_duration),
        }
    }
}
