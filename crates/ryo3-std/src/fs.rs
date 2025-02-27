use bytes::{Bytes, BytesMut};
use pyo3::exceptions::{PyIsADirectoryError, PyNotADirectoryError, PyUnicodeDecodeError};
use pyo3::prelude::*;
use pyo3::types::PyBytes;
use ryo3_bytes::extract_bytes_ref_str;
use ryo3_core::types::PathLike;
use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom};
use std::path::Path;
use std::time::SystemTime;

#[pyclass(name = "FileType", module = "ry", frozen)]
pub struct PyFileType(pub std::fs::FileType);
impl PyFileType {
    #[must_use]
    pub fn new(ft: std::fs::FileType) -> Self {
        Self(ft)
    }
}

impl From<std::fs::FileType> for PyFileType {
    fn from(ft: std::fs::FileType) -> Self {
        Self(ft)
    }
}

#[pymethods]
impl PyFileType {
    #[getter]
    #[must_use]
    pub fn is_dir(&self) -> bool {
        self.0.is_dir()
    }

    #[getter]
    #[must_use]
    pub fn is_file(&self) -> bool {
        self.0.is_file()
    }

    #[getter]
    #[must_use]
    pub fn is_symlink(&self) -> bool {
        self.0.is_symlink()
    }

    pub fn __repr__(&self) -> PyResult<String> {
        let repr = format!(
            "FileType(is_dir={}, is_file={}, is_symlink={})",
            self.0.is_dir(),
            self.0.is_file(),
            self.0.is_symlink()
        );
        Ok(repr)
    }
}

#[pyclass(name = "Metadata", module = "ry", frozen)]
pub struct PyMetadata(pub std::fs::Metadata);

impl From<std::fs::Metadata> for PyMetadata {
    fn from(m: std::fs::Metadata) -> Self {
        Self(m)
    }
}

impl PyMetadata {
    #[must_use]
    pub fn new(m: std::fs::Metadata) -> Self {
        Self(m)
    }
}

#[pymethods]
impl PyMetadata {
    #[getter]
    pub fn accessed(&self) -> PyResult<SystemTime> {
        let accessed = self.0.accessed()?;
        Ok(accessed)
    }

    #[getter]
    pub fn created(&self) -> PyResult<SystemTime> {
        let created = self.0.created()?;
        Ok(created)
    }

    #[getter]
    #[must_use]
    pub fn file_type(&self) -> PyFileType {
        PyFileType::new(self.0.file_type())
    }

    #[getter]
    #[must_use]
    pub fn is_dir(&self) -> bool {
        self.0.is_dir()
    }

    #[getter]
    #[must_use]
    pub fn is_file(&self) -> bool {
        self.0.is_file()
    }

    #[getter]
    #[must_use]
    pub fn is_symlink(&self) -> bool {
        self.0.file_type().is_symlink()
    }

    #[getter]
    #[must_use]
    pub fn len(&self) -> u64 {
        self.0.len()
    }

    #[getter]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.len() == 0
    }

    #[getter]
    pub fn modified(&self) -> PyResult<SystemTime> {
        let modified = self.0.modified()?;
        Ok(modified)
    }

    #[getter]
    #[must_use]
    pub fn readonly(&self) -> bool {
        self.0.permissions().readonly()
    }

    // #[getter]
    // pub fn permissions(&self) -> PyResult<PyObject> {
    //     let permissions = self.0.permissions();
    //     Ok(permissions.into())
    // }
}

// ============================================================================
// FUNCTIONS
// ============================================================================

pub struct FileReadStream {
    file: File,
    chunk_size: usize,
    buffer: BytesMut,
}

impl FileReadStream {
    pub fn new<P: AsRef<Path>>(path: P, chunk_size: usize) -> io::Result<Self> {
        let file = File::open(path)?;
        Ok(Self {
            file,
            chunk_size,
            buffer: BytesMut::with_capacity(chunk_size),
        })
    }

