use serde::Serializer;

// TODO: rm expect; this is scuzz work so I shall leave it to non-fugue-state-jesse
#[expect(dead_code)]
pub(crate) trait PySerializeUnsafe {
    fn serialize_unsafe<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer;
}
