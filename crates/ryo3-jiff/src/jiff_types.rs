use std::{fmt::Display, ops::Deref};

#[derive(Debug, Clone, Copy)]
pub struct JiffDate(pub jiff::civil::Date);
#[derive(Debug, Clone, Copy)]
pub struct JiffTime(pub jiff::civil::Time);
#[derive(Debug, Clone, Copy)]
pub struct JiffDateTime(pub jiff::civil::DateTime);
#[derive(Debug, Clone)]
pub struct JiffZoned(pub jiff::Zoned);
#[derive(Debug, Clone, Copy)]
pub struct JiffSpan(pub jiff::Span);
#[derive(Debug, Clone)]
pub struct JiffTimeZone(pub jiff::tz::TimeZone);
#[derive(Clone, Copy, Debug)]
pub struct JiffOffset(pub jiff::tz::Offset);
#[derive(Clone, Copy, Debug)]
pub struct JiffSignedDuration(pub jiff::SignedDuration);

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct JiffUnit(pub(crate) jiff::Unit);

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct JiffRoundMode(pub(crate) jiff::RoundMode);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct JiffWeekday(pub(crate) jiff::civil::Weekday);
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct JiffEra(pub(crate) jiff::civil::Era);
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct JiffEraYear(pub(crate) (i16, jiff::civil::Era));
#[derive(Debug, Clone, Copy)]
pub struct JiffTzDisambiguation(pub jiff::tz::Disambiguation);
#[derive(Debug, Clone, Copy)]
pub struct JiffTzOffsetConflict(pub jiff::tz::OffsetConflict);

// ============================================================================
impl From<jiff::civil::Date> for JiffDate {
    fn from(value: jiff::civil::Date) -> Self {
        Self(value)
    }
}

impl From<jiff::civil::Time> for JiffTime {
    fn from(value: jiff::civil::Time) -> Self {
        Self(value)
    }
}

impl From<jiff::civil::DateTime> for JiffDateTime {
    fn from(value: jiff::civil::DateTime) -> Self {
        Self(value)
    }
}

impl From<jiff::civil::Era> for JiffEra {
    fn from(value: jiff::civil::Era) -> Self {
        Self(value)
    }
}

impl From<jiff::tz::Disambiguation> for JiffTzDisambiguation {
    fn from(value: jiff::tz::Disambiguation) -> Self {
        Self(value)
    }
}

impl From<jiff::Zoned> for JiffZoned {
    fn from(value: jiff::Zoned) -> Self {
        Self(value)
    }
}

impl From<jiff::Span> for JiffSpan {
    fn from(value: jiff::Span) -> Self {
        Self(value)
    }
}

impl From<jiff::tz::TimeZone> for JiffTimeZone {
    fn from(value: jiff::tz::TimeZone) -> Self {
        Self(value)
    }
}

impl From<jiff::tz::Offset> for JiffOffset {
    fn from(value: jiff::tz::Offset) -> Self {
        Self(value)
    }
}

impl From<jiff::SignedDuration> for JiffSignedDuration {
    fn from(value: jiff::SignedDuration) -> Self {
        Self(value)
    }
}

impl From<jiff::Unit> for JiffUnit {
    fn from(value: jiff::Unit) -> Self {
        Self(value)
    }
}

impl From<jiff::civil::Weekday> for JiffWeekday {
    fn from(value: jiff::civil::Weekday) -> Self {
        Self(value)
    }
}

impl From<jiff::tz::OffsetConflict> for JiffTzOffsetConflict {
    fn from(value: jiff::tz::OffsetConflict) -> Self {
        Self(value)
    }
}

// ============================================================================

impl From<JiffDate> for jiff::civil::Date {
    fn from(val: JiffDate) -> Self {
        val.0
    }
}

impl From<JiffTime> for jiff::civil::Time {
    fn from(val: JiffTime) -> Self {
        val.0
    }
}

impl From<JiffDateTime> for jiff::civil::DateTime {
    fn from(val: JiffDateTime) -> Self {
        val.0
    }
}

impl From<JiffZoned> for jiff::Zoned {
    fn from(val: JiffZoned) -> Self {
        val.0
    }
}

impl From<JiffSpan> for jiff::Span {
    fn from(val: JiffSpan) -> Self {
        val.0
    }
}

impl From<JiffTimeZone> for jiff::tz::TimeZone {
    fn from(val: JiffTimeZone) -> Self {
        val.0
    }
}

impl From<JiffOffset> for jiff::tz::Offset {
    fn from(val: JiffOffset) -> Self {
        val.0
    }
}

impl From<JiffSignedDuration> for jiff::SignedDuration {
    fn from(val: JiffSignedDuration) -> Self {
        val.0
    }
}

impl From<JiffUnit> for jiff::Unit {
    fn from(val: JiffUnit) -> Self {
        val.0
    }
}

impl From<JiffWeekday> for jiff::civil::Weekday {
    fn from(val: JiffWeekday) -> Self {
        val.0
    }
}

// ============================================================================
// JIFF UNIT
// ============================================================================
impl JiffUnit {
    #[must_use]
    pub fn static_str(self) -> &'static str {
        match self.0 {
            jiff::Unit::Year => "year",
            jiff::Unit::Month => "month",
            jiff::Unit::Week => "week",
            jiff::Unit::Day => "day",
            jiff::Unit::Hour => "hour",
            jiff::Unit::Minute => "minute",
            jiff::Unit::Second => "second",
            jiff::Unit::Millisecond => "millisecond",
            jiff::Unit::Microsecond => "microsecond",
            jiff::Unit::Nanosecond => "nanosecond",
        }
    }
}

impl Display for JiffUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self.static_str();
        write!(f, "{s}")
    }
}

impl Deref for JiffUnit {
    type Target = jiff::Unit;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
// ============================================================================
// ROUND MODE
// ============================================================================
impl JiffRoundMode {
    fn static_str(self) -> &'static str {
        match self.0 {
            jiff::RoundMode::Ceil => "ceil",
            jiff::RoundMode::Floor => "floor",
            jiff::RoundMode::Expand => "expand",
            jiff::RoundMode::Trunc => "trunc",
            jiff::RoundMode::HalfCeil => "half-ceil",
            jiff::RoundMode::HalfFloor => "half-floor",
            jiff::RoundMode::HalfExpand => "half-expand",
            jiff::RoundMode::HalfTrunc => "half-trunc",
            jiff::RoundMode::HalfEven => "half-even",
            _ => "round_unknown",
        }
    }
}

impl Display for JiffRoundMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self.static_str();
        write!(f, "{s}")
    }
}

impl Deref for JiffRoundMode {
    type Target = jiff::RoundMode;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// ============================================================================
// WEEKDAY
// ============================================================================
impl JiffWeekday {
    fn static_str(self) -> &'static str {
        match self.0 {
            jiff::civil::Weekday::Monday => "monday",
            jiff::civil::Weekday::Tuesday => "tuesday",
            jiff::civil::Weekday::Wednesday => "wednesday",
            jiff::civil::Weekday::Thursday => "thursday",
            jiff::civil::Weekday::Friday => "friday",
            jiff::civil::Weekday::Saturday => "saturday",
            jiff::civil::Weekday::Sunday => "sunday",
        }
    }
}

impl Display for JiffWeekday {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self.static_str();
        write!(f, "{s}")
    }
}
