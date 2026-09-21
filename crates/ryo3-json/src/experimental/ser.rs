//! Serialize JSON using serde.

use core::fmt::{self, Display, Formatter};

use serde_core::ser::{
    self, Impossible, Serialize, SerializeMap, SerializeSeq, SerializeStruct,
    SerializeStructVariant, SerializeTuple, SerializeTupleStruct, SerializeTupleVariant,
};

use super::format::*;

pub(super) type Result<T> = core::result::Result<T, Error>;

/// JSON serializing structure.
struct Serializer<F: Format> {
    output: Vec<u8>,
    format: F,
}

impl<F: Format> Serializer<F> {
    #[inline(always)]
    fn write(&mut self, v: u8) {
        self.output.push(v);
    }

    #[inline(always)]
    fn write_n(&mut self, v: &[u8]) {
        self.output.extend_from_slice(v);
    }

    #[inline(always)]
    fn comma(&mut self, flag: &mut bool) {
        if *flag {
            self.output.push(b',');
        } else {
            *flag = true;
        }
    }
}

impl<'a, F: Format> ser::Serializer for &'a mut Serializer<F> {
    type Ok = ();
    type Error = Error;
    type SerializeSeq = Container<'a, F>;
    type SerializeTuple = Container<'a, F>;
    type SerializeTupleStruct = Container<'a, F>;
    type SerializeTupleVariant = Container<'a, F>;
    type SerializeMap = Container<'a, F>;
    type SerializeStruct = Container<'a, F>;
    type SerializeStructVariant = Container<'a, F>;

    #[inline]
    fn serialize_bool(self, v: bool) -> Result<()> {
        if v {
            self.output.extend_from_slice(b"true");
        } else {
            self.output.extend_from_slice(b"false");
        }
        Ok(())
    }

    #[inline]
    fn serialize_i8(self, v: i8) -> Result<()> {
        self.serialize_i64(v as _)
    }

    #[inline]
    fn serialize_i16(self, v: i16) -> Result<()> {
        self.serialize_i64(v as _)
    }

    #[inline]
    fn serialize_i32(self, v: i32) -> Result<()> {
        self.serialize_i64(v as _)
    }

    #[inline]
    fn serialize_i64(self, v: i64) -> Result<()> {
        self.write_n(itoa::Buffer::new().format(v).as_bytes());
        Ok(())
    }

    #[inline]
    fn serialize_u8(self, v: u8) -> Result<()> {
        self.serialize_u64(v as _)
    }

    #[inline]
    fn serialize_u16(self, v: u16) -> Result<()> {
        self.serialize_u64(v as _)
    }

    #[inline]
    fn serialize_u32(self, v: u32) -> Result<()> {
        self.serialize_u64(v as _)
    }

    #[inline]
    fn serialize_u64(self, v: u64) -> Result<()> {
        self.write_n(itoa::Buffer::new().format(v).as_bytes());
        Ok(())
    }

    #[inline]
    fn serialize_f32(self, v: f32) -> Result<()> {
        let mut tmp;

        self.write_n(if v.is_finite() {
            tmp = zmij::Buffer::new();
            tmp.format_finite(v).as_bytes()
        } else {
            b"null"
        });
        Ok(())
    }

    #[inline]
    fn serialize_f64(self, v: f64) -> Result<()> {
        let mut tmp;

        self.write_n(if v.is_finite() {
            tmp = zmij::Buffer::new();
            tmp.format_finite(v).as_bytes()
        } else {
            b"null"
        });
        Ok(())
    }

    #[inline]
    fn serialize_char(self, v: char) -> Result<()> {
        self.serialize_str(v.encode_utf8(&mut [0; 4]))
    }

