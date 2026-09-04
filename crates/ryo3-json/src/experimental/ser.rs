//! Serialize JSON using serde.

use core::fmt::{self, Display, Formatter};
use std::io::Write;

use serde_core::ser::{
    self, Impossible, Serialize, SerializeMap, SerializeSeq, SerializeStruct,
    SerializeStructVariant, SerializeTuple, SerializeTupleStruct, SerializeTupleVariant,
};

use super::format::*;

pub(super) type Result<T> = core::result::Result<T, Error>;

/// JSON serializing structure.
struct Serializer<W: Write, F: Format> {
    writer: W,
    format: F,
}

impl<W: Write, F: Format> Serializer<W, F> {
    #[inline(always)]
    pub(super) fn write(&mut self, v: u8) -> Result<()> {
        match self.writer.write_all(&[v]) {
            Ok(_) => Ok(()),
            _ => Err(Error::io()),
        }
    }

    #[inline(always)]
    pub(super) fn write_n(&mut self, v: &[u8]) -> Result<()> {
        match self.writer.write_all(v) {
            Ok(_) => Ok(()),
            _ => Err(Error::io()),
        }
    }

    #[inline(always)]
    fn comma(&mut self, flag: &mut bool) -> Result<()> {
        if *flag {
            match self.writer.write_all(b",") {
                Ok(_) => Ok(()),
                _ => Err(Error::io()),
            }
        } else {
            *flag = true;
            Ok(())
        }
    }
}

impl<'a, W: Write, F: Format> ser::Serializer for &'a mut Serializer<W, F> {
    type Ok = ();
    type Error = Error;
    type SerializeSeq = Container<'a, W, F>;
    type SerializeTuple = Container<'a, W, F>;
    type SerializeTupleStruct = Container<'a, W, F>;
    type SerializeTupleVariant = Container<'a, W, F>;
    type SerializeMap = Container<'a, W, F>;
    type SerializeStruct = Container<'a, W, F>;
    type SerializeStructVariant = Container<'a, W, F>;

    #[inline]
    fn serialize_bool(self, v: bool) -> Result<()> {
        self.write_n(if v { b"true" } else { b"false" })
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
        self.write_n(itoa::Buffer::new().format(v).as_bytes())
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
        self.write_n(itoa::Buffer::new().format(v).as_bytes())
    }

    fn serialize_f32(self, v: f32) -> Result<()> {
        let mut tmp;

        self.write_n(if v.is_finite() {
            tmp = zmij::Buffer::new();
            tmp.format_finite(v).as_bytes()
        } else {
            b"null"
        })
    }

    fn serialize_f64(self, v: f64) -> Result<()> {
        let mut tmp;

        self.write_n(if v.is_finite() {
            tmp = zmij::Buffer::new();
            tmp.format_finite(v).as_bytes()
        } else {
            b"null"
        })
    }

    #[inline]
    fn serialize_char(self, v: char) -> Result<()> {
        self.serialize_str(v.encode_utf8(&mut [0; 4]))
    }

