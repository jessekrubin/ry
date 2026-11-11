//! `std::io::Error` -> python `OSError` w/ errno + filename(s)
//!
//! builds `OSError(errno, strerror, filename, winerror, filename2)` like
//! the cpython `os` module. Python picks the subclass (eg `FileNotFoundError`,
//! `PermissionError`, et al).
//!
//! Io errors without an os code (a custom made up `io::Error`) fall back to
//! pyo3's `From<io::Error>`, which uses `io::ErrorKind` to pick the exception
//! cls.
use std::{io, path};

/// `io::Result` -> `PyResult` w/ the path(s) attached
///
/// paths are taken by value so owned args (`PathLike`, ...) can just be handed
/// over; they drop on the calling thread, so call this after `py.detach`
///
/// ```ignore
/// std::fs::read(path).map_py_io_err_with_path(path)?;
/// std::fs::rename(src, dst).map_py_io_err_with_src_dst(src, dst)?;
/// ```
pub trait PyIoResultExt<T> {
    /// `OSError` subclass w/ `errno` + `filename`
    fn map_py_io_err_with_path(self, path: impl AsRef<path::Path>) -> pyo3::PyResult<T>;

    /// same but w/ `filename2` too ~ for 2-path ops (rename/copy/link),
    /// renders as `[Errno 2] No such file or directory: 'src' -> 'dst'`
    fn map_py_io_err_with_src_dst(
        self,
        src: impl AsRef<path::Path>,
        dst: impl AsRef<path::Path>,
    ) -> pyo3::PyResult<T>;
}

impl<T> PyIoResultExt<T> for io::Result<T> {
    #[inline]
    fn map_py_io_err_with_path(self, path: impl AsRef<path::Path>) -> pyo3::PyResult<T> {
        self.map_err(|e| os_error(e, path.as_ref(), None))
    }

    #[inline]
    fn map_py_io_err_with_src_dst(
        self,
        src: impl AsRef<path::Path>,
        dst: impl AsRef<path::Path>,
    ) -> pyo3::PyResult<T> {
        self.map_err(|e| os_error(e, src.as_ref(), Some(dst.as_ref())))
    }
}

/// `OSError(errno, strerror, filename, winerror, filename2)`
///
/// REF: <https://docs.python.org/3/builtins/exceptions.html#OSError>
fn os_error(e: io::Error, filename: &path::Path, filename2: Option<&path::Path>) -> pyo3::PyErr {
    let Some(code) = e.raw_os_error() else {
        return pyo3::PyErr::from(e);
    };
    let (errno, winerror) = cfg_select! {
        // raw code is a win32 err; python maps `winerror` -> errno
        windows => (0, Some(code)),
        _ => (code, None::<i32>),
    };
    pyo3::exceptions::PyOSError::new_err((
        errno,
        strerror(&e),
        filename.as_os_str().to_owned(),
        winerror,
        filename2.map(|p| p.as_os_str().to_owned()),
    ))
}

/// the os msg minus rust's `" (os error N)"` suffix to match cpython
fn strerror(e: &io::Error) -> String {
    let mut msg = e.to_string();
    if let Some(i) = msg.rfind(" (os error ") {
        msg.truncate(i);
    }
    let len = cfg_select! {
        windows => msg
            .trim_end_matches(|c: char| c == '.' || c.is_whitespace())
            .len(),
        _ => msg.len(),
    };
    msg.truncate(len);
    msg
}
