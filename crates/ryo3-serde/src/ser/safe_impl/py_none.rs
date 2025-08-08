use serde::ser::{Serialize, Serializer};

pub(crate) struct SerializePyNone {}

impl SerializePyNone {
    pub(crate) fn new() -> Self {
        Self {}
    }
}

impl Serialize for SerializePyNone {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_none()
    }
}
