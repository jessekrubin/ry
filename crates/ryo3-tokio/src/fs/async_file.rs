use std::fs::OpenOptions;
use std::io::SeekFrom;
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
use tokio::sync::Mutex;

/// lock (fifo), then run `op` in one blocking task that holds the lock until
/// it's done, so a cancelled await can't leave an op half done
///
/// awaited on the loop thread (`pyloop_coro`), no tokio worker in between
async fn run_blocking<S, T, F>(state: Arc<Mutex<S>>, op: F) -> PyResult<T>
where
    S: Send + 'static,
    T: Send + 'static,
    F: FnOnce(&mut S) -> PyResult<T> + Send + 'static,
{
    let mut guard = state.lock_owned().await;
    get_ry_tokio_runtime()
        .py_spawn_blocking(move || op(&mut guard))
        .await?
}

type FileState = Option<BufferedFile>;

fn open_file(state: &mut FileState) -> PyResult<&mut BufferedFile> {
    state
        .as_mut()
        .ok_or_else(|| py_runtime_error!("File is closed; must open first"))
}

/// Lockless mirror of the file state (used by sync getters)
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
enum FileStatus {
    Closed = 0,
    Seekable = 1,
    Unseekable = 2,
}

impl FileStatus {
    const fn from_u8(v: u8) -> Self {
        match v {
            1 => Self::Seekable,
            2 => Self::Unseekable,
            _ => Self::Closed,
        }
    }
}

impl From<&FileState> for FileStatus {
    fn from(state: &FileState) -> Self {
        match state {
            None => Self::Closed,
            Some(file) if file.seekable() => Self::Seekable,
            Some(_) => Self::Unseekable,
        }
    }
}

#[pyclass(name = "AsyncFile", frozen, immutable_type)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
pub struct PyAsyncFile {
    path: Arc<Path>,
    open_options: PyOpenOptions,
    /// `FileStatus` of `state`; only written by open/close (under the lock)
    status: Arc<AtomicU8>,
    state: Arc<Mutex<FileState>>,
}

impl PyAsyncFile {
    pub(crate) fn new(path: PathBuf, open_options: PyOpenOptions) -> Self {
        Self {
            path: path.into(),
            open_options,
            status: Arc::new(AtomicU8::new(FileStatus::Closed as u8)),
            state: Arc::new(Mutex::new(None)),
        }
    }

    fn status(&self) -> FileStatus {
        FileStatus::from_u8(self.status.load(Ordering::Acquire))
    }

