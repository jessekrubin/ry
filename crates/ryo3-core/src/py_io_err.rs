//! `std::io::Error` -> python `OSError` w/ errno + filename(s)
//!
//! `OSError(errno, strerror, filename)` picks the subclass from errno
//! (`FileNotFoundError`, `PermissionError`, ...) and renders like cpython:
//! `[Errno 2] No such file or directory: 'nope'`
use std::io;
use std::path::Path;

use pyo3::PyErr;
use pyo3::exceptions::PyOSError;

use crate::map_py_io_err_with_path;

/// `io::Result` -> `PyResult` w/ the path(s) attached
///
/// paths are taken by value so owned args (`PathLike`, ...) can just be handed
/// over; they drop on the calling thread, so call this after `py.detach`
///
/// ```ignore
/// std::fs::read(path).map_py_io_err(path)?;
/// std::fs::rename(src, dst).map_py_io_err_with_src_dst(src, dst)?;
/// ```
pub trait PyIoResultExt<T> {
    /// `OSError` subclass w/ `errno` + `filename`
    fn map_py_io_err(self, path: impl AsRef<Path>) -> pyo3::PyResult<T>;

    /// same but w/ `filename2` too ~ for 2-path ops (rename/copy/link),
    /// renders as `[Errno 2] No such file or directory: 'src' -> 'dst'`
    fn map_py_io_err_with_src_dst(
        self,
        src: impl AsRef<Path>,
        dst: impl AsRef<Path>,
    ) -> pyo3::PyResult<T>;
}

impl<T> PyIoResultExt<T> for io::Result<T> {
    #[inline]
    fn map_py_io_err(self, path: impl AsRef<Path>) -> pyo3::PyResult<T> {
        self.map_err(|e| map_py_io_err_with_path(e, path.as_ref()))
    }

    #[inline]
    fn map_py_io_err_with_src_dst(
        self,
        src: impl AsRef<Path>,
        dst: impl AsRef<Path>,
    ) -> pyo3::PyResult<T> {
        self.map_err(|e| map_py_io_err_with_paths(e, src.as_ref(), dst.as_ref()))
    }
}

/// [`map_py_io_err_with_path`] w/ a second path (`filename2`)
#[must_use]
pub fn map_py_io_err_with_paths(e: io::Error, src: &Path, dst: &Path) -> PyErr {
    let Some(code) = e.raw_os_error() else {
        return PyErr::from(e);
    };
    // same strerror cleanup as `map_py_io_err_with_path`
    let msg = e.to_string();
    let strerror = msg
        .strip_suffix(&format!(" (os error {code})"))
        .unwrap_or(&msg);
    #[cfg(windows)]
    let strerror = strerror.trim_end_matches(|c: char| c == '.' || c.is_whitespace());
    let strerror = strerror.to_owned();
    let src = src.as_os_str().to_owned();
    let dst = dst.as_os_str().to_owned();
    #[cfg(windows)]
    let args = (0, strerror, src, code, dst);
    #[cfg(not(windows))]
    let args = (code, strerror, src, None::<i32>, dst);
    PyOSError::new_err(args)
}
