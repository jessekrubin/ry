use bytes::{Bytes, BytesMut};
use pyo3::prelude::*;
use pyo3::{PyRef, PyResult, pyclass, pymethods};
use ryo3_core::PyMutex;
use ryo3_macro_rules::{py_runtime_err, py_value_err};
use std::fs::File;
use std::io::{self, BufReader, Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};
use std::sync::Mutex;

pub(crate) const DEFAULT_CHUNK_SIZE: usize = 65536;

pub(crate) struct FileReadStream<T: Read + Seek> {
    file: T,
    chunk_size: usize,
    buffer: BytesMut,
}

impl<T: Read + Seek> FileReadStream<T> {
    pub(crate) fn new(file: T, chunk_size: usize) -> Self {
        Self {
            file,
            chunk_size,
            buffer: BytesMut::with_capacity(chunk_size),
        }
    }
}

impl FileReadStream<File> {
    fn from_path<P: AsRef<Path>>(path: P, chunk_size: usize) -> io::Result<Self> {
        let file = File::open(path)?;
        Ok(Self::new(file, chunk_size))
    }
}

impl FileReadStream<BufReader<File>> {
    fn from_path_buffered<P: AsRef<Path>>(path: P, chunk_size: usize) -> io::Result<Self> {
        let file = File::open(path)?;
        let reader = BufReader::with_capacity(chunk_size * 2, file);
        Ok(Self::new(reader, chunk_size))
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
        self.buffer.resize(self.chunk_size, 0);
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
    pub(crate) path: PathBuf,
    pub(crate) chunk_size: usize,
    pub(crate) offset: u64,
    pub(crate) buffered: bool,
}

#[pyclass(name = "FileReadStream", frozen, immutable_type)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
pub struct PyFileReadStream {
    pub(crate) options: PyFileReadStreamOptions,
    pub(crate) file_read_stream: PyMutex<FileReadStreamWrapper>,
}

#[pymethods]
impl PyFileReadStream {
    #[new]
    #[pyo3(signature = (path, *, chunk_size = DEFAULT_CHUNK_SIZE, offset = 0, buffered = true))]
    pub fn py_new(path: PathBuf, chunk_size: usize, offset: u64, buffered: bool) -> PyResult<Self> {
        if chunk_size == 0 {
            return py_value_err!("chunk_size must be greater than 0");
        }
        let mut stream = if buffered {
            FileReadStreamWrapper::Buffered(FileReadStream::from_path_buffered(&path, chunk_size)?)
        } else {
            FileReadStreamWrapper::Unbuffered(FileReadStream::from_path(&path, chunk_size)?)
        };
        if offset > 0 {
            stream.seek_to(offset)?;
        }
        Ok(Self {
            options: PyFileReadStreamOptions {
                path,
                chunk_size,
                offset,
                buffered,
            },
            file_read_stream: PyMutex::new(stream),
        })
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
        write!(f, ", chunk_size={}", self.options.chunk_size)?;
        if self.options.offset != 0 {
            write!(f, ", offset={}", self.options.offset)?;
        }
        if self.options.buffered {
            write!(f, ", buffered=True")?;
        } else {
            write!(f, ", buffered=False")?;
        }
        write!(f, ")")
    }
}
