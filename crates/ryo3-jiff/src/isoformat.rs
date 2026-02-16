use jiff::fmt::temporal::DateTimePrinter;
use jiff::{civil::Weekday, tz::Offset};
use pyo3::PyResult;
use ryo3_core::PyAsciiString;
use ryo3_macro_rules::py_value_error;

use crate::{
    RyDate, RyDateTime, RyISOWeekDate, RyOffset, RySignedDuration, RySpan, RyTime, RyTimestamp,
    RyZoned,
};

pub(crate) trait PyIsoFormat {
    fn isoformat(&self) -> PyAsciiString;
}

pub(crate) const ISOFORMAT_PRINTER_NO_MICROS: DateTimePrinter =
    DateTimePrinter::new().separator(b'T').precision(Some(0));
pub(crate) const ISOFORMAT_PRINTER: DateTimePrinter =
    DateTimePrinter::new().separator(b'T').precision(Some(6));

pub(crate) fn print_iso_week_date<W: jiff::fmt::Write>(
    iso_week_date: jiff::civil::ISOWeekDate,
    w: &mut W,
) -> Result<(), jiff::Error> {
    let year = iso_week_date.year();
    let week = iso_week_date.week();
    let weekday = iso_week_date.weekday().to_monday_one_offset();
    w.write_str(&format!("{year:04}-W{week:02}-{weekday}"))
}

/// Convert `jiff::civil::ISOWeekDate` to string in 'YYYY-Www-D' format
///
/// # Panics
///
/// If writing to the string fails.
fn iso_weekdate_to_string(iso_week_date: jiff::civil::ISOWeekDate) -> String {
    let mut s = String::with_capacity(10);
    print_iso_week_date(iso_week_date, &mut s).expect("will not fail bc we're writing to a string");
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

impl PyIsoFormat for RyDate {
    fn isoformat(&self) -> PyAsciiString {
        self.0.to_string().into()
    }
}

impl PyIsoFormat for RyDateTime {
    fn isoformat(&self) -> PyAsciiString {
        if self.0.subsec_nanosecond() == 0 {
            ISOFORMAT_PRINTER_NO_MICROS
                .datetime_to_string(&self.0)
                .into()
        } else {
            ISOFORMAT_PRINTER.datetime_to_string(&self.0).into()
        }
    }
}

impl PyIsoFormat for RyISOWeekDate {
    fn isoformat(&self) -> PyAsciiString {
        iso_weekdate_to_string(self.0).into()
    }
}

pub(crate) fn print_isoformat_offset<W: jiff::fmt::Write>(
    offset: Offset,
    w: &mut W,
) -> Result<(), jiff::Error> {
    if offset.is_zero() {
        return w.write_str("+00:00");
    }
    // total number of seconds
    w.write_str(if offset.is_negative() { "-" } else { "+" })?;
    // let sign = if offset.is_negative() { "-" } else { "+" };
    let total_seconds = offset.seconds();
    // calculate hours and minutes, and seconds
    let hours = total_seconds.abs() / 3600;
    let minutes = (total_seconds.abs() % 3600) / 60;
    let seconds = total_seconds.abs() % 60;

    // write the formatted string
    if seconds == 0 {
        w.write_str(&format!("{hours:02}:{minutes:02}"))
    } else {
        w.write_str(&format!("{hours:02}:{minutes:02}:{seconds:02}"))
    }
}

impl PyIsoFormat for RyOffset {
    fn isoformat(&self) -> PyAsciiString {
        let offset: Offset = self.0;
        let mut s = String::with_capacity(6);
        print_isoformat_offset(offset, &mut s).expect("will not fail bc we're writing to a string");
        s.into()
    }
}

impl PyIsoFormat for RySignedDuration {
    fn isoformat(&self) -> PyAsciiString {
        crate::constants::SPAN_PRINTER
            .duration_to_string(&self.0)
            .into()
    }
}

impl PyIsoFormat for RySpan {
    fn isoformat(&self) -> PyAsciiString {
        crate::constants::SPAN_PRINTER
            .span_to_string(&self.0)
            .into()
    }
}

impl PyIsoFormat for RyTime {
    fn isoformat(&self) -> PyAsciiString {
        if self.0.subsec_nanosecond() == 0 {
            ISOFORMAT_PRINTER_NO_MICROS.time_to_string(&self.0).into()
        } else {
            ISOFORMAT_PRINTER.time_to_string(&self.0).into()
        }
    }
}

impl PyIsoFormat for RyTimestamp {
    fn isoformat(&self) -> PyAsciiString {
        self.0.to_string().into()
    }
}

impl PyIsoFormat for RyZoned {
    // ISO format mismatch:
    // input datetime: 7639-01-01 00:00:00.395000+00:00 (repr: datetime.datetime(7639, 1, 1, 0, 0, 0, 395000, tzinfo=zoneinfo.ZoneInfo(key='UTC')))
    // py: 7639-01-01T00:00:00.395000+00:00
    // ry: 7639-01-01T00:00:00+00
    // is_eq: False
    // ry_prefix_ok: False
    fn isoformat(&self) -> PyAsciiString {
        let offset: Offset = self.0.offset();
        // let ts = self.0.timestamp();
        let dattie = self.0.datetime();
        let mut s = String::with_capacity(32);
        if self.0.datetime().microsecond() == 0 && self.0.subsec_nanosecond() == 0 {
            ISOFORMAT_PRINTER_NO_MICROS
                .print_datetime(&dattie, &mut s)
                .expect("will not fail bc we're writing to a string");
        } else {
            ISOFORMAT_PRINTER
                .print_datetime(&dattie, &mut s)
                .expect("will not fail bc we're writing to a string");
        }
        print_isoformat_offset(offset, &mut s).expect("will not fail bc we're writing to a string");
        s.into()
    }
}
