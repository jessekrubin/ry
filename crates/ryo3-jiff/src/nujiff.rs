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
#[derive(Debug)]
pub struct JiffTimeZone(pub jiff::tz::TimeZone);
#[derive(Debug)]
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

impl Into<jiff::civil::Date> for JiffDate {
    fn into(self) -> jiff::civil::Date {
        self.0
    }
}

impl Into<jiff::civil::Time> for JiffTime {
    fn into(self) -> jiff::civil::Time {
        self.0
    }
}

impl Into<jiff::civil::DateTime> for JiffDateTime {
    fn into(self) -> jiff::civil::DateTime {
        self.0
    }
}

impl Into<jiff::Zoned> for JiffZoned {
    fn into(self) -> jiff::Zoned {
        self.0
    }
}

impl Into<jiff::Span> for JiffSpan {
    fn into(self) -> jiff::Span {
        self.0
    }
}

impl Into<jiff::tz::TimeZone> for JiffTimeZone {
    fn into(self) -> jiff::tz::TimeZone {
        self.0
    }
}

impl Into<jiff::tz::Offset> for JiffOffset {
    fn into(self) -> jiff::tz::Offset {
        self.0
    }
}
