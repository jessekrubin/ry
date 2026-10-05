//! json serialization error/result
use serde_core::ser::{self};

pub(crate) type Result<T> = core::result::Result<T, JsonSerError>;
#[derive(Debug)]
pub struct JsonSerError {
    kind: Box<JsonSerErrorKind>,
}

#[derive(Debug)]
enum JsonSerErrorKind {
    KeyMustBeString,
    FloatKeyMustBeFinite,
    Message(Box<str>),
}

impl JsonSerError {
    #[inline]
    pub(crate) fn key_must_be_string() -> Self {
        Self {
            kind: Box::new(JsonSerErrorKind::KeyMustBeString),
        }
    }

    #[inline]
    pub(crate) fn float_key_must_be_finite() -> Self {
        Self {
            kind: Box::new(JsonSerErrorKind::FloatKeyMustBeFinite),
        }
    }
}

impl ser::Error for JsonSerError {
    fn custom<T: std::fmt::Display>(msg: T) -> Self {
        Self {
            kind: Box::new(JsonSerErrorKind::Message(msg.to_string().into())),
        }
    }
}

impl std::fmt::Display for JsonSerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.kind.as_ref() {
            JsonSerErrorKind::KeyMustBeString => f.write_str("key must be a string"),
            JsonSerErrorKind::FloatKeyMustBeFinite => f.write_str("float key must be finite"),
            JsonSerErrorKind::Message(msg) => f.write_str(msg.as_ref()),
        }
    }
}

impl core::error::Error for JsonSerError {}

#[cfg(test)]
mod tests {
    use core::mem::size_of;

    use super::*;

    #[test]
    fn error_is_pointer_sized() {
        assert_eq!(size_of::<JsonSerError>(), size_of::<usize>());
        assert_eq!(size_of::<super::Result<()>>(), size_of::<usize>());
    }
}
