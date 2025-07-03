use pyo3::exceptions::{PyFileNotFoundError, PyOSError};
use pyo3::prelude::*;
use pyo3::types::PyModule;
use pyo3::{IntoPyObjectExt, PyResult, pyfunction, wrap_pyfunction};
use ryo3_core::types::PathLike;
use ryo3_fspath::PyFsPath;
use std::fs::read_dir;

// TODO: revisit needless pass by value
/// Change the current working directory to the specified path
#[pyfunction]
#[expect(clippy::needless_pass_by_value)]
pub fn cd(p: PathLike) -> PyResult<()> {
    match std::env::set_current_dir(p.as_ref()) {
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
#[pyo3(signature = (fspath = None, *, absolute = false, sort = false, objects = false))]
pub fn ls(
    py: Python<'_>,
    fspath: Option<PathLike>,
    absolute: bool,
    sort: bool,
    objects: bool,
) -> PyResult<Vec<Bound<'_, PyAny>>> {
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

    let mut v = if absolute {
        entries
            .filter_map(Result::ok)
            .filter_map(|dir_entry| dir_entry.path().to_str().map(ToString::to_string))
            .collect::<Vec<String>>()
    } else {
        entries
            .filter_map(Result::ok)
            .filter_map(|dir_entry| {
                dir_entry
                    .path()
                    .file_name()
                    .and_then(|name| name.to_str().map(ToString::to_string))
            })
            .collect::<Vec<String>>()
    };
    if sort {
        v.sort();
    }
    if objects {
        let fspaths = v
            .into_iter()
            .flat_map(|s| PyFsPath::new(s).into_bound_py_any(py))
            // .map(PyObject::from)
            .collect();
        Ok(fspaths)
    } else {
        let strings = v
            .into_iter()
            .flat_map(|s| s.into_bound_py_any(py))
            .collect();
        Ok(strings)
    }
}

#[pyfunction]
#[expect(clippy::needless_pass_by_value)]
pub fn mkdir(path: PathLike) -> PyResult<String> {
    let path = path.as_ref();
    match std::fs::create_dir(path) {
        Ok(()) => Ok(path.to_string_lossy().to_string()),
        Err(e) => {
            let p_string = path.display();
            let emsg = format!("{e}: {p_string}");
            let pye = PyFileNotFoundError::new_err(format!("mkdir: {emsg}"));
            Err(pye)
        }
    }
}

#[pyfunction]
pub fn pwd() -> PyResult<String> {
    let current_dir = std::env::current_dir()?;
    match current_dir.to_str() {
        Some(s) => Ok(s.to_string()),
        None => Err(PyOSError::new_err(
            "pwd: current directory is not a valid UTF-8 string",
        )),
    }
}

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(cd, m)?)?;
    m.add_function(wrap_pyfunction!(ls, m)?)?;
    m.add_function(wrap_pyfunction!(mkdir, m)?)?;
    m.add_function(wrap_pyfunction!(pwd, m)?)?;

    #[cfg(feature = "dirs")]
    m.add_function(wrap_pyfunction!(ryo3_dirs::home, m)?)?;

    Ok(())
}
