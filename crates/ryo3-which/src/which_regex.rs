use pyo3::prelude::*;
use pyo3::types::PyString;
use ryo3_regex::PyRegex;
use std::path::PathBuf;

fn extract_regex(regex: &Bound<PyAny>) -> PyResult<PyRegex> {
    if let Ok(regex) = regex.cast::<PyString>() {
        let regex = regex.to_str()?;
        PyRegex::try_from(regex)
    } else if let Ok(regex) = regex.extract::<PyRegex>() {
        Ok(regex)
    } else {
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "which-re: regex must be a string or a compiled regex",
        ))
    }
}

#[pyfunction]
#[pyo3(signature= (regex, path=None))]
pub fn which_re(regex: &Bound<'_, PyAny>, path: Option<&str>) -> PyResult<Vec<PathBuf>> {
    let regex = extract_regex(regex)?;
    if let Some(p) = path {
        match ::which::which_re_in(regex, Some(p)) {
            Ok(p) => {
                let which_vec = p.into_iter().collect::<Vec<PathBuf>>();
                Ok(which_vec)
            }
            Err(e) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                "which-re-err: {e}"
            ))),
        }
    } else {
        match ::which::which_re(regex) {
            Ok(p) => {
                let which_vec = p.into_iter().collect::<Vec<PathBuf>>();
                Ok(which_vec)
            }
            Err(_e) => Err(PyErr::new::<pyo3::exceptions::PyOSError, _>(
                "which: current directory is not a valid path",
            )),
        }
    }
}

pub(crate) fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(self::which_re, m)?)?;
    Ok(())
}
