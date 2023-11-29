use pyo3::types::PyType;
use pyo3::{pyclass, pymethods, FromPyObject, PyResult};
use std::path::{Path, PathBuf};

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
