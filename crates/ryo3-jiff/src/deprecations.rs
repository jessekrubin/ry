use pyo3::intern;
use pyo3::types::PyTuple;
use pyo3::{exceptions::PyDeprecationWarning, prelude::*};

/// Raise Deprecation warning for `intz`
pub(crate) fn deprecation_warning_intz(py: Python) -> PyResult<()> {
    let warnings_mod = py.import(intern!(py, "warnings"))?;
    let warning = PyDeprecationWarning::new_err("`intz` deprecated use `in_tz` instead");
    let args = PyTuple::new(py, vec![warning])?;
    warnings_mod.call_method1(intern!(py, "warn"), args)?;

    Ok(())
}
