use pyo3::prelude::*;
use ryo3_core::PyCastExactOpt;
use ryo3_core::macros::{py_type_err, py_value_err};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct PyBlockSize(lz4rip::frame::BlockSize);

impl PyBlockSize {
    const AUTO: Self = Self::new(lz4rip::frame::BlockSize::Auto);
    const MAX_64KB: Self = Self::new(lz4rip::frame::BlockSize::Max64KB);
    const MAX_256KB: Self = Self::new(lz4rip::frame::BlockSize::Max256KB);
    const MAX_1MB: Self = Self::new(lz4rip::frame::BlockSize::Max1MB);
    const MAX_4MB: Self = Self::new(lz4rip::frame::BlockSize::Max4MB);

    const fn new(block_size: lz4rip::frame::BlockSize) -> Self {
        Self(block_size)
    }

    #[must_use]
    pub fn inner(&self) -> &lz4rip::frame::BlockSize {
        &self.0
    }

    #[must_use]
    pub fn into_inner(self) -> lz4rip::frame::BlockSize {
        self.0
    }
}

const LZ4RIP_BLOCKSIZE: &str =
    "'auto' = 0, 'max-64kb' = 4, 'max-256kb' = 5, 'max-1mb' = 6, 'max-4mb' = 7";

impl<'py> FromPyObject<'_, 'py> for PyBlockSize {
    type Error = PyErr;
    fn extract(ob: Borrowed<'_, 'py, PyAny>) -> PyResult<Self> {
        // downcast to string...
        if let Ok(s) = ob.extract::<&str>() {
            match s.to_ascii_lowercase().as_str() {
                "auto" => Ok(Self::AUTO),
                "max-64kb" => Ok(Self::MAX_64KB),
                "max-256kb" => Ok(Self::MAX_256KB),
                "max-1mb" => Ok(Self::MAX_1MB),
                "max-4mb" => Ok(Self::MAX_4MB),
                _ => py_value_err!("Invalid block-size: {s} (options: {LZ4RIP_BLOCKSIZE})"),
            }
        } else if let Ok(i) = ob.extract::<u8>() {
            match i {
                0 => Ok(Self::AUTO),
                4 => Ok(Self::MAX_64KB),
                5 => Ok(Self::MAX_256KB),
                6 => Ok(Self::MAX_1MB),
                7 => Ok(Self::MAX_4MB),
                _ => py_value_err!("Invalid block-size: {i} (options: {LZ4RIP_BLOCKSIZE})"),
            }
        } else {
            py_type_err!(
                "Invalid type for block-size, expected a string or integer (options: {LZ4RIP_BLOCKSIZE})"
            )
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct PyBlockMode(lz4rip::frame::BlockMode);

impl PyBlockMode {
    const INDEPENDENT: Self = Self::new(lz4rip::frame::BlockMode::Independent);
    const LINKED: Self = Self::new(lz4rip::frame::BlockMode::Linked);

    const fn new(block_mode: lz4rip::frame::BlockMode) -> Self {
        Self(block_mode)
    }

    #[must_use]
    pub fn inner(&self) -> &lz4rip::frame::BlockMode {
        &self.0
    }

    #[must_use]
    pub fn into_inner(self) -> lz4rip::frame::BlockMode {
        self.0
    }
}
impl<'py> FromPyObject<'_, 'py> for PyBlockMode {
    type Error = PyErr;
    fn extract(ob: Borrowed<'_, 'py, PyAny>) -> PyResult<Self> {
        // downcast to string...
        if let Ok(s) = ob.extract::<&str>() {
            match s.to_ascii_lowercase().as_str() {
                "independent" => Ok(Self::INDEPENDENT),
                "linked" => Ok(Self::LINKED),
                _ => py_value_err!("Invalid block-mode: {s} (options: 'independent', 'linked')"),
            }
        } else {
            py_type_err!(
                "Invalid type for block-mode, expected a string (options: 'independent', 'linked')"
            )
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct PyFrameInfo(lz4rip::frame::FrameInfo);

impl PyFrameInfo {
    #[must_use]
    pub fn new(frame_info: lz4rip::frame::FrameInfo) -> Self {
        Self(frame_info)
    }

    #[must_use]
    pub fn inner(&self) -> &lz4rip::frame::FrameInfo {
        &self.0
    }

    #[must_use]
    pub fn into_inner(self) -> lz4rip::frame::FrameInfo {
        self.0
    }
}

impl<'py> FromPyObject<'_, 'py> for PyFrameInfo {
    type Error = PyErr;

    fn extract(ob: Borrowed<'_, 'py, PyAny>) -> PyResult<Self> {
        if let Some(dict) = ob.cast_exact_opt::<pyo3::types::PyDict>() {
            let py = ob.py();
            let block_size = dict
                .get_item(pyo3::intern!(py, "block_size"))?
                .map_or(Ok(PyBlockSize::AUTO), |value| value.extract())?;
            let block_mode = dict
                .get_item(pyo3::intern!(py, "block_mode"))?
                .map_or(Ok(PyBlockMode::INDEPENDENT), |value| value.extract())?;
            let content_checksum = dict
                .get_item(pyo3::intern!(py, "content_checksum"))?
                .map_or(Ok(false), |value| value.extract())?;
            let content_size = dict
                .get_item(pyo3::intern!(py, "content_size"))?
                .map_or(Ok(None), |value| value.extract())?;
            let frame_info = lz4rip::frame::FrameInfo::new()
                .block_size(block_size.into_inner())
                .block_mode(block_mode.into_inner())
                .content_checksum(content_checksum)
                .content_size(content_size);

            Ok(Self::new(frame_info))
        } else {
            py_type_err!("Expected a dictionary for FrameInfo")
        }
    }
}
