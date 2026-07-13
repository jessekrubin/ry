//! lz4 block (de)compression
use lz4rip::block::{
    CompressError, DecompressError, compress_into, compress_into_with_dict, decompress_into,
    decompress_into_with_dict, get_maximum_output_size,
};
use pyo3::prelude::*;
use ryo3_bytes::{ReadableBuffer, RyBytes};

use crate::Ryo3Lz4ripResult;
use crate::error::RyLz4Error;

/// python-lz4 compatible u32-le uncompressed-size prefix
const SIZE_PREFIX_LEN: usize = 4;
/// max lz4 compression ratio; used to sanity-check size prefixes
const MAX_COMPRESSION_RATIO: usize = 255;

fn size_prefix(input_len: usize) -> Ryo3Lz4ripResult<[u8; SIZE_PREFIX_LEN]> {
    u32::try_from(input_len)
        .map(u32::to_le_bytes)
        .map_err(|_| RyLz4Error::BlockPrefixTooBig.into())
}

/// split the size prefix from the input
fn read_size_prefix(input: &[u8]) -> Ryo3Lz4ripResult<(usize, &[u8])> {
    let Some((prefix, rest)) = input.split_first_chunk::<SIZE_PREFIX_LEN>() else {
        return Err(RyLz4Error::BlockPrefixMissing {
            input_len: input.len(),
        }
        .into());
    };
    let size = u32::from_le_bytes(*prefix) as usize;
    if size > rest.len().saturating_mul(MAX_COMPRESSION_RATIO) {
        return Err(RyLz4Error::BlockPrefixFishy {
            size,
            input_len: rest.len(),
        }
        .into());
    }
    Ok((size, rest))
}

/// compress block with a "compress-into" function
fn compress_block_with<const PREFIX: bool, F>(
    input: &[u8],
    compress_into: F,
) -> Ryo3Lz4ripResult<Vec<u8>>
where
    F: FnOnce(&[u8], &mut [u8]) -> Result<usize, CompressError>,
{
    if PREFIX {
        let prefix = size_prefix(input.len())?;
        let prefix_len = prefix.len();
        let mut output = vec![0; prefix_len + get_maximum_output_size(input.len())];
        output[..prefix_len].copy_from_slice(&prefix);
        let written = compress_into(input, &mut output[prefix_len..])?;
        output.truncate(prefix_len + written);
        Ok(output)
    } else {
        let mut output = vec![0; get_maximum_output_size(input.len())];
        let written = compress_into(input, &mut output)?;
        output.truncate(written);
        Ok(output)
    }
}

/// decompress lz4 block chunk using a block decompress-into fn.
///
/// expects a u32-le uncompressed-size prefix (which must match the actual
/// decompressed size exactly), an explicit `size` means a raw block (and may
/// be an over-estimate).
fn decompress_block_with<F>(
    input: &[u8],
    size: Option<usize>,
    decompress_into: F,
) -> Ryo3Lz4ripResult<Vec<u8>>
where
    F: FnOnce(&[u8], &mut [u8]) -> Result<usize, DecompressError>,
{
    let (input, size, size_is_exact) = if let Some(size) = size {
        (input, size, false)
    } else {
        let (size, rest) = read_size_prefix(input)?;
        (rest, size, true)
    };
    let mut output = vec![0; size];
    let written = decompress_into(input, &mut output)?;
    if size_is_exact && written != size {
        return Err(RyLz4Error::BlockPrefixMismatch {
            expected: size,
            actual: written,
        }
        .into());
    }
    output.truncate(written);
    Ok(output)
}

#[pyfunction]
#[pyo3(signature = (data, *, size = false, dictionary = None))]
#[expect(clippy::needless_pass_by_value)]
pub fn lz4_compress_block(
    py: Python<'_>,
    data: ReadableBuffer,
    size: bool,
    dictionary: Option<ReadableBuffer>,
) -> Ryo3Lz4ripResult<RyBytes> {
    let input = data.as_ref();
    let dict = dictionary.as_ref().map(AsRef::as_ref);
    py.detach(|| {
        let output = match (dict, size) {
            (Some(dict), true) => {
                compress_block_with::<true, _>(input, |i, o| compress_into_with_dict(i, o, dict))?
            }
            (Some(dict), false) => {
                compress_block_with::<false, _>(input, |i, o| compress_into_with_dict(i, o, dict))?
            }
            (None, true) => compress_block_with::<true, _>(input, compress_into)?,
            (None, false) => compress_block_with::<false, _>(input, compress_into)?,
        };
        Ok(output.into())
    })
}

