//! python `tokio::fs` module
use std::path::PathBuf;

pub use async_read_dir::PyAsyncReadDir;
use pyo3::prelude::*;
use pyo3::pybacked::PyBackedStr;
use pyo3::types::PyDict;
use ryo3_bytes::RyBytes;
use ryo3_core::types::PyOpenMode;
use ryo3_macro_rules::py_not_implemented_err;
use ryo3_std::fs::PyMetadata;
use ryo3_tokio_rt::{RyCoroutine, tokio_coro};
mod async_file;
mod async_file_read_stream;
pub use async_file::PyAsyncFile;
pub use async_file_read_stream::PyAsyncFileReadStream;
mod async_read_dir;

#[pyfunction]
pub fn canonicalize_async(py: Python<'_>, path: PathBuf) -> PyResult<Bound<'_, RyCoroutine>> {
    tokio_coro(py, "canonicalize_async", async move {
        tokio::fs::canonicalize(path)
            .await
            .map(PathBuf::into_os_string)
            .map_err(PyErr::from)
    })
}

#[pyfunction]
pub fn copy_async(py: Python<'_>, from: PathBuf, to: PathBuf) -> PyResult<Bound<'_, RyCoroutine>> {
    tokio_coro(py, "copy_async", async move {
        tokio::fs::copy(from, to).await.map_err(PyErr::from)
    })
}

#[pyfunction]
pub fn create_dir_async(py: Python<'_>, path: PathBuf) -> PyResult<Bound<'_, RyCoroutine>> {
    tokio_coro(py, "create_dir_async", async move {
        tokio::fs::create_dir(path).await.map_err(PyErr::from)
    })
}

#[pyfunction]
pub fn create_dir_all_async(py: Python<'_>, path: PathBuf) -> PyResult<Bound<'_, RyCoroutine>> {
    tokio_coro(py, "create_dir_all_async", async move {
        tokio::fs::create_dir_all(path).await.map_err(PyErr::from)
    })
}

#[pyfunction]
pub fn hard_link_async(
    py: Python<'_>,
    from: PathBuf,
    to: PathBuf,
) -> PyResult<Bound<'_, RyCoroutine>> {
    tokio_coro(py, "hard_link_async", async move {
        tokio::fs::hard_link(from, to).await.map_err(PyErr::from)
    })
}

#[pyfunction]
pub fn metadata_async(py: Python<'_>, path: PathBuf) -> PyResult<Bound<'_, RyCoroutine>> {
    tokio_coro(py, "metadata_async", async move {
        tokio::fs::metadata(path)
            .await
            .map(PyMetadata::from)
            .map_err(PyErr::from)
    })
}

#[pyfunction]
pub fn read_async(py: Python<'_>, path: PathBuf) -> PyResult<Bound<'_, RyCoroutine>> {
    tokio_coro(py, "read_async", async move {
        tokio::fs::read(path)
            .await
            .map(RyBytes::from)
            .map_err(PyErr::from)
    })
}

#[pyfunction]
pub fn read_dir_async(py: Python<'_>, path: PathBuf) -> PyResult<Bound<'_, RyCoroutine>> {
    tokio_coro(py, "read_dir_async", async move {
        let readdir = tokio::fs::read_dir(path).await.map_err(PyErr::from)?;
        Ok(PyAsyncReadDir::from(readdir))
    })
}

#[pyfunction]
pub fn read_link_async(py: Python<'_>, path: PathBuf) -> PyResult<Bound<'_, RyCoroutine>> {
    tokio_coro(py, "read_link_async", async move {
        tokio::fs::read_link(path)
            .await
            .map(PathBuf::into_os_string)
            .map_err(PyErr::from)
    })
}

#[pyfunction]
pub fn read_to_string_async(py: Python<'_>, path: PathBuf) -> PyResult<Bound<'_, RyCoroutine>> {
    tokio_coro(py, "read_to_string_async", async move {
        tokio::fs::read_to_string(path).await.map_err(PyErr::from)
    })
}

#[pyfunction]
pub fn remove_dir_async(py: Python<'_>, path: PathBuf) -> PyResult<Bound<'_, RyCoroutine>> {
    tokio_coro(py, "remove_dir_async", async move {
        tokio::fs::remove_dir(path).await.map_err(PyErr::from)
    })
}

#[pyfunction]
pub fn remove_dir_all_async(py: Python<'_>, path: PathBuf) -> PyResult<Bound<'_, RyCoroutine>> {
    tokio_coro(py, "remove_dir_all_async", async move {
        tokio::fs::remove_dir_all(path).await.map_err(PyErr::from)
    })
}

#[pyfunction]
pub fn remove_file_async(py: Python<'_>, path: PathBuf) -> PyResult<Bound<'_, RyCoroutine>> {
    tokio_coro(py, "remove_file_async", async move {
        tokio::fs::remove_file(path).await.map_err(PyErr::from)
    })
}

#[pyfunction]
pub fn rename_async(
    py: Python<'_>,
    from: PathBuf,
    to: PathBuf,
) -> PyResult<Bound<'_, RyCoroutine>> {
    tokio_coro(py, "rename_async", async move {
        tokio::fs::rename(from, to).await.map_err(PyErr::from)
    })
}

#[pyfunction]
pub fn try_exists_async(py: Python<'_>, path: PathBuf) -> PyResult<Bound<'_, RyCoroutine>> {
    tokio_coro(py, "try_exists_async", async move {
        tokio::fs::try_exists(path).await.map_err(PyErr::from)
    })
}

#[pyfunction]
pub fn exists_async(py: Python<'_>, path: PathBuf) -> PyResult<Bound<'_, RyCoroutine>> {
    tokio_coro(py, "exists_async", async move {
        tokio::fs::try_exists(path).await.map_err(PyErr::from)
    })
}

#[pyfunction]
pub fn write_async(
    py: Python<'_>,
    path: PathBuf,
    buf: RyBytes,
) -> PyResult<Bound<'_, RyCoroutine>> {
    tokio_coro(py, "write_async", async move {
        let len = buf.as_slice().len();
        tokio::fs::write(path, buf)
            .await
            .map(|()| len)
            .map_err(PyErr::from)
    })
}

#[pyfunction]
pub fn write_string_async(
    py: Python<'_>,
    path: PathBuf,
    s: PyBackedStr,
) -> PyResult<Bound<'_, RyCoroutine>> {
    tokio_coro(py, "write_string_async", async move {
        let nbytes = s.len();
        tokio::fs::write(path, s).await.map_err(PyErr::from)?;
        Ok(nbytes)
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
    m.add_function(wrap_pyfunction!(try_exists_async, m)?)?;
    m.add_function(wrap_pyfunction!(write_async, m)?)?;
    Ok(())
}
