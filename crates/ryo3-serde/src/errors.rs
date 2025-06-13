use serde::ser::{Error as SerError, SerializeMap, SerializeSeq, Serializer};
use std::fmt;

pub(crate) fn map_py_err<I: fmt::Display, O: SerError>(err: I) -> O {
    O::custom(err.to_string())
}