    fn serialize_str(self, v: &str) -> Result<()> {
        format_escaped_str(&mut self.output, v);
        Ok(())
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<()> {
        self.write(b'[');
        let mut flag = false;

        for &v in v {
            self.comma(&mut flag);
            self.write_n(itoa::Buffer::new().format(v).as_bytes());
        }

        self.write(b']');
        Ok(())
    }

    #[inline]
    fn serialize_none(self) -> Result<()> {
        self.serialize_unit()
    }

    #[inline]
    fn serialize_some<T: ?Sized + Serialize>(self, value: &T) -> Result<()> {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<()> {
        self.write_n(b"null");
        Ok(())
    }

    #[inline]
    fn serialize_unit_struct(self, _: &'static str) -> Result<()> {
        self.serialize_unit()
    }

    #[inline]
    fn serialize_unit_variant(self, _: &'static str, _: u32, variant: &'static str) -> Result<()> {
        self.serialize_str(variant)
    }

    #[inline]
    fn serialize_newtype_struct<T: ?Sized + Serialize>(
        self,
        _: &'static str,
        value: &T,
    ) -> Result<()> {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T: ?Sized + Serialize>(
        self,
        _: &'static str,
        _: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<()> {
        self.write(b'{');
        self.serialize_str(variant)?;
        self.write(b':');
        value.serialize(&mut *self)?;
        self.write(b'}');
        Ok(())
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Container<'a, F>> {
        if let Some(len) = len {
            self.output.reserve(len.saturating_mul(2).saturating_add(1));
        }
        self.write(b'[');
        self.format.inc();
        Ok(Container {
            ser: self,
            flag: false,
        })
    }

    #[inline]
    fn serialize_tuple(self, len: usize) -> Result<Container<'a, F>> {
        self.serialize_seq(Some(len))
    }

    #[inline]
    fn serialize_tuple_struct(self, _: &'static str, len: usize) -> Result<Container<'a, F>> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_variant(
        self,
        _: &'static str,
        _: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Container<'a, F>> {
        self.write(b'{');
        self.serialize_str(variant)?;
        self.write(b':');
        self.serialize_seq(Some(len))
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Container<'a, F>> {
        if let Some(len) = len {
            self.output.reserve(len.saturating_mul(4).saturating_add(1));
        }
        self.write(b'{');
        self.format.inc();
        Ok(Container {
            ser: self,
            flag: false,
        })
    }

    #[inline]
    fn serialize_struct(self, _: &'static str, len: usize) -> Result<Container<'a, F>> {
        self.serialize_map(Some(len))
    }

    fn serialize_struct_variant(
        self,
        _: &'static str,
        _: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Container<'a, F>> {
        self.write(b'{');
        self.serialize_str(variant)?;
        self.write(b':');
        self.serialize_map(Some(len))
    }

    fn collect_str<T>(self, value: &T) -> Result<()>
    where
        T: ?Sized + Display,
    {
        use core::fmt::Write as _;

        struct Adapter<'a>(&'a mut Vec<u8>);

        impl fmt::Write for Adapter<'_> {
            fn write_str(&mut self, s: &str) -> fmt::Result {
                format_escaped_str_contents(self.0, s);
                Ok(())
            }
        }

        self.write(b'"');
        write!(Adapter(&mut self.output), "{value}").map_err(|_| {
            <Error as ser::Error>::custom("Display implementation returned an error")
        })?;
        self.write(b'"');
        Ok(())
    }
}

#[inline]
fn format_escaped_str(output: &mut Vec<u8>, value: &str) {
    output.push(b'"');
    format_escaped_str_contents(output, value);
    output.push(b'"');
}

// Adapted from serde_json's table-driven string escaping for an infallible Vec sink.
#[inline]
fn format_escaped_str_contents(output: &mut Vec<u8>, value: &str) {
    let mut bytes = value.as_bytes();

    let mut i = 0;
    while i < bytes.len() {
        let string_run = &bytes[..i];
        let byte = bytes[i];
        let rest = &bytes[i + 1..];

        let escape = ESCAPE[byte as usize];

        i += 1;
        if escape == 0 {
            continue;
        }

        bytes = rest;
        i = 0;

        if !string_run.is_empty() {
            output.extend_from_slice(string_run);
        }

        if escape == UU {
            const HEX_DIGITS: &[u8; 16] = b"0123456789abcdef";
            output.extend_from_slice(&[
                b'\\',
                b'u',
                b'0',
                b'0',
                HEX_DIGITS[(byte >> 4) as usize],
                HEX_DIGITS[(byte & 0x0f) as usize],
            ]);
        } else {
            output.extend_from_slice(&[b'\\', escape]);
        }
    }

    if !bytes.is_empty() {
        output.extend_from_slice(bytes);
    }
}

const BB: u8 = b'b';
const TT: u8 = b't';
const NN: u8 = b'n';
const FF: u8 = b'f';
const RR: u8 = b'r';
const QU: u8 = b'"';
const BS: u8 = b'\\';
const UU: u8 = b'u';
const __: u8 = 0;

// A value of b'x' at index i means that byte i is escaped as "\x" in JSON.
// A value of 0 means that byte i is not escaped.
static ESCAPE: [u8; 256] = [
    //   1   2   3   4   5   6   7   8   9   A   B   C   D   E   F
    UU, UU, UU, UU, UU, UU, UU, UU, BB, TT, NN, UU, FF, RR, UU, UU, // 0
    UU, UU, UU, UU, UU, UU, UU, UU, UU, UU, UU, UU, UU, UU, UU, UU, // 1
    __, __, QU, __, __, __, __, __, __, __, __, __, __, __, __, __, // 2
    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, // 3
    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, // 4
    __, __, __, __, __, __, __, __, __, __, __, __, BS, __, __, __, // 5
    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, // 6
    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, // 7
    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, // 8
    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, // 9
    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, // A
    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, // B
    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, // C
    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, // D
    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, // E
    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, // F
];

#[doc(hidden)]
pub struct Container<'a, F: Format> {
    ser: &'a mut Serializer<F>,
    flag: bool,
}

impl<F: Format> SerializeSeq for Container<'_, F> {
    type Ok = ();
    type Error = Error;

    #[inline]
    fn serialize_element<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<()> {
        self.ser.comma(&mut self.flag);
        self.ser.format.indent(&mut self.ser.output);
        value.serialize(&mut *self.ser)
    }

    #[inline]
    fn end(self) -> Result<()> {
        self.ser.format.dec();
        if self.flag {
            self.ser.format.indent(&mut self.ser.output);
        }
        self.ser.write(b']');
        Ok(())
    }
}

impl<F: Format> SerializeTuple for Container<'_, F> {
    type Ok = ();
    type Error = Error;

