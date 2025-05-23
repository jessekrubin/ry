use std::fmt::Display;

#[derive(Debug)]
pub struct JiffDate(pub jiff::civil::Date);
#[derive(Debug)]
pub struct JiffTime(pub jiff::civil::Time);
#[derive(Debug)]
pub struct JiffDateTime(pub jiff::civil::DateTime);
#[derive(Debug, Clone)]
pub struct JiffZoned(pub jiff::Zoned);
#[derive(Debug)]
pub struct JiffSpan(pub jiff::Span);
#[derive(Debug, Clone)]
pub struct JiffTimeZone(pub jiff::tz::TimeZone);
#[derive(Debug)]
pub struct JiffOffset(pub jiff::tz::Offset);
#[derive(Debug)]
pub struct JiffSignedDuration(pub jiff::SignedDuration);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct JiffUnit(pub(crate) jiff::Unit);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct JiffRoundMode(pub(crate) jiff::RoundMode);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct JiffWeekday(pub(crate) jiff::civil::Weekday);
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct JiffEra(pub(crate) jiff::civil::Era);
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct JiffEraYear(pub(crate) (i16, jiff::civil::Era));
#[derive(Debug)]
pub struct JiffTzDisambiguation(pub jiff::tz::Disambiguation);
#[derive(Debug)]
pub struct JiffTzOffsetConflict(pub jiff::tz::OffsetConflict);

// ============================================================================
impl From<jiff::civil::Date> for JiffDate {
    fn from(value: jiff::civil::Date) -> Self {
        JiffDate(value)
    }
}

impl From<jiff::civil::Time> for JiffTime {
    fn from(value: jiff::civil::Time) -> Self {
        JiffTime(value)
    }
}

impl From<jiff::civil::DateTime> for JiffDateTime {
    fn from(value: jiff::civil::DateTime) -> Self {
        JiffDateTime(value)
    }
}

impl From<jiff::civil::Era> for JiffEra {
    fn from(value: jiff::civil::Era) -> Self {
        JiffEra(value)
    }
}

impl From<jiff::tz::Disambiguation> for JiffTzDisambiguation {
    fn from(value: jiff::tz::Disambiguation) -> Self {
        JiffTzDisambiguation(value)
    }
}

impl From<jiff::Zoned> for JiffZoned {
    fn from(value: jiff::Zoned) -> Self {
        JiffZoned(value)
    }
}

impl From<jiff::Span> for JiffSpan {
    fn from(value: jiff::Span) -> Self {
        JiffSpan(value)
    }
}

impl From<jiff::tz::TimeZone> for JiffTimeZone {
    fn from(value: jiff::tz::TimeZone) -> Self {
        JiffTimeZone(value)
    }
}

impl From<jiff::tz::Offset> for JiffOffset {
    fn from(value: jiff::tz::Offset) -> Self {
        JiffOffset(value)
    }
}

impl From<jiff::SignedDuration> for JiffSignedDuration {
    fn from(value: jiff::SignedDuration) -> Self {
        JiffSignedDuration(value)
    }
}

impl From<jiff::Unit> for JiffUnit {
    fn from(value: jiff::Unit) -> Self {
        JiffUnit(value)
    }
}

impl From<jiff::civil::Weekday> for JiffWeekday {
    fn from(value: jiff::civil::Weekday) -> Self {
        JiffWeekday(value)
    }
}

impl From<jiff::tz::OffsetConflict> for JiffTzOffsetConflict {
    fn from(value: jiff::tz::OffsetConflict) -> Self {
        JiffTzOffsetConflict(value)
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
            jiff::RoundMode::HalfCeil => "half_ceil",
            jiff::RoundMode::HalfFloor => "half_floor",
            jiff::RoundMode::HalfExpand => "half_expand",
            jiff::RoundMode::HalfTrunc => "half_trunc",
            jiff::RoundMode::HalfEven => "half_even",
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
