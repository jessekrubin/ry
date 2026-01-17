use pyo3::prelude::*;

use pyo3::intern;
use pyo3_async_runtimes::tokio::future_into_py;
use ryo3_core::types::{PyOpenMode, PyOpenOptions};
use ryo3_macro_rules::{py_io_error, py_runtime_err, py_stop_async_iteration_err, pytodo};
use std::io::SeekFrom;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::fs::File;
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncSeekExt, AsyncWriteExt, BufStream};
use tokio::sync::Mutex;

enum FileState {
    Closed,
    Open(Box<BufStream<File>>),
    // Consumed,
}

struct PyAsyncFileInner {
    state: FileState,
    path: PathBuf,
    open_options: PyOpenOptions,
}

impl Drop for PyAsyncFileInner {
    fn drop(&mut self) {
        if let FileState::Open(ref mut b) = self.state {
            // revisit? currently ignores errors on shutdown
            let future = b.flush();
            let rt = pyo3_async_runtimes::tokio::get_runtime();
            let _ = rt.block_on(future);
        }
    }
}

fn apply_py_open_options_to_tokio(py_opts: PyOpenOptions, tokio_opts: &mut tokio::fs::OpenOptions) {
    tokio_opts.read(py_opts.read);
    tokio_opts.write(py_opts.write);
    tokio_opts.append(py_opts.append);
    tokio_opts.truncate(py_opts.truncate);
    tokio_opts.create(py_opts.create);
    tokio_opts.create_new(py_opts.create_new);
}

impl PyAsyncFileInner {
    async fn open(&mut self) -> PyResult<()> {
        let opts = self.open_options;
        let mut open_opts = tokio::fs::OpenOptions::new();
        apply_py_open_options_to_tokio(opts, &mut open_opts);
        let file_res = open_opts.open(&self.path).await;
        let file = file_res
            .map_err(|e| py_io_error!("Failed to open file {}: {}", self.path.display(), e))?;
        self.state = FileState::Open(Box::new(BufStream::new(file)));
        Ok(())
    }

    async fn peek(&mut self, n: usize) -> PyResult<Vec<u8>> {
        let file = self.get_file_mut()?;
        // current position
        let pos = file
            .stream_position()
            .await
            .map_err(|e| py_io_error!("Failed to get stream position: {e}"))?;
        let mut buf = vec![0; n];
        let bytes_read = file
            .read(&mut buf)
            .await
            .map_err(|e| py_io_error!("Failed to read: {e}"))?;
        buf.truncate(bytes_read);
        // seek back to the original position
        file.seek(SeekFrom::Start(pos))
            .await
            .map_err(|e| py_io_error!("Failed to seek: {e}"))?;
        Ok(buf)
    }

    async fn seek(&mut self, seek_from: SeekFrom) -> PyResult<u64> {
        let file = self.get_file_mut()?;
        let r = file
            .seek(seek_from)
            .await
            .map_err(|e| py_io_error!("Failed to seek: {e}"))?;
        file.flush().await?;
        Ok(r)
    }

    async fn flush(&mut self) -> PyResult<()> {
        let file = self.get_file_mut()?;
        file.flush()
            .await
            .map_err(|e| py_io_error!("Failed to flush: {e}"))?;
        Ok(())
    }

    async fn close(&mut self) -> PyResult<()> {
        match std::mem::replace(&mut self.state, FileState::Closed) {
            FileState::Open(mut file) => {
                file.flush()
                    .await
                    .map_err(|e| py_io_error!("Failed to flush: {e}"))?;
            }
            FileState::Closed => {
                // Nothing to flush, no-op
            }
        }
        Ok(())
    }

    async fn tell(&mut self) -> PyResult<u64> {
        let file = self.get_file_mut()?;
        let pos = file
            .stream_position()
            .await
            .map_err(|e| py_io_error!("Failed to get stream position: {e}"))?;
        Ok(pos)
    }

    async fn read_all(&mut self) -> PyResult<Vec<u8>> {
        let file = self.get_file_mut()?;
        let mut buf = Vec::new();
        file.read_to_end(&mut buf)
            .await
            .map_err(|e| py_io_error!("Failed to read to end: {e}"))?;
        Ok(buf)
    }

