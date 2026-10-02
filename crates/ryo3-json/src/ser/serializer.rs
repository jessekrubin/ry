//! json serializer
use serde_core::ser;

use super::{JsonFormat, JsonFormatCompact, JsonFormatPretty, JsonSerError, JsonWriter, Result};

/// JSON serialize compact
///
/// # Errors
///
/// Returns an error if `T`'s `Serialize` implementation fails or `T` contains
/// an unsupported map key.
#[inline]
pub fn to_vec<T: ?Sized + ser::Serialize>(v: &T) -> Result<Vec<u8>> {
    let mut serializer = Serializer {
        w: JsonWriter::with_capacity(4096, JsonFormatCompact),
    };
    v.serialize(&mut serializer)?;
    Ok(serializer.w.into_inner())
}

#[inline]
pub fn to_string<T: ?Sized + ser::Serialize>(v: &T) -> Result<String> {
    let vec = to_vec(v)?;
    // SAFETY: `to_vec` is valid utf8-string (wenodis)
    #[expect(unsafe_code)]
    let s = unsafe { String::from_utf8_unchecked(vec) };
    Ok(s)
}

/// JSON serialize formatted (indent=2)
///
/// # Errors
///
/// Returns an error if `T`'s `Serialize` implementation fails or `T` contains
/// an unsupported map key.
#[inline]
pub fn to_vec_pretty<T: ?Sized + ser::Serialize>(v: &T) -> Result<Vec<u8>> {
    let mut serializer = Serializer {
        w: JsonWriter::with_capacity(4096, JsonFormatPretty::<2>::new()),
    };
    v.serialize(&mut serializer)?;
    Ok(serializer.w.into_inner())
}

#[inline]
pub fn to_string_pretty<T: ?Sized + ser::Serialize>(v: &T) -> Result<String> {
    let vec = to_vec_pretty(v)?;
    // SAFETY: `to_vec_pretty` is valid utf8-string (wenodis)
    #[expect(unsafe_code)]
    let s = unsafe { String::from_utf8_unchecked(vec) };
    Ok(s)
}

/// JSON serializer with vec writer
pub(crate) struct Serializer<F: JsonFormat> {
    w: JsonWriter<F>,
}

impl<F: JsonFormat> Serializer<F> {
    #[inline]
    pub(crate) fn with_capacity_and_format(capacity: usize, fmt: F) -> Self {
        Self {
            w: JsonWriter::with_capacity(capacity, fmt),
        }
    }

    #[inline]
    pub(crate) fn into_inner(self) -> Vec<u8> {
        self.w.into_inner()
    }
}

impl Serializer<JsonFormatCompact> {
    // #[inline]
    // pub(crate) fn compact() -> Self {
    //     Self::compact_with_capacity(4096)
    // }

    #[inline]
    pub(crate) fn compact_with_capacity(capacity: usize) -> Self {
        Self::with_capacity_and_format(capacity, JsonFormatCompact)
    }
}

impl Serializer<JsonFormatPretty<2>> {
    // #[inline]
    // pub(crate) fn pretty() -> Self {
    //     Self::pretty_with_capacity(4096)
    // }

    #[inline]
    pub(crate) fn pretty_with_capacity(capacity: usize) -> Self {
        Self::with_capacity_and_format(capacity, JsonFormatPretty::<2>::new())
    }
}

impl<'a, F: JsonFormat> ser::Serializer for &'a mut Serializer<F> {
    type Ok = ();
    type Error = JsonSerError;
    type SerializeSeq = Container<'a, F>;
    type SerializeTuple = Container<'a, F>;
    type SerializeTupleStruct = Container<'a, F>;
    type SerializeTupleVariant = Container<'a, F>;
    type SerializeMap = Container<'a, F>;
    type SerializeStruct = Container<'a, F>;
    type SerializeStructVariant = Container<'a, F>;

    #[inline]
    fn serialize_bool(self, v: bool) -> Result<()> {
        self.w.write_bool(v);
        Ok(())
    }

    #[inline]
    fn serialize_i8(self, v: i8) -> Result<()> {
        self.w.write_i8(v);
        Ok(())
    }

    #[inline]
    fn serialize_i16(self, v: i16) -> Result<()> {
        self.w.write_i16(v);
        Ok(())
    }

