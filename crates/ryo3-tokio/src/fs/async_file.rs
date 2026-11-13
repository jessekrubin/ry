use std::fs::OpenOptions;
use std::io::{self, SeekFrom};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::{AtomicU8, Ordering};

use pyo3::prelude::*;
use ryo3_bytes::RyBytes;
use ryo3_core::PyIoResultExt;
use ryo3_core::types::{PyOpenMode, PyOpenOptions};
use ryo3_macro_rules::{
    py_runtime_error, py_stop_async_iteration_err, py_value_error, pytodo, pytodo_pyerr,
};
use ryo3_std::fs::{BUF_SIZE, BufferedFile};
use ryo3_tokio_rt::{RyCoroutine, failed_coro, get_ry_tokio_runtime, pyloop_coro};
use tokio::sync::{Mutex, OwnedMutexGuard};

pyo3::import_exception!(io, UnsupportedOperation);

/// open/closed + seekable, readable w/o the lock (for the sync getters)
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
enum FileStatus {
    Closed = 0,
    Seekable = 1,
    Unseekable = 2,
}

/// `BufferedFile` behind a fifo async lock (`tokio::sync::Mutex`)
///
/// every op: take the lock, try the buffer right on the loop thread, else one
/// blocking hop that owns the lock till it's done ~ a cancelled await can't
/// leave a half done op behind
#[derive(Clone)]
struct AsyncBufferedFile {
    file: Arc<Mutex<Option<BufferedFile>>>,
    /// `FileStatus` of `file`; only written under the lock (open/close)
    status: Arc<AtomicU8>,
}

impl AsyncBufferedFile {
    fn new() -> Self {
        Self {
            file: Arc::new(Mutex::new(None)),
            status: Arc::new(AtomicU8::new(FileStatus::Closed as u8)),
        }
    }

    fn status(&self) -> FileStatus {
        match self.status.load(Ordering::Acquire) {
            1 => FileStatus::Seekable,
            2 => FileStatus::Unseekable,
            _ => FileStatus::Closed,
        }
    }

    async fn lock(&self) -> FileLock {
        FileLock(Arc::clone(&self.file).lock_owned().await)
    }

    /// open if not open already (reopening w/ 'w' would truncate)
    async fn open(&self, path: Arc<Path>, opts: PyOpenOptions) -> PyResult<()> {
        let mut lock = self.lock().await;
        if lock.0.is_some() {
            return Ok(());
        }
        let status = Arc::clone(&self.status);
        get_ry_tokio_runtime()
            .py_spawn_blocking(move || {
                let file = BufferedFile::open(
                    &path,
                    OpenOptions::new()
                        .read(opts.read)
                        .write(opts.write)
                        .append(opts.append)
                        .truncate(opts.truncate)
                        .create(opts.create)
                        .create_new(opts.create_new),
                    BUF_SIZE,
                )
                .map_py_io_err_with_path(&path)?;
                let new_status = if file.seekable() {
                    FileStatus::Seekable
                } else {
                    FileStatus::Unseekable
                };
                status.store(new_status as u8, Ordering::Release);
                *lock.0 = Some(file);
                Ok(())
            })
            .await?
    }

    /// always ends up closed, even if the final flush fails
    async fn close(&self) -> PyResult<()> {
        let mut lock = self.lock().await;
        self.status
            .store(FileStatus::Closed as u8, Ordering::Release);
        let Some(mut file) = lock.0.take() else {
            return Ok(());
        };
        // nothing to flush ~ drop it here, no blocking hop
        if !file.has_pending_writes() {
            return Ok(());
        }
        // `lock` rides along so nothing reopens the path mid-flush
        get_ry_tokio_runtime()
            .py_spawn_blocking(move || {
                let _lock = lock;
                file.flush()
            })
            .await??;
        Ok(())
    }

    async fn flush(&self) -> PyResult<()> {
        let mut lock = self.lock().await;
        if !lock.file()?.has_pending_writes() {
            return Ok(());
        }
        lock.unblock(BufferedFile::flush).await
    }

    async fn peek(&self, n: usize) -> PyResult<Vec<u8>> {
        let mut lock = self.lock().await;
        if let Some(buf) = lock.file()?.try_peek(n) {
            return Ok(buf);
        }
        lock.unblock(move |file| file.peek(n)).await
    }

    /// `None` reads to eof
    async fn read(&self, n: Option<usize>) -> PyResult<Vec<u8>> {
        let mut lock = self.lock().await;
        if let Some(n) = n
            && let Some(buf) = lock.file()?.try_read(n)
        {
            return Ok(buf);
        }
        lock.unblock(move |file| match n {
            Some(n) => file.read(n),
            None => file.read_all(),
        })
        .await
    }

