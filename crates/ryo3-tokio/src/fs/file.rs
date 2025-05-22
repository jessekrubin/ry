use pyo3::exceptions::{PyIOError, PyRuntimeError, PyStopAsyncIteration};
use pyo3::prelude::*;

use pyo3::intern;
use pyo3_async_runtimes::tokio::future_into_py;
use std::io::SeekFrom;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::fs::File;
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt};
use tokio::io::{AsyncSeekExt, BufStream};
use tokio::sync::Mutex;

#[derive(Debug, Clone, Copy)]
#[expect(clippy::struct_excessive_bools)]
struct OpenOptions {
    append: bool,
    create: bool,
    create_new: bool,
    read: bool,
    truncate: bool,
    write: bool,
}

enum FileState {
    Closed,
    Open(BufStream<File>),
    // Consumed,
}
struct PyAsyncFileInner {
    state: FileState,
    path: PathBuf,
    open_options: OpenOptions,
}

impl Drop for PyAsyncFileInner {
    fn drop(&mut self) {
        if let FileState::Open(ref mut b) = self.state {
            // best‑effort, ignore errors on shutdown
            let _ = futures::executor::block_on(b.flush());
        }
    }
}

impl PyAsyncFileInner {
    async fn open(&mut self) -> PyResult<()> {
        let opts = &self.open_options;
        let mut open_opts = tokio::fs::OpenOptions::new();
        opts.apply_to(&mut open_opts);
        let file_res = open_opts.open(&self.path).await;
        let file = file_res.map_err(|e| {
            PyIOError::new_err(format!(
                "Failed to open file {}: {}",
                self.path.display(),
                e
            ))
        })?;
        self.state = FileState::Open(BufStream::new(file));
        Ok(())
    }

    async fn peek(&mut self, n: usize) -> PyResult<Vec<u8>> {
        let file = self.get_file_mut()?;
        // current position
        let pos = file
            .stream_position()
            .await
            .map_err(|e| PyIOError::new_err(e.to_string()))?;
        let mut buf = vec![0; n];
        let bytes_read = file
            .read(&mut buf)
            .await
            .map_err(|e| PyIOError::new_err(format!("Failed to read: {e}")))?;
        buf.truncate(bytes_read);
        // seek back to the original position
        file.seek(SeekFrom::Start(pos))
            .await
            .map_err(|e| PyIOError::new_err(format!("Failed to seek: {e}")))?;
        Ok(buf)
    }

    async fn seek(&mut self, seek_from: SeekFrom) -> PyResult<u64> {
        let file = self.get_file_mut()?;
        let r = file
            .seek(seek_from)
            .await
            .map_err(|e| PyIOError::new_err(e.to_string()))?;
        file.flush().await?;
        Ok(r)
    }

    // TODO fix this if we ever swap out bufstream for bufwriter/bufreader?
    #[expect(clippy::unused_async)]
    async fn seekable(&mut self) -> PyResult<bool> {
        Ok(true)
    }

    async fn flush(&mut self) -> PyResult<()> {
        let file = self.get_file_mut()?;
        file.flush()
            .await
            .map_err(|e| PyIOError::new_err(e.to_string()))?;
        Ok(())
    }

