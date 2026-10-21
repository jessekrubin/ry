use std::io::SeekFrom;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use pyo3::prelude::*;
use ryo3_bytes::RyBytes;
use ryo3_core::types::{PyOpenMode, PyOpenOptions};
use ryo3_macro_rules::{
    py_runtime_err, py_stop_async_iteration_err, py_value_error, pytodo, pytodo_pyerr,
};
use ryo3_tokio_rt::{RyCoroutine, failed_coro, get_tokio_runtime, tokio_coro};
use tokio::fs::File;
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncSeekExt, AsyncWriteExt, BufStream};
use tokio::sync::Mutex;

// max up-front alloc for `read(n)`/`peek(n)`; reads grow w/ the data
const READ_PREALLOC_CAP: usize = 8 * 1024 * 1024;

enum FileState {
    Closed,
    Open(Box<BufStream<File>>),
}

/// the open file + its buffer (flushed on drop)
struct AsyncFileHandle {
    state: FileState,
    path: PathBuf,
    open_options: PyOpenOptions,
}

impl Drop for AsyncFileHandle {
    fn drop(&mut self) {
        if let FileState::Open(mut b) = std::mem::replace(&mut self.state, FileState::Closed) {
            // revisit? currently ignores errors on shutdown
            if let Ok(handle) = tokio::runtime::Handle::try_current() {
                // on a tokio thread can panic here w/ `block_on`
                handle.spawn(async move {
                    let _ = b.flush().await;
                });
            } else {
                let _ = get_tokio_runtime().block_on(b.flush());
            }
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

impl AsyncFileHandle {
    async fn open(&mut self) -> PyResult<()> {
        if !self.is_closed() {
            // DO NOT REOPEN: wbuf drops and truncates if opened with 'w'
            return Ok(());
        }
        let opts = self.open_options;
        let mut open_opts = tokio::fs::OpenOptions::new();
        apply_py_open_options_to_tokio(opts, &mut open_opts);
        let file = open_opts
            .open(&self.path)
            .await
            .map_err(|e| map_py_io_err_with_path(e, &self.path))?;
        self.state = FileState::Open(Box::new(BufStream::new(file)));
        Ok(())
    }

    /// to `n`; never moves the position
    async fn peek(&mut self, n: usize) -> PyResult<Vec<u8>> {
        let file = self.get_file_mut()?;
        let buf = file.fill_buf().await?;
        Ok(buf[..n.min(buf.len())].to_vec())
    }

    async fn seek(&mut self, seek_from: SeekFrom) -> PyResult<u64> {
        let file = self.get_file_mut_flush().await?;
        let r = file.seek(seek_from).await?;
        file.flush().await?;
        Ok(r)
    }

    async fn flush(&mut self) -> PyResult<()> {
        let file = self.get_file_mut()?;
        file.flush().await?;
        Ok(())
    }

    async fn close(&mut self) -> PyResult<()> {
        let FileState::Open(file) = &mut self.state else {
            return Ok(());
        };
        let flushed = file.flush().await;
        self.state = FileState::Closed;
        Ok(flushed?)
    }

    async fn tell(&mut self) -> PyResult<u64> {
        let file = self.get_file_mut_flush().await?;
        let pos = file.stream_position().await?;
        Ok(pos)
    }

    async fn read_all(&mut self) -> PyResult<Vec<u8>> {
        let file = self.get_file_mut()?;
        let mut buf = Vec::new();
        file.read_to_end(&mut buf).await?;
        Ok(buf)
    }

    /// read up to `n` bytes (or eof) like `BufferedReader.read(n)`; buffer
    /// grows w/ the data read
    async fn read_up_to(&mut self, n: usize) -> PyResult<Vec<u8>> {
        let file = self.get_file_mut()?;
        let mut buf = Vec::with_capacity(n.min(READ_PREALLOC_CAP));
        let limit = u64::try_from(n).unwrap_or(u64::MAX);
        file.take(limit).read_to_end(&mut buf).await?;
        Ok(buf)
    }

    async fn readline(&mut self, size: Option<usize>) -> PyResult<Option<Vec<u8>>> {
        let file = self.get_file_mut()?;
        let mut buf = Vec::new();
        let bytes_read = match size {
            // python returns b"" for size=0
            Some(0) => return Ok(Some(buf)),
            // keep the read below the requested size so rest of the line stays
            // buffered for the next call
            Some(s) => {
                (&mut *file)
                    .take(s as u64)
                    .read_until(b'\n', &mut buf)
                    .await
            }
            None => file.read_until(b'\n', &mut buf).await,
        }?;
        if bytes_read == 0 {
            Ok(None)
        } else {
            Ok(Some(buf))
        }
    }

    async fn truncate(&mut self, size: Option<u64>) -> PyResult<u64> {
        let file = self.get_file_mut()?;

        // MUST flush before truncating to avoid losing buffered data
        file.flush().await?;

        let size = match size {
            Some(s) => s,
            None => file.stream_position().await?,
        };

        // the actual inner file wrapped by BufStream
        let inner_file = file.get_mut();

        inner_file.set_len(size).await?;
        Ok(size)
    }

    async fn write(&mut self, buf: &[u8]) -> PyResult<usize> {
        let file = self.get_file_mut()?;
        file.write_all(buf).await?;
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

    /// file AND flush
    async fn get_file_mut_flush(&mut self) -> PyResult<&mut BufStream<File>> {
        let file = self.get_file_mut()?;
        file.get_mut().flush().await?;
        Ok(file)
    }

    fn is_closed(&self) -> bool {
        matches!(self.state, FileState::Closed)
    }

    /// `readline` until eof or total lines gt `hint`
    async fn readlines(&mut self, hint: Option<usize>) -> PyResult<Vec<RyBytes>> {
        let mut lines = Vec::new();
        let mut total_size = 0;
        while let Some(line) = self.readline(None).await? {
            total_size += line.len();
            lines.push(RyBytes::from(line));
            if hint.is_some_and(|hint| total_size > hint) {
                break;
            }
        }
        Ok(lines)
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

/// shared inner state for the pyclass w/o propery cache
struct AsyncFileInner {
    /// open state of file to avoid locking to check if open/closed
    closed: AtomicBool,
    handle: Mutex<AsyncFileHandle>,
}

#[pyclass(name = "AsyncFile", frozen, immutable_type)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
pub struct PyAsyncFile {
    props: AsyncFileProperties,
    inner: Arc<AsyncFileInner>,
}

impl PyAsyncFile {
    pub(crate) fn new(p: PathBuf, options: PyOpenOptions) -> Self {
        let handle = AsyncFileHandle {
            state: FileState::Closed,
            path: p,
            open_options: options,
        };
        Self {
            props: AsyncFileProperties::from(&handle.open_options),
            inner: Arc::new(AsyncFileInner {
                closed: AtomicBool::new(true),
                handle: Mutex::new(handle),
            }),
        }
    }

    /// opens (if not already open) and resolves to the file itself
    fn open_pycoro<'py>(
        slf: Py<Self>,
        py: Python<'py>,
        qualname: &'static str,
    ) -> PyResult<Bound<'py, RyCoroutine>> {
        tokio_coro(py, qualname, async move {
            let inner = &slf.get().inner;
            let mut locked = inner.handle.lock().await;
            locked.open().await?;
            inner.closed.store(false, Ordering::Release);
            drop(locked);
            Ok(slf)
        })
    }

    fn close_pycoro<'py>(
        &self,
        py: Python<'py>,
        qualname: &'static str,
    ) -> PyResult<Bound<'py, RyCoroutine>> {
        let inner = Arc::clone(&self.inner);
        tokio_coro(py, qualname, async move {
            let mut locked = inner.handle.lock().await;
            let result = locked.close().await;
            inner.closed.store(locked.is_closed(), Ordering::Release);
            result
        })
    }
}

#[pymethods]
impl PyAsyncFile {
    #[new]
    #[pyo3(
        signature = (p, mode = PyOpenMode::default()),
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
    fn __await__(slf: Py<Self>, py: Python<'_>) -> PyResult<Bound<'_, RyCoroutine>> {
        Self::open_pycoro(slf, py, "AsyncFile.__await__")
    }

    fn __aenter__(slf: Py<Self>, py: Python<'_>) -> PyResult<Bound<'_, RyCoroutine>> {
        Self::open_pycoro(slf, py, "AsyncFile.__aenter__")
    }

    fn __aexit__<'py>(
        &self,
        py: Python<'py>,
        _exc_type: &Bound<'py, PyAny>,
        _exc_value: &Bound<'py, PyAny>,
        _traceback: &Bound<'py, PyAny>,
    ) -> PyResult<Bound<'py, RyCoroutine>> {
        self.close_pycoro(py, "AsyncFile.__aexit__")
    }

    fn __aiter__(this: PyRef<Self>) -> PyRef<Self> {
        this
    }

    #[expect(clippy::single_match_else, reason = "actually cleaner")]
    fn __anext__<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, RyCoroutine>> {
        let inner = Arc::clone(&self.inner);
        tokio_coro(py, "AsyncFile.__anext__", async move {
            match inner.handle.lock().await.readline(None).await? {
                Some(line) => Ok(RyBytes::from(line)),
                None => py_stop_async_iteration_err!("End of stream"),
            }
        })
    }

    fn close<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, RyCoroutine>> {
        self.close_pycoro(py, "AsyncFile.close")
    }

    #[getter]
    fn closed(&self) -> bool {
        self.inner.closed.load(Ordering::Acquire)
    }

    #[expect(clippy::unused_self)]
    fn isatty<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, RyCoroutine>> {
        failed_coro(
            py,
            "AsyncFile.isatty",
            pytodo_pyerr!("isatty() not implemented"),
        )
    }

    fn flush<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, RyCoroutine>> {
        let inner = Arc::clone(&self.inner);
        tokio_coro(py, "AsyncFile.flush", async move {
            inner.handle.lock().await.flush().await
        })
    }

    fn open<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, RyCoroutine>> {
        let inner = Arc::clone(&self.inner);
        tokio_coro(py, "AsyncFile.open", async move {
            let mut locked = inner.handle.lock().await;
            locked.open().await?;
            inner.closed.store(false, Ordering::Release);
            Ok(())
        })
    }

    #[pyo3(signature = (size = 1, /))]
    fn peek<'py>(&self, py: Python<'py>, size: usize) -> PyResult<Bound<'py, RyCoroutine>> {
        let inner = Arc::clone(&self.inner);
        tokio_coro(py, "AsyncFile.peek", async move {
            let buf = inner.handle.lock().await.peek(size).await?;
            Ok(RyBytes::from(buf))
        })
    }