    async fn readline(&self, limit: Option<usize>) -> PyResult<Option<Vec<u8>>> {
        let mut lock = self.lock().await;
        if let Some(line) = lock.file()?.try_readline(limit) {
            return Ok(Some(line));
        }
        lock.unblock(move |file| file.readline(limit)).await
    }

    async fn readlines(&self, hint: Option<usize>) -> PyResult<Vec<Vec<u8>>> {
        let lock = self.lock().await;
        lock.unblock(move |file| file.readlines(hint)).await
    }

    async fn seek(&self, pos: SeekFrom) -> PyResult<u64> {
        let lock = self.lock().await;
        lock.unblock(move |file| file.seek(pos)).await
    }

    async fn tell(&self) -> PyResult<u64> {
        let lock = self.lock().await;
        lock.unblock(BufferedFile::tell).await
    }

    async fn truncate(&self, size: Option<u64>) -> PyResult<u64> {
        let lock = self.lock().await;
        lock.unblock(move |file| file.truncate(size)).await
    }

    async fn write(&self, buf: RyBytes) -> PyResult<usize> {
        let mut lock = self.lock().await;
        if lock.file()?.try_write(buf.as_slice()) {
            return Ok(buf.len());
        }
        lock.unblock(move |file| file.write(buf.as_slice())).await
    }
}

/// held lock on an `AsyncBufferedFile`
struct FileLock(OwnedMutexGuard<Option<BufferedFile>>);

impl FileLock {
    fn file(&mut self) -> PyResult<&mut BufferedFile> {
        self.0
            .as_mut()
            .ok_or_else(|| py_runtime_error!("File is closed; must open first"))
    }

    /// run `f` on the file as one blocking hop (the lock goes along)
    async fn unblock<T: Send + 'static>(
        mut self,
        f: impl FnOnce(&mut BufferedFile) -> io::Result<T> + Send + 'static,
    ) -> PyResult<T> {
        // wenodis: avoid closed bc wenodis right here and can avoid the
        // blocking spwan hop
        self.file()?;
        get_ry_tokio_runtime()
            .py_spawn_blocking(move || Ok(f(self.file()?)?))
            .await?
    }
}

/// converts a potentially negative size into `Option<usize>` limit
macro_rules! size_limit {
    ($size:expr) => {
        $size.and_then(|size: isize| usize::try_from(size).ok())
    };
}

#[pyclass(name = "AsyncFile", frozen, immutable_type)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
pub struct PyAsyncFile {
    path: Arc<Path>,
    open_options: PyOpenOptions,
    file: AsyncBufferedFile,
}

impl PyAsyncFile {
    pub(crate) fn new(path: PathBuf, open_options: PyOpenOptions) -> Self {
        Self {
            path: path.into(),
            open_options,
            file: AsyncBufferedFile::new(),
        }
    }

