#![deny(clippy::all)]
#![deny(clippy::correctness)]
#![deny(clippy::panic)]
#![deny(clippy::perf)]
#![deny(clippy::pedantic)]
#![deny(clippy::style)]
#![deny(clippy::unwrap_used)]
#![warn(clippy::must_use_candidate)]
#![expect(clippy::missing_errors_doc)]
#![expect(clippy::missing_panics_doc)]
#![expect(clippy::unnecessary_wraps)]
#![expect(clippy::module_name_repetitions)]
#![expect(clippy::unwrap_used)]

use pyo3::PyResult;
use pyo3::prelude::*;
use pyo3::types::PyModule;

pub mod anystr;
pub mod sp;
pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    anystr::pymod_add(m)?;
    sp::pymod_add(m)?;
    Ok(())
}
