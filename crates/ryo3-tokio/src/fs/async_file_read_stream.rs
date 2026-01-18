//! Async file read stream implementation for Tokio runtime, based on the ry
//! `FileReadStream` in ryo3-std and the `ryo3-reqwest` response stream.
//!
//! Kinda a fucking nightmare.
#[cfg(feature = "experimental-async")]
use crate::rt::{on_tokio, on_tokio_py};
use bytes::{Bytes, BytesMut};
use pyo3::prelude::*;
use pyo3_async_runtimes::tokio::future_into_py;
use ryo3_macro_rules::py_io_error;
use ryo3_macro_rules::py_stop_async_iteration_err;
use ryo3_macro_rules::py_value_err;
use std::io;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::fs::File;
use tokio::io::{AsyncRead, AsyncReadExt, AsyncSeek, AsyncSeekExt, BufReader, SeekFrom};
use tokio::sync::Mutex;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub(crate) struct PyFileReadStreamOptions {
    pub(crate) strict: bool,
    pub(crate) path: PathBuf,
    pub(crate) chunk_size: usize,
    pub(crate) offset: u64,
    pub(crate) buffered: bool,
}

struct AsyncFileReadStream<T> {
    file: T,
    buffer: BytesMut,
    chunk_size: usize,
}

impl<T: AsyncRead + AsyncSeek + Unpin> AsyncFileReadStream<T> {
    fn new(file: T, chunk_size: usize) -> Self {
        Self {
            file,
            buffer: BytesMut::with_capacity(chunk_size),
            chunk_size,
        }
    }

    async fn seek_to(&mut self, offset: u64) -> io::Result<u64> {
        self.file.seek(SeekFrom::Start(offset)).await
    }

    async fn next_chunk(&mut self) -> io::Result<Option<Bytes>> {
        if self.buffer.capacity() < self.chunk_size {
            self.buffer
                .reserve(self.chunk_size.saturating_sub(self.buffer.len()));
        }
        let bytes_read = self.file.read_buf(&mut self.buffer).await?;
        if bytes_read == 0 {
            return Ok(None);
        }

        Ok(Some(self.buffer.split_to(bytes_read).freeze()))
    }
}

enum AsyncFileReadStreamWrapper {
    Unbuffered(AsyncFileReadStream<File>),
    Buffered(AsyncFileReadStream<BufReader<File>>),
    // put the options in here so we dont gotta pass them around to ensure open
    Closed(Arc<PyFileReadStreamOptions>),
}

impl AsyncFileReadStreamWrapper {
    async fn ensure_open(&mut self) -> io::Result<()> {
        match self {
            Self::Unbuffered(_) | Self::Buffered(_) => Ok(()),
            Self::Closed(options) => {
                let file = File::open(&options.path).await?;
                if options.strict {
                    let meta = file.metadata().await?;
                    let len = meta.len();
                    if options.offset > len {
                        return Err(io::Error::new(
                            io::ErrorKind::InvalidInput,
                            format!("offset ({}) > len ({})", options.offset, len),
                        ));
                    }
                }

                let mut stream = if options.buffered {
                    Self::Buffered(AsyncFileReadStream::new(
                        BufReader::new(file),
                        options.chunk_size,
                    ))
                } else {
                    Self::Unbuffered(AsyncFileReadStream::new(file, options.chunk_size))
                };

                if options.offset > 0 {
                    match &mut stream {
                        Self::Unbuffered(s) => s.seek_to(options.offset).await?,
                        Self::Buffered(s) => s.seek_to(options.offset).await?,
                        Self::Closed(_) => unreachable!(),
                    };
                }
                *self = stream;
                Ok(())
            }
        }
    }

    async fn next_chunk(&mut self) -> io::Result<Option<Bytes>> {
        match self {
            Self::Unbuffered(s) => s.next_chunk().await,
            Self::Buffered(s) => s.next_chunk().await,
            Self::Closed(_) => Err(io::Error::other("Stream not opened")),
        }
    }
}

