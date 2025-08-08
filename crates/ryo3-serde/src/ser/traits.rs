use serde::Serializer;

pub(crate) trait PySerializeUnsafe {
    fn serialize_unsafe<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer;
}
