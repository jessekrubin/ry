use dirs;
use pyo3::exceptions::{PyFileNotFoundError, PyOSError};
use pyo3::prelude::*;
use pyo3::types::PyModule;
use pyo3::{pyfunction, wrap_pyfunction, PyResult};

use crate::fs::fspath::PathLike;

#[pyfunction]
#[must_use]
pub fn home() -> String {
    dirs::home_dir().unwrap().to_str().unwrap().to_string()
}

#[pyfunction]
pub fn pwd() -> PyResult<String> {
    let curdur = std::env::current_dir();
    match curdur {
        Ok(c) => match c.to_str() {
            Some(s) => Ok(s.to_string()),
            None => Err(PyOSError::new_err(
                "pwd: current directory is not a valid UTF-8 string",
            )),
        },
        Err(e) => {
            Err(PyOSError::new_err(format!("pwd: {e}")))
            // let emsg = format!("{e}");
            // let pye = PyFileNotFoundError::new_err(format!("pwd: {emsg}"));
            // Err(pye)
        }
    }
}

/// Change the current working directory to the specified path
#[pyfunction]
pub fn cd(p: PathLike) -> PyResult<()> {
    let r = std::env::set_current_dir(p.as_ref());
    match r {
        Ok(()) => Ok(()),
        Err(e) => {
            let p_string = p.to_string();
            let emsg = format!("{e}: {p_string:?}");
            let pye = PyFileNotFoundError::new_err(format!("cd: {emsg}"));
            Err(pye)
        }
    }
}

/// List the contents of the specified directory as a Vec<String>
#[pyfunction]
pub fn ls(fspath: Option<PathLike>) -> PyResult<Vec<String>> {
    let p = fspath.unwrap_or_else(|| PathLike::PathBuf(std::env::current_dir().unwrap()));
    let r = std::fs::read_dir(p.as_ref());
    match r {
        Ok(r) => {
            let v = r
                .map(|x| x.unwrap())
                .map(|y| y.path())
                .map(|z| z.file_name().unwrap().to_str().unwrap().to_string())
                .collect();
            Ok(v)
        }
        Err(e) => {
            let p_string = String::from(p);
            let emsg = format!("{e}: {p_string:?}");
            let pye = PyFileNotFoundError::new_err(format!("ls: {emsg}"));
            Err(pye)
        }
    }
}

pub fn madd(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(pwd, m)?)?;
    m.add_function(wrap_pyfunction!(cd, m)?)?;
    m.add_function(wrap_pyfunction!(home, m)?)?;
    m.add_function(wrap_pyfunction!(ls, m)?)?;
    Ok(())
}
