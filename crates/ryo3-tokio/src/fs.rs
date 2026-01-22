//! python `tokio::fs` module
use pyo3::prelude::*;
use pyo3::pybacked::PyBackedStr;
use pyo3::types::PyDict;
pub use read_dir::PyAsyncReadDir;
use ryo3_bytes::PyBytes;
use ryo3_core::types::PyOpenMode;
use ryo3_std::fs::PyMetadata;
use std::path::PathBuf;
use tracing::warn;
mod async_file_read_stream;
mod file;
pub use async_file_read_stream::PyAsyncFileReadStream;
pub use file::PyAsyncFile;
mod read_dir;
#[cfg(not(feature = "experimental-async"))]
use crate::rt::future_into_py;
#[cfg(feature = "experimental-async")]
use crate::rt::on_tokio_py;

#[cfg(not(feature = "experimental-async"))]
#[pyfunction]
pub fn canonicalize_async(py: Python<'_>, path: PathBuf) -> PyResult<Bound<'_, PyAny>> {
    future_into_py(py, async move {
        tokio::fs::canonicalize(path)
            .await
            .map(|p| p.to_string_lossy().to_string())
            .map_err(PyErr::from)
    })
}

#[cfg(feature = "experimental-async")]
#[pyfunction]
pub async fn canonicalize_async(path: PathBuf) -> PyResult<String> {
    on_tokio_py(async move {
        tokio::fs::canonicalize(path)
            .await
            .map(|p| p.to_string_lossy().to_string())
            .map_err(PyErr::from)
    })
    .await
}

#[cfg(not(feature = "experimental-async"))]
#[pyfunction]
pub fn copy_async(py: Python<'_>, from: PathBuf, to: PathBuf) -> PyResult<Bound<'_, PyAny>> {
    future_into_py(py, async move {
        tokio::fs::copy(from, to).await.map_err(PyErr::from)
    })
}

#[cfg(feature = "experimental-async")]
#[pyfunction]
pub async fn copy_async(from: PathBuf, to: PathBuf) -> PyResult<u64> {
    on_tokio_py(async move { tokio::fs::copy(from, to).await.map_err(PyErr::from) }).await
}

#[cfg(not(feature = "experimental-async"))]
#[pyfunction]
pub fn create_dir_async(py: Python<'_>, path: PathBuf) -> PyResult<Bound<'_, PyAny>> {
    future_into_py(py, async move {
        tokio::fs::create_dir(path).await.map_err(PyErr::from)
    })
}

#[cfg(feature = "experimental-async")]
#[pyfunction]
pub async fn create_dir_async(path: PathBuf) -> PyResult<()> {
    on_tokio_py(async move { tokio::fs::create_dir(path).await.map_err(PyErr::from) }).await
}

#[cfg(not(feature = "experimental-async"))]
#[pyfunction]
pub fn create_dir_all_async(py: Python<'_>, path: PathBuf) -> PyResult<Bound<'_, PyAny>> {
    future_into_py(py, async move {
        tokio::fs::create_dir_all(path).await.map_err(PyErr::from)
    })
}

#[cfg(feature = "experimental-async")]
#[pyfunction]
pub async fn create_dir_all_async(path: PathBuf) -> PyResult<()> {
    on_tokio_py(async move { tokio::fs::create_dir_all(path).await.map_err(PyErr::from) }).await
}

#[cfg(not(feature = "experimental-async"))]
#[pyfunction]
pub fn hard_link_async(py: Python<'_>, from: PathBuf, to: PathBuf) -> PyResult<Bound<'_, PyAny>> {
    future_into_py(py, async move {
        tokio::fs::hard_link(from, to).await.map_err(PyErr::from)
    })
}

#[cfg(feature = "experimental-async")]
#[pyfunction]
pub async fn hard_link_async(from: PathBuf, to: PathBuf) -> PyResult<()> {
    on_tokio_py(async move { tokio::fs::hard_link(from, to).await.map_err(PyErr::from) }).await
}

#[cfg(not(feature = "experimental-async"))]
#[pyfunction]
pub fn metadata_async(py: Python<'_>, path: PathBuf) -> PyResult<Bound<'_, PyAny>> {
    future_into_py(py, async move {
        tokio::fs::metadata(path)
            .await
            .map(PyMetadata::from)
            .map_err(PyErr::from)
    })
}

#[cfg(feature = "experimental-async")]
#[pyfunction]
pub async fn metadata_async(path: PathBuf) -> PyResult<PyMetadata> {
    on_tokio_py(async move {
        tokio::fs::metadata(path)
            .await
            .map(PyMetadata::from)
            .map_err(PyErr::from)
    })
    .await
}

#[cfg(not(feature = "experimental-async"))]
#[pyfunction]
pub fn read_async(py: Python<'_>, path: PathBuf) -> PyResult<Bound<'_, PyAny>> {
    future_into_py(py, async move {
        tokio::fs::read(path)
            .await
            .map(ryo3_bytes::PyBytes::from)
            .map_err(PyErr::from)
    })
}

#[cfg(feature = "experimental-async")]
#[pyfunction]
pub async fn read_async(path: PathBuf) -> PyResult<PyBytes> {
    on_tokio_py(async move {
        tokio::fs::read(path)
            .await
            .map(ryo3_bytes::PyBytes::from)
            .map_err(PyErr::from)
    })
    .await
}

#[cfg(not(feature = "experimental-async"))]
#[pyfunction]
pub fn read_dir_async(py: Python<'_>, path: PathBuf) -> PyResult<Bound<'_, PyAny>> {
    future_into_py(py, async move {
        let readdir = tokio::fs::read_dir(path).await.map_err(PyErr::from)?;
        Ok(PyAsyncReadDir::from(readdir))
    })
}

#[cfg(feature = "experimental-async")]
#[pyfunction]
pub async fn read_dir_async(path: PathBuf) -> PyResult<PyAsyncReadDir> {
    on_tokio_py(async move {
        let readdir = tokio::fs::read_dir(path).await.map_err(PyErr::from)?;
        Ok(PyAsyncReadDir::from(readdir))
    })
    .await
}

#[cfg(not(feature = "experimental-async"))]
#[pyfunction]
pub fn read_link_async(py: Python<'_>, path: PathBuf) -> PyResult<Bound<'_, PyAny>> {
    future_into_py(py, async move {
        tokio::fs::read_link(path)
            .await
            .map(|p| p.to_string_lossy().to_string())
            .map_err(PyErr::from)
    })
}

#[cfg(feature = "experimental-async")]
#[pyfunction]
pub async fn read_link_async(path: PathBuf) -> PyResult<String> {
    on_tokio_py(async move {
        tokio::fs::read_link(path)
            .await
            .map(|p| p.to_string_lossy().to_string())
            .map_err(PyErr::from)
    })
    .await
}

#[cfg(not(feature = "experimental-async"))]
#[pyfunction]
pub fn read_to_string_async(py: Python<'_>, path: PathBuf) -> PyResult<Bound<'_, PyAny>> {
    future_into_py(py, async move {
        tokio::fs::read_to_string(path).await.map_err(PyErr::from)
    })
}

#[cfg(feature = "experimental-async")]
#[pyfunction]
pub async fn read_to_string_async(path: PathBuf) -> PyResult<String> {
    on_tokio_py(async move { tokio::fs::read_to_string(path).await.map_err(PyErr::from) }).await
}

#[cfg(not(feature = "experimental-async"))]
#[pyfunction]
pub fn remove_dir_async(py: Python<'_>, path: PathBuf) -> PyResult<Bound<'_, PyAny>> {
    future_into_py(py, async move {
        tokio::fs::remove_dir(path).await.map_err(PyErr::from)
    })
}

#[cfg(feature = "experimental-async")]
#[pyfunction]
pub async fn remove_dir_async(path: PathBuf) -> PyResult<()> {
    on_tokio_py(async move { tokio::fs::remove_dir(path).await.map_err(PyErr::from) }).await
}

#[cfg(not(feature = "experimental-async"))]
#[pyfunction]
pub fn remove_dir_all_async(py: Python<'_>, path: PathBuf) -> PyResult<Bound<'_, PyAny>> {
    future_into_py(py, async move {
        tokio::fs::remove_dir_all(path).await.map_err(PyErr::from)
    })
}

#[cfg(feature = "experimental-async")]
#[pyfunction]
pub async fn remove_dir_all_async(path: PathBuf) -> PyResult<()> {
    on_tokio_py(async move { tokio::fs::remove_dir_all(path).await.map_err(PyErr::from) }).await
}

#[cfg(not(feature = "experimental-async"))]
#[pyfunction]
pub fn remove_file_async(py: Python<'_>, path: PathBuf) -> PyResult<Bound<'_, PyAny>> {
    future_into_py(py, async move {
        tokio::fs::remove_file(path).await.map_err(PyErr::from)
    })
}

#[cfg(feature = "experimental-async")]
#[pyfunction]
pub async fn remove_file_async(path: PathBuf) -> PyResult<()> {
    on_tokio_py(async move { tokio::fs::remove_file(path).await.map_err(PyErr::from) }).await
}

#[cfg(not(feature = "experimental-async"))]
#[pyfunction]
pub fn rename_async(py: Python<'_>, from: PathBuf, to: PathBuf) -> PyResult<Bound<'_, PyAny>> {
    future_into_py(py, async move {
        tokio::fs::rename(from, to).await.map_err(PyErr::from)
    })
}

#[cfg(feature = "experimental-async")]
#[pyfunction]
pub async fn rename_async(from: PathBuf, to: PathBuf) -> PyResult<()> {
    on_tokio_py(async move { tokio::fs::rename(from, to).await.map_err(PyErr::from) }).await
}

#[cfg(not(feature = "experimental-async"))]
#[pyfunction]
pub fn try_exists_async(py: Python<'_>, path: PathBuf) -> PyResult<Bound<'_, PyAny>> {
    future_into_py(py, async move {
        tokio::fs::try_exists(path).await.map_err(PyErr::from)
    })
}

#[cfg(feature = "experimental-async")]
#[pyfunction]
pub async fn try_exists_async(path: PathBuf) -> PyResult<bool> {
    on_tokio_py(async move { tokio::fs::try_exists(path).await.map_err(PyErr::from) }).await
}

#[cfg(not(feature = "experimental-async"))]
#[pyfunction]
pub fn exists_async(py: Python<'_>, path: PathBuf) -> PyResult<Bound<'_, PyAny>> {
    future_into_py(py, async move {
        tokio::fs::try_exists(path).await.map_err(PyErr::from)
    })
}

#[cfg(feature = "experimental-async")]
#[pyfunction]
pub async fn exists_async(path: PathBuf) -> PyResult<bool> {
    on_tokio_py(async move { tokio::fs::try_exists(path).await.map_err(PyErr::from) }).await
}

#[cfg(not(feature = "experimental-async"))]
#[pyfunction]
pub fn write_async(py: Python<'_>, path: PathBuf, buf: PyBytes) -> PyResult<Bound<'_, PyAny>> {
    future_into_py(py, async move {
        let bref: &[u8] = buf.as_ref();
        let len = bref.len();
        tokio::fs::write(path, buf)
            .await
            .map(|()| len)
            .map_err(PyErr::from)
    })
}

#[cfg(feature = "experimental-async")]
#[pyfunction]
pub async fn write_async(path: PathBuf, buf: PyBytes) -> PyResult<usize> {
    on_tokio_py(async move {
        let bref: &[u8] = buf.as_ref();
        let len = bref.len();
        tokio::fs::write(path, buf)
            .await
            .map(|()| len)
            .map_err(PyErr::from)
    })
    .await
}

#[cfg(not(feature = "experimental-async"))]
#[pyfunction]
pub fn write_string_async(
    py: Python<'_>,
    path: PathBuf,
    s: PyBackedStr,
) -> PyResult<Bound<'_, PyAny>> {
    future_into_py(py, async move {
        let nbytes = s.len();

        tokio::fs::write(path, s).await.map_err(PyErr::from)?;
        Ok(nbytes)
    })
}

#[cfg(feature = "experimental-async")]
#[pyfunction]
pub async fn write_string_async(path: PathBuf, s: PyBackedStr) -> PyResult<usize> {
    on_tokio_py(async move {
        let nbytes = s.len();

        tokio::fs::write(path, s).await.map_err(PyErr::from)?;
        Ok(nbytes)
    })
    .await
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
        warn!("aopen non-buffered not impl: {kwargs:?}");
    }
    if let Some(kwargs) = kwargs {
        warn!("aopen kwargs not impl: {kwargs:?}");
    }
    if !mode.is_binary() {
        return Err(pyo3::exceptions::PyNotImplementedError::new_err(
            "aopen text mode not implemented",
        ));
    }
    Ok(PyAsyncFile::new(path, mode.into()))
}

#[pyfunction(
    signature = (path, mode = PyOpenMode::default(), buffering = -1, **kwargs),
    text_signature = "(path, mode=\"rb\", buffering=-1, **kwargs)",
    warn(
        message = "`aiopen` is deprecated, use `aopen` instead",
        category = pyo3::exceptions::PyDeprecationWarning
    )
)]
pub fn aiopen(
    path: PathBuf,
    mode: PyOpenMode,
    buffering: i8,
    kwargs: Option<&Bound<'_, PyDict>>,
) -> PyResult<PyAsyncFile> {
    aopen(path, mode, buffering, kwargs)
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
    //fns
    m.add_function(wrap_pyfunction!(aopen, m)?)?;
    m.add_function(wrap_pyfunction!(aiopen, m)?)?;
    m.add_function(wrap_pyfunction!(canonicalize_async, m)?)?;
    m.add_function(wrap_pyfunction!(read_stream_async, m)?)?;
    m.add_function(wrap_pyfunction!(copy_async, m)?)?;
    m.add_function(wrap_pyfunction!(create_dir_async, m)?)?;
    m.add_function(wrap_pyfunction!(create_dir_all_async, m)?)?;
    m.add_function(wrap_pyfunction!(hard_link_async, m)?)?;
    m.add_function(wrap_pyfunction!(metadata_async, m)?)?;
    m.add_function(wrap_pyfunction!(read_async, m)?)?;
    m.add_function(wrap_pyfunction!(read_dir_async, m)?)?;
    m.add_function(wrap_pyfunction!(read_link_async, m)?)?;
    m.add_function(wrap_pyfunction!(read_to_string_async, m)?)?;
    m.add_function(wrap_pyfunction!(remove_dir_async, m)?)?;
    m.add_function(wrap_pyfunction!(remove_dir_all_async, m)?)?;
    m.add_function(wrap_pyfunction!(remove_file_async, m)?)?;
    m.add_function(wrap_pyfunction!(rename_async, m)?)?;
    // m.add_function(wrap_pyfunction!(set_permissions_async, m)?)?;
    // m.add_function(wrap_pyfunction!(symlink_dir_async, m)?)?;
    // m.add_function(wrap_pyfunction!(symlink_file_async, m)?)?;
    // m.add_function(wrap_pyfunction!(symlink_metadata_async, m)?)?;
    m.add_function(wrap_pyfunction!(try_exists_async, m)?)?;
    m.add_function(wrap_pyfunction!(exists_async, m)?)?;
    m.add_function(wrap_pyfunction!(write_async, m)?)?;

    Ok(())
}
