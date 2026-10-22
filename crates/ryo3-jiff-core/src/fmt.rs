//! shared jiff parsers/printers
//!
//! the parsers/printers used by both `ryo3-std` (`Duration`) and `ryo3-jiff`
//! (`SignedDuration`/`Span`) so their string formats stay in sync

// ----------------------------------------------------------------------------
// TEMPORAL (ISO 8601)
// ----------------------------------------------------------------------------
pub const TEMPORAL_SPAN_PARSER: jiff::fmt::temporal::SpanParser =
    jiff::fmt::temporal::SpanParser::new();
pub const TEMPORAL_SPAN_PRINTER: jiff::fmt::temporal::SpanPrinter =
    jiff::fmt::temporal::SpanPrinter::new();

// ----------------------------------------------------------------------------
// FRIENDLY
// ----------------------------------------------------------------------------
pub const FRIENDLY_SPAN_PARSER: jiff::fmt::friendly::SpanParser =
    jiff::fmt::friendly::SpanParser::new();

// Maybe use?`HumanTime` designator for friendly parser/printer to avoid
// ambiguous `µ`:w REF: https://github.com/jessekrubin/ry/discussions/229#discussioncomment-14928815
// ```txt
// RUF001 String contains ambiguous `µ` (MICRO SIGN). Did you mean `μ` (GREEK SMALL LETTER MU)?
//   --> tests\std\test_duration_str.py:45:59
//    |
// 43 |         max_dur = ry.Duration.MAX
// 44 |         iso_str = max_dur.friendly()
// 45 |         assert iso_str == "5124095576030431h 15s 999ms 999µs 999ns"
//    |                                                           ^
// 46 |         parsed_max_dur = ry.Duration.from_str(iso_str)
// 47 |         assert parsed_max_dur == max_dur
//    |
//
// Found 1 error.
// ```
pub const FRIENDLY_SPAN_PRINTER: jiff::fmt::friendly::SpanPrinter =
    jiff::fmt::friendly::SpanPrinter::new();