    pub fn new_with_offset<P: AsRef<Path>>(
        path: P,
        chunk_size: usize,
        offset: u64,
    ) -> io::Result<Self> {
        let mut file = File::open(path)?;
        file.seek(SeekFrom::Start(offset))?;
        Ok(Self {
            file,
            chunk_size,
            buffer: BytesMut::with_capacity(chunk_size),
        })
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
#[pyclass(name = "FileReadStream", module = "ryo3")]
pub struct PyFileReadStream {
    file_read_stream: FileReadStream,
}

#[pymethods]
impl PyFileReadStream {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__(mut slf: PyRefMut<'_, Self>) -> PyResult<Option<ryo3_bytes::PyBytes>> {
        let b = slf.file_read_stream.next();
        match b {
            Some(Ok(b)) => Ok(Some(b.into())),
            Some(Err(e)) => Err(e.into()),
            None => Ok(None),
        }
    }
}

#[pyfunction]
#[pyo3(signature = (pth, chunk_size = 65536, *, offset = 0))]
#[expect(clippy::needless_pass_by_value)]
pub fn read_stream(pth: PathLike, chunk_size: usize, offset: u64) -> PyResult<PyFileReadStream> {
    let pth = pth.as_ref();
    let file_read_stream_res = FileReadStream::new_with_offset(pth, chunk_size, offset);
    match file_read_stream_res {
        Ok(file_read_stream) => Ok(PyFileReadStream { file_read_stream }),
        Err(e) => {
            if pth.is_dir() {
                let pth_str = pth.to_string_lossy();
                Err(PyIsADirectoryError::new_err(format!(
                    "read_stream - parent: {pth_str} - {e}"
                )))
            } else {
                Err(e.into())
            }
        }
    }
}

#[pyfunction]
pub fn read(pth: PathLike) -> PyResult<ryo3_bytes::PyBytes> {
    let fbytes = std::fs::read(pth)?;
    Ok(fbytes.into())
}

#[pyfunction]
pub fn read_bytes(py: Python<'_>, s: PathLike) -> PyResult<PyObject> {
    let fbytes = std::fs::read(s)?;
    Ok(PyBytes::new(py, &fbytes).into())
}

#[pyfunction]
pub fn read_text(py: Python<'_>, s: PathLike) -> PyResult<String> {
    let fbytes = std::fs::read(s)?;
    let r = std::str::from_utf8(&fbytes);
    match r {
        Ok(s) => Ok(s.to_string()),
        Err(e) => {
            let decode_err = PyUnicodeDecodeError::new_utf8(py, &fbytes, e)?;
            Err(decode_err.into())
        }
    }
}

#[expect(clippy::needless_pass_by_value)]
#[pyfunction]
pub fn write(fspath: PathLike, b: &Bound<'_, PyAny>) -> PyResult<usize> {
    let bref = extract_bytes_ref_str(b)?;
    let write_res = std::fs::write(fspath.as_ref(), bref);
    match write_res {
        Ok(()) => Ok(bref.len()),
        Err(e) => Err(PyNotADirectoryError::new_err(format!(
            "write_bytes - parent: {fspath} - {e}"
        ))),
    }
}

#[expect(clippy::needless_pass_by_value)]
#[pyfunction]
pub fn write_bytes(fspath: PathLike, b: &[u8]) -> PyResult<usize> {
    let write_res = std::fs::write(fspath.as_ref(), b);
    match write_res {
        Ok(()) => Ok(b.len()),
        Err(e) => Err(PyNotADirectoryError::new_err(format!(
            "write_bytes - parent: {fspath} - {e}"
        ))),
    }
}

#[expect(clippy::needless_pass_by_value)]
#[pyfunction]
pub fn write_text(fspath: PathLike, string: &str) -> PyResult<usize> {
    let str_bytes = string.as_bytes();
    match std::fs::write(fspath.as_ref(), str_bytes) {
        Ok(()) => Ok(str_bytes.len()),
        Err(e) => Err(PyNotADirectoryError::new_err(format!(
            "write_bytes - parent: {fspath} - {e}"
        ))),
    }
}

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyMetadata>()?;
    m.add_class::<PyFileType>()?;
    m.add_function(wrap_pyfunction!(read, m)?)?;
    m.add_function(wrap_pyfunction!(read_stream, m)?)?;
    m.add_function(wrap_pyfunction!(read_bytes, m)?)?;
    m.add_function(wrap_pyfunction!(read_text, m)?)?;
    m.add_function(wrap_pyfunction!(write, m)?)?;
    m.add_function(wrap_pyfunction!(write_bytes, m)?)?;
    m.add_function(wrap_pyfunction!(write_text, m)?)?;
    Ok(())
}
