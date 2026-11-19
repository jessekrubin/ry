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
use pyo3::prelude::*;
use pyo3::types::PyInt;
use ryo3_core::PyIntRead;

#[pyfunction]
#[must_use]
pub fn devfn() -> &'static str {
    "_ryo3-dev"
}

/// `PyIntRead::read_i128` ~ for testing the inline int reading
#[pyfunction]
#[must_use]
pub fn pyint_read_i128(ob: &Bound<'_, PyInt>) -> Option<i128> {
    ob.read_i128()
}

/// `PyIntRead::read_u128` ~ for testing the inline int reading
#[pyfunction]
#[must_use]
pub fn pyint_read_u128(ob: &Bound<'_, PyInt>) -> Option<u128> {
    ob.read_u128()
}

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(devfn, m)?)?;
    m.add_function(wrap_pyfunction!(pyint_read_i128, m)?)?;
    m.add_function(wrap_pyfunction!(pyint_read_u128, m)?)?;
    Ok(())
}
