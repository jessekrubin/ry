use std::fs::read_dir;

use pyo3::IntoPyObjectExt;
use pyo3::exceptions::PyOSError;
use pyo3::prelude::*;
use ryo3_core::PyIoResultExt;
use ryo3_core::types::PathLike;
use ryo3_fspath::PyFsPath;

/// Change the current working directory to the specified path
#[pyfunction]
pub fn cd(path: PathLike) -> PyResult<()> {
    std::env::set_current_dir(&path).map_py_io_err_with_path(path)
}

// TODO: use `dtype` pattern for return type here
/// List the contents of the specified directory as a `Vec<String>`
#[pyfunction]
#[pyo3(signature = (path = None, *, absolute = false, sort = false, objects = false))]
pub fn ls(
    py: Python<'_>,
    path: Option<PathLike>,
    absolute: bool,
    sort: bool,
    objects: bool,
) -> PyResult<Vec<Bound<'_, PyAny>>> {
    let p = if let Some(p) = path {
        p
    } else {
        let pwd = pwd()?;
        PathLike::Str(pwd)
    };
    let entries = read_dir(&p).map_py_io_err_with_path(p)?;

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

#[pyfunction(
    signature = (path, *, exist_ok = false, recursive = false)
)]
pub fn mkdir(path: PathLike, exist_ok: bool, recursive: bool) -> PyResult<()> {
    let result = if recursive {
        std::fs::create_dir_all(&path)
    } else {
        std::fs::create_dir(&path)
    };
    match result {
        Err(e) if exist_ok && e.kind() == std::io::ErrorKind::AlreadyExists => Ok(()),
        res => res.map_py_io_err_with_path(path),
    }
}

#[pyfunction]
pub fn mkdirp(path: PathLike) -> PyResult<()> {
    std::fs::create_dir_all(&path).map_py_io_err_with_path(path)
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
    m.add_function(wrap_pyfunction!(mkdirp, m)?)?;
    m.add_function(wrap_pyfunction!(pwd, m)?)?;

    #[cfg(feature = "dirs")]
    m.add_function(wrap_pyfunction!(ryo3_dirs::home, m)?)?;

    Ok(())
}