impl AsyncFileReadStreamWrapper {}

#[pyclass(
    name = "AsyncFileReadStream",
    frozen,
    immutable_type,
    skip_from_py_object
)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
pub struct PyAsyncFileReadStream {
    options: Arc<PyFileReadStreamOptions>,
    inner: Arc<Mutex<AsyncFileReadStreamWrapper>>,
}

fn map_open_error(e: &std::io::Error) -> PyErr {
    match e.kind() {
        // not found
        std::io::ErrorKind::NotFound => {
            pyo3::exceptions::PyFileNotFoundError::new_err(format!("Failed to open file: {e}"))
        }
        // strict
        std::io::ErrorKind::InvalidInput => {
            pyo3::exceptions::PyValueError::new_err(format!("Failed to open file: {e}"))
        }
        _ => {
            py_io_error!("Failed to open file: {e}")
        }
    }
}

#[cfg(not(feature = "experimental-async"))]
#[pymethods]
impl PyAsyncFileReadStream {
    #[new]
    #[pyo3(signature = (path, *, chunk_size = 65536, offset = 0, buffered = true, strict = true))]
    pub fn py_new(
        path: PathBuf,
        chunk_size: usize,
        offset: u64,
        buffered: bool,
        strict: bool,
    ) -> PyResult<Self> {
        if chunk_size == 0 {
            return py_value_err!("chunk_size must be greater than 0");
        }
        let options = PyFileReadStreamOptions {
            strict,
            path,
            chunk_size,
            offset,
            buffered,
        };
        let arc_options = Arc::new(options);

        let inner = Arc::new(Mutex::new(AsyncFileReadStreamWrapper::Closed(
            arc_options.clone(),
        )));
        Ok(Self {
            options: arc_options,
            inner,
        })
    }

    fn __await__(slf: Py<Self>, py: Python<'_>) -> PyResult<Bound<'_, PyAny>> {
        let inner = slf.borrow(py).inner.clone();

        let fut = future_into_py(py, async move {
            let mut guard = inner.lock().await;
            guard.ensure_open().await.map_err(|e| map_open_error(&e))?;
            Ok(slf)
        })?;
        fut.getattr(pyo3::intern!(py, "__await__"))?.call0()
    }

    fn __eq__(&self, other: &Self) -> bool {
        self.options == other.options
    }

    fn __aiter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __anext__<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let inner = Arc::clone(&self.inner);

        future_into_py(py, async move {
            let mut guard = inner.lock().await;
            guard.ensure_open().await.map_err(|e| map_open_error(&e))?;
            match guard.next_chunk().await {
                Ok(Some(bytes)) => Ok(Some(ryo3_bytes::PyBytes::from(bytes))),
                Ok(None) => py_stop_async_iteration_err!("stream exhausted"),
                Err(e) => Err(e.into()),
            }
        })
    }

    #[pyo3(signature = (n = 1))]
    fn take<'py>(&self, py: Python<'py>, n: usize) -> PyResult<Bound<'py, PyAny>> {
        let inner = self.inner.clone();
        future_into_py(py, async move {
            let mut guard = inner.lock().await;
            guard.ensure_open().await.map_err(|e| map_open_error(&e))?;
            let mut result = Vec::with_capacity(n);
            for _ in 0..n {
                match guard.next_chunk().await {
                    Ok(Some(b)) => result.push(ryo3_bytes::PyBytes::from(b)),
                    Ok(None) => break,
                    Err(e) => return Err(e.into()),
                }
            }
            Ok(result)
        })
    }

    fn collect<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let inner = self.inner.clone();
        future_into_py(py, async move {
            let mut guard = inner.lock().await;
            guard.ensure_open().await.map_err(|e| map_open_error(&e))?;
            let mut result = Vec::new();
            while let Ok(Some(b)) = guard.next_chunk().await {
                result.push(ryo3_bytes::PyBytes::from(b));
            }
            Ok(result)
        })
    }

    fn __repr__(&self) -> String {
        format!("{self:?}")
    }
}

