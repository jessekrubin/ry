use pyo3::prelude::*;
use pyo3::sync::PyOnceLock;
use ryo3_core::PyFromStr;
use ryo3_macro_rules::py_value_err;

static PY_FILE_TYPE_CACHE: [PyOnceLock<Py<PyFileType>>; 10] = [const { PyOnceLock::new() }; 10];

fn py_file_type_cached(py: Python<'_>, file_type: FileTypeKind) -> PyResult<Py<PyFileType>> {
    let ix = file_type as usize;
    PY_FILE_TYPE_CACHE[ix]
        .get_or_try_init(py, || {
            Py::new(py, PyFileType(FileTypeInner::Faux(file_type)))
        })
        .map(|obj| obj.clone_ref(py))
}

#[pyclass(name = "FileType", frozen, immutable_type, skip_from_py_object)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
#[derive(Copy, Clone, Hash)]
pub struct PyFileType(FileTypeInner);

#[derive(Copy, Clone, Hash)]
enum FileTypeInner {
    Std(std::fs::FileType),
    Faux(FileTypeKind),
}

impl FileTypeInner {
    #[inline]
    fn kind(self) -> FileTypeKind {
        match self {
            Self::Std(ft) => FileTypeKind::from_std(ft),
            Self::Faux(kind) => kind,
        }
    }
}

trait FileTypeMethods {
    fn is_dir(&self) -> bool;
    fn is_file(&self) -> bool;
    fn is_symlink(&self) -> bool;
    fn is_block_device(&self) -> bool;
    fn is_char_device(&self) -> bool;
    fn is_fifo(&self) -> bool;
    fn is_socket(&self) -> bool;
    fn is_symlink_dir(&self) -> bool;
    fn is_symlink_file(&self) -> bool;
    fn is_unknown(&self) -> bool;
}

impl FileTypeMethods for FileTypeInner {
    fn is_dir(&self) -> bool {
        self.kind() == FileTypeKind::Dir
    }
    fn is_file(&self) -> bool {
        self.kind() == FileTypeKind::File
    }
    fn is_symlink(&self) -> bool {
        self.kind() == FileTypeKind::Symlink
    }
    fn is_block_device(&self) -> bool {
        self.kind() == FileTypeKind::BlockDevice
    }
    fn is_char_device(&self) -> bool {
        self.kind() == FileTypeKind::CharDevice
    }
    fn is_fifo(&self) -> bool {
        self.kind() == FileTypeKind::Fifo
    }
    fn is_socket(&self) -> bool {
        self.kind() == FileTypeKind::Socket
    }
    fn is_symlink_dir(&self) -> bool {
        self.kind() == FileTypeKind::SymlinkDir
    }
    fn is_symlink_file(&self) -> bool {
        self.kind() == FileTypeKind::SymlinkFile
    }
    fn is_unknown(&self) -> bool {
        self.kind() == FileTypeKind::Unknown
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum FileTypeKind {
    Dir = 0,
    File = 1,
    Symlink = 2,
    BlockDevice = 3,
    CharDevice = 4,
    Fifo = 5,
    Socket = 6,
    SymlinkDir = 7,
    SymlinkFile = 8,
    Unknown = 9,
}

impl FileTypeKind {
    #[cfg(unix)]
    fn from_std(ft: std::fs::FileType) -> Self {
        use std::os::unix::fs::FileTypeExt;
        if ft.is_dir() {
            Self::Dir
        } else if ft.is_file() {
            Self::File
        } else if ft.is_symlink() {
            Self::Symlink
        } else if ft.is_block_device() {
            Self::BlockDevice
        } else if ft.is_char_device() {
            Self::CharDevice
        } else if ft.is_fifo() {
            Self::Fifo
        } else if ft.is_socket() {
            Self::Socket
        } else {
            Self::Unknown
        }
    }

    #[cfg(windows)]
    fn from_std(ft: std::fs::FileType) -> Self {
        use std::os::windows::fs::FileTypeExt;
        if ft.is_dir() {
            Self::Dir
        } else if ft.is_file() {
            Self::File
        } else if ft.is_symlink() {
            Self::Symlink
        } else if ft.is_symlink_dir() {
            Self::SymlinkDir
        } else if ft.is_symlink_file() {
            Self::SymlinkFile
        } else {
            Self::Unknown
        }
    }

    #[cfg(not(any(unix, windows)))]
    fn from_std(ft: std::fs::FileType) -> Self {
        if ft.is_dir() {
            Self::Dir
        } else if ft.is_file() {
            Self::File
        } else if ft.is_symlink() {
            Self::Symlink
        } else {
            Self::Unknown
        }
    }
}

impl PyFileType {
    #[must_use]
    pub fn new(ft: std::fs::FileType) -> Self {
        Self::from(ft)
    }
}

impl From<std::fs::FileType> for PyFileType {
    fn from(ft: std::fs::FileType) -> Self {
        Self(FileTypeInner::Std(ft))
    }
}

#[expect(clippy::trivially_copy_pass_by_ref)]
#[pymethods]
impl PyFileType {
    #[new]
    fn py_new(py: Python<'_>, t: &str) -> PyResult<Py<Self>> {
        let ft = FileTypeKind::py_from_str(t)?;
        py_file_type_cached(py, ft)
    }

    fn __str__<'py>(&self, py: Python<'py>) -> &'py Bound<'py, pyo3::types::PyString> {
        match self.0.kind() {
            FileTypeKind::Dir => pyo3::intern!(py, "dir"),
            FileTypeKind::File => pyo3::intern!(py, "file"),
            FileTypeKind::Symlink => pyo3::intern!(py, "symlink"),
            FileTypeKind::BlockDevice => pyo3::intern!(py, "block-device"),
            FileTypeKind::CharDevice => pyo3::intern!(py, "char-device"),
            FileTypeKind::Fifo => pyo3::intern!(py, "fifo"),
            FileTypeKind::Socket => pyo3::intern!(py, "socket"),
            FileTypeKind::SymlinkDir => pyo3::intern!(py, "symlink-dir"),
            FileTypeKind::SymlinkFile => pyo3::intern!(py, "symlink-file"),
            FileTypeKind::Unknown => pyo3::intern!(py, "unknown"),
        }
    }

    fn __repr__<'py>(&self, py: Python<'py>) -> &'py Bound<'py, pyo3::types::PyString> {
        match self.0.kind() {
            FileTypeKind::Dir => pyo3::intern!(py, "FileType(\"dir\")"),
            FileTypeKind::File => pyo3::intern!(py, "FileType(\"file\")"),
            FileTypeKind::Symlink => pyo3::intern!(py, "FileType(\"symlink\")"),
            FileTypeKind::BlockDevice => pyo3::intern!(py, "FileType(\"block-device\")"),
            FileTypeKind::CharDevice => pyo3::intern!(py, "FileType(\"char-device\")"),
            FileTypeKind::Fifo => pyo3::intern!(py, "FileType(\"fifo\")"),
            FileTypeKind::Socket => pyo3::intern!(py, "FileType(\"socket\")"),
            FileTypeKind::SymlinkDir => pyo3::intern!(py, "FileType(\"symlink-dir\")"),
            FileTypeKind::SymlinkFile => pyo3::intern!(py, "FileType(\"symlink-file\")"),
            FileTypeKind::Unknown => pyo3::intern!(py, "FileType(\"unknown\")"),
        }
    }

