use crate::functions::span;
use pyo3::prelude::*;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) struct SpanKwargs {
    years: i64,
    months: i64,
    weeks: i64,
    days: i64,
    hours: i64,
    minutes: i64,
    seconds: i64,
    milliseconds: i64,
    microseconds: i64,
    nanoseconds: i64,
}

macro_rules! kw_builder {
    ($($field:ident),*) => {
        $(
            pub(crate) fn $field(mut self, value: i64) -> Self {
                self.$field = value;
                self
            }
        )*
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

    kw_builder!(
        years,
        months,
        weeks,
        days,
        hours,
        minutes,
        seconds,
        milliseconds,
        microseconds,
        nanoseconds
    );

    pub(crate) fn build(self) -> PyResult<jiff::Span> {
        jiff::Span::try_from(self)
    }

    pub(crate) fn is_zero(&self) -> bool {
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
