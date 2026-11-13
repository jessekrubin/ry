//! Async chunked file reads, built on the sync `BufferedFile`.
//!
//! While python works on one chunk, the next one is read in the background.
use std::path::PathBuf;
use std::sync::Arc;
use std::{fmt, io, iter, mem};

use pyo3::prelude::*;
use ryo3_bytes::RyBytes;
use ryo3_macro_rules::{py_runtime_error, py_stop_async_iteration_err, py_value_err};
use ryo3_std::fs::{BufferedFile, FileReadStreamOptions};
use ryo3_tokio_rt::{RyCoroutine, RyJoinHandle, get_ry_tokio_runtime, pyloop_coro};
use tokio::sync::{Mutex, OwnedMutexGuard};

/// the next chunk + the file back (it survives read errors)
struct ChunkRead {
    file: BufferedFile,
    chunk: io::Result<Vec<u8>>,
}

#[derive(Default)]
enum StreamState {
    #[default]
    Closed,
    /// open, no read running
    Idle(BufferedFile),
    /// a read is running on a blocking thread
    Reading {
        read: RyJoinHandle<ChunkRead>,
        /// `false` for pipes/fifos, where a read can block until the writer
        /// sends more
        seekable: bool,
    },
    /// read task died w/ and nuked the file. errors until `close()` (reopening
    /// would re-yield chunks (which is wrong))
    Failed,
}

impl StreamState {
    /// start reading the next chunk on a blocking thread (`Idle` -> `Reading`)
    fn read(self, read_size: usize) -> Self {
        match self {
            Self::Idle(mut file) => {
                let seekable = file.seekable();
                let read = get_ry_tokio_runtime().py_spawn_blocking(move || {
                    let chunk = file.read(read_size);
                    ChunkRead { file, chunk }
                });
                Self::Reading { read, seekable }
            }
            other => other,
        }
    }

    /// read ahead unless the buffer already has the next chunk
    ///
    /// regular files only ~ pipe/fifo read can block till writer writes mo'.
    fn prefetch(&mut self, read_size: usize) {
        if let Self::Idle(file) = self
            && file.seekable()
            && file.buffered_len() < read_size
        {
            *self = mem::take(self).read(read_size);
        }
    }

    async fn ensure_open(&mut self, options: &Arc<FileReadStreamOptions>) -> PyResult<()> {
        if matches!(self, Self::Closed) {
            let options = Arc::clone(options);
            let file = get_ry_tokio_runtime()
                .py_spawn_blocking(move || options.open())
                .await??;
            *self = Self::Idle(file);
        }
        Ok(())
    }

    /// next chunk, `None` at eof; `prefetch` also starts reading the one after
    ///
    /// cancel safe: if the caller stops waiting, the running read stays in
    /// `Reading` and the next call picks it up
    async fn next_chunk(
        &mut self,
        options: &Arc<FileReadStreamOptions>,
        prefetch: bool,
    ) -> PyResult<Option<Vec<u8>>> {
        loop {
            match self {
                Self::Closed => self.ensure_open(options).await?,
                Self::Failed => {
                    return Err(py_runtime_error!(
                        "stream failed on an earlier read; close it to start over"
                    ));
                }
                Self::Idle(file) => {
                    // buffer hit ~ no blocking hop
                    if let Some(chunk) = file.try_read(options.read_size) {
                        if prefetch {
                            self.prefetch(options.read_size);
                        }
                        return Ok(Some(chunk));
                    }
                    *self = mem::take(self).read(options.read_size);
                }
                Self::Reading { read, .. } => {
                    let ChunkRead { file, chunk } = match read.await {
                        Ok(done) => done,
                        Err(e) => {
                            *self = Self::Failed;
                            return Err(e);
                        }
                    };
                    *self = Self::Idle(file);
                    let chunk = chunk?;
                    if chunk.is_empty() {
                        return Ok(None);
                    }
                    if prefetch {
                        self.prefetch(options.read_size);
                    }
                    return Ok(Some(chunk));
                }
            }
        }
    }
}

/// up to `n` chunks for `take`/`collect`
///
/// the first one comes from `next_chunk` (which opens the file and picks up a
/// read-ahead), the rest are read in one go on a blocking thread. that thread
/// holds the lock, so a cancelled `take`/`collect` can't leave the stream
/// half read
async fn read_chunks(
    mut state: OwnedMutexGuard<StreamState>,
    options: &Arc<FileReadStreamOptions>,
    n: usize,
) -> PyResult<Vec<RyBytes>> {
    if n == 0 {
        return Ok(Vec::new());
    }
    let Some(first) = state.next_chunk(options, false).await? else {
        return Ok(Vec::new());
    };
    let read_size = options.read_size;
    let rest = get_ry_tokio_runtime()
        .py_spawn_blocking(move || {
            let StreamState::Idle(file) = &mut *state else {
                return Ok(Vec::new());
            };
            file.chunks(read_size)
                .take(n - 1)
                .collect::<io::Result<Vec<_>>>()
        })
        .await??;
    Ok(iter::once(first).chain(rest).map(RyBytes::from).collect())
}

