#![doc = include_str!("../README.md")]
mod error;
pub mod escape;
mod format;
mod serializer;
mod writer;

#[cfg(test)]
use criterion as _; // benches only
pub use error::JsonSerError;
pub(crate) use error::Result;
pub use format::{JsonFormat, JsonFormatCompact, JsonFormatPretty};
pub use serializer::{Serializer, to_string, to_string_pretty, to_vec, to_vec_pretty};
pub(crate) use writer::JsonWriter;