    fn __eq__(&self, other: &Self) -> bool {
        self.0.kind() == other.0.kind()
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

    #[getter]
    #[must_use]
    fn is_block_device(&self) -> bool {
        self.0.is_block_device()
    }

    #[getter]
    #[must_use]
    fn is_char_device(&self) -> bool {
        self.0.is_char_device()
    }

    #[getter]
    #[must_use]
    fn is_fifo(&self) -> bool {
        self.0.is_fifo()
    }

    #[getter]
    #[must_use]
    fn is_socket(&self) -> bool {
        self.0.is_socket()
    }

    #[getter]
    #[must_use]
    fn is_symlink_dir(&self) -> bool {
        self.0.is_symlink_dir()
    }

    #[getter]
    #[must_use]
    fn is_symlink_file(&self) -> bool {
        self.0.is_symlink_file()
    }

    #[getter]
    #[must_use]
    fn is_unknown(&self) -> bool {
        self.0.is_unknown()
    }

    #[expect(clippy::wrong_self_convention, clippy::trivially_copy_pass_by_ref)]
    pub(crate) fn to_py<'py>(&self, py: Python<'py>) -> &'py Bound<'py, pyo3::types::PyString> {
        self.__str__(py)
    }

    // TODO: add class attrs? eg `ry.FileType.DIR`
}

impl PyFromStr for FileTypeKind {
    fn py_from_str(s: &str) -> PyResult<Self> {
        match s {
            "d" | "dir" | "directory" => Ok(Self::Dir),
            "f" | "file" => Ok(Self::File),
            "s" | "symlink" | "link" => Ok(Self::Symlink),
            "block-device" => Ok(Self::BlockDevice),
            "char-device" => Ok(Self::CharDevice),
            "fifo" => Ok(Self::Fifo),
            "socket" => Ok(Self::Socket),
            "symlink-dir" => Ok(Self::SymlinkDir),
            "symlink-file" => Ok(Self::SymlinkFile),
            "unknown" => Ok(Self::Unknown),
            _ => py_value_err!(
                "invalid file type string: {s} - must be one of 'd'/'dir'/'directory', 'f'/'file', 's'/'symlink'/'link', 'unknown', 'fifo', 'socket', 'block-device', 'char-device', 'symlink-dir', 'symlink-file'"
            ),
        }
    }
}

impl PyFromStr for PyFileType {
    #[inline]
    fn py_from_str(s: &str) -> pyo3::PyResult<Self> {
        let ft_kind = FileTypeKind::py_from_str(s)?;
        Ok(Self(FileTypeInner::Faux(ft_kind)))
    }
}
