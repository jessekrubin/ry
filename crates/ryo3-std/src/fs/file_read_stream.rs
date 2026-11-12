use std::fmt;
use std::fs::OpenOptions;
use std::io::SeekFrom;
use std::path::PathBuf;

use pyo3::prelude::*;
use ryo3_bytes::RyBytes;
use ryo3_core::PyIoResultExt;
use ryo3_core::sync::RyMutex;
use ryo3_macro_rules::py_value_err;

use crate::fs::buffered_file::{BUF_SIZE, BufferedFile};

/// Options for the sync `FileReadStream` + ryo3-tokio's `AsyncFileReadStream`
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct FileReadStreamOptions {
    /// Whether to enforce strict offset checking.
    pub strict: bool,
    /// Path to the file to read.
    pub path: PathBuf,
    /// Size of each chunk to read.
    pub read_size: usize,
    /// Offset to start reading from.
    pub offset: u64,
    /// Whether to use buffered reading.
    pub buffered: bool,
}

impl FileReadStreamOptions {
    /// open + (strict) offset check + seek to `offset`
    pub fn open(&self) -> PyResult<BufferedFile> {
        // reads >= the buffer skip it, so unbuffered is just a 0 size buffer
        let cap = if self.buffered { BUF_SIZE } else { 0 };
        let mut file = BufferedFile::open(&self.path, OpenOptions::new().read(true), cap)
            .map_py_io_err_with_path(&self.path)?;
        if self.strict {
            let len = file.file_len().map_py_io_err_with_path(&self.path)?;
            if self.offset > len {
                return py_value_err!(
                    "offset ({}) is beyond end of file (len = {len})",
                    self.offset
                );
            }
        }
        if self.offset > 0 {
            file.seek(SeekFrom::Start(self.offset))
                .map_py_io_err_with_path(&self.path)?;
        }
        Ok(file)
    }
}

/// The python repr args, e.g. `path='f.txt', read_size=65536, buffered=True, strict=True`.
///
/// TODO: this could very well mean figuring out some sort of `KwargsWriter` thingy
impl fmt::Display for FileReadStreamOptions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let py_bool_str = |b: bool| if b { "True" } else { "False" };
        write!(
            f,
            "path='{}', read_size={}",
            self.path.display(),
            self.read_size
        )?;
        if self.offset != 0 {
            write!(f, ", offset={}", self.offset)?;
        }
        write!(
            f,
            ", buffered={}, strict={}",
            py_bool_str(self.buffered),
            py_bool_str(self.strict)
        )
    }
}

struct StreamState {
    /// `None` = closed ~ reopens at `offset` on next usage
    file: Option<BufferedFile>,
}

impl StreamState {
    fn ensure_open(&mut self, options: &FileReadStreamOptions) -> PyResult<&mut BufferedFile> {
        let file = match self.file.take() {
            Some(file) => file,
            None => options.open()?,
        };
        Ok(self.file.insert(file))
    }

    // `None` at eof
    fn next_chunk(&mut self, options: &FileReadStreamOptions) -> PyResult<Option<RyBytes>> {
        let chunk = self.ensure_open(options)?.read(options.read_size)?;
        Ok((!chunk.is_empty()).then(|| RyBytes::from(chunk)))
    }
}

#[pyclass(name = "FileReadStream", frozen, immutable_type, eq)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
pub struct PyFileReadStream {
    options: FileReadStreamOptions,
    state: RyMutex<StreamState>,
}

impl PartialEq for PyFileReadStream {
    fn eq(&self, other: &Self) -> bool {
        self.options == other.options
    }
}

impl PyFileReadStream {
    pub(crate) fn new(options: FileReadStreamOptions) -> PyResult<Self> {
        if options.read_size == 0 {
            return py_value_err!("read_size must be greater than 0");
        }
        let file = options.open()?;
        Ok(Self {
            options,
            state: RyMutex::new(StreamState { file: Some(file) }),
        })
    }
}

#[pymethods]
impl PyFileReadStream {
    #[new]
    #[pyo3(signature = (path, *, read_size = 65536, offset = 0, buffered = true, strict = true))]
    pub fn py_new(
        path: PathBuf,
        read_size: usize,
        offset: u64,
        buffered: bool,
        strict: bool,
    ) -> PyResult<Self> {
        let options = FileReadStreamOptions {
            strict,
            path,
            read_size,
            offset,
            buffered,
        };
        Self::new(options)
    }

    fn __repr__(&self) -> String {
        format!("{self}")
    }

    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __enter__(slf: Py<Self>, py: Python<'_>) -> PyResult<Py<Self>> {
        {
            let this = slf.get();
            this.state
                .py_lock_attached(py)?
                .ensure_open(&this.options)?;
        }
        Ok(slf)
    }

    #[pyo3(name = "__exit__")]
    fn __exit__(
        &self,
        py: Python<'_>,
        _exc_type: &Bound<'_, PyAny>,
        _exc_value: &Bound<'_, PyAny>,
        _traceback: &Bound<'_, PyAny>,
    ) -> PyResult<()> {
        self.close(py)
    }

    fn __next__(&self, py: Python<'_>) -> PyResult<Option<RyBytes>> {
        self.state.py_lock_attached(py)?.next_chunk(&self.options)
    }

    #[pyo3(signature = (n = 1))]
    fn take(&self, py: Python<'_>, n: usize) -> PyResult<Vec<RyBytes>> {
        let mut state = self.state.py_lock_attached(py)?;
        // open even for `take(0)` (reopens after `close()` like before)
        state.ensure_open(&self.options)?;
        let mut chunks = Vec::new();
        while chunks.len() < n
            && let Some(chunk) = state.next_chunk(&self.options)?
        {
            chunks.push(chunk);
            if chunks.len().is_multiple_of(256) {
                py.check_signals()?;
            }
        }
        Ok(chunks)
    }

    fn collect(&self, py: Python<'_>) -> PyResult<Vec<RyBytes>> {
        let mut state = self.state.py_lock_attached(py)?;
        let mut chunks = Vec::new();
        while let Some(chunk) = state.next_chunk(&self.options)? {
            chunks.push(chunk);
            if chunks.len().is_multiple_of(256) {
                py.check_signals()?;
            }
        }
        Ok(chunks)
    }

    fn close(&self, py: Python<'_>) -> PyResult<()> {
        self.state.py_lock_attached(py)?.file = None;
        Ok(())
    }
}

impl fmt::Display for PyFileReadStream {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "FileReadStream({})", self.options)
    }
}
