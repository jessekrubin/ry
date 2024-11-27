mod nbytes;
pub use nbytes::nbytes_u64;
use pyo3::prelude::*;
use pyo3::types::PyModule;
use pyo3::{wrap_pyfunction, PyResult};
pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(nbytes::fmt_nbytes, m)?)?;
    Ok(())
}

// submopdule
#[pymodule(gil_used = false)]
#[pyo3(name = "fmts")]
pub fn pymod(m: &Bound<'_, PyModule>) -> PyResult<()> {
    pymod_add(m)
}
