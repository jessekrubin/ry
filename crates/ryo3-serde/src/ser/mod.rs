//! Serialize python objects
//!
//! Currently pre-reorg
mod context;
pub(crate) mod dataclass;
pub(crate) mod py_serialize;
mod rytypes;
#[expect(clippy::inline_always)]
pub(crate) mod safe_impl;
mod traits;

pub(crate) use context::PySerializeContext;
pub use py_serialize::SerializePyAny;
