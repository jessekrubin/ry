//! Chunked async file reads on the shared sync `BufferedFile`.
//!
//! While python is busy with one chunk, the next one is already being read
//! (read-ahead).
use std::path::PathBuf;
use std::sync::Arc;
use std::{fmt, io, mem};

use pyo3::prelude::*;
use ryo3_bytes::RyBytes;
use ryo3_macro_rules::{py_stop_async_iteration_err, py_value_err};
use ryo3_std::fs::{BufferedFile, FileReadStreamOptions};
use ryo3_tokio_rt::{RyCoroutine, RyJoinHandle, get_ry_tokio_runtime, pyloop_coro};
use tokio::sync::Mutex;

/// the next chunk + the file back (it survives read errors)
struct ChunkRead {
    file: BufferedFile,
    chunk: io::Result<Vec<u8>>,
}

#[derive(Default)]
enum StreamState {
    #[default]
    Closed,
    /// open, nothing in-flight
    Idle(BufferedFile),
    /// next chunk already being read
    Reading(RyJoinHandle<ChunkRead>),
}

impl StreamState {
    /// start reading the next chunk in the background (`Idle` -> `Reading`)
    fn read_ahead(self, read_size: usize) -> Self {
        match self {
            Self::Idle(mut file) => {
                Self::Reading(get_ry_tokio_runtime().py_spawn_blocking(move || {
                    let chunk = file.read(read_size);
                    ChunkRead { file, chunk }
                }))
            }
            other => other,
        }
    }

    /// read ahead unless the buffer already has the next chunk
    fn prefetch(&mut self, read_size: usize) {
        if let Self::Idle(file) = self
            && file.buffered_len() < read_size
        {
            *self = mem::take(self).read_ahead(read_size);
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

    /// next chunk, `None` at eof
    ///
    /// cancel safe: an in-flight read stays in `Reading` for the next call
    async fn next_chunk(
        &mut self,
        options: &Arc<FileReadStreamOptions>,
    ) -> PyResult<Option<Vec<u8>>> {
        loop {
            match self {
                Self::Closed => self.ensure_open(options).await?,
                Self::Idle(file) => {
                    // buffer hit ~ no blocking hop
                    if let Some(chunk) = file.try_read(options.read_size) {
                        self.prefetch(options.read_size);
                        return Ok(Some(chunk));
                    }
                    *self = mem::take(self).read_ahead(options.read_size);
                }
                Self::Reading(read) => {
                    let ChunkRead { file, chunk } = match read.await {
                        Ok(done) => done,
                        Err(e) => {
                            *self = Self::Closed;
                            return Err(e);
                        }
                    };
                    *self = Self::Idle(file);
                    let chunk = chunk?;
                    if chunk.is_empty() {
                        return Ok(None);
                    }
                    self.prefetch(options.read_size);
                    return Ok(Some(chunk));
                }
            }
        }
    }
}

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

    /// State transition back to `Closed`
    ///
    /// NOTE: in-flight read finishes and closes
    fn close_coro<'py>(
        &self,
        py: Python<'py>,
        qualname: &'static str,
    ) -> PyResult<Bound<'py, RyCoroutine>> {
        let state = Arc::clone(&self.state);
        pyloop_coro(py, qualname, async move {
            *state.lock().await = StreamState::Closed;
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
            match state.lock().await.next_chunk(&options).await? {
                Some(chunk) => Ok(RyBytes::from(chunk)),
                None => py_stop_async_iteration_err!("stream exhausted"),
            }
        })
    }

    #[pyo3(signature = (n = 1))]
    fn take<'py>(&self, py: Python<'py>, n: usize) -> PyResult<Bound<'py, RyCoroutine>> {
        let (state, options) = (Arc::clone(&self.state), Arc::clone(&self.options));
        pyloop_coro(py, "AsyncFileReadStream.take", async move {
            let mut state = state.lock().await;
            // open even for `take(0)` so a bad path still raises (like before)
            state.ensure_open(&options).await?;
            let mut chunks = Vec::new();
            while chunks.len() < n
                && let Some(chunk) = state.next_chunk(&options).await?
            {
                chunks.push(RyBytes::from(chunk));
            }
            Ok(chunks)
        })
    }

    fn collect<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, RyCoroutine>> {
        let (state, options) = (Arc::clone(&self.state), Arc::clone(&self.options));
        pyloop_coro(py, "AsyncFileReadStream.collect", async move {
            let mut state = state.lock().await;
            let mut chunks = Vec::new();
            while let Some(chunk) = state.next_chunk(&options).await? {
                chunks.push(RyBytes::from(chunk));
            }
            Ok(chunks)
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
