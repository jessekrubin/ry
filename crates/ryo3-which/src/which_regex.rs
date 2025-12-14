use pyo3::prelude::*;
use pyo3::types::PyString;
use ryo3_regex::PyRegex;
use std::path::PathBuf;

fn extract_regex(regex: &Bound<PyAny>) -> PyResult<PyRegex> {
    if let Ok(regex) = regex.cast::<PyString>() {
        let regex = regex.to_str()?;
        PyRegex::try_from(regex)
    } else if let Ok(regex) = regex.cast_exact::<PyRegex>() {
        // TODO: rethink cloning etc...
        Ok(regex.get().clone())
    } else {
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "which-re: regex must be a string or a compiled regex",
        ))
    }
}

#[pyfunction]
#[pyo3(signature= (regex, path=None))]
pub fn which_re(
    py: Python<'_>,
    regex: &Bound<'_, PyAny>,
    path: Option<&str>,
) -> PyResult<Vec<PathBuf>> {
    let regex = extract_regex(regex)?;
    if let Some(p) = path {
        // let do with detach
        py.detach(|| {
            ::which::which_re_in(regex, Some(p)).map(|p| p.into_iter().collect::<Vec<PathBuf>>())
        })
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("which-re-err: {e}")))
    } else {
        py.detach(|| ::which::which_re(regex).map(|p| p.into_iter().collect::<Vec<PathBuf>>()))
            .map_err(|e| {
                PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("which-re-err: {e}"))
            })
    }
}

pub(crate) fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(self::which_re, m)?)?;
    Ok(())
}
