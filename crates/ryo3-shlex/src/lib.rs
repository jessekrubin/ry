// #![deny(clippy::all)]
// #![deny(clippy::correctness)]
// #![deny(clippy::panic)]
// #![deny(clippy::perf)]
// #![deny(clippy::pedantic)]
// #![deny(clippy::style)]
// #![deny(clippy::unwrap_used)]
// #![warn(clippy::must_use_candidate)]
// #![allow(clippy::missing_errors_doc)]
// #![allow(clippy::missing_panics_doc)]
// #![allow(clippy::unnecessary_wraps)]
// #![allow(clippy::needless_pass_by_value)]
// #![allow(clippy::module_name_repetitions)]
// #![allow(clippy::unused_self)]

use pyo3::types::{PyModule, PyModuleMethods};
use pyo3::{pyfunction, wrap_pyfunction, Bound, PyResult};

#[pyfunction(signature = (string), text_signature = "(string: str) -> list[str] | None")]
#[must_use]
pub fn shplit(string: &str) -> Option<Vec<String>> {
    shlex::split(string)
}

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(shplit, m)?)?;
    Ok(())
}
