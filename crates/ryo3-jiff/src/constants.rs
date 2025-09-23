use jiff::fmt::temporal::{DateTimeParser, SpanParser, SpanPrinter};

pub(crate) static DATETIME_PARSER: DateTimeParser = DateTimeParser::new();
pub(crate) static SPAN_PARSER: SpanParser = SpanParser::new();
pub(crate) static SPAN_PRINTER: SpanPrinter = SpanPrinter::new();
