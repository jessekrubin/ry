use jiff::fmt::temporal::DateTimePrinter;

pub(crate) const ISOFORMAT_PRINTER_NO_MICROS: DateTimePrinter =
    DateTimePrinter::new().separator(b'T').precision(Some(0));
pub(crate) const ISOFORMAT_PRINTER: DateTimePrinter =
    DateTimePrinter::new().separator(b'T').precision(Some(6));