    async fn close(&mut self) -> PyResult<()> {
        match std::mem::replace(&mut self.state, FileState::Closed) {
            FileState::Open(mut file) => {
                file.flush()
                    .await
                    .map_err(|e| PyIOError::new_err(e.to_string()))?;
                // File is flushed and dropped now
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
            .map_err(|e| PyIOError::new_err(e.to_string()))?;
        Ok(pos)
    }

    async fn read_all(&mut self) -> PyResult<Vec<u8>> {
        let file = self.get_file_mut()?;
        let mut buf = Vec::new();
        file.read_to_end(&mut buf)
            .await
            .map_err(|e| PyIOError::new_err(e.to_string()))?;
        Ok(buf)
    }

    async fn read(&mut self, buf: &mut [u8]) -> PyResult<usize> {
        let file = self.get_file_mut()?;
        file.read(buf)
            .await
            .map_err(|e| PyIOError::new_err(e.to_string()))
    }

    async fn readline(&mut self) -> PyResult<Option<Vec<u8>>> {
        let file = self.get_file_mut()?;
        let mut buf = Vec::new();
        let bytes_read = file
            .read_until(b'\n', &mut buf)
            .await
            .map_err(|e| PyIOError::new_err(format!("Failed to read line: {e}")))?;
        if bytes_read == 0 {
            Ok(None)
        } else {
            Ok(Some(buf))
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
            .map_err(|e| PyIOError::new_err(format!("Failed to truncate: {e}")))?;
        Ok(size)
    }

    async fn write(&mut self, buf: &[u8]) -> PyResult<usize> {
        let file = self.get_file_mut()?;
        file.write_all(buf)
            .await
            .map_err(|e| PyIOError::new_err(e.to_string()))?;
        Ok(buf.len())
    }

    #[expect(clippy::unused_async)]
    async fn writable(&mut self) -> PyResult<bool> {
        Ok(self.open_options.write)
    }

    #[expect(clippy::unused_async)]
    async fn readable(&mut self) -> PyResult<bool> {
        Ok(self.open_options.read)
    }

    fn get_file_mut(&mut self) -> PyResult<&mut BufStream<File>> {
        match self.state {
            FileState::Open(ref mut file) => Ok(file),
            FileState::Closed => Err(PyRuntimeError::new_err("File is closed; must open first")),
            // FileState::Consumed => Err(PyRuntimeError::new_err(
            //     "File is consumed; cannot be used again",
            // )),
        }
    }

    fn is_closed(&self) -> bool {
        matches!(self.state, FileState::Closed)
    }
}

#[pyclass(name = "AsyncFile", module = "ry.ryo3", frozen)]
pub struct PyAsyncFile {
    inner: Arc<Mutex<PyAsyncFileInner>>,
}

impl PyAsyncFile {
    pub fn new<M: AsRef<str>>(p: PathBuf, mode: Option<M>) -> PyResult<Self> {
        let path = p;
        let mode: String = mode.map_or_else(|| "r".to_string(), |m| m.as_ref().to_string());
        let open_options = OpenOptions::from_mode_string(&mode)?;
        let inner = PyAsyncFileInner {
            state: FileState::Closed,
            path,
            open_options,
        };
        Ok(PyAsyncFile {
            inner: Arc::new(Mutex::new(inner)),
        })
    }
}

#[pymethods]
impl PyAsyncFile {
    #[new]
    fn py_new(p: PathBuf, mode: Option<&str>) -> PyResult<Self> {
        PyAsyncFile::new(p, mode)
    }

    fn open<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let inner = Arc::clone(&self.inner);

        future_into_py(py, async move {
            let mut locked = inner.lock().await;
            locked.open().await?;
            Ok(())
        })
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

    #[pyo3(name = "__aexit__")]
    #[expect(clippy::needless_pass_by_value)]
    fn __aexit__<'py>(
        slf: PyRef<Self>,
        py: Python<'py>,
        _exc_type: PyObject,
        _exc_value: PyObject,
        _traceback: PyObject,
    ) -> PyResult<Bound<'py, PyAny>> {
        let inner = Arc::clone(&slf.inner);

        future_into_py(py, async move {
            let mut locked = inner.lock().await;
            match std::mem::replace(&mut locked.state, FileState::Closed) {
                FileState::Open(mut file) => {
                    file.flush().await.map_err(PyErr::from)?;
                    // File is flushed and dropped now
                }
                FileState::Closed => {
                    // Nothing to flush, no-op
                } // FileState::Consumed => {
                  //     return Err(PyRuntimeError::new_err("File already closed"));
                  // }
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
            // let inner_ref = locked
            //     .as_mut()
            //     .ok_or_else(|| pyo3::exceptions::PyRuntimeError::new_err("Already consumed"))?;
            let line = locked.readline().await?;
            match line {
                Some(line) => Ok(line),
                None => Err(PyStopAsyncIteration::new_err("End of stream")),
            }
        })
    }

    fn flush<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let inner = Arc::clone(&self.inner);
        future_into_py(py, async move {
            let mut locked = inner.lock().await;
            locked.flush().await?;
            Ok(())
        })
    }

    #[pyo3(signature = (n = 1, /))]
    fn peek<'py>(&'py self, py: Python<'py>, n: usize) -> PyResult<Bound<'py, PyAny>> {
        let inner = Arc::clone(&self.inner);
        future_into_py(py, async move {
            let mut locked = inner.lock().await;
            let buf = locked.peek(n).await?;
            Ok(ryo3_bytes::PyBytes::from(buf))
        })
    }

    #[pyo3(
        signature = (size = None),
    )]
    fn read<'py>(&self, py: Python<'py>, size: Option<usize>) -> PyResult<Bound<'py, PyAny>> {
        let inner = Arc::clone(&self.inner);

        future_into_py(py, async move {
            let mut file = inner.lock().await;
            if let Some(s) = size {
                let mut buf = vec![0u8; s];
                let n = file
                    .read(&mut buf)
                    .await
                    .map_err(|e| PyIOError::new_err(format!("Failed to read: {e}")))?;
                buf.truncate(n);
                Ok(ryo3_bytes::PyBytes::from(buf))
            } else {
                let r = file.read_all().await?;
                Ok(ryo3_bytes::PyBytes::from(r))
            }
        })
    }

    fn readable<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let inner = Arc::clone(&self.inner);
        future_into_py(py, async move {
            let mut locked = inner.lock().await;
            locked.readable().await?;
            Ok(())
        })
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

    fn readline<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let inner = Arc::clone(&self.inner);
        future_into_py(py, async move {
            let mut locked = inner.lock().await;
            let line = locked.readline().await?;
            match line {
                Some(line) => Ok(Some(ryo3_bytes::PyBytes::from(line))),
                None => Ok(None),
            }
        })
    }

    fn readlines<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let inner = Arc::clone(&self.inner);
        future_into_py(py, async move {
            let mut locked = inner.lock().await;
            let mut lines = Vec::new();
            while let Ok(Some(line)) = locked.readline().await {
                lines.push(line);
            }

            Ok(lines)
        })
    }

    #[pyo3(
        signature = (offset, whence=0, /),
        text_signature = "(offset, whence=os.SEEK_SET, /)")
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
                    .map_err(|_| PyIOError::new_err("Offset out of range"))?;
                SeekFrom::Start(offset)
            }
            1 => SeekFrom::Current(offset as _),
            2 => SeekFrom::End(offset as _),
            other => {
                return Err(PyIOError::new_err(format!(
                    "Invalid value for whence in seek: {other}"
                )))
            }
        };
        let inner = Arc::clone(&self.inner);

        future_into_py(py, async move {
            let mut locked = inner.lock().await;
            locked.seek(pos).await?;
            Ok(())
        })
    }

    fn seekable<'py>(&'py self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let inner = Arc::clone(&self.inner);
        future_into_py(py, async move {
            let mut locked = inner.lock().await;
            locked.seekable().await?;
            Ok(())
        })
    }

    fn tell<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let inner = Arc::clone(&self.inner);
        future_into_py(py, async move {
            let mut locked = inner.lock().await;
            let pos = locked.tell().await?;
            Ok(pos)
        })
    }

    #[pyo3(signature = (size = None))]
    fn truncate<'py>(&self, py: Python<'py>, size: Option<usize>) -> PyResult<Bound<'py, PyAny>> {
        let inner = Arc::clone(&self.inner);
        future_into_py(py, async move {
            let mut locked = inner.lock().await;
            let size = locked.truncate(size).await?;
            Ok(size)
        })
    }

    fn write<'py>(
        &self,
        py: Python<'py>,
        data: ryo3_bytes::PyBytes,
    ) -> PyResult<Bound<'py, PyAny>> {
        let inner = Arc::clone(&self.inner);
        future_into_py(py, async move {
            let mut locked = inner.lock().await;
            locked.write(data.as_ref()).await
        })
    }

    fn writable<'py>(&'py self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let inner = Arc::clone(&self.inner);
        future_into_py(py, async move {
            let mut locked = inner.lock().await;
            locked.writable().await?;
            Ok(())
        })
    }
}