    #[inline]
    fn serialize_element<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<()> {
        SerializeSeq::serialize_element(self, value)
    }

    #[inline]
    fn end(self) -> Result<()> {
        SerializeSeq::end(self)
    }
}

impl<F: Format> SerializeTupleStruct for Container<'_, F> {
    type Ok = ();
    type Error = Error;

    #[inline]
    fn serialize_field<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<()> {
        SerializeSeq::serialize_element(self, value)
    }

    #[inline]
    fn end(self) -> Result<()> {
        SerializeSeq::end(self)
    }
}

impl<F: Format> SerializeTupleVariant for Container<'_, F> {
    type Ok = ();
    type Error = Error;

    #[inline]
    fn serialize_field<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<()> {
        SerializeSeq::serialize_element(self, value)
    }

    #[inline]
    fn end(self) -> Result<()> {
        self.ser.format.dec();
        if self.flag {
            self.ser.format.indent(&mut self.ser.output);
        }
        self.ser.write_n(b"]}");
        Ok(())
    }
}

impl<F: Format> SerializeMap for Container<'_, F> {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T: ?Sized + Serialize>(&mut self, key: &T) -> Result<()> {
        self.ser.comma(&mut self.flag);
        self.ser.format.indent(&mut self.ser.output);
        key.serialize(MapKey(self.ser))
    }

    fn serialize_value<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<()> {
        self.ser.write(b':');
        self.ser.format.sep(&mut self.ser.output);
        value.serialize(&mut *self.ser)
    }

    #[inline]
    fn end(self) -> Result<()> {
        self.ser.format.dec();
        if self.flag {
            self.ser.format.indent(&mut self.ser.output);
        }
        self.ser.write(b'}');
        Ok(())
    }
}

