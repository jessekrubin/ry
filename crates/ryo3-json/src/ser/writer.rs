//! JSON byte(s) writer
use super::escape;
use super::format::JsonFormat;

pub(crate) struct JsonWriter<F: JsonFormat> {
    buf: Vec<u8>,
    fmt: F,
}

macro_rules! impl_write_int {
    ($name:ident, $int_type:ty) => {
        #[inline]
        pub(crate) fn $name(&mut self, v: $int_type) {
            self.raw(itoa::Buffer::new().format(v).as_bytes());
        }
    };
}

macro_rules! impl_write_int_key {
    ($name:ident, $int_type:ty) => {
        #[inline]
        pub(crate) fn $name(&mut self, v: $int_type) {
            self.raw_byte(b'"');
            self.raw(itoa::Buffer::new().format(v).as_bytes());
            self.raw_byte(b'"');
        }
    };
}

impl<F: JsonFormat> JsonWriter<F> {
    #[inline]
    pub(crate) fn with_capacity(capacity: usize, fmt: F) -> Self {
        Self {
            buf: Vec::with_capacity(capacity),
            fmt,
        }
    }

    #[inline]
    pub(crate) fn into_inner(self) -> Vec<u8> {
        self.buf
    }

    #[inline]
    pub(crate) fn reserve(&mut self, additional: usize) {
        self.buf.reserve(additional);
    }

    // ------------------------------------------------------------------------
    // RAW
    // ------------------------------------------------------------------------

    /// Write a single byte verbatim (no formatting hooks).
    #[inline]
    pub(crate) fn raw_byte(&mut self, v: u8) {
        self.buf.push(v);
    }

    /// Write bytes verbatim (no formatting hooks).
    #[inline]
    pub(crate) fn raw(&mut self, v: &[u8]) {
        self.buf.extend_from_slice(v);
    }

    // ------------------------------------------------------------------------
    // PRIMITIVES
    // ------------------------------------------------------------------------

    #[inline]
    pub(crate) fn write_null(&mut self) {
        self.raw(b"null");
    }

    #[inline]
    pub(crate) fn write_bool(&mut self, v: bool) {
        if v {
            self.raw(b"true");
        } else {
            self.raw(b"false");
        }
    }

    impl_write_int!(write_i8, i8);
    impl_write_int!(write_i16, i16);
    impl_write_int!(write_i32, i32);
    impl_write_int!(write_i64, i64);
    impl_write_int!(write_i128, i128);
    impl_write_int!(write_u8, u8);
    impl_write_int!(write_u16, u16);
    impl_write_int!(write_u32, u32);
    impl_write_int!(write_u64, u64);
    impl_write_int!(write_u128, u128);

    // FUTURE: support optionally writing `-Infinity`/`Infinity`/`NaN`?

    // #[inline]
    // pub(crate) fn write_infinity<const NEGATIVE: bool>(&mut self) {
    //     if NEGATIVE {
    //         self.raw(b"-Infinity");
    //     } else {
    //         self.raw(b"Infinity");
    //     }
    // }

    // pub(crate) fn write_nan(&mut self) {
    //     self.raw(b"NaN");
    // }

    #[inline]
    pub(crate) fn write_f32_finite(&mut self, v: f32) {
        self.raw(zmij::Buffer::new().format_finite(v).as_bytes());
    }

    #[inline]
    pub(crate) fn write_f32(&mut self, v: f32) {
        if v.is_finite() {
            self.write_f32_finite(v);
        } else {
            self.write_null();
        }
    }

    #[inline]
    pub(crate) fn write_f64_finite(&mut self, v: f64) {
        self.raw(zmij::Buffer::new().format_finite(v).as_bytes());
    }

    #[inline]
    pub(crate) fn write_f64(&mut self, v: f64) {
        if v.is_finite() {
            self.write_f64_finite(v);
        } else {
            self.write_null();
        }
    }

    /// write quotes, and escaped string
    #[inline]
    pub(crate) fn write_str(&mut self, v: &str) {
        self.raw_byte(b'"');
        escape::format_escaped_str_contents(&mut self.buf, v);
        self.raw_byte(b'"');
    }

    /// write escaped string w/o quotes
    #[inline]
    pub(crate) fn str_contents(&mut self, v: &str) {
        escape::format_escaped_str_contents(&mut self.buf, v);
    }

    // ------------------------------------------------------------------------
    // MAP KEY PRIMITIVES
    // ------------------------------------------------------------------------
    #[inline]
    pub(crate) fn write_bool_key(&mut self, v: bool) {
        if v {
            self.raw(br#""true""#);
        } else {
            self.raw(br#""false""#);
        }
    }

    impl_write_int_key!(write_i8_key, i8);
    impl_write_int_key!(write_i16_key, i16);
    impl_write_int_key!(write_i32_key, i32);
    impl_write_int_key!(write_i64_key, i64);
    impl_write_int_key!(write_i128_key, i128);
    impl_write_int_key!(write_u8_key, u8);
    impl_write_int_key!(write_u16_key, u16);
    impl_write_int_key!(write_u32_key, u32);
    impl_write_int_key!(write_u64_key, u64);
    impl_write_int_key!(write_u128_key, u128);

    /// Must be finite
    #[inline]
    pub(crate) fn write_f32_key(&mut self, v: f32) {
        debug_assert!(v.is_finite(), "f32 key must be finite");
        self.raw_byte(b'"');
        self.raw(zmij::Buffer::new().format_finite(v).as_bytes());
        self.raw_byte(b'"');
    }

    #[inline]
    pub(crate) fn write_f64_key(&mut self, v: f64) {
        debug_assert!(v.is_finite(), "f64 key must be finite");
        self.raw_byte(b'"');
        self.raw(zmij::Buffer::new().format_finite(v).as_bytes());
        self.raw_byte(b'"');
    }
    // ------------------------------------------------------------------------
    // structure
    // ------------------------------------------------------------------------

    #[inline]
    pub(crate) fn begin_array(&mut self) {
        self.raw_byte(b'[');
        self.fmt.inc();
    }

    /// `nonempty` is whether any element was written.
    #[inline]
    pub(crate) fn end_array(&mut self, nonempty: bool) {
        self.fmt.dec();
        if nonempty {
            self.fmt.indent(&mut self.buf);
        }
        self.raw_byte(b']');
    }

    #[inline]
    pub(crate) fn begin_object(&mut self) {
        self.raw_byte(b'{');
        self.fmt.inc();
    }

    /// `nonempty` is whether any entry was written.
    #[inline]
    pub(crate) fn end_object(&mut self, nonempty: bool) {
        self.fmt.dec();
        if nonempty {
            self.fmt.indent(&mut self.buf);
        }
        self.raw_byte(b'}');
    }

    /// Separator before an array element or object key: a comma if one was
    /// already written (tracked via `written`), then indentation.
    #[inline]
    pub(crate) fn elem_sep(&mut self, written: &mut bool) {
        self.comma(written);
        self.fmt.indent(&mut self.buf);
    }

    /// Separator between an object key and its value.
    #[inline]
    pub(crate) fn key_sep(&mut self) {
        self.raw_byte(b':');
        self.fmt.sep(&mut self.buf);
    }

    /// A comma if one was already written (tracked via `written`); no
    /// indentation. For compact sub-structures such as byte arrays.
    #[inline]
    pub(crate) fn comma(&mut self, written: &mut bool) {
        if *written {
            self.raw_byte(b',');
        } else {
            *written = true;
        }
    }
}
