use crate::http_types::{HttpHeaderMap, HttpHeaderNameRef};
use crate::{PyHeaders, PyHttpStatus};
use http::{HeaderMap, HeaderValue};
use serde::ser::SerializeSeq;
use serde::{Deserializer, de};
use std::fmt;

impl<'de> serde::Deserialize<'de> for PyHttpStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let code = u16::deserialize(deserializer)?;
        Self::py_new(code).map_err(serde::de::Error::custom)
    }
}

impl serde::Serialize for PyHttpStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u16(self.0.as_u16())
    }
}

// ============================================================================
// HEADERS-MAP serde impls
// ============================================================================

impl serde::Serialize for PyHeaders {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let header_map = self.0.lock();
        let header_map_ref = HttpHeaderMapRef(&header_map);
        header_map_ref.serialize(serializer)
    }
}

#[derive(Debug)]
pub(crate) struct HeaderValuesRef<'a>(http::header::GetAll<'a, HeaderValue>);

impl serde::Serialize for HeaderValuesRef<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let values = self.0.iter().collect::<Vec<_>>();
        if values.len() == 1 {
            let value = values[0];
            if let Ok(s) = value.to_str() {
                serializer.serialize_str(s)
            } else {
                let bytes = value.as_bytes();
                serializer.serialize_bytes(bytes)
            }
        } else {
            let mut seq = serializer.serialize_seq(Some(values.len()))?;
            for value in &values {
                if let Ok(s) = value.to_str() {
                    seq.serialize_element(s)?;
                } else {
                    seq.serialize_element(value.as_bytes())?;
                }
            }
            seq.end()
        }
    }
}

impl serde::Serialize for HttpHeaderNameRef<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.0.as_str())
    }
}

#[derive(Debug)]
pub(crate) struct HttpHeaderMapRef<'a>(pub &'a HeaderMap);
impl serde::Serialize for HttpHeaderMapRef<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut keys = self.0.keys().collect::<Vec<_>>();
        keys.sort_by_cached_key(|k| k.as_str().to_lowercase());

        serializer.collect_map(
            keys.into_iter()
                .map(|k| (HttpHeaderNameRef(k), HeaderValuesRef(self.0.get_all(k)))),
        )
    }
}

#[derive(serde::Deserialize)]
#[serde(untagged)]
enum HeaderValuesDe<T> {
    One(T),
    Many(Vec<T>),
}

struct HeaderMapVisitor;

impl<'de> de::Visitor<'de> for HeaderMapVisitor {
    type Value = HeaderMap;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "a map of HTTP header names to string values")
    }

    fn visit_map<A>(self, mut map: A) -> Result<HeaderMap, A::Error>
    where
        A: de::MapAccess<'de>,
    {
        let mut header_map = HeaderMap::with_capacity(map.size_hint().unwrap_or(0));

        while let Some((key, value)) = map.next_entry::<String, HeaderValuesDe<String>>()? {
            let name = http::HeaderName::from_bytes(key.as_bytes())
                .map_err(|e| de::Error::custom(format!("invalid header name '{key}': {e}")))?;

            match value {
                HeaderValuesDe::One(value) => {
                    let val = HeaderValue::from_str(&value).map_err(|e| {
                        de::Error::custom(format!("invalid header value for '{key}': {e}"))
                    })?;
                    header_map.insert(name, val);
                }
                HeaderValuesDe::Many(values) => {
                    for value in values {
                        let val = HeaderValue::from_str(&value).map_err(|e| {
                            de::Error::custom(format!("invalid header value for '{key}': {e}"))
                        })?;
                        header_map.append(name.clone(), val);
                    }
                }
            }
        }
        Ok(header_map)
    }
}

impl<'de> serde::Deserialize<'de> for HttpHeaderMap {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer
            .deserialize_map(HeaderMapVisitor)
            .map(std::convert::Into::into)
    }
}
