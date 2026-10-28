//! python-repr helpers
//!
//! Inspired by ruff's `ruff_python_literal` crate implementation
//!
//! REF: <https://github.com/astral-sh/ruff/blob/main/crates/ruff_python_literal/src/escape.rs>

use std::fmt::Write;

/// quote character used to wrap a repr
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum Quote {
    #[default]
    Double,
    Single,
}

impl Quote {
    #[inline]
    #[must_use]
    pub const fn as_char(self) -> char {
        match self {
            Self::Single => '\'',
            Self::Double => '"',
        }
    }
}

/// Quoted python-str-literal repr of a `&str`
///
/// What it do?
/// - always escapes `\`
/// - always escapes the chosen quote char
/// - always escapes `\n`, `\r`, `\t`, and other ASCII control chars
/// - if `ascii` is set, non-ascii chars are escaped (`\xHH`/`\uHHHH`/`\UHHHHHHHH`)
///
/// ```
/// use ryo3_core::repr::{PyStrRepr, Quote};
///
/// assert_eq!(PyStrRepr::new("it's").to_string(), r#""it's""#);
/// assert_eq!(PyStrRepr::new("it's").quote(Quote::Single).to_string(), r"'it\'s'");
/// assert_eq!(PyStrRepr::new("café").ascii(true).to_string(), r#""caf\xe9""#);
/// ```
#[derive(Debug, Clone, Copy)]
pub struct PyStrRepr<'a> {
    s: &'a str,
    quote: Quote,
    ascii: bool,
}

impl<'a> PyStrRepr<'a> {
    #[inline]
    #[must_use]
    pub const fn new(s: &'a str) -> Self {
        Self {
            s,
            quote: Quote::Double,
            ascii: false,
        }
    }

    /// Set the wrapping quote (default: [`Quote::Double`])
    #[inline]
    #[must_use]
    pub const fn quote(mut self, quote: Quote) -> Self {
        self.quote = quote;
        self
    }

    #[inline]
    #[must_use]
    pub const fn double_quote(mut self) -> Self {
        self.quote = Quote::Double;
        self
    }

    #[inline]
    #[must_use]
    pub const fn single_quote(mut self) -> Self {
        self.quote = Quote::Single;
        self
    }

    /// escape all non-ASCII chars as `\xHH`/`\uHHHH`/`\UHHHHHHHH` (default: `false`)
    #[inline]
    #[must_use]
    pub const fn ascii(mut self, ascii: bool) -> Self {
        self.ascii = ascii;
        self
    }

    #[inline]
    fn needs_escape(&self, c: char) -> bool {
        match c {
            '\\' => true,
            ' '..='~' => c == self.quote.as_char(),
            c if c.is_ascii() => true, // control chars + DEL
            _ => self.ascii,
        }
    }
}

impl std::fmt::Display for PyStrRepr<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let q = self.quote.as_char();
        f.write_char(q)?;
        // write runs of chars that need no escaping in one go
        let mut run_start = 0;
        for (i, c) in self.s.char_indices() {
            if !self.needs_escape(c) {
                continue;
            }
            f.write_str(&self.s[run_start..i])?;
            run_start = i + c.len_utf8();
            match c {
                '\\' => f.write_str("\\\\")?,
                '\n' => f.write_str("\\n")?,
                '\r' => f.write_str("\\r")?,
                '\t' => f.write_str("\\t")?,
                c if c == q => {
                    f.write_char('\\')?;
                    f.write_char(c)?;
                }
                c => match c as u32 {
                    n @ 0..=0xff => write!(f, "\\x{n:02x}")?,
                    n @ 0x100..=0xffff => write!(f, "\\u{n:04x}")?,
                    n => write!(f, "\\U{n:08x}")?,
                },
            }
        }
        f.write_str(&self.s[run_start..])?;
        f.write_char(q)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn repr(s: &str) -> String {
        PyStrRepr::new(s).to_string()
    }

    #[test]
    fn test_plain() {
        assert_eq!(repr(""), "\"\"");
        assert_eq!(repr("/usr/share/zoneinfo"), "\"/usr/share/zoneinfo\"");
    }

    #[test]
    fn test_quotes() {
        assert_eq!(repr(r#"it's "x""#), r#""it's \"x\"""#);
        let sq = PyStrRepr::new(r#"it's "x""#).quote(Quote::Single);
        assert_eq!(sq.to_string(), r#"'it\'s "x"'"#);
        let sq = PyStrRepr::new(r#"it's "x""#).single_quote();
        assert_eq!(sq.to_string(), r#"'it\'s "x"'"#);
        let dq = sq.double_quote();
        assert_eq!(dq.to_string(), r#""it's \"x\"""#);
    }

    #[test]
    fn test_bslash_and_whitespace() {
        assert_eq!(repr(r"C:\tz"), r#""C:\\tz""#);
        assert_eq!(repr("a\tb\nc\rd"), r#""a\tb\nc\rd""#);
    }

    #[test]
    fn test_ctrl_chars() {
        assert_eq!(repr("x\0y\x1b\x7f"), r#""x\x00y\x1b\x7f""#);
    }

    #[test]
    fn test_non_ascii_nonescaped() {
        assert_eq!(repr("café \u{1f600}"), "\"café \u{1f600}\"");
    }

    #[test]
    fn test_ascii_escapes() {
        let r = PyStrRepr::new("é\u{a0}\u{200b}\u{1f600}").ascii(true);
        assert_eq!(r.to_string(), r#""\xe9\xa0\u200b\U0001f600""#);
    }
}
