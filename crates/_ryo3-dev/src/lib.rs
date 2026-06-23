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
#![expect(unused_crate_dependencies)]
use pyo3::prelude::*;

#[pyfunction]
#[must_use]
pub fn devfn() -> &'static str {
    "_ryo3-dev"
}

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(devfn, m)?)?;
    Ok(())
}
