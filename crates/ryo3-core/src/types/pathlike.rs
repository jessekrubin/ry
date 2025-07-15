use pyo3::FromPyObject;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, FromPyObject)]
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
            Self::PathBuf(p) => p.as_ref(),
            Self::Str(s) => Path::new(s),
        }
    }
}

impl From<&Path> for PathLike {
    fn from(p: &Path) -> Self {
        Self::PathBuf(p.to_path_buf())
    }
}

impl std::fmt::Display for PathLike {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PathBuf(p) => write!(f, "{}", p.to_string_lossy()),
            Self::Str(s) => write!(f, "{s}"),
        }
    }
}
