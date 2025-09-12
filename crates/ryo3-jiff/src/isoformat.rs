use jiff::civil::Weekday;
use jiff::fmt::temporal::DateTimePrinter;
use pyo3::PyResult;
use ryo3_macro_rules::py_value_error;

pub(crate) const ISOFORMAT_PRINTER_NO_MICROS: DateTimePrinter =
    DateTimePrinter::new().separator(b'T').precision(Some(0));
pub(crate) const ISOFORMAT_PRINTER: DateTimePrinter =
    DateTimePrinter::new().separator(b'T').precision(Some(6));

pub(crate) fn print_iso_week_date<W: std::fmt::Write>(
    iso_week_date: &jiff::civil::ISOWeekDate,
    w: &mut W,
) -> std::fmt::Result {
    let year = iso_week_date.year();
    let week = iso_week_date.week();
    let weekday = iso_week_date.weekday().to_monday_one_offset();
    write!(w, "{year:04}-W{week:02}-{weekday}")
}

/// Convert `jiff::civil::ISOWeekDate` to string in 'YYYY-Www-D' format
///
/// # Panics
///
/// If writing to the string fails.
pub(crate) fn iso_weekdate_to_string(iso_week_date: &jiff::civil::ISOWeekDate) -> String {
    let mut s = String::with_capacity(10);
    print_iso_week_date(iso_week_date, &mut s).expect("yolo");
    s
}

pub(crate) fn parse_iso_week_date(s: &str) -> PyResult<jiff::civil::ISOWeekDate> {
    if s.len() != 10 || &s[4..6] != "-W" || &s[8..9] != "-" {
        return Err(py_value_error!(
            "Invalid ISO week date format, expected 'YYYY-Www-D'",
        ));
    }

    // Extract pieces
    let year = s[0..4].parse()?;
    let week = s[6..8].parse()?;
    let weekday = s[9..10].parse().map(Weekday::from_monday_one_offset)??;

    jiff::civil::ISOWeekDate::new(year, week, weekday)
        .map_err(|e| py_value_error!("Invalid ISO week date: {e}"))
}
