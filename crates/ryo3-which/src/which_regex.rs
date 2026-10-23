use std::path::PathBuf;

use pyo3::prelude::*;
use pyo3::types::PyString;
use ryo3_regex::{PyRegex, Regex};

pub struct PyArgWhichRegex(Regex);

impl FromPyObject<'_, '_> for PyArgWhichRegex {
    type Error = PyErr;

    fn extract(obj: Borrowed<'_, '_, PyAny>) -> PyResult<Self> {
        if let Ok(regex) = obj.cast::<PyString>() {
            let r = PyRegex::try_from(regex.to_str()?)?;
            Ok(Self(r.re))
        } else if let Ok(regex) = obj.cast_exact::<PyRegex>() {
            Ok(Self(regex.get().re.clone()))
        } else {
            const EMSG: &str = "which-re: regex must be a string or a compiled regex";
            Err(pyo3::exceptions::PyTypeError::new_err(EMSG))
        }
    }
}

#[pyfunction]
#[pyo3(signature = (regex, path = None))]
pub fn which_re(
    py: Python<'_>,
    regex: PyArgWhichRegex,
    path: Option<&str>,
) -> PyResult<Vec<PathBuf>> {
    let regex = regex.0;
    py.detach(move || match path {
        Some(p) => ::which::which_re_in(regex, Some(p)).map(Iterator::collect),
        None => ::which::which_re(regex).map(Iterator::collect),
    })
    .map_err(|e| pyo3::exceptions::PyValueError::new_err(format!("which-re-err: {e}")))
}

pub(crate) fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(self::which_re, m)?)?;
    Ok(())
}