    /// open if not open already
    fn open_coro(&self) -> impl Future<Output = PyResult<()>> + Send + 'static {
        let (file, path, opts) = (self.file.clone(), Arc::clone(&self.path), self.open_options);
        async move { file.open(path, opts).await }
    }

    /// open if needed, resolves to the file itself
    fn open_self_coro<'py>(
        slf: Py<Self>,
        py: Python<'py>,
        qualname: &'static str,
    ) -> PyResult<Bound<'py, RyCoroutine>> {
        let open = slf.get().open_coro();
        pyloop_coro(py, qualname, async move {
            open.await?;
            Ok(slf)
        })
    }

    fn close_coro<'py>(
        &self,
        py: Python<'py>,
        qualname: &'static str,
    ) -> PyResult<Bound<'py, RyCoroutine>> {
        let file = self.file.clone();
        pyloop_coro(py, qualname, async move { file.close().await })
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
        Self::open_self_coro(slf, py, "AsyncFile.__await__")
    }

    fn __aenter__(slf: Py<Self>, py: Python<'_>) -> PyResult<Bound<'_, RyCoroutine>> {
        Self::open_self_coro(slf, py, "AsyncFile.__aenter__")
    }

    fn __aexit__<'py>(
        &self,
        py: Python<'py>,
        _exc_type: &Bound<'py, PyAny>,
        _exc_value: &Bound<'py, PyAny>,
        _traceback: &Bound<'py, PyAny>,
    ) -> PyResult<Bound<'py, RyCoroutine>> {
        self.close_coro(py, "AsyncFile.__aexit__")
    }

    fn __aiter__(this: PyRef<Self>) -> PyRef<Self> {
        this
    }

    #[expect(clippy::single_match_else, reason = "actually cleaner")]
    fn __anext__<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, RyCoroutine>> {
        let file = self.file.clone();
        pyloop_coro(py, "AsyncFile.__anext__", async move {
            match file.readline(None).await? {
                Some(line) => Ok(RyBytes::from(line)),
                None => py_stop_async_iteration_err!("End of stream"),
            }
        })
    }

    fn close<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, RyCoroutine>> {
        self.close_coro(py, "AsyncFile.close")
    }

    #[getter]
    fn closed(&self) -> bool {
        self.file.status() == FileStatus::Closed
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
        let file = self.file.clone();
        pyloop_coro(py, "AsyncFile.flush", async move { file.flush().await })
    }

    fn open<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, RyCoroutine>> {
        pyloop_coro(py, "AsyncFile.open", self.open_coro())
    }

    #[pyo3(signature = (size = 1, /))]
    fn peek<'py>(&self, py: Python<'py>, size: usize) -> PyResult<Bound<'py, RyCoroutine>> {
        let file = self.file.clone();
        pyloop_coro(py, "AsyncFile.peek", async move {
            Ok(RyBytes::from(file.peek(size).await?))
        })
    }

    #[pyo3(signature = (size = None, /))]
    fn read<'py>(&self, py: Python<'py>, size: Option<isize>) -> PyResult<Bound<'py, RyCoroutine>> {
        let size = size_limit!(size);
        let file = self.file.clone();
        pyloop_coro(py, "AsyncFile.read", async move {
            Ok(RyBytes::from(file.read(size).await?))
        })
    }

    fn readable(&self) -> bool {
        self.open_options.read
    }

    fn readall<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, RyCoroutine>> {
        let file = self.file.clone();
        pyloop_coro(py, "AsyncFile.readall", async move {
            Ok(RyBytes::from(file.read(None).await?))
        })
    }

    #[pyo3(signature = (size = None, /))]
    fn readline<'py>(
        &self,
        py: Python<'py>,
        size: Option<isize>,
    ) -> PyResult<Bound<'py, RyCoroutine>> {
        let size = size_limit!(size);
        let file = self.file.clone();
        pyloop_coro(py, "AsyncFile.readline", async move {
            let line = file.readline(size).await?;
            if let Some(line) = line {
                Ok(RyBytes::from(line))
            } else {
                Ok(RyBytes::empty())
            }
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
        hint: Option<isize>,
    ) -> PyResult<Bound<'py, RyCoroutine>> {
        let hint = size_limit!(hint);
        let file = self.file.clone();
        pyloop_coro(py, "AsyncFile.readlines", async move {
            let lines = file.readlines(hint).await?;
            Ok(lines.into_iter().map(RyBytes::from).collect::<Vec<_>>())
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
            1 => SeekFrom::Current(offset),
            2 => SeekFrom::End(offset),
            other => {
                return Err(py_value_error!(
                    "invalid whence ({other}, should be 0, 1 or 2)"
                ));
            }
        };
        let file = self.file.clone();
        pyloop_coro(py, "AsyncFile.seek", async move { file.seek(pos).await })
    }

    fn seekable(&self) -> bool {
        // not known until opened; `True` like before
        self.file.status() != FileStatus::Unseekable
    }

    fn tell<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, RyCoroutine>> {
        let file = self.file.clone();
        pyloop_coro(py, "AsyncFile.tell", async move { file.tell().await })
    }

    #[pyo3(signature = (pos = None, /))]
    fn truncate<'py>(
        &self,
        py: Python<'py>,
        pos: Option<u64>,
    ) -> PyResult<Bound<'py, RyCoroutine>> {
        let file = self.file.clone();
        pyloop_coro(
            py,
            "AsyncFile.truncate",
            async move { file.truncate(pos).await },
        )
    }

    #[pyo3(signature = (buffer, /))]
    fn write<'py>(&self, py: Python<'py>, buffer: RyBytes) -> PyResult<Bound<'py, RyCoroutine>> {
        // check that write is not allowed on read-only files
        if !self.writable() {
            return failed_coro(
                py,
                "AsyncFile.write",
                UnsupportedOperation::new_err("write"),
            );
        }
        let file = self.file.clone();
        pyloop_coro(
            py,
            "AsyncFile.write",
            async move { file.write(buffer).await },
        )
    }

    fn writable(&self) -> bool {
        self.open_options.write || self.open_options.append
    }
}
