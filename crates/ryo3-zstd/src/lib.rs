#![doc = include_str!("../README.md")]

mod api;
mod compression_level;
pub mod oneshot;
mod submodule;

pub use api::pymod_add;
pub use submodule::pysubmod_register;