    async fn read(&mut self, buf: &mut [u8]) -> PyResult<usize> {
        let file = self.get_file_mut()?;
        file.read(buf)
            .await
            .map_err(|e| py_io_error!("Failed to read: {e}"))
    }

    async fn readline(&mut self, size: Option<usize>) -> PyResult<Option<Vec<u8>>> {
        if let Some(s) = size {
            let file = self.get_file_mut()?;
            let mut buf = Vec::new();
            let bytes_read = file
                .read_until(b'\n', &mut buf)
                .await
                .map_err(|e| py_io_error!("Failed to read line: {e}"))?;
            if bytes_read == 0 {
                return Ok(None);
            }
            if buf.len() > s {
                buf.truncate(s);
            }
            Ok(Some(buf))
        } else {
            let file = self.get_file_mut()?;
            let mut buf = Vec::new();
            let bytes_read = file
                .read_until(b'\n', &mut buf)
                .await
                .map_err(|e| py_io_error!("Failed to read line: {e}"))?;
            if bytes_read == 0 {
                Ok(None)
            } else {
                Ok(Some(buf))
            }
        }
    }

    async fn truncate(&mut self, size: Option<usize>) -> PyResult<u64> {
        let file = self.get_file_mut()?;

        // MUST flush before truncating to avoid losing buffered data
        file.flush().await?;

        let size = match size {
            Some(s) => s as u64,
            None => file.stream_position().await?,
        };

        // the actual inner file wrapped by BufStream
        let inner_file = file.get_mut();

        inner_file
            .set_len(size)
            .await
            .map_err(|e| py_io_error!("Failed to truncate: {e}"))?;
        Ok(size)
    }

    async fn write(&mut self, buf: &[u8]) -> PyResult<usize> {
        let file = self.get_file_mut()?;
        file.write_all(buf)
            .await
            .map_err(|e| py_io_error!("Failed to write: {e}"))?;
        Ok(buf.len())
    }

    fn get_file_mut(&mut self) -> PyResult<&mut BufStream<File>> {
        match self.state {
            FileState::Open(ref mut file) => Ok(file),
            FileState::Closed => py_runtime_err!("File is closed; must open first"),
            // FileState::Consumed => Err(PyRuntimeError::new_err(
            //     "File is consumed; cannot be used again",
            // )),
        }
    }

    fn is_closed(&self) -> bool {
        matches!(self.state, FileState::Closed)
    }
}

struct AsyncFileProperties {
    readable: bool,
    writable: bool,
    seekable: bool,
}

impl From<&PyOpenOptions> for AsyncFileProperties {
    fn from(opts: &PyOpenOptions) -> Self {
        Self {
            readable: opts.read,
            writable: opts.write || opts.append,
            seekable: true, // always true for files for now...
        }
    }
}

#[pyclass(name = "AsyncFile", frozen, immutable_type, skip_from_py_object)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
pub struct PyAsyncFile {
    props: AsyncFileProperties,
    inner: Arc<Mutex<PyAsyncFileInner>>,
}

impl PyAsyncFile {
    pub(crate) fn new(p: PathBuf, options: PyOpenOptions) -> Self {
        let inner = PyAsyncFileInner {
            state: FileState::Closed,
            path: p,
            open_options: options,
        };
        Self {
            props: AsyncFileProperties::from(&inner.open_options),
            inner: Arc::new(Mutex::new(inner)),
        }
    }
}

