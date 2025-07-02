use crate::{
    RyDate, RyDateTime, RySignedDuration, RySpan, RyTime, RyTimeZone, RyTimestamp, RyZoned,
};
use serde::Serialize;

impl Serialize for RyTimeZone {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        jiff::fmt::serde::tz::required::serialize(&self.0, serializer)
    }
}

// implement the deserialize macro for a ry-jiff-wrapper type to avoid the
// annoying clippy error messages
macro_rules! impl_deserialize {
    ($ry_type:ty, $jiff_type:ty) => {
        impl<'de> serde::Deserialize<'de> for $ry_type {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                let inner: $jiff_type = serde::Deserialize::deserialize(deserializer)?;
                Ok(Self::from(inner))
            }
        }
    };
}

impl_deserialize!(RyDate, jiff::civil::Date);
impl_deserialize!(RyDateTime, jiff::civil::DateTime);
impl_deserialize!(RySignedDuration, jiff::SignedDuration);
impl_deserialize!(RySpan, jiff::Span);
impl_deserialize!(RyTime, jiff::civil::Time);
impl_deserialize!(RyTimestamp, jiff::Timestamp);
impl_deserialize!(RyZoned, jiff::Zoned);
