#![deny(clippy::all)]
#![deny(clippy::correctness)]
#![deny(clippy::panic)]
#![deny(clippy::perf)]
#![deny(clippy::pedantic)]
#![deny(clippy::style)]
#![deny(clippy::unwrap_used)]
#![warn(clippy::must_use_candidate)]
#![expect(clippy::missing_errors_doc)]

pub mod libs;
mod reexports;

#[cfg(feature = "ry")]
pub mod ry;
pub use reexports::*;
