use pyo3::prelude::*;
use ryo3_core::sync::RyMutex;

use crate::RyWalkDirEntry;

#[pyclass(name = "WalkDirIter", frozen, immutable_type)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
pub struct RyWalkDirIter(
    pub(crate) RyMutex<Box<dyn Iterator<Item = ::walkdir::DirEntry> + Send + Sync>, false>,
);

impl RyWalkDirIter {
    fn next_entry(&self, py: Python<'_>) -> Option<RyWalkDirEntry> {
        py.detach(|| self.0.py_lock().next()).map(Into::into)
    }
}

#[pymethods]
impl RyWalkDirIter {
    /// __iter__ just returns self
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    /// __next__ just pulls one item from the underlying iterator
    fn __next__(&self, py: Python<'_>) -> Option<RyWalkDirEntry> {
        self.next_entry(py)
    }

    fn next(&self, py: Python<'_>) -> Option<RyWalkDirEntry> {
        self.next_entry(py)
    }

    /// Take n entries from the iterator
    #[pyo3(signature = (n = 1))]
    fn take(&self, py: Python<'_>, n: usize) -> Vec<RyWalkDirEntry> {
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
    fn collect(&self, py: Python<'_>) -> Vec<RyWalkDirEntry> {
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

impl From<::walkdir::WalkDir> for RyWalkDirIter {
    fn from(wd: ::walkdir::WalkDir) -> Self {
        Self(RyMutex::new(Box::new(
            wd.into_iter().filter_map(Result::ok),
        )))
    }
}