    #[pyo3(signature = (size = None, /))]
    fn read<'py>(&self, py: Python<'py>, size: Option<usize>) -> PyResult<Bound<'py, RyCoroutine>> {
        let inner = Arc::clone(&self.inner);
        tokio_coro(py, "AsyncFile.read", async move {
            let mut file = inner.handle.lock().await;
            let buf = match size {
                Some(s) => file.read_up_to(s).await?,
                None => file.read_all().await?,
            };
            Ok(RyBytes::from(buf))
        })
    }

    fn readable(&self) -> bool {
        self.props.readable
    }

    fn readall<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, RyCoroutine>> {
        let inner = Arc::clone(&self.inner);
        tokio_coro(py, "AsyncFile.readall", async move {
            let buf = inner.handle.lock().await.read_all().await?;
            Ok(RyBytes::from(buf))
        })
    }

    #[pyo3(signature = (size = None, /))]
    fn readline<'py>(
        &self,
        py: Python<'py>,
        size: Option<usize>,
    ) -> PyResult<Bound<'py, RyCoroutine>> {
        let inner = Arc::clone(&self.inner);
        tokio_coro(py, "AsyncFile.readline", async move {
            let line = inner.handle.lock().await.readline(size).await?;
            Ok(line.map(RyBytes::from))
        })
    }

    /// Return a list of lines from the stream.
    ///
    /// hint can be specified to control the number of lines read: no more
    /// lines will be read if the total size (in bytes/characters) of all
    /// lines so far exceeds hint.
    #[pyo3(signature = (hint = None, /))]
    fn readlines<'py>(
        &self,
        py: Python<'py>,
        hint: Option<usize>,
    ) -> PyResult<Bound<'py, RyCoroutine>> {
        let inner = Arc::clone(&self.inner);
        tokio_coro(py, "AsyncFile.readlines", async move {
            inner.handle.lock().await.readlines(hint).await
        })
    }

    #[pyo3(
        signature = (offset, whence = 0, /),
        text_signature = "(self, offset, whence=os.SEEK_SET, /)")
    ]
    fn seek<'py>(
        &self,
        py: Python<'py>,
        offset: i64,
        whence: usize,
    ) -> PyResult<Bound<'py, RyCoroutine>> {
        let pos = match whence {
            0 => {
                let offset = offset
                    .try_into()
                    .map_err(|_| py_value_error!("negative seek position {offset}"))?;
                SeekFrom::Start(offset)
            }
            1 => SeekFrom::Current(offset as _),
            2 => SeekFrom::End(offset as _),
            other => {
                return Err(py_value_error!(
                    "invalid whence ({other}, should be 0, 1 or 2)"
                ));
            }
        };
        let inner = Arc::clone(&self.inner);
        tokio_coro(py, "AsyncFile.seek", async move {
            inner.handle.lock().await.seek(pos).await
        })
    }

    fn seekable(&self) -> bool {
        // TODO MAKE NOT ALWAYS TRUE???
        self.props.seekable
    }

    fn tell<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, RyCoroutine>> {
        let inner = Arc::clone(&self.inner);
        tokio_coro(py, "AsyncFile.tell", async move {
            inner.handle.lock().await.tell().await
        })
    }

    #[pyo3(signature = (pos = None, /))]
    fn truncate<'py>(
        &self,
        py: Python<'py>,
        pos: Option<u64>,
    ) -> PyResult<Bound<'py, RyCoroutine>> {
        let inner = Arc::clone(&self.inner);
        tokio_coro(py, "AsyncFile.truncate", async move {
            inner.handle.lock().await.truncate(pos).await
        })
    }

    #[pyo3(signature = (buffer, /))]
    fn write<'py>(&self, py: Python<'py>, buffer: RyBytes) -> PyResult<Bound<'py, RyCoroutine>> {
        let inner = Arc::clone(&self.inner);
        tokio_coro(py, "AsyncFile.write", async move {
            inner.handle.lock().await.write(buffer.as_ref()).await
        })
    }

    fn writable(&self) -> bool {
        self.props.writable
    }
}

/// map `std::io::Error` to Python `OSError` with a filepath str
///
/// adapted from pyo3 src
#[must_use]
pub(crate) fn map_py_io_err_with_path(e: std::io::Error, path: &Path) -> PyErr {
    let Some(code) = e.raw_os_error() else {
        return PyErr::from(e);
    };
    let msg = e.to_string();
    let strerror = msg
        .strip_suffix(&format!(" (os error {code})"))
        .unwrap_or(&msg)
        .to_owned();
    let filename = path.as_os_str().to_owned();
    #[cfg(windows)]
    let args = (0, strerror, filename, code);
    #[cfg(not(windows))]
    let args = (code, strerror, filename);
    pyo3::exceptions::PyOSError::new_err(args)
}
