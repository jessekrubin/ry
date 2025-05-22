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
#![cfg_attr(feature = "serde", expect(clippy::unsafe_derive_deserialize))]
extern crate core;

pub mod pydatetime_conversions;

mod dev;

mod api;
mod delta_arithmetic_self;
mod deprecations;
mod errors;
mod functions;
mod into_span_arithmetic;
mod jiff_types;
mod ry_date;
mod ry_date_difference;
mod ry_datetime;
mod ry_datetime_difference;
mod ry_datetime_round;
mod ry_iso_week_date;
mod ry_offset;
mod ry_signed_duration;
mod ry_span;
mod ry_time;
mod ry_time_difference;
mod ry_timestamp;
mod ry_timestamp_difference;
mod ry_timestamp_round;
mod ry_timezone;
mod ry_timezone_database;
mod ry_weekday;
mod ry_zoned;
mod ry_zoned_difference;
mod ry_zoned_round;
mod span_relative_to;
mod test;

pub use api::*;
pub use jiff_types::*;