    #[inline]
    fn serialize_i32(self, v: i32) -> Result<()> {
        self.w.write_i32(v);
        Ok(())
    }

    #[inline]
    fn serialize_i64(self, v: i64) -> Result<()> {
        self.w.write_i64(v);
        Ok(())
    }

    #[inline]
    fn serialize_i128(self, v: i128) -> Result<()> {
        self.w.write_i128(v);
        Ok(())
    }

    #[inline]
    fn serialize_u8(self, v: u8) -> Result<()> {
        self.w.write_u8(v);
        Ok(())
    }

    #[inline]
    fn serialize_u16(self, v: u16) -> Result<()> {
        self.w.write_u16(v);
        Ok(())
    }

    #[inline]
    fn serialize_u32(self, v: u32) -> Result<()> {
        self.w.write_u32(v);
        Ok(())
    }

    #[inline]
    fn serialize_u64(self, v: u64) -> Result<()> {
        self.w.write_u64(v);
        Ok(())
    }

    #[inline]
    fn serialize_u128(self, v: u128) -> Result<()> {
        self.w.write_u128(v);
        Ok(())
    }

    #[inline]
    fn serialize_f32(self, v: f32) -> Result<()> {
        self.w.write_f32(v);
        Ok(())
    }

    #[inline]
    fn serialize_f64(self, v: f64) -> Result<()> {
        self.w.write_f64(v);
        Ok(())
    }

    #[inline]
    fn serialize_char(self, v: char) -> Result<()> {
        self.serialize_str(v.encode_utf8(&mut [0; 4]))
    }

    #[inline]
    fn serialize_str(self, v: &str) -> Result<()> {
        self.w.write_str(v);
        Ok(())
    }

    #[inline]
    fn serialize_bytes(self, v: &[u8]) -> Result<()> {
        self.w.raw_byte(b'[');
        match v.len() {
            0 => {}
            1 => {
                self.w.write_u8(v[0]);
            }
            _ => {
                let (f, rest) = v.split_first().expect("wenodis");
                self.w.write_u8(*f);
                for &v in rest {
                    self.w.raw_byte(b',');
                    self.w.write_u8(v);
                }
            }
        }
        self.w.raw_byte(b']');
        Ok(())
    }

    #[inline]
    fn serialize_none(self) -> Result<()> {
        self.serialize_unit()
    }

    #[inline]
    fn serialize_some<T: ?Sized + ser::Serialize>(self, value: &T) -> Result<()> {
        value.serialize(self)
    }

    #[inline]
    fn serialize_unit(self) -> Result<()> {
        self.w.write_null();
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
    fn serialize_newtype_struct<T: ?Sized + ser::Serialize>(
        self,
        _: &'static str,
        value: &T,
    ) -> Result<()> {
        value.serialize(self)
    }

    #[inline]
    fn serialize_newtype_variant<T: ?Sized + ser::Serialize>(
        self,
        _: &'static str,
        _: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<()> {
        self.w.raw_byte(b'{');
        self.w.write_str(variant);
        self.w.raw_byte(b':');
        value.serialize(&mut *self)?;
        self.w.raw_byte(b'}');
        Ok(())
    }

    #[inline]
    fn serialize_seq(self, len: Option<usize>) -> Result<Container<'a, F>> {
        if let Some(len) = len {
            self.w.reserve(len.saturating_mul(2).saturating_add(1));
        }
        self.w.begin_array();
        Ok(Container {
            ser: self,
            written: false,
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

    #[inline]
    fn serialize_tuple_variant(
        self,
        _: &'static str,
        _: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Container<'a, F>> {
        self.w.raw_byte(b'{');
        self.w.write_str(variant);
        self.w.raw_byte(b':');
        self.serialize_seq(Some(len))
    }

    #[inline]
    fn serialize_map(self, len: Option<usize>) -> Result<Container<'a, F>> {
        if let Some(len) = len {
            self.w.reserve(len.saturating_mul(4).saturating_add(1));
        }
        self.w.begin_object();
        Ok(Container {
            ser: self,
            written: false,
        })
    }

    #[inline]
    fn serialize_struct(self, _: &'static str, len: usize) -> Result<Container<'a, F>> {
        self.serialize_map(Some(len))
    }

    #[inline]
    fn serialize_struct_variant(
        self,
        _: &'static str,
        _: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Container<'a, F>> {
        self.w.raw_byte(b'{');
        self.w.write_str(variant);
        self.w.raw_byte(b':');
        self.serialize_map(Some(len))
    }

    fn collect_str<T>(self, value: &T) -> Result<()>
    where
        T: ?Sized + std::fmt::Display,
    {
        use core::fmt::Write as _;

        struct Adapter<'a, F: JsonFormat>(&'a mut JsonWriter<F>);

        impl<F: JsonFormat> std::fmt::Write for Adapter<'_, F> {
            fn write_str(&mut self, s: &str) -> std::fmt::Result {
                self.0.str_contents(s);
                Ok(())
            }
        }

        self.w.raw_byte(b'"');
        write!(Adapter(&mut self.w), "{value}").map_err(|_| {
            <JsonSerError as ser::Error>::custom("Display implementation returned an error")
        })?;
        self.w.raw_byte(b'"');
        Ok(())
    }
}

#[doc(hidden)]
pub struct Container<'a, F: JsonFormat> {
    ser: &'a mut Serializer<F>,
    written: bool,
}

impl<F: JsonFormat> ser::SerializeSeq for Container<'_, F> {
    type Ok = ();
    type Error = JsonSerError;

