use pyo3::prelude::*;
use ryo3_regex::PyRegex;
use std::path::PathBuf;

#[pyfunction]
#[pyo3(signature= (regex, path=None))]
pub fn which_re(regex: PyRegex, path: Option<&str>) -> PyResult<Vec<PathBuf>> {
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