impl<F: Format> SerializeStruct for Container<'_, F> {
    type Ok = ();
    type Error = Error;

    #[inline]
    fn serialize_field<T: ?Sized + Serialize>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<()> {
        SerializeMap::serialize_entry(self, key, value)
    }

    #[inline]
    fn end(self) -> Result<()> {
        SerializeMap::end(self)
    }
}

impl<F: Format> SerializeStructVariant for Container<'_, F> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized + Serialize>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<()> {
        SerializeStruct::serialize_field(self, key, value)
    }

    fn end(self) -> Result<()> {
        self.ser.format.dec();
        if self.flag {
            self.ser.format.indent(&mut self.ser.output);
        }
        self.ser.write_n(b"}}");
        Ok(())
    }
}

#[repr(transparent)]
struct MapKey<'a, F: Format>(&'a mut Serializer<F>);

impl<F: Format> ser::Serializer for MapKey<'_, F> {
    type Ok = ();
    type Error = Error;
    type SerializeSeq = Impossible<(), Error>;
    type SerializeTuple = Impossible<(), Error>;
    type SerializeTupleStruct = Impossible<(), Error>;
    type SerializeTupleVariant = Impossible<(), Error>;
    type SerializeMap = Impossible<(), Error>;
    type SerializeStruct = Impossible<(), Error>;
    type SerializeStructVariant = Impossible<(), Error>;

    #[inline]
    fn serialize_bool(self, v: bool) -> Result<()> {
        self.0.write_n(match v {
            true => br#""true""#,
            _ => br#""false""#,
        });
        Ok(())
    }

    #[inline]
    fn serialize_i8(self, v: i8) -> Result<()> {
        self.serialize_i64(v as _)
    }

    #[inline]
    fn serialize_i16(self, v: i16) -> Result<()> {
        self.serialize_i64(v as _)
    }

    #[inline]
    fn serialize_i32(self, v: i32) -> Result<()> {
        self.serialize_i64(v as _)
    }

    fn serialize_i64(self, v: i64) -> Result<()> {
        self.0.write(b'"');
        self.0.write_n(itoa::Buffer::new().format(v).as_bytes());
        self.0.write(b'"');
        Ok(())
    }

    #[inline]
    fn serialize_u8(self, v: u8) -> Result<()> {
        self.serialize_u64(v as _)
    }

    #[inline]
    fn serialize_u16(self, v: u16) -> Result<()> {
        self.serialize_u64(v as _)
    }

    #[inline]
    fn serialize_u32(self, v: u32) -> Result<()> {
        self.serialize_u64(v as _)
    }

    fn serialize_u64(self, v: u64) -> Result<()> {
        self.0.write(b'"');
        self.0.write_n(itoa::Buffer::new().format(v).as_bytes());
        self.0.write(b'"');
        Ok(())
    }

    fn serialize_f32(self, v: f32) -> Result<()> {
        self.0.write(b'"');
        match v.is_finite() {
            true => {
                let mut tmp = zmij::Buffer::new();
                self.0.write_n(tmp.format_finite(v).as_bytes());
            }
            _ => self.0.write_n(b"null"),
        }
        self.0.write(b'"');
        Ok(())
    }

    fn serialize_f64(self, v: f64) -> Result<()> {
        self.0.write(b'"');
        match v.is_finite() {
            true => {
                let mut tmp = zmij::Buffer::new();
                self.0.write_n(tmp.format_finite(v).as_bytes());
            }
            _ => self.0.write_n(b"null"),
        }
        self.0.write(b'"');
        Ok(())
    }

    #[inline]
    fn serialize_char(self, v: char) -> Result<()> {
        self.0.serialize_char(v)
    }

    #[inline]
    fn serialize_str(self, v: &str) -> Result<()> {
        self.0.serialize_str(v)
    }

    #[inline]
    fn serialize_bytes(self, _: &[u8]) -> Result<()> {
        Err(Error::key_must_be_string())
    }

    #[inline]
    fn serialize_none(self) -> Result<()> {
        Err(Error::key_must_be_string())
    }

    #[inline]
    fn serialize_some<T: ?Sized + Serialize>(self, value: &T) -> Result<()> {
        value.serialize(self)
    }

    #[inline]
    fn serialize_unit(self) -> Result<()> {
        Err(Error::key_must_be_string())
    }

    #[inline]
    fn serialize_unit_struct(self, _: &'static str) -> Result<()> {
        Err(Error::key_must_be_string())
    }

    #[inline]
    fn serialize_unit_variant(self, _: &'static str, _: u32, variant: &'static str) -> Result<()> {
        self.0.serialize_str(variant)
    }

    #[inline]
    fn serialize_newtype_struct<T: ?Sized + Serialize>(
        self,
        _: &'static str,
        value: &T,
    ) -> Result<()> {
        value.serialize(self)
    }

    #[inline]
    fn serialize_newtype_variant<T: ?Sized + Serialize>(
        self,
        _: &'static str,
        _: u32,
        _: &'static str,
        _: &T,
    ) -> Result<()> {
        Err(Error::key_must_be_string())
    }

    #[inline]
    fn serialize_seq(self, _: Option<usize>) -> Result<Impossible<(), Error>> {
        Err(Error::key_must_be_string())
    }

    #[inline]
    fn serialize_tuple(self, _: usize) -> Result<Impossible<(), Error>> {
        Err(Error::key_must_be_string())
    }

    #[inline]
    fn serialize_tuple_struct(self, _: &'static str, _: usize) -> Result<Impossible<(), Error>> {
        Err(Error::key_must_be_string())
    }

    #[inline]
    fn serialize_tuple_variant(
        self,
        _: &'static str,
        _: u32,
        _: &'static str,
        _: usize,
    ) -> Result<Impossible<(), Error>> {
        Err(Error::key_must_be_string())
    }

    #[inline]
    fn serialize_map(self, _: Option<usize>) -> Result<Impossible<(), Error>> {
        Err(Error::key_must_be_string())
    }

    #[inline]
    fn serialize_struct(self, _: &'static str, _: usize) -> Result<Impossible<(), Error>> {
        Err(Error::key_must_be_string())
    }

    #[inline]
    fn serialize_struct_variant(
        self,
        _: &'static str,
        _: u32,
        _: &'static str,
        _: usize,
    ) -> Result<Impossible<(), Error>> {
        Err(Error::key_must_be_string())
    }

    fn collect_str<T>(self, value: &T) -> Result<()>
    where
        T: ?Sized + Display,
    {
        self.0.collect_str(value)
    }
}

