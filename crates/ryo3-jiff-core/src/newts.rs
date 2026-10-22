//! jiff-new-types
//!
//! new type wrappers for the jiff types + their consts/`Display` impls; the
//! python conversions live in `to_py.rs` and `from_py.rs`

// ----------------------------------------------------------------------------
// MACROS
// ----------------------------------------------------------------------------

/// impl `new()`, `inner()` (ref to inner) and `into_inner()` (consume and
/// return the inner value) + `From` both ways for a new-type
///
/// const form used for `Copy` inners
macro_rules! impl_newtype {
    (const $newt:ty, $inner:ty) => {
        impl $newt {
            #[inline]
            #[must_use]
            pub const fn into_inner(self) -> $inner {
                self.0
            }
        }
        impl_newtype!(@common $newt, $inner);
    };
    ($newt:ty, $inner:ty) => {
        impl $newt {
            #[inline]
            #[must_use]
            pub fn into_inner(self) -> $inner {
                self.0
            }
        }
        impl_newtype!(@common $newt, $inner);
    };
    (@common $newt:ty, $inner:ty) => {
        impl $newt {
            #[inline]
            #[must_use]
            pub const fn new(inner: $inner) -> Self {
                Self(inner)
            }

            #[inline]
            #[must_use]
            pub const fn inner(&self) -> &$inner {
                &self.0
            }
        }

        impl From<$inner> for $newt {
            #[inline]
            fn from(inner: $inner) -> Self {
                Self::new(inner)
            }
        }

        impl From<$newt> for $inner {
            #[inline]
            fn from(newt: $newt) -> Self {
                newt.into_inner()
            }
        }
    };
}

/// impl `new()` and `inner()` + `From<&inner>` for a borrowed new-type
macro_rules! impl_new_type_ref_functions {
    ($newt:ident, $inner:ty) => {
        impl<'a> $newt<'a> {
            #[inline]
            #[must_use]
            pub const fn new(inner: &'a $inner) -> Self {
                Self(inner)
            }

            #[inline]
            #[must_use]
            pub const fn inner(&self) -> &'a $inner {
                self.0
            }
        }

        impl<'a> From<&'a $inner> for $newt<'a> {
            #[inline]
            fn from(inner: &'a $inner) -> Self {
                Self::new(inner)
            }
        }
    };
}

// ----------------------------------------------------------------------------
// NEWTS
// ----------------------------------------------------------------------------
#[derive(Debug, Clone, Copy)]
pub struct JiffDate(jiff::civil::Date);

#[derive(Debug, Clone, Copy)]
pub struct JiffTime(jiff::civil::Time);

#[derive(Debug, Clone, Copy)]
pub struct JiffDateTime(jiff::civil::DateTime);

#[derive(Debug, Clone, Copy)]
pub struct JiffTimestamp(jiff::Timestamp);

#[derive(Debug, Clone)]
pub struct JiffZoned(jiff::Zoned);

#[derive(Debug, Clone, Copy)]
pub struct JiffSpan(jiff::Span);

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct JiffTimeZone(jiff::tz::TimeZone);

#[derive(Clone, Copy, Debug)]
pub struct JiffOffset(jiff::tz::Offset);

#[derive(Clone, Copy, Debug)]
pub struct JiffSignedDuration(jiff::SignedDuration);

// ----------------------------------------------------------------------------
// REF NEWTS
// ----------------------------------------------------------------------------
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct JiffTimeZoneRef<'a>(&'a jiff::tz::TimeZone);

#[derive(Debug, Clone, Copy)]
pub struct JiffZonedRef<'a>(&'a jiff::Zoned);

#[derive(Debug, Clone, Copy)]
pub struct JiffSignedDurationRef<'a>(&'a jiff::SignedDuration);

// ----------------------------------------------------------------------------
// ENUMS ~ ENUMS ~ ENUMS ~ ENUMS ~ ENUMS ~ ENUMS ~ ENUMS ~ ENUMS ~ ENUMS
// ----------------------------------------------------------------------------
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct JiffUnit(jiff::Unit);

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct JiffRoundMode(jiff::RoundMode);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct JiffWeekday(jiff::civil::Weekday);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct JiffEra(jiff::civil::Era);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct JiffEraYear((i16, jiff::civil::Era));

#[derive(Debug, Clone, Copy)]
pub struct JiffTzDisambiguation(jiff::tz::Disambiguation);

#[derive(Debug, Clone, Copy)]
pub struct JiffTzOffsetConflict(jiff::tz::OffsetConflict);

#[derive(Debug, Clone, Copy)]
pub struct JiffFriendlyDesignator(jiff::fmt::friendly::Designator);

// ----------------------------------------------------------------------------
// EXTRACTORS
// ----------------------------------------------------------------------------

