#![doc = include_str!("../README.md")]
#![deny(clippy::all)]
#![deny(clippy::correctness)]
#![deny(clippy::panic)]
#![deny(clippy::perf)]
#![deny(clippy::pedantic)]
#![deny(clippy::style)]
#![deny(clippy::unwrap_used)]
#![warn(clippy::must_use_candidate)]
#![deny(clippy::missing_panics_doc)]
#![deny(clippy::arithmetic_side_effects)]
#![expect(clippy::missing_errors_doc)]
extern crate core;

mod api;
mod util;
mod civil;
mod constants;
mod deprecations;
mod dev;
mod difference;
mod from;
mod from_str;
mod functions;
mod interns;
mod isoformat;
mod jiff_types;
mod py_temporal_like;
#[cfg(feature = "pydantic")]
mod pydantic;
pub mod pydatetime_conversions;
mod round;
mod ry_date;
mod ry_datetime;
mod ry_iso_week_date;
mod ry_offset;
mod ry_signed_duration;
mod ry_span;
mod ry_time;
mod ry_timestamp;
mod ry_timezone;
mod ry_timezone_database;
mod ry_weekday;
mod ry_zoned;
#[cfg(feature = "serde")]
mod serde;
mod series;
mod spanish;
mod test;
mod tz;

pub use api::*;
pub use jiff_types::*;
