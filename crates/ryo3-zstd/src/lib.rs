#![doc = include_str!("../README.md")]

mod api;
mod compression_level;
mod constants;
mod decompressor;
mod dict;
pub mod oneshot;

pub use api::{pymod_add, pysubmod_register};
