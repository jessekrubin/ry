#![doc = include_str!("../README.md")]

#[cfg(feature = "regex")]
mod which_regex;
#[cfg(feature = "regex")]
pub use which_regex::which_re;

use pyo3::types::{PyModule, PyModuleMethods};
use pyo3::{Bound, PyResult, pyfunction, wrap_pyfunction};

use std::env;
use std::ffi::OsString;
use std::path::PathBuf;

use ::which as which_rs;
use pyo3::prelude::*;

#[pyfunction]
#[pyo3(signature= (cmd, path=None))]
pub fn which(cmd: &str, path: Option<&str>) -> PyResult<Option<PathBuf>> {
    if let Some(p) = path {
        // get current directory w/o unwrapping
        match env::current_dir() {
            Ok(c) => {
                let which_res = ::which::which_in(cmd, Some(p), c);
                match which_res {
                    Ok(p) => Ok(Some(p)),
                    Err(_e) => Ok(None),
                }
            }
            Err(_e) => Err(PyErr::new::<pyo3::exceptions::PyOSError, _>(
                "which: current directory is not a valid path",
            )),
        }
    } else {
        let r = which_rs::which(cmd);
        match r {
            Ok(p) => Ok(Some(p)),
            Err(_e) => Ok(None),
        }
    }
}

#[pyfunction]
#[pyo3(signature= (cmd, path=None))]
pub fn which_all(cmd: &str, path: Option<&str>) -> PyResult<Vec<PathBuf>> {
    let search_path: Option<OsString> = match path {
        Some(p) => Some(OsString::from(p)),
        None => env::var_os("PATH"),
    };
    let curdir = env::current_dir()?;
    let which_iter = which_rs::which_in_all(cmd, search_path, curdir).map_err(|_| {
        PyErr::new::<pyo3::exceptions::PyOSError, _>(
            "which_all: current directory is not a valid path",
        )
    })?;
    let which_vec = which_iter.into_iter().collect::<Vec<_>>();
    Ok(which_vec)
}

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(self::which, m)?)?;
    m.add_function(wrap_pyfunction!(self::which_all, m)?)?;

    #[cfg(feature = "regex")]
    which_regex::pymod_add(m)?;
    Ok(())
}
