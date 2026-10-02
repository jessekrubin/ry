//! JSON serializer (serde) writing to a `Vec<u8>`; no pyo3 in here.
mod error;
mod escape;
mod format;
mod serializer;
mod writer;

pub(crate) use error::{JsonSerError, Result};
pub(crate) use format::{JsonFormat, JsonFormatCompact, JsonFormatPretty};
pub(crate) use serializer::Serializer;
pub use serializer::{to_string, to_string_pretty, to_vec, to_vec_pretty};
pub(crate) use writer::JsonWriter;
