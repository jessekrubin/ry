#![doc = include_str!("../README.md")]

mod any_repr;
mod constants;
mod errors;
mod macro_rules;
mod py_serialize;
mod pytypes;
mod rytypes;
mod safe_impl;
mod type_cache;
pub(crate) use constants::{Depth, MAX_DEPTH};
pub use py_serialize::SerializePyAny;
