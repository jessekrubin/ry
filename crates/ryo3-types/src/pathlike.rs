use pyo3::FromPyObject;
use std::path::{Path, PathBuf};

#[derive(Debug, FromPyObject, Clone)]
pub enum PathLike {
    PathBuf(PathBuf),
    Str(String),
}

impl From<PathLike> for String {
    fn from(p: PathLike) -> Self {
        match p {
            PathLike::PathBuf(p) => p.to_string_lossy().to_string(),
            PathLike::Str(s) => s,
        }
    }
}

impl AsRef<Path> for PathLike {
    fn as_ref(&self) -> &Path {
        match self {
            PathLike::PathBuf(p) => p.as_ref(),
            PathLike::Str(s) => Path::new(s),
        }
    }
}

impl From<&Path> for PathLike {
    fn from(p: &Path) -> Self {
        PathLike::PathBuf(p.to_path_buf())
    }
}

impl std::fmt::Display for PathLike {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PathLike::PathBuf(p) => write!(f, "{}", p.to_string_lossy()),
            PathLike::Str(s) => write!(f, "{s}"),
        }
    }
}
