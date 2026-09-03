pub(crate) static DATETIME_PARSER: jiff::fmt::temporal::DateTimeParser =
    jiff::fmt::temporal::DateTimeParser::new();
pub(crate) static SPAN_PARSER: jiff::fmt::temporal::SpanParser =
    jiff::fmt::temporal::SpanParser::new();
pub(crate) static SPAN_PRINTER_TEMPORAL: jiff::fmt::temporal::SpanPrinter =
    jiff::fmt::temporal::SpanPrinter::new();
pub(crate) static SPAN_PRINTER_FRIENDLY: jiff::fmt::friendly::SpanPrinter =
    jiff::fmt::friendly::SpanPrinter::new();

// UNITS
pub(crate) const NANOSECONDS_PER_SECOND: i32 = 1_000_000_000;
pub(crate) const SECONDS_PER_MINUTE: i64 = 60;
pub(crate) const MINUTES_PER_HOUR: i64 = 60;

// SPAN-RANGES
// span-years
pub(crate) const SPAN_YEARS_MAX: i16 = 19_998;
pub(crate) const SPAN_YEARS_MIN: i16 = -SPAN_YEARS_MAX;
// span-months
pub(crate) const SPAN_MONTHS_MAX: i32 = 239_976;
pub(crate) const SPAN_MONTHS_MIN: i32 = -SPAN_MONTHS_MAX;
// span-weeks
pub(crate) const SPAN_WEEKS_MAX: i32 = 1_043_497;
pub(crate) const SPAN_WEEKS_MIN: i32 = -SPAN_WEEKS_MAX;
// span-days
pub(crate) const SPAN_DAYS_MAX: i32 = 7_304_484;
pub(crate) const SPAN_DAYS_MIN: i32 = -SPAN_DAYS_MAX;
// span-hours

pub(crate) const SPAN_HOURS_MAX: i32 = 175_307_616;
pub(crate) const SPAN_HOURS_MIN: i32 = -SPAN_HOURS_MAX;
// span-minutes
pub(crate) const SPAN_MINUTES_MAX: i64 = 10_518_456_960;
pub(crate) const SPAN_MINUTES_MIN: i64 = -SPAN_MINUTES_MAX;
// span-seconds
pub(crate) const SPAN_SECONDS_MAX: i64 = 631_107_417_600;
pub(crate) const SPAN_SECONDS_MIN: i64 = -SPAN_SECONDS_MAX;

// span-milliseconds
pub(crate) const SPAN_MILLISECONDS_MAX: i64 = 631_107_417_600_000;
pub(crate) const SPAN_MILLISECONDS_MIN: i64 = -SPAN_MILLISECONDS_MAX;

// span-microseconds
pub(crate) const SPAN_MICROSECONDS_MAX: i64 = 631_107_417_600_000_000;
pub(crate) const SPAN_MICROSECONDS_MIN: i64 = -SPAN_MICROSECONDS_MAX;

// span-nanoseconds
pub(crate) const SPAN_NANOSECONDS_MAX: i64 = 9_223_372_036_854_775_807;
pub(crate) const SPAN_NANOSECONDS_MIN: i64 = -SPAN_NANOSECONDS_MAX;

#[cfg(test)]
mod test {
    use jiff::Span;

    use super::*;

    macro_rules! test_span_range {
        (
            $test_name_min:ident,
            $test_name_max:ident,
            $setter:ident,
            $max:ident,
            $min:ident
        ) => {
            #[test]
            fn $test_name_max() {
                let span_ok_max = Span::new().$setter($max);
                assert!(span_ok_max.is_ok());
                let span_err_max = Span::new().$setter($max + 1);
                assert!(span_err_max.is_err());
            }

            #[test]
            fn $test_name_min() {
                let span_ok_min = Span::new().$setter($min);
                assert!(span_ok_min.is_ok());
                let span_err_min = Span::new().$setter($min - 1);
                assert!(span_err_min.is_err());
            }
        };
    }
    test_span_range!(
        test_span_years_min,
        test_span_years_max,
        try_years,
        SPAN_YEARS_MAX,
        SPAN_YEARS_MIN
    );

    test_span_range!(
        test_span_months_min,
        test_span_months_max,
        try_months,
        SPAN_MONTHS_MAX,
        SPAN_MONTHS_MIN
    );

    test_span_range!(
        test_span_weeks_min,
        test_span_weeks_max,
        try_weeks,
        SPAN_WEEKS_MAX,
        SPAN_WEEKS_MIN
    );

    test_span_range!(
        test_span_days_min,
        test_span_days_max,
        try_days,
        SPAN_DAYS_MAX,
        SPAN_DAYS_MIN
    );

    test_span_range!(
        test_span_hours_min,
        test_span_hours_max,
        try_hours,
        SPAN_HOURS_MAX,
        SPAN_HOURS_MIN
    );

    test_span_range!(
        test_span_minutes_min,
        test_span_minutes_max,
        try_minutes,
        SPAN_MINUTES_MAX,
        SPAN_MINUTES_MIN
    );

    test_span_range!(
        test_span_seconds_min,
        test_span_seconds_max,
        try_seconds,
        SPAN_SECONDS_MAX,
        SPAN_SECONDS_MIN
    );

    test_span_range!(
        test_span_milliseconds_min,
        test_span_milliseconds_max,
        try_milliseconds,
        SPAN_MILLISECONDS_MAX,
        SPAN_MILLISECONDS_MIN
    );

    test_span_range!(
        test_span_microseconds_min,
        test_span_microseconds_max,
        try_microseconds,
        SPAN_MICROSECONDS_MAX,
        SPAN_MICROSECONDS_MIN
    );
}