    #[inline]
    fn serialize_element<T: ?Sized + ser::Serialize>(&mut self, value: &T) -> Result<()> {
        self.ser.w.elem_sep(&mut self.written);
        value.serialize(&mut *self.ser)
    }

    #[inline]
    fn end(self) -> Result<()> {
        self.ser.w.end_array(self.written);
        Ok(())
    }
}

impl<F: JsonFormat> ser::SerializeTuple for Container<'_, F> {
    type Ok = ();
    type Error = JsonSerError;

    #[inline]
    fn serialize_element<T: ?Sized + ser::Serialize>(&mut self, value: &T) -> Result<()> {
        ser::SerializeSeq::serialize_element(self, value)
    }

    #[inline]
    fn end(self) -> Result<()> {
        ser::SerializeSeq::end(self)
    }
}

impl<F: JsonFormat> ser::SerializeTupleStruct for Container<'_, F> {
    type Ok = ();
    type Error = JsonSerError;

    #[inline]
    fn serialize_field<T: ?Sized + ser::Serialize>(&mut self, value: &T) -> Result<()> {
        ser::SerializeSeq::serialize_element(self, value)
    }

    #[inline]
    fn end(self) -> Result<()> {
        ser::SerializeSeq::end(self)
    }
}

impl<F: JsonFormat> ser::SerializeTupleVariant for Container<'_, F> {
    type Ok = ();
    type Error = JsonSerError;

    #[inline]
    fn serialize_field<T: ?Sized + ser::Serialize>(&mut self, value: &T) -> Result<()> {
        ser::SerializeSeq::serialize_element(self, value)
    }

    #[inline]
    fn end(self) -> Result<()> {
        self.ser.w.end_array(self.written);
        self.ser.w.raw_byte(b'}');
        Ok(())
    }
}

impl<F: JsonFormat> ser::SerializeMap for Container<'_, F> {
    type Ok = ();
    type Error = JsonSerError;

    fn serialize_key<T: ?Sized + ser::Serialize>(&mut self, key: &T) -> Result<()> {
        self.ser.w.elem_sep(&mut self.written);
        key.serialize(MapKey(self.ser))
    }

    fn serialize_value<T: ?Sized + ser::Serialize>(&mut self, value: &T) -> Result<()> {
        self.ser.w.key_sep();
        value.serialize(&mut *self.ser)
    }

    #[inline]
    fn end(self) -> Result<()> {
        self.ser.w.end_object(self.written);
        Ok(())
    }
}

impl<F: JsonFormat> ser::SerializeStruct for Container<'_, F> {
    type Ok = ();
    type Error = JsonSerError;

    #[inline]
    fn serialize_field<T: ?Sized + ser::Serialize>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<()> {
        ser::SerializeMap::serialize_entry(self, key, value)
    }

    #[inline]
    fn end(self) -> Result<()> {
        ser::SerializeMap::end(self)
    }
}

