use std::env;
use std::ffi::OsString;

use ::which as which_rs;
use pyo3::prelude::*;

#[pyfunction]
#[pyo3(signature= (cmd, path=None))]
pub fn which(cmd: &str, path: Option<&str>) -> PyResult<Option<std::path::PathBuf>> {
    if let Some(p) = path {
        // get current directory w/o unwrapping
        match env::current_dir() {
            Ok(c) => {
                let which_res = which_rs::which_in(cmd, Some(p), c);
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
pub fn which_all(cmd: &str, path: Option<&str>) -> PyResult<Vec<String>> {
    let search_path: Option<OsString> = match path {
        Some(p) => Some(OsString::from(p)),
        None => env::var_os("PATH"),
    };
    let which_iter = which_rs::which_in_all(
        cmd,
        search_path,
        env::current_dir().expect("which_all: current directory is not a valid path"),
    )
    .map_err(|_| {
        PyErr::new::<pyo3::exceptions::PyOSError, _>(
            "which_all: current directory is not a valid path",
        )
    })?;
    let which_vec = which_iter
        .into_iter()
        .map(|p| {
            p.to_str()
                .expect("which_all: path contains invalid unicode characters")
                .to_string()
        })
        .collect::<Vec<String>>();
    Ok(which_vec)
}

#[pyfunction]
#[pyo3(signature= (cmd, path=None))]
pub fn whicha(cmd: &str, path: Option<&str>) -> PyResult<Vec<String>> {
    which_all(cmd, path)
}

pub fn madd(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(self::which, m)?)?;
    m.add_function(wrap_pyfunction!(self::which_all, m)?)?;
    m.add_function(wrap_pyfunction!(self::whicha, m)?)?;
    Ok(())
}