    /// open if not open already (reopening w/ 'w' would truncate)
    fn open_op(&self) -> impl Future<Output = PyResult<()>> + Send + 'static {
        let (path, opts, status) = (
            Arc::clone(&self.path),
            self.open_options,
            Arc::clone(&self.status),
        );
        run_blocking(Arc::clone(&self.state), move |state| {
            if state.is_none() {
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
                *state = Some(file);
            }
            status.store(FileStatus::from(&*state) as u8, Ordering::Release);
            Ok(())
        })
    }

    /// open if needed, resolves to the file itself
    fn open_self_coro<'py>(
        slf: Py<Self>,
        py: Python<'py>,
        qualname: &'static str,
    ) -> PyResult<Bound<'py, RyCoroutine>> {
        let open = slf.get().open_op();
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
        let (state, status) = (Arc::clone(&self.state), Arc::clone(&self.status));
        pyloop_coro(py, qualname, async move {
            // nothing to flush, drop it here (no blocking hop)
            if let Ok(mut guard) = state.try_lock()
                && guard.as_ref().is_none_or(|file| !file.has_pending_writes())
            {
                *guard = None;
                status.store(FileStatus::Closed as u8, Ordering::Release);
                return Ok(());
            }
            run_blocking(state, move |state| {
                // always ends up closed, even if the final flush fails
                let flushed = state.take().map_or(Ok(()), |mut file| file.flush());
                status.store(FileStatus::Closed as u8, Ordering::Release);
                Ok(flushed?)
            })
            .await
        })
    }

    /// `fast` on the loop thread if the buffers can do it, else `slow` as one
    /// blocking hop (both at await time)
    fn op<'py, A, T>(
        &self,
        py: Python<'py>,
        qualname: &'static str,
        arg: A,
        fast: impl FnOnce(&mut BufferedFile, &A) -> Option<T> + Send + 'static,
        slow: impl FnOnce(&mut BufferedFile, A) -> PyResult<T> + Send + 'static,
    ) -> PyResult<Bound<'py, RyCoroutine>>
    where
        A: Send + 'static,
        T: for<'a> IntoPyObject<'a> + Send + 'static,
    {
        let state = Arc::clone(&self.state);
        pyloop_coro(py, qualname, async move {
            if let Ok(mut guard) = state.try_lock()
                && let Some(file) = guard.as_mut()
                && let Some(out) = fast(file, &arg)
            {
                return Ok(out);
            }
            run_blocking(state, move |state| slow(open_file(state)?, arg)).await
        })
    }

    /// op w/o a buffer fast path
    fn blocking_op<'py, T>(
        &self,
        py: Python<'py>,
        qualname: &'static str,
        op: impl FnOnce(&mut BufferedFile) -> PyResult<T> + Send + 'static,
    ) -> PyResult<Bound<'py, RyCoroutine>>
    where
        T: for<'a> IntoPyObject<'a> + Send + 'static,
    {
        let run = run_blocking(Arc::clone(&self.state), move |state| op(open_file(state)?));
        pyloop_coro(py, qualname, run)
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
        self.op(
            py,
            "AsyncFile.__anext__",
            (),
            |file, ()| file.try_readline(None).map(RyBytes::from),
            |file, ()| match file.readline(None)? {
                Some(line) => Ok(RyBytes::from(line)),
                None => py_stop_async_iteration_err!("End of stream"),
            },
        )
    }

    fn close<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, RyCoroutine>> {
        self.close_coro(py, "AsyncFile.close")
    }

    #[getter]
    fn closed(&self) -> bool {
        self.status() == FileStatus::Closed
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
        self.op(
            py,
            "AsyncFile.flush",
            (),
            |file, ()| (!file.has_pending_writes()).then_some(()),
            |file, ()| Ok(file.flush()?),
        )
    }

    fn open<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, RyCoroutine>> {
        pyloop_coro(py, "AsyncFile.open", self.open_op())
    }

    #[pyo3(signature = (size = 1, /))]
    fn peek<'py>(&self, py: Python<'py>, size: usize) -> PyResult<Bound<'py, RyCoroutine>> {
        self.op(
            py,
            "AsyncFile.peek",
            size,
            |file, &size| file.try_peek(size).map(RyBytes::from),
            |file, size| Ok(RyBytes::from(file.peek(size)?)),
        )
    }

    #[pyo3(signature = (size = None, /))]
    fn read<'py>(&self, py: Python<'py>, size: Option<usize>) -> PyResult<Bound<'py, RyCoroutine>> {
        self.op(
            py,
            "AsyncFile.read",
            size,
            |file, &size| file.try_read(size?).map(RyBytes::from),
            |file, size| {
                let buf = match size {
                    Some(size) => file.read(size)?,
                    None => file.read_all()?,
                };
                Ok(RyBytes::from(buf))
            },
        )
    }

    fn readable(&self) -> bool {
        self.open_options.read
    }

    fn readall<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, RyCoroutine>> {
        self.blocking_op(py, "AsyncFile.readall", |file| {
            Ok(RyBytes::from(file.read_all()?))
        })
    }

    #[pyo3(signature = (size = None, /))]
    fn readline<'py>(
        &self,
        py: Python<'py>,
        size: Option<usize>,
    ) -> PyResult<Bound<'py, RyCoroutine>> {
        self.op(
            py,
            "AsyncFile.readline",
            size,
            |file, &size| {
                file.try_readline(size)
                    .map(|line| Some(RyBytes::from(line)))
            },
            |file, size| Ok(file.readline(size)?.map(RyBytes::from)),
        )
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
        self.blocking_op(py, "AsyncFile.readlines", move |file| {
            let lines = file.readlines(hint)?;
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
        self.blocking_op(py, "AsyncFile.seek", move |file| Ok(file.seek(pos)?))
    }

    fn seekable(&self) -> bool {
        // not known until opened; `True` like before
        self.status() != FileStatus::Unseekable
    }

    fn tell<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, RyCoroutine>> {
        self.blocking_op(py, "AsyncFile.tell", |file| Ok(file.tell()?))
    }

    #[pyo3(signature = (pos = None, /))]
    fn truncate<'py>(
        &self,
        py: Python<'py>,
        pos: Option<u64>,
    ) -> PyResult<Bound<'py, RyCoroutine>> {
        self.blocking_op(
            py,
            "AsyncFile.truncate",
            move |file| Ok(file.truncate(pos)?),
        )
    }

    #[pyo3(signature = (buffer, /))]
    fn write<'py>(&self, py: Python<'py>, buffer: RyBytes) -> PyResult<Bound<'py, RyCoroutine>> {
        self.op(
            py,
            "AsyncFile.write",
            buffer,
            |file, buffer| file.try_write(buffer.as_slice()).then(|| buffer.len()),
            |file, buffer| Ok(file.write(buffer.as_slice())?),
        )
    }

    fn writable(&self) -> bool {
        self.open_options.write || self.open_options.append
    }
}
