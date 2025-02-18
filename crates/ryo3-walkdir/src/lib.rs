#![doc = include_str!("../README.md")]
use std::path::Path;

use ::walkdir as walkdir_rs;
use pyo3::{prelude::*, IntoPyObjectExt};
use ryo3_globset::{GlobsterLike, PyGlobster};
use ryo3_types::PathLike;

#[pyclass(name = "WalkDirEntry", module = "ryo3")]
#[derive(Clone, Debug)]
pub struct PyWalkDirEntry {
    de: walkdir_rs::DirEntry,
}

#[pymethods]
impl PyWalkDirEntry {
    fn __fspath__(&self) -> PyResult<String> {
        Ok(self.de.path().to_path_buf().to_string_lossy().to_string())
        // .to_str()
        // .map(ToString::to_string)
        // .ok_or_else(|| {
        //     PyErr::new::<pyo3::exceptions::PyUnicodeDecodeError, _>(
        //         "Path contains invalid unicode characters",
        //     )
        // })
    }

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

    // Methods
    // file_name
    // file_type
    // into_path
    // metadata
    // path
    // path_is_symlink

    #[getter]
    fn path_is_symlink(&self) -> PyResult<bool> {
        Ok(self.de.path_is_symlink())
    }

    fn metadata(&self) -> PyResult<ryo3_std::fs::PyMetadata> {
         self
            .de
            .metadata()
            .map(|md| md.into())
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyPermissionError, _>(format!("{e}")))

    }

    #[getter]
    fn file_type(&self) -> PyResult<ryo3_std::fs::PyFileType> {
        let ft = self.de.file_type().into();
        Ok(ft)
    }

    #[getter]
    fn is_dir(&self) -> bool {
        self.de.file_type().is_dir()
    }

    #[getter]
    fn is_file(&self) -> bool {
        self.de.file_type().is_file()
    }

    #[getter]
    fn is_symlink(&self) -> bool {
        self.de.file_type().is_symlink()
    }

    #[getter]
    fn len(&self) -> PyResult<u64> {
        let mlen = self
            .de
            .metadata()
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyPermissionError, _>(format!("{e}")))?
            .len();
        Ok(mlen)
    }
}

impl From<walkdir_rs::DirEntry> for PyWalkDirEntry {
    fn from(de: walkdir_rs::DirEntry) -> Self {
        Self { de }
    }
}

#[pyclass(name = "WalkdirGen", module = "ryo3")]
pub struct PyWalkdirGen {
    iter: Box<dyn Iterator<Item = walkdir_rs::DirEntry> + Send + Sync>,
    // files: bool,
    // dirs: bool,
    // glob: Option<PyGlobster>,
}
#[pymethods]
impl PyWalkdirGen {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    // fn __next__(mut slf: PyRefMut<'_, Self>) -> Option<String> {
    //     while let Some(Ok(entry)) = slf.iter.next() {
    //         if (entry.file_type().is_file() && slf.files)
    //             || (entry.file_type().is_dir() && slf.dirs)
    //         {
    //             if let Some(globs) = &slf.glob {
    //                 let path_str = entry.path().to_string_lossy().to_string();
    //                 if globs.is_match_str(&path_str) {
    //                     return Some(path_str);
    //                 }
    //             } else if let Some(path_str) = entry.path().to_str() {
    //                 return Some(path_str.to_string());
    //             }
    //         }
    //     }
    //     None
    // }
    // fn collect(&mut self) -> Vec<String> {
    //     let files = self.files;
    //     let dirs = self.dirs;
    //     let globs = &self.glob;
    //
    //     self.iter
    //         .by_ref() // Allows us to consume items from self.iter
    //         .filter_map(Result::ok) // Filter out Err results
    //         .filter_map(move |entry| {
    //             let ftype = entry.file_type();
    //             // Filter by whether we want files and/or directories
    //             if (ftype.is_file() && files) || (ftype.is_dir() && dirs) {
    //                 let path = entry.path();
    //                 if let Some(globs) = globs {
    //                     // If we have a glob, we need a string
    //                     let path_str = path.to_string_lossy();
    //                     if globs.is_match_str(&path_str) {
    //                         // Convert from Cow<str> to owned String
    //                         return Some(path_str.into_owned());
    //                     }
    //                 } else if let Some(path_str) = path.to_str() {
    //                     // No glob, just return the path as a String
    //                     return Some(path_str.to_string());
    //                 }
    //             }
    //             None
    //         })
    //         .collect()
    // }
    /// __next__ just pulls one item from our underlying iterator.
    /// Depending on `yield_entries`, we wrap it in PyWalkDirEntry or just return a path string.
    fn __next__(mut slf: PyRefMut<'_, Self>) -> PyResult<Option<Bound<PyAny>>> {
        let py = slf.py();
        if let Some(entry) = slf.iter.next() {
            // let path_str = entry.path().to_string_lossy().to_string();
            // Some(path_str.into_py(py))
            // if slf.yield_entries {
            //     let obj = PyWalkDirEntry::from(entry).into_py(py);
            //     Some(obj)
            // } else {
            //     Return just the path as a String.
            let path_str = entry.path().to_string_lossy().to_string();

            let bound_py_any = Some(path_str.into_bound_py_any(py)).transpose();
            bound_py_any

            // }
        } else {
            Ok(None)
        }
    }