// TODO: investigate if 2 arcs are really needed?
#[pyclass(name = "AsyncFileReadStream", frozen, immutable_type, eq)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
pub struct PyAsyncFileReadStream {
    options: Arc<FileReadStreamOptions>,
    state: Arc<Mutex<StreamState>>,
}

impl PartialEq for PyAsyncFileReadStream {
    fn eq(&self, other: &Self) -> bool {
        self.options == other.options
    }
}

impl PyAsyncFileReadStream {
    /// opens the file (if not already open) and resolves to the stream itself
    fn ensure_open_coro<'py>(
        slf: Py<Self>,
        py: Python<'py>,
        qualname: &'static str,
    ) -> PyResult<Bound<'py, RyCoroutine>> {
        let (state, options) = {
            let this = slf.get();
            (Arc::clone(&this.state), Arc::clone(&this.options))
        };
        pyloop_coro(py, qualname, async move {
            state.lock().await.ensure_open(&options).await?;
            Ok(slf)
        })
    }

    /// back to `Closed`
    fn close_coro<'py>(
        &self,
        py: Python<'py>,
        qualname: &'static str,
    ) -> PyResult<Bound<'py, RyCoroutine>> {
        let state = Arc::clone(&self.state);
        pyloop_coro(py, qualname, async move {
            let mut state = state.lock().await;
            // abort wont kill an inflight blocking read, SO wait?
            // AFAICT file is actually closed once `close()` finishes.
            // await here RIGHT NOW so a cancelled close doesn't leave file in
            // invalid state
            // NOTE: not on a pipe/fifo though! it can block til writer sends more which may never ever ever happen stupid moi
            if let StreamState::Reading {
                read,
                seekable: true,
            } = &mut *state
            {
                let _ = read.await;
            }
            *state = StreamState::Closed;
            Ok(())
        })
    }
}

#[pymethods]
impl PyAsyncFileReadStream {
    #[new]
    #[pyo3(signature = (path, *, read_size = 65536, offset = 0, buffered = true, strict = true))]
    pub fn py_new(
        path: PathBuf,
        read_size: usize,
        offset: u64,
        buffered: bool,
        strict: bool,
    ) -> PyResult<Self> {
        if read_size == 0 {
            return py_value_err!("read_size must be greater than 0");
        }
        let opts = FileReadStreamOptions {
            strict,
            path,
            read_size,
            offset,
            buffered,
        };
        Ok(Self {
            options: Arc::new(opts),
            state: Arc::new(Mutex::new(StreamState::Closed)),
        })
    }

    fn __await__(slf: Py<Self>, py: Python<'_>) -> PyResult<Bound<'_, RyCoroutine>> {
        Self::ensure_open_coro(slf, py, "AsyncFileReadStream.__await__")
    }

    fn __aenter__(slf: Py<Self>, py: Python<'_>) -> PyResult<Bound<'_, RyCoroutine>> {
        Self::ensure_open_coro(slf, py, "AsyncFileReadStream.__aenter__")
    }

    fn __aexit__<'py>(
        &self,
        py: Python<'py>,
        _exc_type: &Bound<'py, PyAny>,
        _exc_value: &Bound<'py, PyAny>,
        _traceback: &Bound<'py, PyAny>,
    ) -> PyResult<Bound<'py, RyCoroutine>> {
        self.close_coro(py, "AsyncFileReadStream.__aexit__")
    }

    fn __aiter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    #[expect(clippy::single_match_else, reason = "actually cleaner")]
    fn __anext__<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, RyCoroutine>> {
        let (state, options) = (Arc::clone(&self.state), Arc::clone(&self.options));
        pyloop_coro(py, "AsyncFileReadStream.__anext__", async move {
            match state.lock().await.next_chunk(&options, true).await? {
                Some(chunk) => Ok(RyBytes::from(chunk)),
                None => py_stop_async_iteration_err!("stream exhausted"),
            }
        })
    }

    #[pyo3(signature = (n = 1))]
    fn take<'py>(&self, py: Python<'py>, n: usize) -> PyResult<Bound<'py, RyCoroutine>> {
        let (state, options) = (Arc::clone(&self.state), Arc::clone(&self.options));
        pyloop_coro(py, "AsyncFileReadStream.take", async move {
            let mut state = state.lock_owned().await;
            // open even for `take(0)` so a bad path still raises (like before)
            state.ensure_open(&options).await?;
            read_chunks(state, &options, n).await
        })
    }

    fn collect<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, RyCoroutine>> {
        let (state, options) = (Arc::clone(&self.state), Arc::clone(&self.options));
        pyloop_coro(py, "AsyncFileReadStream.collect", async move {
            read_chunks(state.lock_owned().await, &options, usize::MAX).await
        })
    }

    fn close<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, RyCoroutine>> {
        self.close_coro(py, "AsyncFileReadStream.close")
    }

    fn __repr__(&self) -> String {
        format!("{self}")
    }
}

impl fmt::Display for PyAsyncFileReadStream {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "AsyncFileReadStream({})", self.options)
    }
}
