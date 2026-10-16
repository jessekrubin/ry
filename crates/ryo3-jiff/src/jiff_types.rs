//! jiff-new-types
//!
//! new type wrappers for the jiff types; this is how things get extracted
//! from the jiff library...
//!
//! Why not use the `pyo3/jiff` feature?
//!
//! - i have found a few bugs (which I plan to PR into pyo3)
//! - the ry jiff conversions actually predate the `pyo3/jiff` feature, however
//!   they have been influenced by the official pyo3 code
//!

// ----------------------------------------------------------------------------
// NEWTS
// ----------------------------------------------------------------------------
// #[derive(Debug, Clone)]
// pub struct JiffError(jiff::Error);

// impl JiffError {
//     #[inline]
//     #[must_use]
//     pub const fn new(error: jiff::Error) -> Self {
//         Self(error)
//     }

//     #[inline]
//     #[must_use]
//     pub const fn inner(&self) -> &jiff::Error {
//         &self.0
//     }

//     #[inline]
//     #[must_use]
//     pub fn into_inner(self) -> jiff::Error {
//         self.0
//     }
// }

// impl From<JiffError> for pyo3::PyErr {
//     fn from(e: JiffError) -> Self {
//         if  (e.inner().is_range()) {
//             pyo3::exceptions::PyOverflowError::new_err(e.inner().to_string())
//         } else {
//             pyo3::exceptions::PyValueError::new_err(e.inner().to_string())
//         }
//     }
// }

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

/// datetime extractor for `datetime.datetime` obs
///
/// primarily used for extracting naive or aware datetimes
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JiffPyDateTime {
    Aware(jiff::Zoned),
    Naive(jiff::civil::DateTime),
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

// imp the fns `inner()` ref to inner and `into_inner()` to consume and return the inner value
macro_rules! impl_new_type_functions {
    ($newt:ty, $inner:ty) => {
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

            #[inline]
            #[must_use]
            pub fn into_inner(self) -> $inner {
                self.0
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

// jiff::*
impl_new_type_functions!(JiffSignedDuration, jiff::SignedDuration);
impl_new_type_functions!(JiffSpan, jiff::Span);
impl_new_type_functions!(JiffTimestamp, jiff::Timestamp);
impl_new_type_functions!(JiffUnit, jiff::Unit);
impl_new_type_functions!(JiffRoundMode, jiff::RoundMode);
impl_new_type_functions!(JiffZoned, jiff::Zoned);
// jiff::civil::*
impl_new_type_functions!(JiffDate, jiff::civil::Date);
impl_new_type_functions!(JiffDateTime, jiff::civil::DateTime);
impl_new_type_functions!(JiffEra, jiff::civil::Era);
impl_new_type_functions!(JiffTime, jiff::civil::Time);
impl_new_type_functions!(JiffWeekday, jiff::civil::Weekday);
// jiff::tz::*
impl_new_type_functions!(JiffOffset, jiff::tz::Offset);
impl_new_type_functions!(JiffTimeZone, jiff::tz::TimeZone);
impl_new_type_functions!(JiffTzDisambiguation, jiff::tz::Disambiguation);
impl_new_type_functions!(JiffTzOffsetConflict, jiff::tz::OffsetConflict);
// jiff::fmt::friendly::*
impl_new_type_functions!(JiffFriendlyDesignator, jiff::fmt::friendly::Designator);
// jiff-other
impl_new_type_functions!(JiffEraYear, (i16, jiff::civil::Era));

// REFS
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

impl std::fmt::Display for JiffUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.static_str())
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
            _ => "round-unknown",
        }
    }
}

impl std::fmt::Display for JiffRoundMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self.static_str();
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

impl std::fmt::Display for JiffWeekday {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.static_str())
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