impl<F: JsonFormat> ser::SerializeStructVariant for Container<'_, F> {
    type Ok = ();
    type Error = JsonSerError;

    fn serialize_field<T: ?Sized + ser::Serialize>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<()> {
        ser::SerializeStruct::serialize_field(self, key, value)
    }

    fn end(self) -> Result<()> {
        self.ser.w.end_object(self.written);
        self.ser.w.raw_byte(b'}');
        Ok(())
    }
}

#[repr(transparent)]
struct MapKey<'a, F: JsonFormat>(&'a mut Serializer<F>);

impl<F: JsonFormat> ser::Serializer for MapKey<'_, F> {
    type Ok = ();
    type Error = JsonSerError;
    type SerializeSeq = ser::Impossible<(), JsonSerError>;
    type SerializeTuple = ser::Impossible<(), JsonSerError>;
    type SerializeTupleStruct = ser::Impossible<(), JsonSerError>;
    type SerializeTupleVariant = ser::Impossible<(), JsonSerError>;
    type SerializeMap = ser::Impossible<(), JsonSerError>;
    type SerializeStruct = ser::Impossible<(), JsonSerError>;
    type SerializeStructVariant = ser::Impossible<(), JsonSerError>;

    #[inline]
    fn serialize_bool(self, v: bool) -> Result<()> {
        self.0.w.write_bool_key(v);
        Ok(())
    }

    #[inline]
    fn serialize_i8(self, v: i8) -> Result<()> {
        self.0.w.write_i8_key(v);
        Ok(())
    }

    #[inline]
    fn serialize_i16(self, v: i16) -> Result<()> {
        self.0.w.write_i16_key(v);
        Ok(())
    }

    #[inline]
    fn serialize_i32(self, v: i32) -> Result<()> {
        self.0.w.write_i32_key(v);
        Ok(())
    }

    #[inline]
    fn serialize_i64(self, v: i64) -> Result<()> {
        self.0.w.write_i64_key(v);
        Ok(())
    }

    #[inline]
    fn serialize_i128(self, v: i128) -> Result<()> {
        self.0.w.write_i128_key(v);
        Ok(())
    }

    #[inline]
    fn serialize_u8(self, v: u8) -> Result<()> {
        self.0.w.write_u8_key(v);
        Ok(())
    }

    #[inline]
    fn serialize_u16(self, v: u16) -> Result<()> {
        self.0.w.write_u16_key(v);
        Ok(())
    }

    #[inline]
    fn serialize_u32(self, v: u32) -> Result<()> {
        self.0.w.write_u32_key(v);
        Ok(())
    }

    #[inline]
    fn serialize_u64(self, v: u64) -> Result<()> {
        self.0.w.write_u64_key(v);
        Ok(())
    }

    #[inline]
    fn serialize_u128(self, v: u128) -> Result<()> {
        self.0.w.write_u128_key(v);
        Ok(())
    }

    fn serialize_f32(self, v: f32) -> Result<()> {
        if v.is_finite() {
            self.0.w.write_f32_key(v);
            Ok(())
        } else {
            Err(JsonSerError::float_key_must_be_finite())
        }
    }

    fn serialize_f64(self, v: f64) -> Result<()> {
        if v.is_finite() {
            self.0.w.write_f64_key(v);
            Ok(())
        } else {
            Err(JsonSerError::float_key_must_be_finite())
        }
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
        Err(JsonSerError::key_must_be_string())
    }

    #[inline]
    fn serialize_none(self) -> Result<()> {
        Err(JsonSerError::key_must_be_string())
    }

    #[inline]
    fn serialize_some<T: ?Sized + ser::Serialize>(self, value: &T) -> Result<()> {
        value.serialize(self)
    }

    #[inline]
    fn serialize_unit(self) -> Result<()> {
        Err(JsonSerError::key_must_be_string())
    }

    #[inline]
    fn serialize_unit_struct(self, _: &'static str) -> Result<()> {
        Err(JsonSerError::key_must_be_string())
    }

    #[inline]
    fn serialize_unit_variant(self, _: &'static str, _: u32, variant: &'static str) -> Result<()> {
        self.0.serialize_str(variant)
    }

    #[inline]
    fn serialize_newtype_struct<T: ?Sized + ser::Serialize>(
        self,
        _: &'static str,
        value: &T,
    ) -> Result<()> {
        value.serialize(self)
    }

    #[inline]
    fn serialize_newtype_variant<T: ?Sized + ser::Serialize>(
        self,
        _: &'static str,
        _: u32,
        _: &'static str,
        _: &T,
    ) -> Result<()> {
        Err(JsonSerError::key_must_be_string())
    }

    #[inline]
    fn serialize_seq(self, _: Option<usize>) -> Result<ser::Impossible<(), JsonSerError>> {
        Err(JsonSerError::key_must_be_string())
    }

    #[inline]
    fn serialize_tuple(self, _: usize) -> Result<ser::Impossible<(), JsonSerError>> {
        Err(JsonSerError::key_must_be_string())
    }

    #[inline]
    fn serialize_tuple_struct(
        self,
        _: &'static str,
        _: usize,
    ) -> Result<ser::Impossible<(), JsonSerError>> {
        Err(JsonSerError::key_must_be_string())
    }

    #[inline]
    fn serialize_tuple_variant(
        self,
        _: &'static str,
        _: u32,
        _: &'static str,
        _: usize,
    ) -> Result<ser::Impossible<(), JsonSerError>> {
        Err(JsonSerError::key_must_be_string())
    }

    #[inline]
    fn serialize_map(self, _: Option<usize>) -> Result<ser::Impossible<(), JsonSerError>> {
        Err(JsonSerError::key_must_be_string())
    }

    #[inline]
    fn serialize_struct(
        self,
        _: &'static str,
        _: usize,
    ) -> Result<ser::Impossible<(), JsonSerError>> {
        Err(JsonSerError::key_must_be_string())
    }

    #[inline]
    fn serialize_struct_variant(
        self,
        _: &'static str,
        _: u32,
        _: &'static str,
        _: usize,
    ) -> Result<ser::Impossible<(), JsonSerError>> {
        Err(JsonSerError::key_must_be_string())
    }

    fn collect_str<T>(self, value: &T) -> Result<()>
    where
        T: ?Sized + std::fmt::Display,
    {
        self.0.collect_str(value)
    }
}

