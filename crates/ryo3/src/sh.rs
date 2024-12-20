use pyo3::exceptions::{PyFileNotFoundError, PyOSError};
use pyo3::prelude::*;
use pyo3::types::PyModule;
use pyo3::{pyfunction, wrap_pyfunction, PyResult};
use ryo3_types::PathLike;
use std::fs::read_dir;

// #[pyfunction]
// pub fn home() -> PyResult<String> {
//     match dirs::home_dir() {
//         Some(x) => match x.to_str() {
//             Some(s) => Ok(s.to_string()),
//             None => Err(PyOSError::new_err(
//                 "home: home directory is not a valid UTF-8 string",
//             )),
//         },
//         None => Err(PyOSError::new_err(
//             "home: could not determine home directory",
//         )),
//     }
// }

#[pyfunction]
pub fn pwd() -> PyResult<String> {
    let curdur = std::env::current_dir()?;
    match curdur.to_str() {
        Some(s) => Ok(s.to_string()),
        None => Err(PyOSError::new_err(
            "pwd: current directory is not a valid UTF-8 string",
        )),
    }
}

// TODO: revisit needless pass by value
/// Change the current working directory to the specified path
#[pyfunction]
#[allow(clippy::needless_pass_by_value)]
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

/// List the contents of the specified directory as a `Vec<String>`
#[pyfunction]
#[pyo3(signature = (fspath = None, *, sort = None))]
pub fn ls(fspath: Option<PathLike>, sort: Option<bool>) -> PyResult<Vec<String>> {
    let p = if let Some(p) = fspath {
        p
    } else {
        let pwd = pwd()?;
        PathLike::Str(pwd)
    };
    let entries = read_dir(p.as_ref()).map_err(|e| {
        let p_string = format!("{p:?}");
        let emsg = format!("{e}: {p_string}");
        PyFileNotFoundError::new_err(format!("ls: {emsg}"))
    })?;

    let mut v = entries
        .filter_map(Result::ok)
        .filter_map(|dir_entry| {
            dir_entry
                .path()
                .file_name()
                .and_then(|name| name.to_str().map(ToString::to_string))
        })
        .collect::<Vec<String>>();
    if let Some(true) = sort {
        v.sort();
    }
    Ok(v)
}

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(pwd, m)?)?;
    m.add_function(wrap_pyfunction!(cd, m)?)?;
    m.add_function(wrap_pyfunction!(ls, m)?)?;

    #[cfg(feature = "dirs")]
    m.add_function(wrap_pyfunction!(ryo3_dirs::home, m)?)?;

    Ok(())
}
