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

    /// write uno byte
    #[inline]
    pub(crate) fn raw_byte(&mut self, v: u8) {
        self.buf.push(v);
    }

    /// write n+1 raw bytes
    #[inline]
    pub(crate) fn raw(&mut self, v: &[u8]) {
        debug_assert!(!v.is_empty(), "raw bytes aint not empty");
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

    #[inline]
    pub(crate) fn write_f32(&mut self, v: f32) {
        if v.is_finite() {
            self.raw(zmij::Buffer::new().format_finite(v).as_bytes());
        } else {
            self.write_null();
        }
    }

    #[inline]
    pub(crate) fn write_f64(&mut self, v: f64) {
        if v.is_finite() {
            self.raw(zmij::Buffer::new().format_finite(v).as_bytes());
        } else {
            self.write_null();
        }
    }

    /// write quote + escaped string + quote
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

    #[inline]
    pub(crate) fn end_array(&mut self) {
        self.fmt.dec();
        if self.buf.last() == Some(&b',') {
            self.buf.pop();
            self.fmt.indent(&mut self.buf);
        }
        self.raw_byte(b']');
    }

    #[inline]
    pub(crate) fn begin_object(&mut self) {
        self.raw_byte(b'{');
        self.fmt.inc();
    }

    #[inline]
    pub(crate) fn end_object(&mut self) {
        self.fmt.dec();
        if self.buf.last() == Some(&b',') {
            self.buf.pop();
            self.fmt.indent(&mut self.buf);
        }
        self.raw_byte(b'}');
    }

    #[inline]
    pub(crate) fn elem_begin(&mut self) {
        self.fmt.indent(&mut self.buf);
    }

    #[inline]
    pub(crate) fn comma(&mut self) {
        self.raw_byte(b',');
    }

    #[inline]
    pub(crate) fn colon(&mut self) {
        self.raw_byte(b':');
        self.fmt.sep(&mut self.buf);
    }
}
