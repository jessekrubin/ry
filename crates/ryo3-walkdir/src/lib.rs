#![doc = include_str!("../README.md")]

mod py_walkdir_entry;
use std::path::Path;

use pyo3::prelude::*;
use ryo3_core::sync::RyMutex;
use ryo3_core::types::PathLike;
use ryo3_globset::{GlobsterLike, PyGlobster};

pub use crate::py_walkdir_entry::PyWalkDirEntry;

#[pyclass(name = "WalkDirIter", frozen, immutable_type, skip_from_py_object)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
pub struct PyWalkDirIter(
    RyMutex<Box<dyn Iterator<Item = ::walkdir::DirEntry> + Send + Sync>, false>,
);

impl PyWalkDirIter {
    fn next_entry(&self, py: Python<'_>) -> Option<PyWalkDirEntry> {
        py.detach(|| self.0.py_lock().next()).map(Into::into)
    }
}

#[pymethods]
impl PyWalkDirIter {
    /// __iter__ just returns self
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    /// __next__ just pulls one item from the underlying iterator
    fn __next__(&self, py: Python<'_>) -> Option<PyWalkDirEntry> {
        self.next_entry(py)
    }

    fn next(&self, py: Python<'_>) -> Option<PyWalkDirEntry> {
        self.next_entry(py)
    }

    /// Take n entries from the iterator
    #[pyo3(signature = (n = 1))]
    fn take(&self, py: Python<'_>, n: usize) -> Vec<PyWalkDirEntry> {
        py.detach(|| {
            self.0
                .py_lock()
                .by_ref()
                .take(n)
                .map(Into::into)
                .collect::<Vec<_>>()
        })
    }

    /// Collect all the entries into a Vec<Bound<PyAny>>
    fn collect(&self, py: Python<'_>) -> Vec<PyWalkDirEntry> {
        py.detach(|| {
            self.0
                .py_lock()
                .by_ref()
                .map(Into::into)
                .collect::<Vec<_>>()
        })
    }

    // possible future thang:
    // fn skip(slf: PyRef<'_, Self>, py: Python<'_>, n: usize) -> PyRef<'_, Self> {
    //     py.detach(|| {
    //         slf.0.py_lock().by_ref().take(n).for_each(drop);
    //     });
    //     slf
    // }
}

impl From<::walkdir::WalkDir> for PyWalkDirIter {
    fn from(wd: ::walkdir::WalkDir) -> Self {
        Self(RyMutex::new(Box::new(
            wd.into_iter().filter_map(Result::ok),
        )))
    }
}

#[expect(clippy::struct_excessive_bools)]
struct WalkDirOptions {
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

impl Default for WalkDirOptions {
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

impl WalkDirOptions {
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

#[expect(clippy::too_many_arguments, clippy::fn_params_excessive_bools)]
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
) -> PyResult<PyWalkDirIter> {
    let walk_globster = match glob {
        Some(g) => Some(PyGlobster::try_from(&g)?),
        None => None,
    };
    let opts = WalkDirOptions {
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
    Ok(PyWalkDirIter(RyMutex::new(final_iter)))
}

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyWalkDirIter>()?;
    m.add_class::<PyWalkDirEntry>()?;
    m.add_function(wrap_pyfunction!(self::walkdir, m)?)?;
    Ok(())
}
