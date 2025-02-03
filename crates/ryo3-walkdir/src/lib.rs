#![doc = include_str!("../README.md")]
use std::path::Path;

use ::walkdir as walkdir_rs;
use pyo3::prelude::*;
use ryo3_globset::{GlobsterLike, PyGlobster};
use ryo3_types::PathLike;

#[pyclass(name = "WalkDirEntry", module = "ryo3")]
#[derive(Clone, Debug)]
pub struct PyWalkDirEntry {
    de: walkdir_rs::DirEntry,
}

#[pymethods]
impl PyWalkDirEntry {
    #[getter]
    fn path(&self) -> PyResult<String> {
        self.de
            .path()
            .to_str()
            .map(ToString::to_string)
            .ok_or_else(|| {
                PyErr::new::<pyo3::exceptions::PyUnicodeDecodeError, _>(
                    "Path contains invalid unicode characters",
                )
            })
    }

    #[getter]
    fn file_name(&self) -> String {
        self.de.file_name().to_string_lossy().to_string()
    }

    #[getter]
    fn depth(&self) -> usize {
        self.de.depth()
    }

    fn __str__(&self) -> PyResult<String> {
        self.de
            .path()
            .to_str()
            .map(ToString::to_string)
            .ok_or_else(|| {
                PyErr::new::<pyo3::exceptions::PyUnicodeDecodeError, _>(
                    "Path contains invalid unicode characters",
                )
            })
    }

    fn __repr__(&self) -> String {
        let s = self.__str__().unwrap_or_else(|_| String::from("???"));
        format!("WalkDirEntry({s:?})")
    }
}

impl From<walkdir_rs::DirEntry> for PyWalkDirEntry {
    fn from(de: walkdir_rs::DirEntry) -> Self {
        Self { de }
    }
}

#[pyclass(name = "WalkdirGen", module = "ryo3")]
pub struct PyWalkdirGen {
    iter: walkdir_rs::IntoIter,
    files: bool,
    dirs: bool,
    glob: Option<PyGlobster>,
}
#[pymethods]
impl PyWalkdirGen {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__(mut slf: PyRefMut<'_, Self>) -> Option<String> {
        while let Some(Ok(entry)) = slf.iter.next() {
            if (entry.file_type().is_file() && slf.files)
                || (entry.file_type().is_dir() && slf.dirs)
            {
                if let Some(globs) = &slf.glob {
                    let path_str = entry.path().to_string_lossy().to_string();
                    if globs.is_match_str(&path_str) {
                        return Some(path_str);
                    }
                } else if let Some(path_str) = entry.path().to_str() {
                    return Some(path_str.to_string());
                }
            }
        }
        None
    }
    fn collect(&mut self) -> Vec<String> {
        let files = self.files;
        let dirs = self.dirs;
        let globs = &self.glob;

        self.iter
            .by_ref() // Allows us to consume items from self.iter
            .filter_map(Result::ok) // Filter out Err results
            .filter_map(move |entry| {
                let ftype = entry.file_type();
                // Filter by whether we want files and/or directories
                if (ftype.is_file() && files) || (ftype.is_dir() && dirs) {
                    let path = entry.path();
                    if let Some(globs) = globs {
                        // If we have a glob, we need a string
                        let path_str = path.to_string_lossy();
                        if globs.is_match_str(&path_str) {
                            // Convert from Cow<str> to owned String
                            return Some(path_str.into_owned());
                        }
                    } else if let Some(path_str) = path.to_str() {
                        // No glob, just return the path as a String
                        return Some(path_str.to_string());
                    }
                }
                None
            })
            .collect()
    }
}

impl From<walkdir_rs::WalkDir> for PyWalkdirGen {
    fn from(wd: walkdir_rs::WalkDir) -> Self {
        let wdit = wd.into_iter();
        Self {
            iter: wdit,
            files: true,
            dirs: true,
            glob: None,
        }
    }
}

fn build_walkdir(
    path: &Path,
    contents_first: Option<bool>, // false
    min_depth: Option<usize>,     // default 0
    max_depth: Option<usize>,     // default None
    follow_links: Option<bool>,   // default false
    same_file_system: Option<bool>,
) -> walkdir_rs::WalkDir {
    let mut wd = walkdir_rs::WalkDir::new(path)
        .contents_first(contents_first.unwrap_or(false))
        .follow_links(follow_links.unwrap_or(false))
        .same_file_system(same_file_system.unwrap_or(false))
        .min_depth(min_depth.unwrap_or(0));
    if let Some(max_depth) = max_depth {
        wd = wd.max_depth(max_depth);
    }
    wd
}

#[allow(clippy::too_many_arguments)]
#[pyfunction]
#[pyo3(
    signature = (
        path = None,
        /,
        *,
        files = true,
        dirs = true,
        contents_first = false,
        min_depth = 0,
        max_depth = None,
        follow_links = false,
        same_file_system = false,
        glob = None,
        objects = false
    )
)]
pub fn walkdir(
    path: Option<PathLike>,
    files: Option<bool>,            // true
    dirs: Option<bool>,             // true
    contents_first: Option<bool>,   // false
    min_depth: Option<usize>,       // default 0
    max_depth: Option<usize>,       // default None
    follow_links: Option<bool>,     // default false
    same_file_system: Option<bool>, // default false
    glob: Option<GlobsterLike>,     // default None
    objects: Option<bool>,          // default false
) -> PyResult<PyWalkdirGen> {
    let wd = build_walkdir(
        path.unwrap_or(PathLike::Str(String::from("."))).as_ref(),
        contents_first,
        min_depth,
        max_depth,
        follow_links,
        same_file_system,
    );
    if objects.unwrap_or(false) {
        Err(PyErr::new::<pyo3::exceptions::PyNotImplementedError, _>(
            "objects=True not yet implemented",
        ))
    } else {
        let walk_globster = if let Some(glob) = glob {
            let globster = PyGlobster::try_from(&glob)?;
            Some(globster)
        } else {
            None
        };

        Ok(PyWalkdirGen {
            iter: wd.into_iter(),
            files: files.unwrap_or(true),
            dirs: dirs.unwrap_or(true),
            glob: walk_globster,
        })
    }
}

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // m.add_class::<PyWalkDirEntry>()?;  // not sure if should be exposed...
    m.add_class::<PyWalkdirGen>()?;
    m.add_function(wrap_pyfunction!(self::walkdir, m)?)?;
    Ok(())
}
