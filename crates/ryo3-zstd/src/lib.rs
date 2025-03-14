#![doc = include_str!("../README.md")]

mod api;
mod compression_level;
mod constants;
pub mod oneshot;

pub use api::{pymod_add, pysubmod_register};
