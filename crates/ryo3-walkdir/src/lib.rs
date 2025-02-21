#![doc = include_str!("../README.md")]

mod walkdir_entry;

use pyo3::{prelude::*, IntoPyObjectExt};
use ryo3_core::types::PathLike;
use ryo3_globset::{GlobsterLike, PyGlobster};
use std::path::Path;

#[pyclass(name = "WalkdirGen", module = "ryo3")]
pub struct PyWalkdirGen {
    iter: Box<dyn Iterator<Item = ::walkdir::DirEntry> + Send + Sync>,
}

#[pymethods]
impl PyWalkdirGen {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    /// __next__ just pulls one item from the underlying iterator
    fn __next__(mut slf: PyRefMut<'_, Self>) -> PyResult<Option<Bound<PyAny>>> {
        let py = slf.py();
        if let Some(entry) = slf.iter.next() {
            let path_str = entry.path().to_string_lossy().to_string();
            let bound_py_any = Some(path_str.into_bound_py_any(py)).transpose();
            bound_py_any
        } else {
            Ok(None)
        }
    }

    fn take<'py>(&mut self, py: Python<'py>, n: usize) -> PyResult<Vec<Bound<'py, PyAny>>> {
        let mut results = Vec::new();
        for _ in 0..n {
            if let Some(entry) = self.iter.next() {
                let path_str = entry.path().to_string_lossy().to_string();
                let py_any = path_str.into_bound_py_any(py)?;
                results.push(py_any);
            } else {
                break;
            }
        }
        Ok(results)
    }

    pub fn collect<'py>(&mut self, py: Python<'py>) -> PyResult<Vec<Bound<'py, PyAny>>> {
        let mut results = Vec::new();
        for entry in self.iter.by_ref() {
            let path_str = entry.path().to_string_lossy().to_string();
            let py_any = path_str.into_bound_py_any(py)?;
            results.push(py_any);
        }
        Ok(results)
    }
}

impl From<::walkdir::WalkDir> for PyWalkdirGen {
    fn from(wd: ::walkdir::WalkDir) -> Self {
        let wdit = wd.into_iter();
        Self {
            iter: Box::new(wdit.filter_map(Result::ok)),
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
) -> ::walkdir::WalkDir {
    let mut wd = ::walkdir::WalkDir::new(path)
        .contents_first(contents_first.unwrap_or(false))
        .follow_links(follow_links.unwrap_or(false))
        .same_file_system(same_file_system.unwrap_or(false))
        .min_depth(min_depth.unwrap_or(0));
    if let Some(max_depth) = max_depth {
        wd = wd.max_depth(max_depth);
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
    objects: bool,                  // default false
) -> PyResult<PyWalkdirGen> {
    if objects {
        return Err(PyErr::new::<pyo3::exceptions::PyNotImplementedError, _>(
            "objects=True not yet implemented",
        ));
    }
    let wd = build_walkdir(
        path.unwrap_or(PathLike::Str(String::from("."))).as_ref(),
        contents_first,
        min_depth,
        max_depth,
        follow_links,
        same_file_system,
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
    Ok(PyWalkdirGen { iter: final_iter })
}

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyWalkdirGen>()?;
    m.add_function(wrap_pyfunction!(self::walkdir, m)?)?;
    Ok(())
}
