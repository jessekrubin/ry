#![doc = include_str!("../README.md")]

mod walkdir_entry;

use parking_lot::Mutex;
use pyo3::{prelude::*, IntoPyObjectExt};
use ryo3_core::types::PathLike;
use ryo3_globset::{GlobsterLike, PyGlobster};
use std::path::Path;

#[pyclass(name = "WalkdirGen", module = "ry.ryo3", frozen)]
pub struct PyWalkdirGen {
    objects: bool,
    iter: Mutex<Box<dyn Iterator<Item = ::walkdir::DirEntry> + Send + Sync>>,
}

#[pymethods]
impl PyWalkdirGen {
    /// __iter__ just returns self
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    /// __next__ just pulls one item from the underlying iterator
    fn __next__<'py>(&self, py: Python<'py>) -> PyResult<Option<Bound<'py, PyAny>>> {
        if let Some(entry) = self.iter.lock().next() {
            let bound_py_any = if self.objects {
                // if objects is true, we return a DirEntry object
                walkdir_entry::PyWalkDirEntry::from(entry).into_bound_py_any(py)
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
    fn take<'py>(&self, py: Python<'py>, n: usize) -> PyResult<Vec<Bound<'py, PyAny>>> {
        if self.objects {
            self.iter
                .lock()
                .by_ref()
                .take(n)
                .map(|entry| walkdir_entry::PyWalkDirEntry::from(entry).into_bound_py_any(py))
                .collect::<PyResult<Vec<_>>>()
        } else {
            self.iter
                .lock()
                .by_ref()
                .take(n)
                .map(|entry| {
                    let path_str = entry.path().to_string_lossy().to_string();
                    path_str.into_bound_py_any(py)
                })
                .collect::<PyResult<Vec<_>>>()
        }
    }

    /// Collect all the entries into a Vec<Bound<PyAny>>
    pub fn collect<'py>(&self, py: Python<'py>) -> PyResult<Vec<Bound<'py, PyAny>>> {
        // if objects is true, we return a DirEntry object
        // if objects is false, we return a string
        if self.objects {
            let mut results = Vec::new();
            for entry in self.iter.lock().by_ref() {
                let pyentry = walkdir_entry::PyWalkDirEntry::from(entry);
                let py_any = pyentry.into_bound_py_any(py)?;
                results.push(py_any);
            }
            Ok(results)
        } else {
            let mut results = Vec::new();
            for entry in self.iter.lock().by_ref() {
                let path_str = entry.path().to_string_lossy().to_string();
                let py_any = path_str.into_bound_py_any(py)?;
                results.push(py_any);
            }
            Ok(results)
        }
    }
}

impl From<::walkdir::WalkDir> for PyWalkdirGen {
    fn from(wd: ::walkdir::WalkDir) -> Self {
        let wdit = wd.into_iter();
        Self {
            objects: false,
            iter: Mutex::new(Box::new(wdit.filter_map(Result::ok))),
        }
    }
}

#[expect(clippy::too_many_arguments)]
fn build_walkdir(
    path: &Path,
    contents_first: Option<bool>,    // false
    min_depth: Option<usize>,        // default 0
    max_depth: Option<usize>,        // default None
    follow_links: Option<bool>,      // default false
    follow_root_links: Option<bool>, // default true
    same_file_system: Option<bool>,  // default false
    sort_by_file_name: Option<bool>, // default false
) -> ::walkdir::WalkDir {
    let mut wd = ::walkdir::WalkDir::new(path)
        .contents_first(contents_first.unwrap_or(false))
        .follow_links(follow_links.unwrap_or(false))
        .follow_root_links(follow_root_links.unwrap_or(true))
        .same_file_system(same_file_system.unwrap_or(false))
        .min_depth(min_depth.unwrap_or(0));

    if let Some(max_depth) = max_depth {
        wd = wd.max_depth(max_depth);
    }
    if sort_by_file_name.unwrap_or(false) {
        wd = wd.sort_by(|a, b| a.file_name().cmp(b.file_name()));
    }
    wd
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
    files: Option<bool>,             // true
    dirs: Option<bool>,              // true
    contents_first: Option<bool>,    // false
    min_depth: Option<usize>,        // default 0
    max_depth: Option<usize>,        // default None
    follow_links: Option<bool>,      // default false
    follow_root_links: Option<bool>, // default true
    same_file_system: Option<bool>,  // default false
    sort_by_file_name: Option<bool>, // default false
    glob: Option<GlobsterLike>,      // default None
    objects: bool,                   // default false
) -> PyResult<PyWalkdirGen> {
    let wd = build_walkdir(
        path.unwrap_or(PathLike::Str(String::from("."))).as_ref(),
        contents_first,
        min_depth,
        max_depth,
        follow_links,
        follow_root_links,
        same_file_system,
        sort_by_file_name,
    );

    // convert the WalkDir into an iterator of `walkdir::DirEntry` filtering
    // out any `Err`
    let base_iter = wd.into_iter().filter_map(Result::ok);

    // Apply .filter() for files/dirs.
    let want_files = files.unwrap_or(true);
    let want_dirs = dirs.unwrap_or(true);

    let filtered_iter = base_iter.filter(move |entry: &::walkdir::DirEntry| {
        let ftype = entry.file_type();
        (ftype.is_file() && want_files) || (ftype.is_dir() && want_dirs)
    });

    // filter again if there is a glob...
    let walk_globster = match glob {
        Some(g) => Some(PyGlobster::try_from(&g)?),
        None => None,
    };

    // this is the final iterator
    let final_iter = if let Some(gs) = walk_globster {
        Box::new(filtered_iter.filter(move |entry| gs.is_match(entry.path())))
            as Box<dyn Iterator<Item = ::walkdir::DirEntry> + Send + Sync>
    } else {
        Box::new(filtered_iter) as Box<dyn Iterator<Item = ::walkdir::DirEntry> + Send + Sync>
    };
    Ok(PyWalkdirGen {
        objects,
        iter: Mutex::new(final_iter),
    })
}

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyWalkdirGen>()?;
    m.add_function(wrap_pyfunction!(self::walkdir, m)?)?;
    Ok(())
}
