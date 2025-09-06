#![doc = include_str!("../README.md")]

mod any_repr;
mod constants;
mod errors;
mod macro_rules;

mod ob_type;
mod ob_type_cache;
pub mod ser;

pub(crate) use constants::{Depth, MAX_DEPTH};
pub use ser::py_serialize::SerializePyAny;
