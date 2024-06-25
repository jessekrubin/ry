use crate::fs::fspath::PathLike;
use dirs;
use pyo3::exceptions::{PyFileNotFoundError, PyOSError};
use pyo3::prelude::*;
use pyo3::types::PyModule;
use pyo3::{pyfunction, wrap_pyfunction, PyResult};
use std::fs::read_dir;

#[pyfunction]
pub fn home() -> PyResult<String> {
    match dirs::home_dir() {
        Some(x) => match x.to_str() {
            Some(s) => Ok(s.to_string()),
            None => Err(PyOSError::new_err(
                "home: home directory is not a valid UTF-8 string",
            )),
        },
        None => Err(PyOSError::new_err(
            "home: could not determine home directory",
        )),
    }
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
        Err(e) => Err(PyOSError::new_err(format!("pwd: {e}"))),
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
#[pyo3(signature = (fspath = None))]
pub fn ls(fspath: Option<PathLike>) -> PyResult<Vec<String>> {
    let p = if let Some(p) = fspath {
        p
    } else {
        let pwd = pwd()?;
        PathLike::Str(pwd)
    };
    // let r = std::fs::read_dir(p.as_ref());

    let entries = read_dir(p.as_ref()).map_err(|e| {
        let p_string = format!("{p:?}");
        let emsg = format!("{e}: {p_string}");
        PyFileNotFoundError::new_err(format!("ls: {emsg}"))
    })?;

    let v: Vec<String> = entries
        .filter_map(Result::ok)
        .filter_map(|dir_entry| {
            dir_entry
                .path()
                .file_name()
                .and_then(|name| name.to_str().map(ToString::to_string))
        })
        .collect();
    Ok(v)
}

pub fn madd(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(pwd, m)?)?;
    m.add_function(wrap_pyfunction!(cd, m)?)?;
    m.add_function(wrap_pyfunction!(home, m)?)?;
    m.add_function(wrap_pyfunction!(ls, m)?)?;
    Ok(())
}
