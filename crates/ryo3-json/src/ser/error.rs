//! json serialzation error/result
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
    Recursion,
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

    /// `true` if the error is `ryo3_serde`'s recursion err msg
    #[inline]
    pub(crate) fn is_recursion(&self) -> bool {
        matches!(self.kind.as_ref(), JsonSerErrorKind::Recursion)
    }
}

impl ser::Error for JsonSerError {
    fn custom<T: std::fmt::Display>(msg: T) -> Self {
        let msg = msg.to_string();
        let kind = if msg == ryo3_serde::RECURSION_ERR_MSG {
            JsonSerErrorKind::Recursion
        } else {
            JsonSerErrorKind::Message(Box::from(msg))
        };
        Self {
            kind: Box::new(kind),
        }
    }
}

impl std::fmt::Display for JsonSerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.kind.as_ref() {
            JsonSerErrorKind::KeyMustBeString => f.write_str("key must be a string"),
            JsonSerErrorKind::FloatKeyMustBeFinite => f.write_str("float key must be finite"),
            JsonSerErrorKind::Recursion => f.write_str(ryo3_serde::RECURSION_ERR_MSG),
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
    fn recursion_message_becomes_recursion_error() {
        use serde_core::ser::Error as _;

        let err = JsonSerError::custom(ryo3_serde::RECURSION_ERR_MSG);
        assert!(err.is_recursion());
        assert_eq!(err.to_string(), ryo3_serde::RECURSION_ERR_MSG);

        let err = JsonSerError::custom("recursion is fun");
        assert!(!err.is_recursion());
    }

    #[test]
    fn error_is_pointer_sized() {
        assert_eq!(size_of::<JsonSerError>(), size_of::<usize>());
        assert_eq!(size_of::<super::Result<()>>(), size_of::<usize>());
    }
}
