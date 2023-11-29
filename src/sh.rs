use std::path::{Path, PathBuf};

use dirs;
use pyo3::{FromPyObject, pyfunction, PyResult, wrap_pyfunction};
use pyo3::exceptions::PyFileNotFoundError;
use pyo3::prelude::*;
use pyo3::types::{PyModule, PyType};

#[pyclass(name = "Path")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PyPath {
    pth: PathBuf,
}


#[pymethods]
impl PyPath {
    #[new]
    fn new(p: PathLike) -> Self {
        Self {
            pth: p.as_ref().to_path_buf(),
        }
    }

    fn __repr__(&self) -> PyResult<String> {
        let s = serde_json::to_string(&self.pth).unwrap();
        Ok(s)
    }
    fn __str__(&self) -> PyResult<String> {
        self.__repr__()
    }

    fn __eq__(&self, other: PathLike) -> bool {
        // let other = other.extract::<PyPath>().unwrap();
        self.pth == other.as_ref()
    }

    fn __ne__(&self, other: PathLike) -> bool {
        // let other = other.extract::<PyPath>().unwrap();
        self.pth != other.as_ref()
    }

    fn is_file(&self) -> bool {
        self.pth.is_file()
    }

    fn is_dir(&self) -> bool {
        self.pth.is_dir()
    }

    fn is_symlink(&self) -> bool {
        self.pth.is_symlink()
    }

    fn is_absolute(&self) -> bool {
        self.pth.is_absolute()
    }

    fn is_relative(&self) -> bool {
        self.pth.is_relative()
    }

    fn exists(&self) -> bool {
        self.pth.exists()
    }

    fn __fspath__(&self) -> PyResult<String> {
        let s = self.pth.to_str().unwrap().to_string();
        Ok(s)
    }

    #[getter]
    fn parent(&self) -> PyResult<PyPath> {
        let p = self.pth.parent().unwrap();
        Ok(PyPath::new(PathLike::PathBuf(p.to_path_buf())))
    }

    #[getter]
    fn name(&self) -> PyResult<String> {
        let s = self.pth.file_name().unwrap().to_str().unwrap().to_string();
        Ok(s)
    }

    #[getter]
    fn stem(&self) -> PyResult<String> {
        let s = self.pth.file_stem().unwrap().to_str().unwrap().to_string();
        Ok(s)
    }

    #[classmethod]
    fn home(_cls: &PyType) -> PyResult<PyPath> {
        let p = dirs::home_dir().unwrap();
        Ok(PyPath::new(PathLike::PathBuf(p.to_path_buf())))
    }

    #[classmethod]
    fn cwd(_cls: &PyType) -> PyResult<PyPath> {
        let p = std::env::current_dir().unwrap();
        Ok(PyPath::new(PathLike::PathBuf(p.to_path_buf())))
    }
}


#[derive(Debug, FromPyObject, Clone)]
pub enum PathLike {
    PathBuf(std::path::PathBuf),
    // Path(std::path::Path),
    Str(String),
}

impl AsRef<Path> for PathLike {
    fn as_ref(&self) -> &Path {
        match self {
            PathLike::PathBuf(p) => p.as_ref(),
            PathLike::Str(s) => Path::new(s),
        }
    }
}


impl std::fmt::Display for PathLike {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PathLike::PathBuf(p) => write!(f, "{}", p.to_str().unwrap()),
            PathLike::Str(s) => write!(f, "{}", s),
        }
    }
}
// impl AsRef<PathBuf> for PathLike {
//     fn as_ref(&self) -> &PathBuf {
//         match self {
//             PathLike::PathBuf(p) => p,
//             PathLike::Str(s) => Path::new(s).to_path_buf().as_ref()
//         }
//     }
// }


///////////////////////////////////////////////
#[pyfunction]
pub fn home() -> String {
    dirs::home_dir().unwrap().to_str().unwrap().to_string()
}

#[pyfunction]
pub fn pwd() -> String {
    std::env::current_dir().unwrap().to_str().unwrap().to_string()
}

#[pyfunction]
pub fn cd(
    // py: Python<'_>,
    p: PathLike,
) -> PyResult<()> {
    let r = std::env::set_current_dir(p.as_ref());
    match r {
        Ok(_) => Ok(()),
        Err(e) => {
            let p_string = p.to_string();
            let emsg = format!("{}: {:?}", e.to_string(), p_string);
            let pye = PyFileNotFoundError::new_err(
                format!("cd: {}", emsg)
            );
            // pye.set_filename("cd");
            // pye.set_lineno(1);
            // pye.set_colno(1);
            // pye.set_function("cd");
            // pye.set_traceback(py, vec![]);

            Err(pye)
        }
    }
}


pub fn madd(m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(pwd, m)?)?;
    m.add_function(wrap_pyfunction!(cd, m)?)?;
    m.add_function(wrap_pyfunction!(home, m)?)?;
    m.add_class::<PyPath>()?;
    Ok(())
}
