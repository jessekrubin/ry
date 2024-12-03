pub struct JiffDate(pub jiff::civil::Date);
pub struct JiffTime(pub jiff::civil::Time);
pub struct JiffDateTime(pub jiff::civil::DateTime);
pub struct JiffZoned(pub jiff::Zoned);
pub struct JiffSpan(pub jiff::Span);
pub struct JiffTimeZone(pub jiff::tz::TimeZone);
pub struct JiffOffset(pub jiff::tz::Offset);

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
