use jiff::fmt::temporal::{DateTimeParser, SpanParser};

pub(crate) static DATETIME_PARSER: DateTimeParser = DateTimeParser::new();
pub(crate) static SPAN_PARSER: SpanParser = SpanParser::new();
