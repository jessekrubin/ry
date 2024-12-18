#![deny(clippy::all)]
#![deny(clippy::correctness)]
#![deny(clippy::panic)]
#![deny(clippy::perf)]
#![deny(clippy::pedantic)]
#![deny(clippy::style)]
#![deny(clippy::unwrap_used)]
#![warn(clippy::must_use_candidate)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::unnecessary_wraps)]
#![allow(clippy::needless_pass_by_value)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::unused_self)]

use pyo3::prelude::*;
use time::duration;

pub mod fs;
pub mod time;

pub use fs::*;
pub use time::duration::PyDuration;
pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    duration::pymod_add(m)?;
    fs::pymod_add(m)?;
    Ok(())
}
