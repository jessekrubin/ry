use bytes::{Bytes, BytesMut};
use pyo3::prelude::*;
use pyo3::{PyRef, PyResult, pyclass, pymethods};
use ryo3_core::RyMutex;
use ryo3_macro_rules::py_value_err;
use std::fs::File;
use std::io::{self, BufReader, Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};

pub(crate) const DEFAULT_READ_SIZE: usize = 65536;

pub(crate) struct FileReadStream<T: Read + Seek> {
    file: T,
    read_size: usize,
    buffer: BytesMut,
}

impl<T: Read + Seek> FileReadStream<T> {
    pub(crate) fn new(file: T, read_size: usize) -> Self {
        Self {
            file,
            read_size,
            buffer: BytesMut::with_capacity(read_size),
        }
    }
}

impl FileReadStream<File> {
    fn from_path<P: AsRef<Path>>(path: P, read_size: usize) -> io::Result<Self> {
        let file = File::open(path)?;
        Ok(Self::new(file, read_size))
    }
}

impl FileReadStream<BufReader<File>> {
    fn from_path_buffered<P: AsRef<Path>>(path: P, read_size: usize) -> io::Result<Self> {
        let file = File::open(path)?;
        let reader = BufReader::with_capacity(read_size * 2, file);
        Ok(Self::new(reader, read_size))
    }
}

impl<T: Read + Seek> FileReadStream<T> {
    fn seek_to(&mut self, offset: u64) -> io::Result<u64> {
        self.file.seek(SeekFrom::Start(offset))
    }
}

impl<T: Read + Seek> Iterator for FileReadStream<T> {
    type Item = io::Result<Bytes>;

    fn next(&mut self) -> Option<Self::Item> {
        // can resize buffer without reallocating (I think)
        self.buffer.resize(self.read_size, 0);
        match self.file.read(&mut self.buffer) {
            Ok(0) => None,
            Ok(n) => Some(Ok(self.buffer.split_to(n).freeze())),
            Err(e) => Some(Err(e)),
        }
    }
}

pub(crate) enum FileReadStreamWrapper {
    Unbuffered(FileReadStream<File>),
    Buffered(FileReadStream<BufReader<File>>),
}

impl FileReadStreamWrapper {
    fn seek_to(&mut self, offset: u64) -> io::Result<u64> {
        match self {
            Self::Unbuffered(stream) => stream.seek_to(offset),
            Self::Buffered(stream) => stream.seek_to(offset),
        }
    }
}

impl Iterator for FileReadStreamWrapper {
    type Item = io::Result<Bytes>;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Unbuffered(stream) => stream.next(),
            Self::Buffered(stream) => stream.next(),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub(crate) struct PyFileReadStreamOptions {
    /// Whether to enforce strict offset checking
    pub(crate) strict: bool,
    /// Path to the file to read
    pub(crate) path: PathBuf,
    /// Size of each chunk to read
    pub(crate) read_size: usize,
    /// Offset to start reading from
    pub(crate) offset: u64,
    /// Whether to use buffered reading
    pub(crate) buffered: bool,
}

#[pyclass(name = "FileReadStream", frozen, immutable_type, skip_from_py_object)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
pub struct PyFileReadStream {
    pub(crate) options: PyFileReadStreamOptions,
    pub(crate) file_read_stream: RyMutex<FileReadStreamWrapper>,
}

impl PyFileReadStream {
    pub(crate) fn new(options: PyFileReadStreamOptions) -> PyResult<Self> {
        if options.read_size == 0 {
            return py_value_err!("read_size must be greater than 0");
        }

        // Open once so we can check length
        let file = File::open(&options.path)?;
        let len = file.metadata()?.len();

        if options.strict && options.offset > len {
            return py_value_err!(
                "offset ({}) is beyond end of file (len = {})",
                options.offset,
                len
            );
        }

        // Build the stream from this file
        let mut stream = if options.buffered {
            FileReadStreamWrapper::Buffered(FileReadStream::from_path_buffered(
                &options.path,
                options.read_size,
            )?)
        } else {
            FileReadStreamWrapper::Unbuffered(FileReadStream::from_path(
                &options.path,
                options.read_size,
            )?)
        };

        if options.offset > 0 {
            stream.seek_to(options.offset)?;
        }
        Ok(Self {
            options,
            file_read_stream: RyMutex::new(stream),
        })
    }
}

#[pymethods]
impl PyFileReadStream {
    #[new]
    #[pyo3(signature = (path, *, read_size = DEFAULT_READ_SIZE, offset = 0, buffered = true, strict = true))]
    pub fn py_new(
        path: PathBuf,
        read_size: usize,
        offset: u64,
        buffered: bool,
        strict: bool,
    ) -> PyResult<Self> {
        let options = PyFileReadStreamOptions {
            strict,
            path,
            read_size,
            offset,
            buffered,
        };
        Self::new(options)
    }

    fn __repr__(&self) -> String {
        format!("{self:?}")
    }

    fn __eq__(&self, other: &Self) -> bool {
        self.options == other.options
    }

    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__(&self) -> PyResult<Option<ryo3_bytes::PyBytes>> {
        let mut inner = self.file_read_stream.py_lock()?;
        match inner.next() {
            Some(Ok(b)) => Ok(Some(b.into())),
            Some(Err(e)) => Err(e.into()),
            None => Ok(None),
        }
    }

    #[pyo3(signature = (n = 1))]
    fn take(&self, n: usize) -> PyResult<Vec<ryo3_bytes::PyBytes>> {
        let mut inner = self.file_read_stream.py_lock()?;
        let mut result = Vec::with_capacity(n);
        for _ in 0..n {
            match inner.next() {
                Some(Ok(b)) => result.push(b.into()),
                Some(Err(e)) => return Err(e.into()),
                None => break,
            }
        }
        Ok(result)
    }

    fn collect(&self, py: Python<'_>) -> PyResult<Vec<ryo3_bytes::PyBytes>> {
        let mut inner = self.file_read_stream.py_lock()?;
        let mut result = Vec::new();
        while let Some(Ok(b)) = inner.next() {
            result.push(b.into());
            if result.len().is_multiple_of(256) {
                py.check_signals()?;
            }
        }
        Ok(result)
    }
}

impl std::fmt::Debug for PyFileReadStream {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FileReadStream(path='{}'", self.options.path.display(),)?;
        write!(f, ", read_size={}", self.options.read_size)?;
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