    fn take<'py>(&mut self, py: Python<'py>, n: usize) -> PyResult<Vec<Bound<'py, PyAny>>> {
        // let py = self.py();
        let mut results = Vec::new();
        for _ in 0..n {
            if let Some(entry) = self.iter.next() {
                // if self.yield_entries {
                //     results.push(PyWalkDirEntry::from(entry).into_py(py));
                // } else {
                let path_str = entry.path().to_string_lossy().to_string();
                let py_any = path_str.into_bound_py_any(py)?;

                results.push(py_any);

                // }
            } else {
                break;
            }
        }
        Ok(results)
    }

    /// Example: collect everything eagerly into a list of either strings or `WalkDirEntry`.
    fn collect<'py>(&mut self, py: Python<'py>) -> PyResult<Vec< Bound<'py, PyAny>>>{
        let mut results = Vec::new();
        for entry in self.iter.by_ref() {
            let path_str = entry.path().to_string_lossy().to_string();
            let py_any = path_str.into_bound_py_any(py)?;
            results.push(py_any);
        }
        Ok(results)
    }
}

impl From<walkdir_rs::WalkDir> for PyWalkdirGen {
    fn from(wd: walkdir_rs::WalkDir) -> Self {
        let wdit = wd.into_iter();
        Self {
            iter: Box::new(wdit.filter_map(|res| res.ok())),
            // files: true,
            // dirs: true,
            // glob: None,
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
    objects: bool,          // default false
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

    // Turn the WalkDir into an iterator of DirEntry (filtering out any Err).
    let base_iter = wd.into_iter().filter_map(|res| res.ok());

    // Apply .filter() for files/dirs.
    let want_files = files.unwrap_or(true);
    let want_dirs = dirs.unwrap_or(true);

    let filtered_iter = base_iter.filter(move |entry: &walkdir_rs::DirEntry| {
        let ftype = entry.file_type();
        (ftype.is_file() && want_files) || (ftype.is_dir() && want_dirs)
    });

    // If there's a glob, filter again.
    let walk_globster = match glob {
        Some(g) => Some(PyGlobster::try_from(&g)?),
        None => None,
    };
    let final_iter = if let Some(gs) = walk_globster {
        Box::new(filtered_iter.filter(move |entry| {
            let path_str = entry.path().to_string_lossy();
            gs.is_match_str(&path_str)
        })) as Box<dyn Iterator<Item = walkdir_rs::DirEntry> + Send + Sync>
    } else {
        Box::new(filtered_iter) as Box<dyn Iterator<Item = walkdir_rs::DirEntry> + Send + Sync>
    };
    Ok(PyWalkdirGen {
        iter: final_iter,
        // yield_entries,
    })
    // if objects.unwrap_or(false) {
    //     Err(PyErr::new::<pyo3::exceptions::PyNotImplementedError, _>(
    //         "objects=True not yet implemented",
    //     ))
    // } else {
    //     let walk_globster = if let Some(glob) = glob {
    //         let globster = PyGlobster::try_from(&glob)?;
    //         Some(globster)
    //     } else {
    //         None
    //     };
    //
    //     Ok(PyWalkdirGen {
    //         iter: wd.into_iter(),
    //         files: files.unwrap_or(true),
    //         dirs: dirs.unwrap_or(true),
    //         glob: walk_globster,
    //     })
    // }
}

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // m.add_class::<PyWalkDirEntry>()?;  // not sure if should be exposed...
    m.add_class::<PyWalkdirGen>()?;
    m.add_function(wrap_pyfunction!(self::walkdir, m)?)?;
    Ok(())
}
