use bytes::{Bytes, BytesMut};
use pyo3::exceptions::PyRuntimeError;
use pyo3::{pyclass, pymethods, PyRef, PyResult};
use std::fs::File;
use std::io;
use std::io::{Read, Seek, SeekFrom};
use std::path::Path;
use std::sync::Mutex;

pub(crate) struct FileReadStream {
    file: File,
    chunk_size: usize,
    buffer: BytesMut,
}

impl FileReadStream {
    pub(crate) fn new<P: AsRef<Path>>(path: P, chunk_size: usize) -> io::Result<Self> {
        let file = File::open(path)?;
        Ok(Self {
            file,
            chunk_size,
            buffer: BytesMut::with_capacity(chunk_size),
        })
    }

    pub(crate) fn new_with_offset<P: AsRef<Path>>(
        path: P,
        chunk_size: usize,
        offset: u64,
    ) -> io::Result<Self> {
        if offset == 0 {
            Self::new(path, chunk_size)
        } else {
            let mut file = File::open(path)?;
            file.seek(SeekFrom::Start(offset))?;
            Ok(Self {
                file,
                chunk_size,
                buffer: BytesMut::with_capacity(chunk_size),
            })
        }
    }
}

impl Iterator for FileReadStream {
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

#[pyclass(name = "FileReadStream", module = "ry.ryo3", frozen)]
pub struct PyFileReadStream {
    pub(crate) file_read_stream: Mutex<FileReadStream>,
}

impl From<FileReadStream> for PyFileReadStream {
    fn from(file_read_stream: FileReadStream) -> Self {
        Self {
            file_read_stream: Mutex::new(file_read_stream),
        }
    }
}

#[pymethods]
impl PyFileReadStream {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__(&self) -> PyResult<Option<ryo3_bytes::PyBytes>> {
        if let Ok(mut inner) = self.file_read_stream.lock() {
            match inner.next() {
                Some(Ok(b)) => Ok(Some(b.into())),
                Some(Err(e)) => Err(e.into()),
                None => Ok(None),
            }
        } else {
            Err(PyRuntimeError::new_err("lock error on file read stream"))
        }
    }
}