#[cfg(test)]
mod tests {
    use core::fmt::{self, Display, Formatter};
    use std::collections::BTreeMap;

    use serde_core::Serialize;
    use serde_json::json;

    use super::{to_vec, to_vec_pretty};

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
    fn serializes_integer_arrays() {
        let value: Vec<i64> = vec![0, 7, -1, 1234, i64::MIN, i64::MAX];
        assert_eq!(
            serialize(&value),
            serde_json::to_vec(&value).expect("serde_json failed")
        );
        let value: Vec<u64> = (0..1000).map(|i| i * 7919).chain([u64::MAX]).collect();
        assert_eq!(
            serialize(&value),
            serde_json::to_vec(&value).expect("serde_json failed")
        );
        let value: (i8, i16, i32, u8, u16, u32) =
            (i8::MIN, i16::MIN, i32::MIN, u8::MAX, u16::MAX, u32::MAX);
        assert_eq!(
            serialize(&value),
            serde_json::to_vec(&value).expect("serde_json failed")
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
    fn non_finite_float_key_is_an_error() {
        struct NanKey;
        impl Serialize for NanKey {
            fn serialize<S>(&self, s: S) -> core::result::Result<S::Ok, S::Error>
            where
                S: serde_core::Serializer,
            {
                use serde_core::ser::SerializeMap as _;
                let mut m = s.serialize_map(Some(1))?;
                m.serialize_entry(&f64::NAN, &1)?;
                m.end()
            }
        }
        let err = to_vec(&NanKey).expect_err("NaN key should fail");
        assert_eq!(err.to_string(), "float key must be finite");
    }

    #[test]
    fn collect_str_escapes_without_an_intermediate_string() {
        assert_eq!(serialize(&Collected), br#""a\"\n\\z""#);

        let map = BTreeMap::from([(Collected, 1)]);
        assert_eq!(serialize(&map), br#"{"a\"\n\\z":1}"#);
    }
}