    fn serialize_str(self, v: &str) -> Result<()> {
        const HEX: &[u8; 16] = b"0123456789abcdef";

        let bytes = v.as_bytes();
        if !bytes
            .iter()
            .any(|&byte| matches!(byte, b'"' | b'\\' | 0x00..=0x1f))
        {
            self.write(b'"')?;
            self.write_n(bytes)?;
            return self.write(b'"');
        }

        self.write(b'"')?;

        let mut start = 0;
        for (idx, &byte) in bytes.iter().enumerate() {
            let escaped = match byte {
                b'"' => Some(br#"\""#.as_slice()),
                b'\\' => Some(br#"\\"#.as_slice()),
                b'\n' => Some(br#"\n"#.as_slice()),
                b'\r' => Some(br#"\r"#.as_slice()),
                b'\t' => Some(br#"\t"#.as_slice()),
                b'\x08' => Some(br#"\b"#.as_slice()),
                b'\x0c' => Some(br#"\f"#.as_slice()),
                0x00..=0x1f => {
                    if start < idx {
                        self.write_n(&bytes[start..idx])?;
                    }
                    self.write_n(&[
                        b'\\',
                        b'u',
                        b'0',
                        b'0',
                        HEX[(byte >> 4) as usize],
                        HEX[(byte & 0x0f) as usize],
                    ])?;
                    start = idx + 1;
                    None
                }
                _ => None,
            };

            if let Some(escaped) = escaped {
                if start < idx {
                    self.write_n(&bytes[start..idx])?;
                }
                self.write_n(escaped)?;
                start = idx + 1;
            }
        }

        if start < bytes.len() {
            self.write_n(&bytes[start..])?;
        }

        self.write(b'"')
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<()> {
        self.write(b'[')?;
        let mut flag = false;

        for &v in v {
            self.comma(&mut flag)?;
            self.write_n(itoa::Buffer::new().format(v).as_bytes())?;
        }

        self.write(b']')
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
        self.write_n(b"null")
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
        self.write(b'{')?;
        self.serialize_str(variant)?;
        self.write(b':')?;
        value.serialize(&mut *self)?;
        self.write(b'}')
    }

    fn serialize_seq(self, _: Option<usize>) -> Result<Container<'a, W, F>> {
        self.write(b'[')?;
        self.format.inc();
        Ok(Container {
            ser: self,
            flag: false,
        })
    }

    #[inline]
    fn serialize_tuple(self, len: usize) -> Result<Container<'a, W, F>> {
        self.serialize_seq(Some(len))
    }

    #[inline]
    fn serialize_tuple_struct(self, _: &'static str, len: usize) -> Result<Container<'a, W, F>> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_variant(
        self,
        _: &'static str,
        _: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Container<'a, W, F>> {
        self.write(b'{')?;
        self.serialize_str(variant)?;
        self.write(b':')?;
        self.serialize_seq(Some(len))
    }

    fn serialize_map(self, _: Option<usize>) -> Result<Container<'a, W, F>> {
        self.write(b'{')?;
        self.format.inc();
        Ok(Container {
            ser: self,
            flag: false,
        })
    }

    #[inline]
    fn serialize_struct(self, _: &'static str, len: usize) -> Result<Container<'a, W, F>> {
        self.serialize_map(Some(len))
    }

    fn serialize_struct_variant(
        self,
        _: &'static str,
        _: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Container<'a, W, F>> {
        self.write(b'{')?;
        self.serialize_str(variant)?;
        self.write(b':')?;
        self.serialize_map(Some(len))
    }
}

#[doc(hidden)]
pub struct Container<'a, W: Write, F: Format> {
    ser: &'a mut Serializer<W, F>,
    flag: bool,
}

impl<W: Write, F: Format> SerializeSeq for Container<'_, W, F> {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<()> {
        self.ser.comma(&mut self.flag)?;
        self.ser.format.indent(&mut self.ser.writer)?;
        value.serialize(&mut *self.ser)
    }

    #[inline]
    fn end(self) -> Result<()> {
        self.ser.format.dec();
        if self.flag {
            self.ser.format.indent(&mut self.ser.writer)?
        }
        self.ser.write(b']')
    }
}

impl<W: Write, F: Format> SerializeTuple for Container<'_, W, F> {
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

impl<W: Write, F: Format> SerializeTupleStruct for Container<'_, W, F> {
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

impl<W: Write, F: Format> SerializeTupleVariant for Container<'_, W, F> {
    type Ok = ();
    type Error = Error;

    #[inline]
    fn serialize_field<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<()> {
        SerializeSeq::serialize_element(self, value)
    }

    #[inline]
    fn end(self) -> Result<()> {
        match self.ser.writer.write(b"]}") {
            Ok(_) => Ok(()),
            _ => Err(Error::io()),
        }
    }
}

impl<W: Write, F: Format> SerializeMap for Container<'_, W, F> {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T: ?Sized + Serialize>(&mut self, key: &T) -> Result<()> {
        self.ser.comma(&mut self.flag)?;
        self.ser.format.indent(&mut self.ser.writer)?;
        key.serialize(MapKey(self.ser))
    }

    fn serialize_value<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<()> {
        self.ser.write(b':')?;
        self.ser.format.sep(&mut self.ser.writer)?;
        value.serialize(&mut *self.ser)
    }

    #[inline]
    fn end(self) -> Result<()> {
        self.ser.format.dec();
        if self.flag {
            self.ser.format.indent(&mut self.ser.writer)?
        }
        self.ser.write(b'}')
    }
}

impl<W: Write, F: Format> SerializeStruct for Container<'_, W, F> {
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

impl<W: Write, F: Format> SerializeStructVariant for Container<'_, W, F> {
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
        match self.ser.writer.write(b"}}") {
            Ok(_) => Ok(()),
            _ => Err(Error::io()),
        }
    }
}

