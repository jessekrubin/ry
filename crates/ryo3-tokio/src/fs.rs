//! python `tokio::fs` module
use std::fs;
use std::path::PathBuf;

pub use async_read_dir::PyAsyncReadDir;
use pyo3::prelude::*;
use pyo3::pybacked::PyBackedStr;
use pyo3::types::PyDict;
use ryo3_bytes::RyBytes;
use ryo3_core::PyIoResultExt;
use ryo3_core::types::PyOpenMode;
use ryo3_macro_rules::py_not_implemented_err;
use ryo3_std::fs::PyMetadata;
use ryo3_tokio_rt::{RyCoroutine, blocking_coro, tokio_coro};
mod async_file;
mod async_file_read_stream;
pub use async_file::PyAsyncFile;
pub use async_file_read_stream::PyAsyncFileReadStream;
mod async_read_dir;

#[pyfunction]
pub fn canonicalize_async(py: Python<'_>, path: PathBuf) -> PyResult<Bound<'_, RyCoroutine>> {
    blocking_coro(py, "canonicalize_async", move || {
        Ok(fs::canonicalize(&path)
            .map_py_io_err_with_path(&path)?
            .into_os_string())
    })
}

#[pyfunction]
pub fn copy_async(py: Python<'_>, from: PathBuf, to: PathBuf) -> PyResult<Bound<'_, RyCoroutine>> {
    blocking_coro(py, "copy_async", move || {
        fs::copy(&from, &to).map_py_io_err_with_src_dst(&from, &to)
    })
}

#[pyfunction]
pub fn create_dir_async(py: Python<'_>, path: PathBuf) -> PyResult<Bound<'_, RyCoroutine>> {
    blocking_coro(py, "create_dir_async", move || {
        fs::create_dir(&path).map_py_io_err_with_path(&path)
    })
}

#[pyfunction]
pub fn create_dir_all_async(py: Python<'_>, path: PathBuf) -> PyResult<Bound<'_, RyCoroutine>> {
    blocking_coro(py, "create_dir_all_async", move || {
        fs::create_dir_all(&path).map_py_io_err_with_path(&path)
    })
}

#[pyfunction]
pub fn hard_link_async(
    py: Python<'_>,
    from: PathBuf,
    to: PathBuf,
) -> PyResult<Bound<'_, RyCoroutine>> {
    blocking_coro(py, "hard_link_async", move || {
        fs::hard_link(&from, &to).map_py_io_err_with_src_dst(&from, &to)
    })
}

#[pyfunction]
pub fn metadata_async(py: Python<'_>, path: PathBuf) -> PyResult<Bound<'_, RyCoroutine>> {
    blocking_coro(py, "metadata_async", move || {
        Ok(PyMetadata::from(
            fs::metadata(&path).map_py_io_err_with_path(&path)?,
        ))
    })
}

#[pyfunction]
pub fn read_async(py: Python<'_>, path: PathBuf) -> PyResult<Bound<'_, RyCoroutine>> {
    blocking_coro(py, "read_async", move || {
        Ok(RyBytes::from(
            fs::read(&path).map_py_io_err_with_path(&path)?,
        ))
    })
}

#[pyfunction]
pub fn read_dir_async(py: Python<'_>, path: PathBuf) -> PyResult<Bound<'_, RyCoroutine>> {
    tokio_coro(py, "read_dir_async", async move {
        let readdir = tokio::fs::read_dir(&path)
            .await
            .map_py_io_err_with_path(&path)?;
        Ok(PyAsyncReadDir::from(readdir))
    })
}

#[pyfunction]
pub fn read_link_async(py: Python<'_>, path: PathBuf) -> PyResult<Bound<'_, RyCoroutine>> {
    blocking_coro(py, "read_link_async", move || {
        Ok(fs::read_link(&path)
            .map_py_io_err_with_path(&path)?
            .into_os_string())
    })
}

#[pyfunction]
pub fn read_to_string_async(py: Python<'_>, path: PathBuf) -> PyResult<Bound<'_, RyCoroutine>> {
    blocking_coro(py, "read_to_string_async", move || {
        fs::read_to_string(&path).map_py_io_err_with_path(&path)
    })
}

#[pyfunction]
pub fn remove_dir_async(py: Python<'_>, path: PathBuf) -> PyResult<Bound<'_, RyCoroutine>> {
    blocking_coro(py, "remove_dir_async", move || {
        fs::remove_dir(&path).map_py_io_err_with_path(&path)
    })
}

#[pyfunction]
pub fn remove_dir_all_async(py: Python<'_>, path: PathBuf) -> PyResult<Bound<'_, RyCoroutine>> {
    blocking_coro(py, "remove_dir_all_async", move || {
        fs::remove_dir_all(&path).map_py_io_err_with_path(&path)
    })
}

#[pyfunction]
pub fn remove_file_async(py: Python<'_>, path: PathBuf) -> PyResult<Bound<'_, RyCoroutine>> {
    blocking_coro(py, "remove_file_async", move || {
        fs::remove_file(&path).map_py_io_err_with_path(&path)
    })
}

#[pyfunction]
pub fn rename_async(
    py: Python<'_>,
    from: PathBuf,
    to: PathBuf,
) -> PyResult<Bound<'_, RyCoroutine>> {
    blocking_coro(py, "rename_async", move || {
        fs::rename(&from, &to).map_py_io_err_with_src_dst(&from, &to)
    })
}

#[pyfunction]
pub fn exists_async(py: Python<'_>, path: PathBuf) -> PyResult<Bound<'_, RyCoroutine>> {
    blocking_coro(py, "exists_async", move || {
        path.try_exists().map_py_io_err_with_path(&path)
    })
}

#[pyfunction]
pub fn write_async(
    py: Python<'_>,
    path: PathBuf,
    buf: RyBytes,
) -> PyResult<Bound<'_, RyCoroutine>> {
    blocking_coro(py, "write_async", move || {
        fs::write(&path, buf.as_slice()).map_py_io_err_with_path(&path)?;
        Ok(buf.len())
    })
}

#[pyfunction]
pub fn write_string_async(
    py: Python<'_>,
    path: PathBuf,
    s: PyBackedStr,
) -> PyResult<Bound<'_, RyCoroutine>> {
    blocking_coro(py, "write_string_async", move || {
        fs::write(&path, &*s).map_py_io_err_with_path(&path)?;
        Ok(s.len())
    })
}

#[pyfunction(
    signature = (path, mode = PyOpenMode::default(), buffering = -1, **kwargs),
    text_signature = "(path, mode=\"rb\", buffering=-1, **kwargs)",
)]
pub fn aopen(
    path: PathBuf,
    mode: PyOpenMode,
    buffering: i8,
    kwargs: Option<&Bound<'_, PyDict>>,
) -> PyResult<PyAsyncFile> {
    if buffering != -1 {
        return py_not_implemented_err!("aopen buffering not implemented: {buffering}");
    }
    if let Some(kwargs) = kwargs {
        return py_not_implemented_err!("aopen kwargs not implemented: {kwargs:?}");
    }
    if !mode.is_binary() {
        return Err(pyo3::exceptions::PyNotImplementedError::new_err(
            "aopen text mode not implemented",
        ));
    }
    Ok(PyAsyncFile::new(path, mode.into()))
}

#[pyfunction]
#[pyo3(signature = (path, read_size = 65536, *, offset = 0, buffered = true, strict = true))]
pub fn read_stream_async(
    path: PathBuf,
    read_size: usize,
    offset: u64,
    buffered: bool,
    strict: bool,
) -> PyResult<PyAsyncFileReadStream> {
    PyAsyncFileReadStream::py_new(path, read_size, offset, buffered, strict)
}

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // classes
    m.add_class::<PyAsyncFile>()?;
    m.add_class::<PyAsyncFileReadStream>()?;
    // functions
    m.add_function(wrap_pyfunction!(aopen, m)?)?;
    m.add_function(wrap_pyfunction!(canonicalize_async, m)?)?;
    m.add_function(wrap_pyfunction!(copy_async, m)?)?;
    m.add_function(wrap_pyfunction!(create_dir_all_async, m)?)?;
    m.add_function(wrap_pyfunction!(create_dir_async, m)?)?;
    m.add_function(wrap_pyfunction!(exists_async, m)?)?;
    m.add_function(wrap_pyfunction!(hard_link_async, m)?)?;
    m.add_function(wrap_pyfunction!(metadata_async, m)?)?;
    m.add_function(wrap_pyfunction!(read_async, m)?)?;
    m.add_function(wrap_pyfunction!(read_dir_async, m)?)?;
    m.add_function(wrap_pyfunction!(read_link_async, m)?)?;
    m.add_function(wrap_pyfunction!(read_stream_async, m)?)?;
    m.add_function(wrap_pyfunction!(read_to_string_async, m)?)?;
    m.add_function(wrap_pyfunction!(remove_dir_all_async, m)?)?;
    m.add_function(wrap_pyfunction!(remove_dir_async, m)?)?;
    m.add_function(wrap_pyfunction!(remove_file_async, m)?)?;
    m.add_function(wrap_pyfunction!(rename_async, m)?)?;
    m.add_function(wrap_pyfunction!(write_async, m)?)?;
    m.add_function(wrap_pyfunction!(write_string_async, m)?)?;
    Ok(())
}
