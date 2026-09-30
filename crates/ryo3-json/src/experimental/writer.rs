//! JSON byte(s) writer
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
    // RAW
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
    // PRIMITIVES
    // ------------------------------------------------------------------------

    #[inline(always)]
    pub(super) fn null(&mut self) {
        self.raw(b"null");
    }

    #[inline(always)]
    pub(super) fn bool(&mut self, v: bool) {
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

    #[inline(always)]
    pub(super) fn write_i8(&mut self, v: i8) {
        self.raw(itoa::Buffer::new().format(v).as_bytes());
    }

    #[inline(always)]
    pub(super) fn write_u8(&mut self, v: u8) {
        self.raw(itoa::Buffer::new().format(v).as_bytes());
    }

    #[inline(always)]
    pub(super) fn write_i16(&mut self, v: i16) {
        self.raw(itoa::Buffer::new().format(v).as_bytes());
    }

    #[inline(always)]
    pub(super) fn write_u16(&mut self, v: u16) {
        self.raw(itoa::Buffer::new().format(v).as_bytes());
    }

    #[inline(always)]
    pub(super) fn write_i32(&mut self, v: i32) {
        self.raw(itoa::Buffer::new().format(v).as_bytes());
    }

    #[inline(always)]
    pub(super) fn write_u32(&mut self, v: u32) {
        self.raw(itoa::Buffer::new().format(v).as_bytes());
    }

    #[inline(always)]
    pub(super) fn write_i64(&mut self, v: i64) {
        self.raw(itoa::Buffer::new().format(v).as_bytes());
    }

    #[inline(always)]
    pub(super) fn write_u64(&mut self, v: u64) {
        self.raw(itoa::Buffer::new().format(v).as_bytes());
    }

    #[inline(always)]
    pub(super) fn write_i128(&mut self, v: i128) {
        self.raw(itoa::Buffer::new().format(v).as_bytes());
    }

    #[inline(always)]
    pub(super) fn write_u128(&mut self, v: u128) {
        self.raw(itoa::Buffer::new().format(v).as_bytes());
    }

    /// Finite floats are written as numbers; NaN/inf are written as `null`.
    // #[inline(always)]
    // pub(super) fn write_infinity<const NEGATIVE: bool>(&mut self) {
    //     if NEGATIVE {
    //         self.raw(b"-Infinity");
    //     } else {
    //         self.raw(b"Infinity");
    //     }
    // }

    // #[inline(always)]
    // pub(super) fn write_nan(&mut self) {
    //     self.raw(b"NaN");
    // }

    #[inline(always)]
    pub(super) fn write_f32(&mut self, v: f32) {
        if v.is_finite() {
            self.raw(zmij::Buffer::new().format_finite(v).as_bytes());
        } else {
            self.null();
        }
    }

    #[inline(always)]
    pub(super) fn write_f64(&mut self, v: f64) {
        if v.is_finite() {
            self.raw(zmij::Buffer::new().format_finite(v).as_bytes());
        } else {
            self.null();
        }
    }

    /// write quotes, and escaped string
    #[inline(always)]
    pub(super) fn str(&mut self, v: &str) {
        escape::format_escaped_str(&mut self.buf, v);
    }

    /// write escaped string w/o quotes
    #[inline(always)]
    pub(super) fn str_contents(&mut self, v: &str) {
        escape::format_escaped_str_contents(&mut self.buf, v);
    }

    // ------------------------------------------------------------------------
    // MAP KEY PRIMITIVES
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
}
