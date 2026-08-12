//! Serialize python objects
mod context;
pub(crate) mod dataclass;
pub(crate) mod py_serialize;
#[expect(clippy::inline_always)]
pub(crate) mod py_types;
mod ry_types;
mod target;

pub(crate) use context::PySerializeContext;
pub use py_serialize::PyAnySerializer;
pub use target::{JsonTarget, PySerializeTarget, SerdeTarget};
