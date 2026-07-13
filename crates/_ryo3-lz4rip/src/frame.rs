//! lz4 frame (de)compression
//!
//! one shot functions and streaming compressor class
//!
//! ---
//!
//! ## NOTES:
//!
//! ### Decompressor
//!
//! a decompressor is more involved and I am not totally sure how to do it
//! w/o some sort of streaming interface seeing as AFAICT (me/jesse) the
//! current `lz4rip::frame::FrameDecoder` is pull based but for python,
//! it would have to be push based and allow emitting chunks..
//!
//! mayhaps will ask @paddor what he thinks
use std::io::{Read, Write};

use lz4rip::frame::{FrameDecoder, FrameEncoder, FrameInfo};
use pyo3::prelude::*;
use ryo3_bytes::{ReadableBuffer, RyBytes};
use ryo3_core::macros::{py_type_err, py_type_error, py_value_err, py_value_error};
use ryo3_core::py_dict::BorrowedDictIter;
use ryo3_core::{PyCastExactOpt, pystr_read_fast};

use crate::Ryo3Lz4ripResult;
use crate::constants::{LZ4F_FLG_CONTENT_SIZE, LZ4F_MAGIC};
use crate::error::{Error, RyLz4Error};

/// max bytes are pre-allocated purely on the header's declared content-size.
/// larger outputs grow on demand as real decoded data materializes, so a
/// lying header can't force a giant up-front allocation
///
/// python-lz4's oneshot, mallocs the declared size blindly which seems stupid
/// to me, but idk?
const MAX_TRUSTED_HINT: usize = 64 << 20; // 64 MiB

/// capacity hint for decompressing a frame
///
/// snag it from the header bc lz4rip dont expose it under the hood.
/// a declared size is trusted only up to `MAX_TRUSTED_HINT`. w/o a size from
/// the header guess
///
/// REF: <https://github.com/lz4/lz4/blob/dev/doc/lz4_Frame_format.md#frame-descriptor>
fn frame_capacity_hint(input: &[u8]) -> Ryo3Lz4ripResult<usize> {
    let has_content_size =
        input.len() >= 14 && input[..4] == LZ4F_MAGIC && input[4] & LZ4F_FLG_CONTENT_SIZE != 0;
    if !has_content_size {
        // no header size, so guess away
        return Ok(input.len().saturating_mul(2));
    }
    let declared = u64::from_le_bytes(input[6..14].try_into().expect("14-byte bound checked"));
    let declared =
        usize::try_from(declared).map_err(|_| RyLz4Error::FrameContentSizeTooBig { declared })?;
    // trust the header only up to a cap and a theoretical 255x max ratio
    Ok(declared
        .min(input.len().saturating_mul(255))
        .min(MAX_TRUSTED_HINT))
}

#[pyfunction]
#[pyo3(signature = (data, *, dictionary = None, dict_id = None, frame_info = None))]
#[expect(clippy::needless_pass_by_value)]
pub fn lz4_compress(
    py: Python<'_>,
    data: ReadableBuffer,
    dictionary: Option<ReadableBuffer>,
    dict_id: Option<u32>,
    frame_info: Option<PyFrameInfo>,
) -> Ryo3Lz4ripResult<RyBytes> {
    let input = data.as_ref();
    let dict = dictionary.as_ref().map(AsRef::as_ref);
    let frame_info = frame_info.map_or_else(
        || FrameInfo::new().content_size(Some(input.len() as u64)),
        PyFrameInfo::into_inner,
    );
    py.detach(|| {
        let output = Vec::with_capacity(input.len() / 2 + 64);
        let mut encoder = if let Some(dict) = dict {
            FrameEncoder::with_dictionary(output, dict, dict_id.unwrap_or(0), Some(frame_info))
                .map_err(Error::from)?
        } else {
            FrameEncoder::with_frame_info(frame_info, output)
        };
        encoder.write_all(input).map_err(Error::from)?;
        let output = encoder.finish().map_err(Error::from)?;
        Ok(output.into())
    })
}