/// datetime extractor for `datetime.datetime` obs
///
/// primarily used for extracting naive or aware datetimes
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JiffDateTimeOrZoned {
    Aware(jiff::Zoned),
    Naive(jiff::civil::DateTime),
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

// jiff::*
impl_newtype!(const JiffSignedDuration, jiff::SignedDuration);
impl_newtype!(const JiffSpan, jiff::Span);
impl_newtype!(const JiffTimestamp, jiff::Timestamp);
impl_newtype!(const JiffUnit, jiff::Unit);
impl_newtype!(const JiffRoundMode, jiff::RoundMode);
impl_newtype!(JiffZoned, jiff::Zoned);
// jiff::civil::*
impl_newtype!(const JiffDate, jiff::civil::Date);
impl_newtype!(const JiffDateTime, jiff::civil::DateTime);
impl_newtype!(const JiffEra, jiff::civil::Era);
impl_newtype!(const JiffTime, jiff::civil::Time);
impl_newtype!(const JiffWeekday, jiff::civil::Weekday);
// jiff::tz::*
impl_newtype!(const JiffOffset, jiff::tz::Offset);
impl_newtype!(JiffTimeZone, jiff::tz::TimeZone);
impl_newtype!(const JiffTzDisambiguation, jiff::tz::Disambiguation);
impl_newtype!(const JiffTzOffsetConflict, jiff::tz::OffsetConflict);
// jiff::fmt::friendly::*
impl_newtype!(const JiffFriendlyDesignator, jiff::fmt::friendly::Designator);
// jiff-other
impl_newtype!(const JiffEraYear, (i16, jiff::civil::Era));

// REFS
impl_new_type_ref_functions!(JiffZonedRef, jiff::Zoned);
impl_new_type_ref_functions!(JiffTimeZoneRef, jiff::tz::TimeZone);
impl_new_type_ref_functions!(JiffSignedDurationRef, jiff::SignedDuration);

// ============================================================================
// JIFF UNIT
// ============================================================================
impl JiffUnit {
    pub const YEAR: Self = Self(jiff::Unit::Year);
    pub const MONTH: Self = Self(jiff::Unit::Month);
    pub const WEEK: Self = Self(jiff::Unit::Week);
    pub const DAY: Self = Self(jiff::Unit::Day);
    pub const HOUR: Self = Self(jiff::Unit::Hour);
    pub const MINUTE: Self = Self(jiff::Unit::Minute);
    pub const SECOND: Self = Self(jiff::Unit::Second);
    pub const MILLISECOND: Self = Self(jiff::Unit::Millisecond);
    pub const MICROSECOND: Self = Self(jiff::Unit::Microsecond);
    pub const NANOSECOND: Self = Self(jiff::Unit::Nanosecond);

    #[must_use]
    pub const fn as_str(self) -> &'static str {
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

impl std::fmt::Display for JiffUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

// ============================================================================
// ROUND MODE
// ============================================================================
impl JiffRoundMode {
    pub const CEIL: Self = Self(jiff::RoundMode::Ceil);
    pub const FLOOR: Self = Self(jiff::RoundMode::Floor);
    pub const EXPAND: Self = Self(jiff::RoundMode::Expand);
    pub const TRUNC: Self = Self(jiff::RoundMode::Trunc);
    pub const HALF_CEIL: Self = Self(jiff::RoundMode::HalfCeil);
    pub const HALF_FLOOR: Self = Self(jiff::RoundMode::HalfFloor);
    pub const HALF_EXPAND: Self = Self(jiff::RoundMode::HalfExpand);
    pub const HALF_TRUNC: Self = Self(jiff::RoundMode::HalfTrunc);
    pub const HALF_EVEN: Self = Self(jiff::RoundMode::HalfEven);

    const fn as_str(self) -> &'static str {
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
            _ => "round-unknown",
        }
    }
}

impl std::fmt::Display for JiffRoundMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self.as_str();
        write!(f, "{s}")
    }
}

// ============================================================================
// FRIENDLY DESIGNATOR
// ============================================================================
impl JiffFriendlyDesignator {
    pub const VERBOSE: Self = Self(jiff::fmt::friendly::Designator::Verbose);
    pub const SHORT: Self = Self(jiff::fmt::friendly::Designator::Short);
    pub const COMPACT: Self = Self(jiff::fmt::friendly::Designator::Compact);
    pub const HUMAN_TIME: Self = Self(jiff::fmt::friendly::Designator::HumanTime);
}

// ============================================================================
// WEEKDAY
// ============================================================================
impl JiffWeekday {
    pub const MONDAY: Self = Self(jiff::civil::Weekday::Monday);
    pub const TUESDAY: Self = Self(jiff::civil::Weekday::Tuesday);
    pub const WEDNESDAY: Self = Self(jiff::civil::Weekday::Wednesday);
    pub const THURSDAY: Self = Self(jiff::civil::Weekday::Thursday);
    pub const FRIDAY: Self = Self(jiff::civil::Weekday::Friday);
    pub const SATURDAY: Self = Self(jiff::civil::Weekday::Saturday);
    pub const SUNDAY: Self = Self(jiff::civil::Weekday::Sunday);

    const fn as_str(self) -> &'static str {
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

impl std::fmt::Display for JiffWeekday {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

// ============================================================================
// ERA
// ============================================================================
impl JiffEra {
    pub const BCE: Self = Self(jiff::civil::Era::BCE);
    pub const CE: Self = Self(jiff::civil::Era::CE);
}

// ============================================================================
// TZ DISAMBIGUATION
// ============================================================================
impl JiffTzDisambiguation {
    pub const COMPATIBLE: Self = Self(jiff::tz::Disambiguation::Compatible);
    pub const EARLIER: Self = Self(jiff::tz::Disambiguation::Earlier);
    pub const LATER: Self = Self(jiff::tz::Disambiguation::Later);
    pub const REJECT: Self = Self(jiff::tz::Disambiguation::Reject);
}

// ============================================================================
// TZ OFFSET CONFLICT
// ============================================================================
impl JiffTzOffsetConflict {
    pub const ALWAYS_OFFSET: Self = Self(jiff::tz::OffsetConflict::AlwaysOffset);
    pub const ALWAYS_TIME_ZONE: Self = Self(jiff::tz::OffsetConflict::AlwaysTimeZone);
    pub const PREFER_OFFSET: Self = Self(jiff::tz::OffsetConflict::PreferOffset);
    pub const REJECT: Self = Self(jiff::tz::OffsetConflict::Reject);
}
