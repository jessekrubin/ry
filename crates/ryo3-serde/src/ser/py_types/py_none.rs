use serde::ser::{Serialize, Serializer};

pub(crate) struct PyNoneSerializer {}

impl PyNoneSerializer {
    pub(crate) fn new() -> Self {
        Self {}
    }
}

impl Serialize for PyNoneSerializer {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_none()
    }
}
