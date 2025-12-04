use crate::{
    JiffOffset, JiffSignedDuration, JiffSpan, JiffTime, RyDate, RyDateTime, RyISOWeekDate,
    RyOffset, RySignedDuration, RySpan, RyTime, RyTimeZone, RyTimeZoneDatabase, RyTimestamp,
    RyZoned, errors::map_py_overflow_err,
};
use jiff::{
    SignedDuration, Span, Timestamp, Zoned,
    civil::{Date, DateTime, ISOWeekDate, Time},
    tz::{Offset, TimeZone, TimeZoneDatabase},
};

impl From<&RyTimestamp> for RyZoned {
    fn from(value: &RyTimestamp) -> Self {
        value.0.to_zoned(jiff::tz::TimeZone::UTC).into()
    }
}

macro_rules! impl_from_jiff_for_ry {
    ($jiff_type:ty, $ryo3_type:ty) => {
        impl From<$jiff_type> for $ryo3_type {
            fn from(value: $jiff_type) -> Self {
                Self(value)
            }
        }
    };
}

impl_from_jiff_for_ry!(Date, RyDate);
impl_from_jiff_for_ry!(DateTime, RyDateTime);
impl_from_jiff_for_ry!(ISOWeekDate, RyISOWeekDate);
impl_from_jiff_for_ry!(Offset, RyOffset);
impl_from_jiff_for_ry!(SignedDuration, RySignedDuration);
impl_from_jiff_for_ry!(Span, RySpan);
impl_from_jiff_for_ry!(Time, RyTime);
impl_from_jiff_for_ry!(Timestamp, RyTimestamp);
impl_from_jiff_for_ry!(Zoned, RyZoned);
impl From<JiffTime> for RyTime {
    fn from(value: JiffTime) -> Self {
        Self(value.0)
    }
}

impl From<JiffSignedDuration> for RySignedDuration {
    fn from(d: JiffSignedDuration) -> Self {
        Self(d.0)
    }
}

impl From<jiff::civil::Date> for RyISOWeekDate {
    fn from(date: jiff::civil::Date) -> Self {
        Self(ISOWeekDate::from(date))
    }
}

impl From<TimeZone> for RyTimeZone {
    fn from(value: TimeZone) -> Self {
        Self(std::sync::Arc::new(value))
    }
}

impl From<&TimeZone> for RyTimeZone {
    fn from(value: &TimeZone) -> Self {
        Self(std::sync::Arc::new(value.clone()))
    }
}

impl From<RyTimeZone> for TimeZone {
    fn from(value: RyTimeZone) -> Self {
        (*value.0).clone()
    }
}

impl From<&RyTimeZone> for TimeZone {
    fn from(value: &RyTimeZone) -> Self {
        (*value.0).clone()
    }
}

impl From<JiffOffset> for RyOffset {
    fn from(value: JiffOffset) -> Self {
        Self::from(value.0)
    }
}

impl From<JiffSpan> for RySpan {
    fn from(span: JiffSpan) -> Self {
        Self(span.0)
    }
}

impl TryFrom<SignedDuration> for RySpan {
    type Error = pyo3::PyErr;

    fn try_from(value: SignedDuration) -> Result<Self, Self::Error> {
        Span::try_from(value)
            .map(Self::from)
            .map_err(map_py_overflow_err)
    }
}

impl From<TimeZoneDatabase> for RyTimeZoneDatabase {
    fn from(db: TimeZoneDatabase) -> Self {
        Self { inner: Some(db) }
    }
}
