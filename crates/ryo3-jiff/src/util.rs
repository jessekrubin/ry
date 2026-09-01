use pyo3::prelude::*;

use crate::functions::span;
use crate::span_units::{SpanUnit, SpanUnitsMask};

/// `SpanKwargs` struct used to build a `jiff::Span` from keyword arguments
///
/// Designed w/ mask to detect kwarg presence.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) struct SpanKwargs {
    /// presence mask
    mask: SpanUnitsMask,
    // __unit_values__
    years: i16,
    months: i32,
    weeks: i32,
    days: i32,
    hours: i32,
    minutes: i64,
    seconds: i64,
    milliseconds: i64,
    microseconds: i64,
    nanoseconds: i64,
}

macro_rules! kw_builder {
    ($field:ident, $field_opt:ident, $unit:ident, $itype:ty) => {
        pub(crate) const fn $field(mut self, value: $itype) -> Self {
            if value != 0 {
                self.mask = self.mask.with_unit(SpanUnit::$unit);
            }
            self.$field = value;
            self
        }

        pub(crate) const fn $field_opt(mut self, value: Option<$itype>) -> Self {
            if let Some(value) = value {
                self.mask = self.mask.with_unit(SpanUnit::$unit);
                self.$field = value;
            }
            self
        }
    };
}

impl TryFrom<SpanKwargs> for jiff::Span {
    type Error = PyErr;

    fn try_from(value: SpanKwargs) -> Result<Self, Self::Error> {
        span(
            value.years,
            value.months,
            value.weeks,
            value.days,
            value.hours,
            value.minutes,
            value.seconds,
            value.milliseconds,
            value.microseconds,
            value.nanoseconds,
        )
    }
}

impl SpanKwargs {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    pub(crate) const fn is_empty(&self) -> bool {
        self.mask.count() == 0
    }

    kw_builder!(years, years_opt, Years, i16);
    kw_builder!(months, months_opt, Months, i32);
    kw_builder!(weeks, weeks_opt, Weeks, i32);
    kw_builder!(days, days_opt, Days, i32);
    kw_builder!(hours, hours_opt, Hours, i32);
    kw_builder!(minutes, minutes_opt, Minutes, i64);
    kw_builder!(seconds, seconds_opt, Seconds, i64);
    kw_builder!(milliseconds, milliseconds_opt, Milliseconds, i64);
    kw_builder!(microseconds, microseconds_opt, Microseconds, i64);
    kw_builder!(nanoseconds, nanoseconds_opt, Nanoseconds, i64);

    pub(crate) fn build(self) -> PyResult<jiff::Span> {
        jiff::Span::try_from(self)
    }

    pub(crate) const fn is_zero(&self) -> bool {
        self.years == 0
            && self.months == 0
            && self.weeks == 0
            && self.days == 0
            && self.hours == 0
            && self.minutes == 0
            && self.seconds == 0
            && self.milliseconds == 0
            && self.microseconds == 0
            && self.nanoseconds == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const SIZEOF_SPAN_KWARGS: usize = std::mem::size_of::<SpanKwargs>();

    #[test]
    fn test_size_of_span_kwargs() {
        assert_eq!(SIZEOF_SPAN_KWARGS, 64);
    }
}