/// Represents error occurred while serializing.
#[derive(Debug)]
pub struct Error {
    kind: Box<ErrorKind>,
}

#[derive(Debug)]
enum ErrorKind {
    KeyMustBeString,
    Message(String),
}

impl Error {
    #[inline]
    fn key_must_be_string() -> Self {
        Self {
            kind: Box::new(ErrorKind::KeyMustBeString),
        }
    }
}

impl ser::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Self {
            kind: Box::new(ErrorKind::Message(msg.to_string())),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self.kind.as_ref() {
            ErrorKind::KeyMustBeString => f.write_str("key must be a string"),
            ErrorKind::Message(msg) => f.write_str(msg),
        }
    }
}

impl core::error::Error for Error {}

/// Serializes the given data into a JSON byte vector.
///
/// # Errors
///
/// Returns an error if `T`'s `Serialize` implementation fails or `T` contains
/// an unsupported map key.
#[inline]
pub(super) fn to_vec<T: ?Sized + Serialize>(v: &T) -> Result<Vec<u8>> {
    let mut serializer = Serializer {
        output: Vec::with_capacity(4096),
        format: Compact,
    };
    v.serialize(&mut serializer)?;
    Ok(serializer.output)
}

/// Serializes the given data into a pretty-printed JSON byte vector.
///
/// # Errors
///
/// Returns an error if `T`'s `Serialize` implementation fails or `T` contains
/// an unsupported map key.
#[inline]
pub(super) fn to_vec_pretty<T: ?Sized + Serialize>(v: &T) -> Result<Vec<u8>> {
    let mut serializer = Serializer {
        output: Vec::with_capacity(4096),
        format: Pretty::new(),
    };
    v.serialize(&mut serializer)?;
    Ok(serializer.output)
}

