use jiff::Span;
use pyo3::prelude::*;

pub(crate) enum SpanUnit {
    Years = 1 << 0,
    Months = 1 << 1,
    Weeks = 1 << 2,
    Days = 1 << 3,
    Hours = 1 << 4,
    Minutes = 1 << 5,
    Seconds = 1 << 6,
    Milliseconds = 1 << 7,
    Microseconds = 1 << 8,
    Nanoseconds = 1 << 9,
}

impl<'py> IntoPyObject<'py> for SpanUnit {
    type Target = pyo3::types::PyString;
    type Output = Borrowed<'py, 'py, Self::Target>;
    type Error = std::convert::Infallible;

    #[inline]
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        let s = match self {
            Self::Years => crate::interns::years(py),
            Self::Months => crate::interns::months(py),
            Self::Weeks => crate::interns::weeks(py),
            Self::Days => crate::interns::days(py),
            Self::Hours => crate::interns::hours(py),
            Self::Minutes => crate::interns::minutes(py),
            Self::Seconds => crate::interns::seconds(py),
            Self::Milliseconds => crate::interns::milliseconds(py),
            Self::Microseconds => crate::interns::microseconds(py),
            Self::Nanoseconds => crate::interns::nanoseconds(py),
        };
        Ok(s.as_borrowed())
    }
}

/// bit flags for wot span units a span has
pub(crate) struct SpanUnits(u16);

impl SpanUnits {
    #[expect(
        clippy::cast_possible_truncation,
        reason = "wenodis: cant fail bc there are only 10 span units"
    )]
    pub(crate) fn count(&self) -> u8 {
        self.0.count_ones() as u8
    }
}

impl From<&Span> for SpanUnits {
    fn from(span: &Span) -> Self {
        let mut units = 0;
        if span.get_years() != 0 {
            units |= SpanUnit::Years as u16;
        }
        if span.get_months() != 0 {
            units |= SpanUnit::Months as u16;
        }
        if span.get_weeks() != 0 {
            units |= SpanUnit::Weeks as u16;
        }
        if span.get_days() != 0 {
            units |= SpanUnit::Days as u16;
        }
        if span.get_hours() != 0 {
            units |= SpanUnit::Hours as u16;
        }
        if span.get_minutes() != 0 {
            units |= SpanUnit::Minutes as u16;
        }
        if span.get_seconds() != 0 {
            units |= SpanUnit::Seconds as u16;
        }
        if span.get_milliseconds() != 0 {
            units |= SpanUnit::Milliseconds as u16;
        }
        if span.get_microseconds() != 0 {
            units |= SpanUnit::Microseconds as u16;
        }
        if span.get_nanoseconds() != 0 {
            units |= SpanUnit::Nanoseconds as u16;
        }
        Self(units)
    }
}