#[repr(transparent)]
struct MapKey<'a, W: Write, F: Format>(&'a mut Serializer<W, F>);

impl<W: Write, F: Format> ser::Serializer for MapKey<'_, W, F> {
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
        })
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
        self.0.write(b'"')?;
        self.0.write_n(itoa::Buffer::new().format(v).as_bytes())?;
        self.0.write(b'"')
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
        self.0.write(b'"')?;
        self.0.write_n(itoa::Buffer::new().format(v).as_bytes())?;
        self.0.write(b'"')
    }

    fn serialize_f32(self, v: f32) -> Result<()> {
        self.0.write(b'"')?;
        if let Err(_) = match v.is_finite() {
            true => {
                let mut tmp = zmij::Buffer::new();
                self.0.write_n(tmp.format_finite(v).as_bytes())
            }
            _ => self.0.write_n(b"null"),
        } {
            return Err(Error::io());
        }
        self.0.write(b'"')
    }

    fn serialize_f64(self, v: f64) -> Result<()> {
        self.0.write(b'"')?;
        if let Err(_) = match v.is_finite() {
            true => {
                let mut tmp = zmij::Buffer::new();
                self.0.write_n(tmp.format_finite(v).as_bytes())
            }
            _ => self.0.write_n(b"null"),
        } {
            return Err(Error::io());
        }
        self.0.write(b'"')
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
        Err(Error::io())
    }

    #[inline]
    fn serialize_none(self) -> Result<()> {
        Err(Error::io())
    }

    #[inline]
    fn serialize_some<T: ?Sized + Serialize>(self, value: &T) -> Result<()> {
        value.serialize(self)
    }

    #[inline]
    fn serialize_unit(self) -> Result<()> {
        Err(Error::io())
    }

    #[inline]
    fn serialize_unit_struct(self, _: &'static str) -> Result<()> {
        Err(Error::io())
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
        Err(Error::io())
    }

    #[inline]
    fn serialize_seq(self, _: Option<usize>) -> Result<Impossible<(), Error>> {
        Err(Error::io())
    }

    #[inline]
    fn serialize_tuple(self, _: usize) -> Result<Impossible<(), Error>> {
        Err(Error::io())
    }

    #[inline]
    fn serialize_tuple_struct(self, _: &'static str, _: usize) -> Result<Impossible<(), Error>> {
        Err(Error::io())
    }

    #[inline]
    fn serialize_tuple_variant(
        self,
        _: &'static str,
        _: u32,
        _: &'static str,
        _: usize,
    ) -> Result<Impossible<(), Error>> {
        Err(Error::io())
    }

    #[inline]
    fn serialize_map(self, _: Option<usize>) -> Result<Impossible<(), Error>> {
        Err(Error::io())
    }

    #[inline]
    fn serialize_struct(self, _: &'static str, _: usize) -> Result<Impossible<(), Error>> {
        Err(Error::io())
    }

    #[inline]
    fn serialize_struct_variant(
        self,
        _: &'static str,
        _: u32,
        _: &'static str,
        _: usize,
    ) -> Result<Impossible<(), Error>> {
        Err(Error::io())
    }
}

/// Represents error occurred while serializing.
#[derive(Debug)]
pub struct Error {
    msg: Option<String>,
}

impl Error {
    #[inline]
    pub(super) const fn io() -> Self {
        Self { msg: None }
    }
}

impl ser::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Self {
            msg: Some(msg.to_string()),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(self.msg.as_deref().unwrap_or("failed to write JSON"))
    }
}

impl core::error::Error for Error {}

/// Serializes the given data into the provided writer as a JSON.
///
/// # Errors
///
/// Returns error if `T`'s `Serialize` implementation fails, `T` contains
/// non-string map keys, or an I/O error occurs while writing.
#[inline]
pub(super) fn to_writer<T: Serialize>(w: impl Write, v: T) -> Result<()> {
    v.serialize(&mut Serializer {
        writer: w,
        format: Compact,
    })
}

/// Serializes the given data into the provided writer as a pretty-printed JSON.
///
/// # Errors
///
/// Returns error if `T`'s `Serialize` implementation fails, `T` contains
/// non-string map keys, or an I/O error occurs while writing.
#[inline]
pub(super) fn to_writer_pretty<T: Serialize>(w: impl Write, v: T) -> Result<()> {
    v.serialize(&mut Serializer {
        writer: w,
        format: Pretty::new(),
    })
}
