use crate::RyTimeZone;
use serde::Serialize;

impl Serialize for RyTimeZone {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        jiff::fmt::serde::tz::required::serialize(&self.0, serializer)
    }
}