#[pyfunction]
#[pyo3(signature = (data, size = None, *, dictionary = None))]
#[expect(clippy::needless_pass_by_value)]
pub fn lz4_decompress_block(
    py: Python<'_>,
    data: ReadableBuffer,
    size: Option<usize>,
    dictionary: Option<ReadableBuffer>,
) -> Ryo3Lz4ripResult<RyBytes> {
    let input = data.as_ref();
    let dict = dictionary.as_ref().map(AsRef::as_ref);
    py.detach(|| {
        let output = if let Some(dict) = dict {
            decompress_block_with(input, size, |i, o| decompress_into_with_dict(i, o, dict))?
        } else {
            decompress_block_with(input, size, decompress_into)?
        };
        Ok(output.into())
    })
}

#[derive(Debug)]
enum BlockCompressor {
    Vanilla(lz4rip::block::Compressor),
    Dict(lz4rip::block::DictCompressor),
}

impl BlockCompressor {
    fn compress_into(&mut self, input: &[u8], output: &mut [u8]) -> Result<usize, CompressError> {
        match self {
            Self::Vanilla(c) => c.compress_into(input, output),
            Self::Dict(c) => c.compress_into(input, output),
        }
    }
}

#[pyclass(name = "Lz4BlockCompressor", immutable_type, skip_from_py_object)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
#[derive(Debug)]
pub struct PyLz4BlockCompressor(
    // boxed: the compressors embed 64-byte-aligned hash tables, which exceeds
    // what pyo3's pyclass allocation guarantees - inlined, construction
    // dereferences a misaligned pointer (UB; aborts in debug):
    //
    //   thread '<unnamed>' panicked at pyo3-0.29.0\src\pyclass_init.rs:160:18:
    //   misaligned pointer dereference: address must be a multiple of 0x40 but is 0x19ef54173b0
    //   thread caused non-unwinding panic. aborting.
    Box<BlockCompressor>,
);

#[pymethods]
impl PyLz4BlockCompressor {
    #[new]
    #[pyo3(signature = (dictionary = None))]
    fn py_new(dictionary: Option<ReadableBuffer>) -> Self {
        let inner = if let Some(dict) = dictionary {
            BlockCompressor::Dict(lz4rip::block::DictCompressor::new(dict.as_ref()))
        } else {
            BlockCompressor::Vanilla(lz4rip::block::Compressor::new())
        };
        Self(Box::new(inner))
    }

    #[expect(clippy::needless_pass_by_value, reason = "python extract")]
    #[pyo3(name = "compress", signature = (data, *, size = false))]
    fn py_compress(
        &mut self,
        py: Python<'_>,
        data: ReadableBuffer,
        size: bool,
    ) -> Ryo3Lz4ripResult<RyBytes> {
        let input = data.as_ref();
        py.detach(|| {
            if size {
                compress_block_with::<true, _>(input, |i, o| self.0.compress_into(i, o))
            } else {
                compress_block_with::<false, _>(input, |i, o| self.0.compress_into(i, o))
            }
            .map(RyBytes::from)
        })
    }
}

#[pyclass(name = "Lz4BlockDecompressor", immutable_type, skip_from_py_object)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
#[derive(Debug)]
pub struct PyLz4BlockDecompressor(lz4rip::block::Decompressor);

#[pymethods]
impl PyLz4BlockDecompressor {
    #[new]
    #[pyo3(signature = (dictionary = None))]
    fn py_new(dictionary: Option<ReadableBuffer>) -> Self {
        if let Some(dict) = dictionary {
            Self(lz4rip::block::Decompressor::with_dict(dict.as_ref()))
        } else {
            Self(lz4rip::block::Decompressor::new())
        }
    }

    #[pyo3(name = "decompress", signature = (data, size = None))]
    #[expect(clippy::needless_pass_by_value, reason = "python extract")]
    fn py_decompress(
        &self,
        py: Python<'_>,
        data: ReadableBuffer,
        size: Option<usize>,
    ) -> Ryo3Lz4ripResult<RyBytes> {
        let input = data.as_ref();
        py.detach(|| {
            decompress_block_with(input, size, |i, o| self.0.decompress_into(i, o))
                .map(RyBytes::from)
        })
    }
}
