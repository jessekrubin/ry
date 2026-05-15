#![warn(clippy::must_use_candidate)]
// clippy
#![deny(clippy::all)]
#![deny(clippy::correctness)]
#![deny(clippy::panic)]
#![deny(clippy::pedantic)]
#![deny(clippy::perf)]
#![deny(clippy::style)]
#![deny(clippy::unwrap_used)]
#![expect(clippy::missing_errors_doc)]
#![expect(clippy::missing_panics_doc)]
#![expect(clippy::module_name_repetitions)]
#![expect(clippy::unnecessary_wraps)]
#![expect(clippy::unwrap_used)]
#![expect(unused_crate_dependencies)]
use pyo3::prelude::*;

pub mod anystr;
pub mod sp;
pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    anystr::pymod_add(m)?;
    sp::pymod_add(m)?;
    Ok(())
}
