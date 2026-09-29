//! Low-level JSON byte writer.
//!
//! Owns the output buffer and the [`Format`]; knows how JSON bytes are laid out
//! but nothing about serde. The serde `Serializer` in `ser.rs` is a thin
//! frontend over this.
#![expect(clippy::inline_always, reason = "perf")]

use super::escape;
use super::format::Format;

pub(super) struct JsonWriter<F: Format> {
    buf: Vec<u8>,
    fmt: F,
}

impl<F: Format> JsonWriter<F> {
    #[inline]
    pub(super) fn with_capacity(capacity: usize, fmt: F) -> Self {
        Self {
            buf: Vec::with_capacity(capacity),
            fmt,
        }
    }

    #[inline]
    pub(super) fn into_inner(self) -> Vec<u8> {
        self.buf
    }

    #[inline(always)]
    pub(super) fn reserve(&mut self, additional: usize) {
        self.buf.reserve(additional);
    }

    // ------------------------------------------------------------------------
    // raw
    // ------------------------------------------------------------------------

    /// Write a single byte verbatim (no formatting hooks).
    #[inline(always)]
    pub(super) fn raw_byte(&mut self, v: u8) {
        self.buf.push(v);
    }

    /// Write bytes verbatim (no formatting hooks).
    #[inline(always)]
    pub(super) fn raw(&mut self, v: &[u8]) {
        self.buf.extend_from_slice(v);
    }

    // ------------------------------------------------------------------------
    // scalars
    // ------------------------------------------------------------------------

    #[inline(always)]
    pub(super) fn null(&mut self) {
        self.raw(b"null");
    }

    #[inline(always)]
    pub(super) fn bool(&mut self, v: bool) {
        // Separate branches keep each copy a constant length; a single
        // `raw(if v { .. } else { .. })` becomes a variable-length memcpy call.
        if v {
            self.raw(b"true");
        } else {
            self.raw(b"false");
        }
    }

    #[inline(always)]
    pub(super) fn int(&mut self, v: impl itoa::Integer) {
        self.raw(itoa::Buffer::new().format(v).as_bytes());
    }

    /// Finite floats are written as numbers; NaN/inf are written as `null`.
    #[inline(always)]
    pub(super) fn float<T: zmij::Float + FloatExt>(&mut self, v: T) {
        if v.is_finite() {
            self.raw(zmij::Buffer::new().format_finite(v).as_bytes());
        } else {
            self.null();
        }
    }

    /// Write a quoted, escaped string.
    #[inline(always)]
    pub(super) fn str(&mut self, v: &str) {
        escape::format_escaped_str(&mut self.buf, v);
    }

    /// Write escaped string contents without surrounding quotes.
    #[inline(always)]
    pub(super) fn str_contents(&mut self, v: &str) {
        escape::format_escaped_str_contents(&mut self.buf, v);
    }

    // ------------------------------------------------------------------------
    // quoted scalars (map keys)
    // ------------------------------------------------------------------------

    #[inline(always)]
    pub(super) fn quoted_bool(&mut self, v: bool) {
        if v {
            self.raw(br#""true""#);
        } else {
            self.raw(br#""false""#);
        }
    }

    #[inline(always)]
    pub(super) fn quoted_int(&mut self, v: impl itoa::Integer) {
        self.raw_byte(b'"');
        self.int(v);
        self.raw_byte(b'"');
    }

    /// Caller must ensure `v` is finite.
    #[inline(always)]
    pub(super) fn quoted_finite_float<T: zmij::Float>(&mut self, v: T) {
        self.raw_byte(b'"');
        self.raw(zmij::Buffer::new().format_finite(v).as_bytes());
        self.raw_byte(b'"');
    }

    // ------------------------------------------------------------------------
    // structure
    // ------------------------------------------------------------------------

    #[inline(always)]
    pub(super) fn begin_array(&mut self) {
        self.raw_byte(b'[');
        self.fmt.inc();
    }

    /// `nonempty` is whether any element was written.
    #[inline(always)]
    pub(super) fn end_array(&mut self, nonempty: bool) {
        self.fmt.dec();
        if nonempty {
            self.fmt.indent(&mut self.buf);
        }
        self.raw_byte(b']');
    }

    #[inline(always)]
    pub(super) fn begin_object(&mut self) {
        self.raw_byte(b'{');
        self.fmt.inc();
    }

    /// `nonempty` is whether any entry was written.
    #[inline(always)]
    pub(super) fn end_object(&mut self, nonempty: bool) {
        self.fmt.dec();
        if nonempty {
            self.fmt.indent(&mut self.buf);
        }
        self.raw_byte(b'}');
    }

    /// Separator before an array element or object key: a comma if one was
    /// already written (tracked via `written`), then indentation.
    #[inline(always)]
    pub(super) fn elem_sep(&mut self, written: &mut bool) {
        self.comma(written);
        self.fmt.indent(&mut self.buf);
    }

    /// Separator between an object key and its value.
    #[inline(always)]
    pub(super) fn key_sep(&mut self) {
        self.raw_byte(b':');
        self.fmt.sep(&mut self.buf);
    }

    /// A comma if one was already written (tracked via `written`); no
    /// indentation. For compact sub-structures such as byte arrays.
    #[inline(always)]
    pub(super) fn comma(&mut self, written: &mut bool) {
        if *written {
            self.raw_byte(b',');
        } else {
            *written = true;
        }
    }

    #[inline(always)]
    fn close(&mut self, nonempty: bool, close: u8) {
        self.fmt.dec();
        if nonempty {
            self.fmt.indent(&mut self.buf);
        }
        self.raw_byte(close);
    }
}

/// `is_finite` for the float types `zmij` formats.
pub(super) trait FloatExt: Copy {
    fn is_finite(self) -> bool;
}

impl FloatExt for f32 {
    #[inline(always)]
    fn is_finite(self) -> bool {
        Self::is_finite(self)
    }
}

impl FloatExt for f64 {
    #[inline(always)]
    fn is_finite(self) -> bool {
        Self::is_finite(self)
    }
}
