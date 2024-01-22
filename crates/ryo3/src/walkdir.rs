use std::path::Path;

use ::walkdir as walkdir_rs;
use pyo3::prelude::*;

use crate::fs::fspath::PathLike;

#[pyclass(name = "WalkDirEntry")]
#[derive(Clone, Debug)]
pub struct PyWalkDirEntry {
    de: walkdir_rs::DirEntry,
}

#[pymethods]
impl PyWalkDirEntry {
    #[getter]
    fn path(&self) -> String {
        self.de.path().to_str().unwrap().to_string()
    }

    #[getter]
    fn file_name(&self) -> String {
        self.de.file_name().to_str().unwrap().to_string()
    }

    #[getter]
    fn depth(&self) -> usize {
        self.de.depth()
    }

    fn __str__(&self) -> String {
        self.de.path().to_str().unwrap().to_string()
    }

    fn __repr__(&self) -> String {
        let s = self.de.path().to_str().unwrap().to_string();
        format!("WalkDirEntry({:?})", s)
    }
}

impl From<walkdir_rs::DirEntry> for PyWalkDirEntry {
    fn from(de: walkdir_rs::DirEntry) -> Self {
        Self {
            de: de,
        }
    }
}

#[pyclass(name = "WalkdirGen")]
pub struct PyWalkdirGen {
    // wd: walkdir_rs::WalkDir,
    iter: walkdir_rs::IntoIter,
    // iter: Box<dyn Iterator<Item=String> + Send>,
}

#[pyclass(name = "WalkdirGen")]
pub struct PyFspathsGen {
    // wd: walkdir_rs::WalkDir,
    iter: walkdir_rs::IntoIter,
    // iter: Box<dyn Iterator<Item=String> + Send>,
}




// #[pymethods]
// impl PyWalkdirGen {
//     fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
//         slf
//     }
//     fn __next__(mut slf: PyRefMut<'_, Self>) -> Option<String> {
//         let n = slf.iter.next();
//         match n {
//             Some(Ok(n)) => {
//                 let path = n.path();
//                 let path = path.to_str().unwrap().to_string();
//                 Some(path)
//             }
//             _ => None,
//         }
//     }
// }
#[pymethods]
impl PyWalkdirGen {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }
    fn __next__(mut slf: PyRefMut<'_, Self>) -> Option<PyWalkDirEntry> {
        let n = slf.iter.next();
        match n {
            Some(Ok(n)) => {
                Some(
                    PyWalkDirEntry::from(n)
                )
            }
            _ => None,
        }
    }
}

#[pymethods]
impl PyFspathsGen {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }
    fn __next__(mut slf: PyRefMut<'_, Self>) -> Option<String> {
        let n = slf.iter.next();
        match n {
            Some(Ok(n)) => {
                let path = n.path();
                let path = path.to_str().unwrap().to_string();
                Some(path)
            }
            _ => None,
        }
    }
}

impl From<walkdir_rs::WalkDir> for PyWalkdirGen {
    fn from(wd: walkdir_rs::WalkDir) -> Self {
        let wdit = wd.into_iter();
        Self {
            // wd: wd,
            iter: wdit,
        }
    }
}

impl From<walkdir_rs::WalkDir> for PyFspathsGen {
    fn from(wd: walkdir_rs::WalkDir) -> Self {
        let wdit = wd.into_iter();
        Self {
            // wd: wd,
            iter: wdit,
        }
    }
}

fn build_walkdir(
    path: &Path,
    // files: Option<bool>, // true
    // dirs: Option<bool>, // true
    contents_first: Option<bool>, // false
    min_depth: Option<usize>, // default 0
    max_depth: Option<usize>, // default None
    follow_links: Option<bool>, // default false
    same_file_system: Option<bool>,
) -> walkdir_rs::WalkDir
{
    let mut wd = walkdir_rs::WalkDir::new(path);
    if let Some(contents_first) = contents_first {
        wd = wd.contents_first(contents_first);
    }
    if let Some(min_depth) = min_depth {
        wd = wd.min_depth(min_depth);
    }
    if let Some(max_depth) = max_depth {
        wd = wd.max_depth(max_depth);
    }
    if let Some(follow_links) = follow_links {
        wd = wd.follow_links(follow_links);
    }
    if let Some(same_file_system) = same_file_system {
        wd = wd.same_file_system(same_file_system);
    }
    wd
}

#[pyfunction]
pub fn walkdir(
    path: Option<PathLike>,
    files: Option<bool>, // true
    dirs: Option<bool>, // true
    contents_first: Option<bool>, // false
    min_depth: Option<usize>, // default 0
    max_depth: Option<usize>, // default None
    follow_links: Option<bool>, // default false
    same_file_system: Option<bool>,
) -> PyResult<PyWalkdirGen> {
    let wd = build_walkdir(
        path.unwrap_or(
            PathLike::Str(
                String::from(".")
            )
        ).as_ref(),
        contents_first,
        min_depth,
        max_depth,
        follow_links,
        same_file_system,
    );
    let pywd = PyWalkdirGen::from(wd);
    Ok(pywd)
}


#[pyfunction]
pub fn fspaths(
    path: Option<PathLike>,
    files: Option<bool>, // true
    dirs: Option<bool>, // true
    contents_first: Option<bool>, // false
    min_depth: Option<usize>, // default 0
    max_depth: Option<usize>, // default None
    follow_links: Option<bool>, // default false
    same_file_system: Option<bool>,
) -> PyResult<PyFspathsGen> {
    let wd = build_walkdir(
        path.unwrap_or(
            PathLike::Str(
                String::from(".")
            )
        ).as_ref(),
        contents_first,
        min_depth,
        max_depth,
        follow_links,
        same_file_system,
    );
    let pywd = PyFspathsGen::from(wd);
    Ok(pywd)
}

pub fn madd(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyWalkDirEntry>()?;
    m.add_class::<PyWalkdirGen>()?;
    m.add_function(wrap_pyfunction!(self::walkdir, m)?)?;
    Ok(())
}