#[pymethods]
impl PyAsyncFile {
    #[new]
    #[pyo3(
        signature = (p, mode=PyOpenMode::default()),
        text_signature = "(path, mode='rb')"
    )]
    fn py_new(p: PathBuf, mode: PyOpenMode) -> PyResult<Self> {
        if !mode.is_binary() {
            pytodo!("Text mode not implemented for AsyncFile");
        }
        Ok(Self::new(p, mode.into()))
    }

    /// This is a coroutine that returns `self` when awaited... so you
    /// can `await` to open the file
    fn __await__(slf: Py<Self>, py: Python<'_>) -> PyResult<Bound<'_, PyAny>> {
        let inner = Arc::clone(&slf.borrow(py).inner);

        // Create an actual coroutine that returns `slf`, then call `__await__()` on it
        let fut = future_into_py(py, async move {
            let mut locked = inner.lock().await;
            locked.open().await?;
            Ok(slf)
        })?;
        // have to then call `__await__()` on the future and return that.
        fut.getattr(intern!(py, "__await__"))?.call0()
    }

    fn __aenter__(slf: Py<Self>, py: Python) -> PyResult<Bound<PyAny>> {
        let inner = Arc::clone(&slf.borrow(py).inner);
        future_into_py(py, async move {
            let mut locked = inner.lock().await;
            locked.open().await?;
            Ok(slf)
        })
    }

    #[pyo3(name = "__aexit__")]
    #[expect(clippy::needless_pass_by_value)]
    fn __aexit__<'py>(
        slf: PyRef<Self>,
        py: Python<'py>,
        _exc_type: Py<PyAny>,
        _exc_value: Py<PyAny>,
        _traceback: Py<PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let inner = Arc::clone(&slf.inner);

        future_into_py(py, async move {
            let mut locked = inner.lock().await;
            match std::mem::replace(&mut locked.state, FileState::Closed) {
                FileState::Open(mut file) => {
                    file.flush()
                        .await
                        .map_err(|e| py_io_error!("Failed to flush file on __aexit__: {e}"))?;
                    // file is flushed and dropped now
                }
                FileState::Closed => {
                    // nothing to flush...
                }
            }

            Ok(())
        })
    }

    fn __aiter__(this: PyRef<Self>) -> PyRef<Self> {
        this
    }

    fn __anext__<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let inner = Arc::clone(&self.inner);
        future_into_py(py, async move {
            let mut locked = inner.lock().await;
            let line = locked.readline(None).await?;
            match line {
                Some(line) => Ok(line),
                None => py_stop_async_iteration_err!("End of stream"),
            }
        })
    }

    fn close<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let inner = Arc::clone(&self.inner);
        future_into_py(py, async move {
            let mut locked = inner.lock().await;
            locked.close().await?;
            Ok(())
        })
    }

    #[getter]
    fn closed(&self) -> bool {
        let locked = self.inner.blocking_lock();
        locked.is_closed()
    }

    #[expect(clippy::unused_self)]
    fn isatty<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        future_into_py::<_, Py<PyAny>>(py, async move { pytodo!("isatty() not implemented") })
    }

    fn flush<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let inner = Arc::clone(&self.inner);
        future_into_py(py, async move {
            let mut locked = inner.lock().await;
            locked.flush().await?;
            Ok(())
        })
    }

    fn open<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let inner = Arc::clone(&self.inner);

        future_into_py(py, async move {
            let mut locked = inner.lock().await;
            locked.open().await?;
            Ok(())
        })
    }

    #[pyo3(signature = (size = 1, /))]
    fn peek<'py>(&'py self, py: Python<'py>, size: usize) -> PyResult<Bound<'py, PyAny>> {
        let inner = Arc::clone(&self.inner);
        future_into_py(py, async move {
            let mut locked = inner.lock().await;
            let buf = locked.peek(size).await?;
            Ok(ryo3_bytes::PyBytes::from(buf))
        })
    }

    #[pyo3(
        signature = (size = None, /),
    )]
    fn read<'py>(&self, py: Python<'py>, size: Option<usize>) -> PyResult<Bound<'py, PyAny>> {
        let inner = Arc::clone(&self.inner);

        future_into_py(py, async move {
            let mut file = inner.lock().await;
            if let Some(s) = size {
                let mut buf = vec![0u8; s];
                let n = file.read(&mut buf).await?;
                buf.truncate(n);
                Ok(ryo3_bytes::PyBytes::from(buf))
            } else {
                let r = file.read_all().await?;
                Ok(ryo3_bytes::PyBytes::from(r))
            }
        })
    }

    fn readable(&self) -> bool {
        self.props.readable
    }

    fn readall<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let inner = Arc::clone(&self.inner);
        future_into_py(py, async move {
            let mut file = inner.lock().await;
            let r = file.read_all().await?;
            let rybytes = ryo3_bytes::PyBytes::from(r);
            Ok(rybytes)
        })
    }

    #[pyo3(signature = (size = None, /))]
    fn readline<'py>(&self, py: Python<'py>, size: Option<usize>) -> PyResult<Bound<'py, PyAny>> {
        let inner = Arc::clone(&self.inner);
        future_into_py(py, async move {
            let mut locked = inner.lock().await;
            let line = locked.readline(size).await?;
            match line {
                Some(line) => Ok(Some(ryo3_bytes::PyBytes::from(line))),
                None => Ok(None),
            }
        })
    }

    /// Return a list of lines from the stream.
    ///
    /// hint can be specified to control the number of lines read: no more
    /// lines will be read if the total size (in bytes/characters) of all
    /// lines so far exceeds hint.
    #[pyo3(signature = (hint = None, /))]
    fn readlines<'py>(&self, py: Python<'py>, hint: Option<usize>) -> PyResult<Bound<'py, PyAny>> {
        let inner = Arc::clone(&self.inner);
        if let Some(hint) = hint {
            future_into_py(py, async move {
                let mut locked = inner.lock().await;
                let mut lines = Vec::new();
                let mut total_size = 0;
                while let Ok(Some(line)) = locked.readline(None).await {
                    total_size += line.len();
                    lines.push(line);
                    if total_size > hint {
                        break;
                    }
                }
                Ok(lines)
            })
        } else {
            future_into_py(py, async move {
                let mut locked = inner.lock().await;
                let mut lines = Vec::new();
                while let Ok(Some(line)) = locked.readline(None).await {
                    lines.push(line);
                }
                Ok(lines)
            })
        }
    }

    #[pyo3(
        signature = (offset, whence=0, /),
        text_signature = "(self, offset, whence=os.SEEK_SET, /)")
    ]
    fn seek<'py>(
        &'py self,
        py: Python<'py>,
        offset: i64,
        whence: usize,
    ) -> PyResult<Bound<'py, PyAny>> {
        let pos = match whence {
            0 => {
                let offset = offset
                    .try_into()
                    .map_err(|e| py_io_error!("Offset out of range: {e}"))?;
                SeekFrom::Start(offset)
            }
            1 => SeekFrom::Current(offset as _),
            2 => SeekFrom::End(offset as _),
            other => {
                return Err(py_io_error!("Invalid value for whence in seek: {other}"));
            }
        };
        let inner = Arc::clone(&self.inner);

        future_into_py(py, async move {
            let mut locked = inner.lock().await;
            locked.seek(pos).await?;
            Ok(())
        })
    }

    fn seekable(&self) -> bool {
        // TODO MAKE NOT ALWAYS TRUE???
        self.props.seekable
    }

    fn tell<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let inner = Arc::clone(&self.inner);
        future_into_py(py, async move {
            let mut locked = inner.lock().await;
            let pos = locked.tell().await?;
            Ok(pos)
        })
    }

    #[pyo3(signature = (pos = None, /))]
    fn truncate<'py>(&self, py: Python<'py>, pos: Option<usize>) -> PyResult<Bound<'py, PyAny>> {
        let inner = Arc::clone(&self.inner);
        future_into_py(py, async move {
            let mut locked = inner.lock().await;
            let size = locked.truncate(pos).await?;
            Ok(size)
        })
    }

    #[pyo3(signature = (buffer, /))]
    fn write<'py>(
        &self,
        py: Python<'py>,
        buffer: ryo3_bytes::PyBytes,
    ) -> PyResult<Bound<'py, PyAny>> {
        let inner = Arc::clone(&self.inner);
        future_into_py(py, async move {
            let mut locked = inner.lock().await;
            locked.write(buffer.as_ref()).await
        })
    }

    fn writable(&self) -> bool {
        self.props.writable
    }
}
