#![doc = include_str!("../README.md")]

mod walkdir_entry;
use pyo3::{IntoPyObjectExt, prelude::*};
use ryo3_core::PyMutex;
use ryo3_core::types::PathLike;
use ryo3_globset::{GlobsterLike, PyGlobster};
use std::path::Path;

use crate::walkdir_entry::PyWalkDirEntry;

#[pyclass(name = "WalkdirGen", frozen, immutable_type)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
pub struct PyWalkdirGen {
    objects: bool,
    iter: PyMutex<Box<dyn Iterator<Item = ::walkdir::DirEntry> + Send + Sync>, false>,
}

#[pymethods]
impl PyWalkdirGen {
    /// __iter__ just returns self
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    /// __next__ just pulls one item from the underlying iterator
    fn __next__<'py>(&self, py: Python<'py>) -> PyResult<Option<Bound<'py, PyAny>>> {
        let value = py.detach(|| self.iter.py_lock().next());
        if let Some(entry) = value {
            let bound_py_any = if self.objects {
                // if objects is true, we return a DirEntry object
                PyWalkDirEntry::from(entry).into_bound_py_any(py)
            } else {
                let path_str = entry.path().to_string_lossy().to_string();
                path_str.into_bound_py_any(py)
            }?;
            Ok(Some(bound_py_any))
        } else {
            Ok(None)
        }
    }

    /// Take n entries from the iterator
    #[pyo3(signature = (n = 1))]
    fn take<'py>(&self, py: Python<'py>, n: usize) -> PyResult<Bound<'py, PyAny>> {
        if self.objects {
            let entries = py.detach(|| {
                self.iter
                    .py_lock()
                    .by_ref()
                    .take(n)
                    .map(PyWalkDirEntry::from)
                    .collect::<Vec<_>>()
            });
            entries.into_bound_py_any(py)
        } else {
            let entries = py.detach(|| {
                self.iter
                    .py_lock()
                    .by_ref()
                    .take(n)
                    .map(|entry| entry.path().to_string_lossy().to_string())
                    .collect::<Vec<_>>()
            });
            entries.into_bound_py_any(py)
        }
    }

    /// Collect all the entries into a Vec<Bound<PyAny>>
    fn collect<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        // if objects is true, we return a DirEntry object
        // if objects is false, we return a string
        if self.objects {
            let entries = py.detach(|| {
                self.iter
                    .py_lock()
                    .by_ref()
                    .map(PyWalkDirEntry::from)
                    .collect::<Vec<_>>()
            });
            entries.into_bound_py_any(py)
        } else {
            let entries = py.detach(|| {
                self.iter
                    .py_lock()
                    .by_ref()
                    .map(|entry| entry.path().to_string_lossy().to_string())
                    .collect::<Vec<_>>()
            });
            entries.into_bound_py_any(py)
        }
    }
}

impl From<::walkdir::WalkDir> for PyWalkdirGen {
    fn from(wd: ::walkdir::WalkDir) -> Self {
        let wdit = wd.into_iter();

        Self {
            objects: false,
            iter: PyMutex::new(Box::new(wdit.filter_map(Result::ok))),
        }
    }
}

#[expect(clippy::struct_excessive_bools)]
struct WalkdirOptions {
    files: bool,
    dirs: bool,
    contents_first: bool,
    min_depth: usize,
    max_depth: Option<usize>,
    follow_links: bool,
    follow_root_links: bool,
    same_file_system: bool,
    sort_by_file_name: bool,
}

impl Default for WalkdirOptions {
    fn default() -> Self {
        Self {
            files: true,
            dirs: true,
            contents_first: false,
            min_depth: 0,
            max_depth: None,
            follow_links: false,
            follow_root_links: true,
            same_file_system: false,
            sort_by_file_name: false,
        }
    }
}

impl WalkdirOptions {
    fn build_walkdir<T: AsRef<Path>>(&self, path: T) -> ::walkdir::WalkDir {
        let mut wd = ::walkdir::WalkDir::new(path)
            .contents_first(self.contents_first)
            .follow_links(self.follow_links)
            .follow_root_links(self.follow_root_links)
            .same_file_system(self.same_file_system)
            .min_depth(self.min_depth);

        if let Some(max_depth) = self.max_depth {
            wd = wd.max_depth(max_depth);
        }
        if self.sort_by_file_name {
            wd = wd.sort_by(|a, b| a.file_name().cmp(b.file_name()));
        }
        wd
    }

    fn build_iter<T: AsRef<Path>>(
        &self,
        path: T,
    ) -> impl Iterator<Item = ::walkdir::DirEntry> + use<T> {
        let wd = self.build_walkdir(path.as_ref());
        let dirs = self.dirs;
        let files = self.files;
        let predicate = move |entry: &::walkdir::DirEntry| {
            let ftype = entry.file_type();
            (ftype.is_file() && files) || (ftype.is_dir() && dirs)
        };

        wd.into_iter()
            .filter_map(Result::ok)
            .filter(move |entry: &::walkdir::DirEntry| predicate(entry))
    }
}

#[expect(clippy::too_many_arguments)]
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
        follow_root_links = true,
        same_file_system = false,
        sort_by_file_name = false,
        glob = None,
        objects = false
    )
)]
pub fn walkdir(
    path: Option<PathLike>,
    files: bool,                // true
    dirs: bool,                 // true
    contents_first: bool,       // false
    min_depth: usize,           // default 0
    max_depth: Option<usize>,   // default None
    follow_links: bool,         // default false
    follow_root_links: bool,    // default true
    same_file_system: bool,     // default false
    sort_by_file_name: bool,    // default false
    glob: Option<GlobsterLike>, // default None
    objects: bool,              // default false
) -> PyResult<PyWalkdirGen> {
    let walk_globster = match glob {
        Some(g) => Some(PyGlobster::try_from(&g)?),
        None => None,
    };
    let opts = WalkdirOptions {
        files,
        dirs,
        contents_first,
        min_depth,
        max_depth,
        follow_links,
        follow_root_links,
        same_file_system,
        sort_by_file_name,
    };
    let wd_iter = opts.build_iter(path.unwrap_or_else(|| PathLike::Str(String::from("."))));
    let final_iter = if let Some(gs) = walk_globster {
        Box::new(wd_iter.filter(move |entry| gs.is_match(entry.path())))
            as Box<dyn Iterator<Item = ::walkdir::DirEntry> + Send + Sync>
    } else {
        Box::new(wd_iter) as Box<dyn Iterator<Item = ::walkdir::DirEntry> + Send + Sync>
    };
    Ok(PyWalkdirGen {
        objects,
        iter: PyMutex::new(final_iter),
    })
}

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyWalkdirGen>()?;
    m.add_function(wrap_pyfunction!(self::walkdir, m)?)?;
    Ok(())
}