#[cfg(test)]
mod tests {
    use core::fmt::{self, Display, Formatter};
    use core::mem::size_of;
    use std::collections::BTreeMap;

    use serde_core::Serialize;
    use serde_json::json;

    use super::{Error, to_vec, to_vec_pretty};

    fn serialize<T: ?Sized + Serialize>(value: &T) -> Vec<u8> {
        match to_vec(value) {
            Ok(output) => output,
            Err(error) => panic!("serialization failed: {error}"),
        }
    }

    fn serialize_pretty<T: ?Sized + Serialize>(value: &T) -> Vec<u8> {
        match to_vec_pretty(value) {
            Ok(output) => output,
            Err(error) => panic!("serialization failed: {error}"),
        }
    }

    #[test]
    fn serializes_compact_json_to_vec() {
        let value = json!({"answer": 42, "items": [true, null, "a\nb"]});

        assert_eq!(
            serialize(&value),
            br#"{"answer":42,"items":[true,null,"a\nb"]}"#
        );
    }

    #[test]
    fn serializes_pretty_json_to_vec() {
        let value = json!({"items": [1, 2]});

        assert_eq!(
            serialize_pretty(&value),
            b"{\n  \"items\": [\n    1,\n    2\n  ]\n}"
        );
    }

    #[test]
    fn escapes_strings_like_serde_json() {
        let mut value = String::new();
        for byte in 0..=0x1f {
            value.push(char::from(byte));
        }
        value.push_str(" quote=\" slash=\\ solidus=/ unicode=❤");

        assert_eq!(serialize(&value), serialize_with_serde_json(&value));
    }

    #[derive(Eq, Ord, PartialEq, PartialOrd)]
    struct Collected;

    impl Display for Collected {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            f.write_str("a\"")?;
            f.write_str("\n\\z")
        }
    }

    impl Serialize for Collected {
        fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
        where
            S: serde_core::Serializer,
        {
            serializer.collect_str(self)
        }
    }

    fn serialize_with_serde_json<T: ?Sized + Serialize>(value: &T) -> Vec<u8> {
        match serde_json::to_vec(value) {
            Ok(output) => output,
            Err(error) => panic!("serde_json serialization failed: {error}"),
        }
    }

    #[test]
    fn error_is_pointer_sized() {
        assert_eq!(size_of::<Error>(), size_of::<usize>());
        assert_eq!(size_of::<super::Result<()>>(), size_of::<usize>());
    }

    #[test]
    fn collect_str_escapes_without_an_intermediate_string() {
        assert_eq!(serialize(&Collected), br#""a\"\n\\z""#);

        let map = BTreeMap::from([(Collected, 1)]);
        assert_eq!(serialize(&map), br#"{"a\"\n\\z":1}"#);
    }
}
