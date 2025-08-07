#![doc = include_str!("../README.md")]

mod any_repr;
mod constants;
mod errors;
mod macro_rules;

pub mod ser;
mod type_cache;

pub(crate) use constants::{Depth, MAX_DEPTH};
pub use ser::py_serialize::SerializePyAny;
