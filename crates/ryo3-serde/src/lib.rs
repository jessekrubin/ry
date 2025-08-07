#![doc = include_str!("../README.md")]

mod any_repr;
mod constants;
mod errors;
mod macro_rules;
mod py_serialize;
mod rytypes;
mod safe_impl;
pub mod ser;
mod type_cache;

pub(crate) use constants::{Depth, MAX_DEPTH};
pub use py_serialize::SerializePyAny;