#[cfg(feature = "experimental-async")]
#[pymethods]
impl PyAsyncFileReadStream {
    #[new]
    #[pyo3(signature = (path, *, chunk_size = 65536, offset = 0, buffered = true, strict = true))]
    pub fn py_new(
        path: PathBuf,
        chunk_size: usize,
        offset: u64,
        buffered: bool,
        strict: bool,
    ) -> PyResult<Self> {
        if chunk_size == 0 {
            return py_value_err!("chunk_size must be greater than 0");
        }
        let options = PyFileReadStreamOptions {
            strict,
            path,
            chunk_size,
            offset,
            buffered,
        };
        let arc_options = Arc::new(options);

        let inner = Arc::new(Mutex::new(AsyncFileReadStreamWrapper::Closed(
            arc_options.clone(),
        )));
        Ok(Self {
            options: arc_options,
            inner,
        })
    }

    fn __await__(slf: Py<Self>, py: Python<'_>) -> PyResult<Bound<'_, PyAny>> {
        let inner = slf.borrow(py).inner.clone();

        let fut = future_into_py(py, async move {
            let mut guard = inner.lock().await;
            guard.ensure_open().await.map_err(|e| map_open_error(&e))?;
            Ok(slf)
        })?;
        fut.getattr(pyo3::intern!(py, "__await__"))?.call0()
    }

    fn __eq__(&self, other: &Self) -> bool {
        self.options == other.options
    }

    fn __aiter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __anext__<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let inner = Arc::clone(&self.inner);

        future_into_py(py, async move {
            let mut guard = inner.lock().await;
            guard.ensure_open().await.map_err(|e| map_open_error(&e))?;
            match guard.next_chunk().await {
                Ok(Some(bytes)) => Ok(Some(ryo3_bytes::PyBytes::from(bytes))),
                Ok(None) => py_stop_async_iteration_err!("stream exhausted"),
                Err(e) => Err(e.into()),
            }
        })
    }

    #[pyo3(signature = (n = 1))]
    async fn take(&self, n: usize) -> PyResult<Vec<ryo3_bytes::PyBytes>> {
        let inner = self.inner.clone();
        let vbytes = on_tokio(async move {
            let mut guard = inner.lock().await;
            guard.ensure_open().await.map_err(|e| map_open_error(&e))?;
            let mut result = Vec::with_capacity(n);
            for _ in 0..n {
                match guard.next_chunk().await {
                    Ok(Some(b)) => result.push(ryo3_bytes::PyBytes::from(b)),
                    Ok(None) => break,
                    Err(e) => return Err(e),
                }
            }
            Ok(result)
        })
        .await??;
        Ok(vbytes)
    }

    async fn collect(&self) -> PyResult<Vec<ryo3_bytes::PyBytes>> {
        let inner = self.inner.clone();
        on_tokio_py(async move {
            let mut guard = inner.lock().await;
            guard.ensure_open().await.map_err(|e| map_open_error(&e))?;
            let mut result = Vec::new();
            while let Ok(Some(b)) = guard.next_chunk().await {
                result.push(ryo3_bytes::PyBytes::from(b));
            }
            Ok::<_, PyErr>(result)
        })
        .await
    }

    fn __repr__(&self) -> String {
        format!("{self:?}")
    }
}

impl std::fmt::Debug for PyAsyncFileReadStream {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AsyncFileReadStream(")?;
        write!(f, "path='{}'", self.options.path.display(),)?;
        write!(f, ", chunk_size={}", self.options.chunk_size)?;
        if self.options.offset != 0 {
            write!(f, ", offset={}", self.options.offset)?;
        }
        if self.options.buffered {
            write!(f, ", buffered=True")?;
        } else {
            write!(f, ", buffered=False")?;
        }
        if self.options.strict {
            write!(f, ", strict=True")?;
        } else {
            write!(f, ", strict=False")?;
        }
        write!(f, ")")
    }
}
