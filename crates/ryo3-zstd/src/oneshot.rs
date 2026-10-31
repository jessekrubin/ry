use std::cell;

use pyo3::prelude::*;
use ryo3_bytes::{ReadableBuffer, RyBytes};
use zstd::{bulk as zbulk, zstd_safe};

use crate::compression_level::PyCompressionLevel;

/// Max bulk decompressed size before flip flopping to use the stream decompression path
const MAX_PREALLOC: u64 = 64 * 1024 * 1024;

/// Max input size for the cached (thread-local) compressor
///
/// Big bufs get a temp compressor instead of using a cache size doesnt explode (which it did in testing)
const MAX_CACHED_INPUT: usize = 128 * 1024;

type ZstdCompressorCache = cell::RefCell<Option<zbulk::Compressor<'static>>>;
thread_local! {
    static ZSTD_COMPRESSOR: ZstdCompressorCache = const { cell::RefCell::new(None) };
}

fn zstd_compress_impl(data: &[u8], level: i32) -> PyResult<Vec<u8>> {
    if data.len() > MAX_CACHED_INPUT {
        zbulk::compress(data, level)
    } else {
        ZSTD_COMPRESSOR.with_borrow_mut(|cached| match cached {
            Some(compressor) => {
                compressor.set_compression_level(level)?;
                compressor.compress(data)
            }
            None => cached.insert(zbulk::Compressor::new(level)?).compress(data),
        })
    }
    .map_err(|e| pyo3::exceptions::PyValueError::new_err(format!("zstd compress error: {e}")))
}

type ZstdDecompressorCache = cell::RefCell<Option<zbulk::Decompressor<'static>>>;
thread_local! {
    static ZSTD_DECOMPRESSOR: ZstdDecompressorCache = const { cell::RefCell::new(None) };
}

fn zstd_decompress_impl(data: &[u8]) -> PyResult<Vec<u8>> {
    // detect decompressed size if available and use bulk for it
    match zstd_safe::find_decompressed_size(data) {
        // wow clippy-fixing this made it totally un readable...
        Ok(Some(size)) if size <= MAX_PREALLOC => ZSTD_DECOMPRESSOR.with_borrow_mut(|cached| {
            let decompressor = match cached {
                Some(decompressor) => decompressor,
                None => cached.insert(zbulk::Decompressor::new()?),
            };
            #[expect(clippy::cast_possible_truncation)]
            decompressor.decompress(data, size as usize)
        }),
        _ => ::zstd::stream::decode_all(data),
    }
    .map_err(|e| pyo3::exceptions::PyValueError::new_err(format!("zstd decompress error: {e}")))
}

#[expect(clippy::needless_pass_by_value)]
#[pyfunction]
#[pyo3(
    signature = (data, /, level = PyCompressionLevel::default()),
    text_signature = "(data, /, level=3)"
)]
pub fn zstd_compress(
    py: Python<'_>,
    data: ReadableBuffer,
    level: PyCompressionLevel,
) -> PyResult<RyBytes> {
    let bin = data.as_slice();
    py.detach(|| zstd_compress_impl(bin, level.into_inner()))
        .map(RyBytes::from)
}

#[expect(clippy::needless_pass_by_value)]
#[pyfunction]
#[pyo3(signature = (data, /))]
pub fn zstd_decompress(py: Python<'_>, data: ReadableBuffer) -> PyResult<RyBytes> {
    let bin = data.as_slice();
    py.detach(|| zstd_decompress_impl(bin)).map(RyBytes::from)
}

#[expect(clippy::needless_pass_by_value)]
#[pyfunction]
#[pyo3(signature = (data, /))]
#[must_use]
pub fn is_zstd(data: ReadableBuffer) -> bool {
    data.as_slice().starts_with(b"\x28\xB5\x2F\xFD")
}

// ============================================================================
// DEPRECATED ~ remove in v0.0.109
// ============================================================================

#[pyfunction]
#[pyo3(
    signature = (data, level = PyCompressionLevel::default()),
    text_signature = "(data, level=3)",
    warn(
        message = "`zstd_encode` is deprecated; use `zstd_compress` instead [removal: v0.0.109]",
        category = pyo3::exceptions::PyDeprecationWarning
    )
)]
pub fn zstd_encode(
    py: Python<'_>,
    data: ReadableBuffer,
    level: PyCompressionLevel,
) -> PyResult<RyBytes> {
    zstd_compress(py, data, level)
}

#[pyfunction]
#[pyo3(warn(
    message = "`zstd_decode` is deprecated; use `zstd_decompress` instead [removal: v0.0.109]",
    category = pyo3::exceptions::PyDeprecationWarning
))]
pub fn zstd_decode(py: Python<'_>, data: ReadableBuffer) -> PyResult<RyBytes> {
    zstd_decompress(py, data)
}
