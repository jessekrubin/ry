use pyo3::types::PyDict;
use pyo3::{intern, prelude::*};
use ryo3_macro_rules::py_value_err;

#[pyclass(name = "FileType", frozen)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct PyFileType(FileTypeInner);

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
enum FileTypeInner {
    FileType(std::fs::FileType),
    FauxFileType(FauxFileType),
}

trait FileTypeMethods {
    fn is_dir(&self) -> bool;
    fn is_file(&self) -> bool;
    fn is_symlink(&self) -> bool;
}

impl FileTypeMethods for FauxFileType {
    fn is_dir(&self) -> bool {
        matches!(self, Self::Dir)
    }
    fn is_file(&self) -> bool {
        matches!(self, Self::File)
    }
    fn is_symlink(&self) -> bool {
        matches!(self, Self::Symlink)
    }
}

impl FileTypeMethods for FileTypeInner {
    fn is_dir(&self) -> bool {
        match self {
            Self::FileType(ft) => ft.is_dir(),
            Self::FauxFileType(ft) => ft.is_dir(),
        }
    }
    fn is_file(&self) -> bool {
        match self {
            Self::FileType(ft) => ft.is_file(),
            Self::FauxFileType(ft) => ft.is_file(),
        }
    }
    fn is_symlink(&self) -> bool {
        match self {
            Self::FileType(ft) => ft.is_symlink(),
            Self::FauxFileType(ft) => ft.is_symlink(),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub(crate) enum FauxFileType {
    Dir,
    File,
    Symlink,
}

impl PyFileType {
    #[must_use]
    pub fn new(ft: std::fs::FileType) -> Self {
        Self(FileTypeInner::FileType(ft))
    }
}

impl From<std::fs::FileType> for PyFileType {
    fn from(ft: std::fs::FileType) -> Self {
        Self(FileTypeInner::FileType(ft))
    }
}

#[expect(clippy::trivially_copy_pass_by_ref)]
#[pymethods]
impl PyFileType {
    #[new]
    fn py_new(t: &str) -> PyResult<Self> {
        let ft = match t {
            "d" | "dir" | "directory" => FauxFileType::Dir,
            "f" | "file" => FauxFileType::File,
            "s" | "symlink" | "link" => FauxFileType::Symlink,
            other => {
                return py_value_err!(
                    "invalid file type string: {other} - must be one of 'd'/'dir'/'directory', 'f'/'file', 's'/'symlink'/'link'"
                );
            }
        };
        Ok(Self(FileTypeInner::FauxFileType(ft)))
    }

    fn __str__<'py>(&self, py: Python<'py>) -> &'py Bound<'py, pyo3::types::PyString> {
        if self.0.is_dir() {
            pyo3::intern!(py, "dir")
        } else if self.0.is_file() {
            pyo3::intern!(py, "file")
        } else {
            pyo3::intern!(py, "symlink")
        }
    }

    fn __repr__<'py>(&self, py: Python<'py>) -> &'py Bound<'py, pyo3::types::PyString> {
        if self.0.is_dir() {
            pyo3::intern!(py, "FileType(\"dir\")")
        } else if self.0.is_file() {
            pyo3::intern!(py, "FileType(\"file\")")
        } else {
            pyo3::intern!(py, "FileType(\"symlink\")")
        }
    }

    #[getter]
    #[must_use]
    fn is_dir(&self) -> bool {
        self.0.is_dir()
    }

    #[getter]
    #[must_use]
    fn is_file(&self) -> bool {
        self.0.is_file()
    }

    #[getter]
    #[must_use]
    fn is_symlink(&self) -> bool {
        self.0.is_symlink()
    }

    #[expect(clippy::wrong_self_convention, clippy::trivially_copy_pass_by_ref)]
    fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        let file_type_dict = PyDict::new(py);
        file_type_dict.set_item(intern!(py, "is_dir"), self.is_dir())?;
        file_type_dict.set_item(intern!(py, "is_file"), self.is_file())?;
        file_type_dict.set_item(intern!(py, "is_symlink"), self.is_symlink())?;
        Ok(file_type_dict)
    }

    #[expect(clippy::wrong_self_convention, clippy::trivially_copy_pass_by_ref)]
    pub(crate) fn to_py<'py>(&self, py: Python<'py>) -> &'py Bound<'py, pyo3::types::PyString> {
        self.__str__(py)
    }
}
