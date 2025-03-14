#![doc = include_str!("../README.md")]
#![deny(clippy::all)]
#![deny(clippy::correctness)]
#![deny(clippy::panic)]
#![deny(clippy::perf)]
#![deny(clippy::pedantic)]
#![deny(clippy::style)]
#![deny(clippy::unwrap_used)]
#![warn(clippy::must_use_candidate)]
#![expect(clippy::missing_errors_doc)]

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

pub use api::*;
pub use jiff_types::*;
