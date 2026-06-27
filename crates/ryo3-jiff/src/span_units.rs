use jiff::Span;
use pyo3::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

impl SpanUnit {
    /// Return the `SpanUnit` from an index (0-9) for iteration
    ///
    /// # Panics
    ///
    /// Panics if the index is not in 0..=9
    fn from_idx(index: u8) -> Self {
        match index {
            0 => Self::Years,
            1 => Self::Months,
            2 => Self::Weeks,
            3 => Self::Days,
            4 => Self::Hours,
            5 => Self::Minutes,
            6 => Self::Seconds,
            7 => Self::Milliseconds,
            8 => Self::Microseconds,
            9 => Self::Nanoseconds,
            _ => unreachable!(),
        }
    }
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

pub(crate) struct SpanUnitsIter {
    units: u16,
    index: u8,
}

impl SpanUnitsIter {
    const MAX_INDEX: u8 = 10;
}

impl Iterator for SpanUnitsIter {
    type Item = SpanUnit;

    #[expect(
        clippy::arithmetic_side_effects,
        reason = "wenodis: index is always < 10, so no overflow can occur"
    )]
    fn next(&mut self) -> Option<Self::Item> {
        while self.index < Self::MAX_INDEX {
            let unit = SpanUnit::from_idx(self.index);
            self.index += 1;
            if (self.units & (unit as u16)) != 0 {
                return Some(unit);
            }
        }
        None
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.units.count_ones() as usize;
        (remaining, Some(remaining))
    }
}

impl IntoIterator for SpanUnits {
    type Item = SpanUnit;
    type IntoIter = SpanUnitsIter;

    fn into_iter(self) -> Self::IntoIter {
        SpanUnitsIter {
            units: self.0,
            index: 0,
        }
    }
}

impl ExactSizeIterator for SpanUnitsIter {
    fn len(&self) -> usize {
        self.units.count_ones() as usize
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
