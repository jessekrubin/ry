#[derive(Debug)]
pub struct JiffDate(pub jiff::civil::Date);
#[derive(Debug)]
pub struct JiffTime(pub jiff::civil::Time);
#[derive(Debug)]
pub struct JiffDateTime(pub jiff::civil::DateTime);
#[derive(Debug)]
pub struct JiffZoned(pub jiff::Zoned);
#[derive(Debug)]
pub struct JiffSpan(pub jiff::Span);

#[derive(Debug, Clone)]
pub struct JiffTimeZone(pub jiff::tz::TimeZone);
#[derive(Debug)]
pub struct JiffOffset(pub jiff::tz::Offset);
#[derive(Debug)]
pub struct JiffSignedDuration(pub jiff::SignedDuration);

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