#[pyfunction]
#[pyo3(signature = (data, *, dictionary = None, dict_id = None))]
#[expect(clippy::needless_pass_by_value)]
pub fn lz4_decompress(
    py: Python<'_>,
    data: ReadableBuffer,
    dictionary: Option<ReadableBuffer>,
    dict_id: Option<u32>,
) -> Ryo3Lz4ripResult<RyBytes> {
    let input = data.as_ref();
    let dict = dictionary.as_ref().map(AsRef::as_ref);
    py.detach(|| {
        let mut output = Vec::with_capacity(frame_capacity_hint(input)?);
        let mut decoder = if let Some(dict) = dict {
            FrameDecoder::with_dictionary(input, dict, dict_id.unwrap_or(0))
        } else {
            FrameDecoder::new(input)
        };
        decoder.read_to_end(&mut output).map_err(Error::from)?;
        Ok(output.into())
    })
}

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
        if let Some(pyint) = ob.cast_exact_opt::<pyo3::types::PyInt>() {
            let i = pyint
                .extract::<u8>()
                .map_err(|_| py_value_error!("Invalid block-size (options: {LZ4RIP_BLOCKSIZE})"))?;
            match i {
                0 => Ok(Self::AUTO),
                4 => Ok(Self::MAX_64KB),
                5 => Ok(Self::MAX_256KB),
                6 => Ok(Self::MAX_1MB),
                7 => Ok(Self::MAX_4MB),
                _ => py_value_err!("Invalid block-size (options: {LZ4RIP_BLOCKSIZE})"),
            }
        } else if let Some(pystr) = ob.cast_exact_opt::<pyo3::types::PyString>() {
            let s = pystr.to_str()?;
            match s {
                "auto" => Ok(Self::AUTO),
                "max-64kb" => Ok(Self::MAX_64KB),
                "max-256kb" => Ok(Self::MAX_256KB),
                "max-1mb" => Ok(Self::MAX_1MB),
                "max-4mb" => Ok(Self::MAX_4MB),
                _ => py_value_err!("Invalid block-size: {s} (options: {LZ4RIP_BLOCKSIZE})"),
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
        if let Ok(s) = ob.extract::<&str>() {
            match s {
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

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
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

impl From<FrameInfo> for PyFrameInfo {
    fn from(frame_info: FrameInfo) -> Self {
        Self::new(frame_info)
    }
}

impl<'py> FromPyObject<'_, 'py> for PyFrameInfo {
    type Error = PyErr;

    fn extract(ob: Borrowed<'_, 'py, PyAny>) -> PyResult<Self> {
        if let Some(dict) = ob.cast_exact_opt::<pyo3::types::PyDict>() {
            let mut frame_info = lz4rip::frame::FrameInfo::new();
            for (k, v) in BorrowedDictIter::new(dict) {
                let pystr = k
                    .cast_exact_opt::<pyo3::types::PyString>()
                    .ok_or_else(|| py_type_error!("FrameInfo keys must be strings"))?;
                let s = pystr_read_fast(pystr)?;
                match s {
                    "block_size" => {
                        let block_size = v.extract::<PyBlockSize>()?;
                        frame_info = frame_info.block_size(block_size.into_inner());
                    }
                    "block_mode" => {
                        let block_mode = v.extract::<PyBlockMode>()?;
                        frame_info = frame_info.block_mode(block_mode.into_inner());
                    }
                    "block_checksums" => {
                        let block_checksums = v.extract::<bool>()?;
                        frame_info = frame_info.block_checksums(block_checksums);
                    }
                    "content_checksum" => {
                        let content_checksum = v.extract::<bool>()?;
                        frame_info = frame_info.content_checksum(content_checksum);
                    }
                    "content_size" => {
                        let content_size = v.extract::<Option<u64>>()?;
                        frame_info = frame_info.content_size(content_size);
                    }
                    _ => {
                        return py_value_err!("Invalid FrameInfo key: {s}");
                    }
                }
            }
            Ok(Self::new(frame_info))
        } else {
            py_type_err!(
                "Expected a dictionary for FrameInfo (keys: 'block_size', 'block_mode', 'block_checksums', 'content_checksum', 'content_size')"
            )
        }
    }
}

#[derive(Clone, Debug)]
struct PyLz4FrameCompressorConfig {
    frame_info: PyFrameInfo,
    dictionary: Option<RyBytes>,
    dict_id: u32,
}

impl PyLz4FrameCompressorConfig {
    fn build_encoder(&self) -> Ryo3Lz4ripResult<FrameEncoder<Vec<u8>>> {
        let frame_info = *self.frame_info.inner();
        if let Some(dict) = &self.dictionary {
            FrameEncoder::with_dictionary(Vec::new(), dict.as_ref(), self.dict_id, Some(frame_info))
                .map_err(Error::from)
        } else {
            Ok(FrameEncoder::with_frame_info(frame_info, Vec::new()))
        }
    }

    fn build_encoder_unchecked(&self) -> FrameEncoder<Vec<u8>> {
        self.build_encoder()
            .expect("wenodis: only called in `reset`")
    }
}

/// streaming lz4 frame compressor wrapping `lz4rip::frame::FrameEncoder`
///
/// - writes into an owned `Vec<u8>`, which is drained on each call
/// - `compress`/`glush`/`finish` only return the bytes newly produced
/// - `finish` writes the stream terminator and returns the tail.
/// - `reset` resets the compressor to its initial state, allowing reuse of the same config.
/// - `copy` creates a new compressor with the same config
#[pyclass(name = "Lz4FrameCompressor", immutable_type, skip_from_py_object)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
pub struct PyLz4FrameCompressor {
    config: PyLz4FrameCompressorConfig,
    encoder: Option<FrameEncoder<Vec<u8>>>,
}

impl PyLz4FrameCompressor {
    fn encoder_mut(&mut self) -> PyResult<&mut FrameEncoder<Vec<u8>>> {
        self.encoder
            .as_mut()
            .ok_or_else(|| py_value_error!("Lz4FrameCompressor is finished"))
    }

    /// take it all!
    fn drain(encoder: &mut FrameEncoder<Vec<u8>>) -> Vec<u8> {
        std::mem::take(encoder.get_mut())
    }
}

#[pymethods]
impl PyLz4FrameCompressor {
    #[new]
    #[pyo3(signature = (*, dictionary = None, dict_id = None, frame_info = None))]
    fn py_new(
        dictionary: Option<ReadableBuffer>,
        dict_id: Option<u32>,
        frame_info: Option<PyFrameInfo>,
    ) -> Ryo3Lz4ripResult<Self> {
        let frame_info = frame_info.map(PyFrameInfo::into_inner).unwrap_or_default();
        let dictionary = dictionary.map(|d| d.to_rybytes());
        let dict_id = dict_id.unwrap_or(0);

        let encoder = if let Some(dict) = &dictionary {
            FrameEncoder::with_dictionary(Vec::new(), dict.as_ref(), dict_id, Some(frame_info))
                .map_err(Error::from)?
        } else {
            FrameEncoder::with_frame_info(frame_info, Vec::new())
        };

        Ok(Self {
            config: PyLz4FrameCompressorConfig {
                dictionary,
                dict_id,
                frame_info: frame_info.into(),
            },
            encoder: Some(encoder),
        })
    }

    /// feed compressor and return any new data if `data` is provided
    #[expect(clippy::needless_pass_by_value)]
    fn compress(&mut self, py: Python<'_>, data: ReadableBuffer) -> PyResult<RyBytes> {
        let input = data.as_ref();
        let encoder = self.encoder_mut()?;
        let b = py.detach(|| {
            encoder.write_all(input).map_err(Error::from)?;
            Ok::<_, Error>(Self::drain(encoder))
        })?;
        Ok(b.into())
    }

    fn flush(&mut self, py: Python<'_>) -> PyResult<RyBytes> {
        let encoder = self.encoder_mut()?;
        let v = py.detach(|| {
            encoder.flush().map_err(Error::from)?;
            Ok::<_, Error>(Self::drain(encoder))
        })?;
        Ok(v.into())
    }

    /// finish and terminate the frame
    fn finish(&mut self, py: Python<'_>) -> PyResult<RyBytes> {
        let encoder = self
            .encoder
            .take()
            .ok_or_else(|| py_value_error!("Lz4FrameCompressor is finished"))?;
        py.detach(|| {
            let output = encoder.finish().map_err(Error::from)?;
            Ok(output.into())
        })
    }

    fn reset(&mut self) {
        let encoder = self.config.build_encoder_unchecked();
        self.encoder = Some(encoder);
    }

    #[pyo3(name = "copy")]
    fn py_copy(&self) -> Self {
        let encoder = self.config.build_encoder_unchecked();
        Self {
            config: self.config.clone(),
            encoder: Some(encoder),
        }
    }
}

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(lz4_compress, m)?)?;
    m.add_function(wrap_pyfunction!(lz4_decompress, m)?)?;
    m.add_class::<PyLz4FrameCompressor>()?;
    Ok(())
}