impl OpenOptions {
    pub(crate) fn from_mode_string(mode: &str) -> PyResult<Self> {
        use pyo3::exceptions::PyValueError;

        let mut opts = OpenOptions {
            read: false,
            write: false,
            append: false,
            create: false,
            truncate: false,
            create_new: false,
        };

        match mode {
            "r" | "rb" => {
                opts.read = true;
            }
            "r+" | "rb+" => {
                opts.read = true;
                opts.write = true;
            }
            "w" | "wb" => {
                opts.write = true;
                opts.create = true;
                opts.truncate = true;
            }
            "w+" | "wb+" => {
                opts.read = true;
                opts.write = true;
                opts.create = true;
                opts.truncate = true;
            }
            "a" | "ab" => {
                opts.write = true;
                opts.append = true;
                opts.create = true;
            }
            "a+" | "ab+" => {
                opts.read = true;
                opts.write = true;
                opts.append = true;
                opts.create = true;
            }
            "x" => {
                opts.write = true;
                opts.create_new = true;
            }
            "x+" => {
                opts.read = true;
                opts.write = true;
                opts.create_new = true;
            }
            _ => {
                return Err(PyValueError::new_err(format!(
                    "Unsupported open mode: {mode:?}"
                )))
            }
        }

        Ok(opts)
    }

    pub(crate) fn apply_to(self, open: &mut tokio::fs::OpenOptions) {
        open.read(self.read);
        open.write(self.write);
        open.append(self.append);
        open.create(self.create);
        open.truncate(self.truncate);
        open.create_new(self.create_new);
    }
}
