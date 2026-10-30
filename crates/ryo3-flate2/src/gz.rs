use std::{cell, io};

use pyo3::prelude::*;
use ryo3_bytes::{ReadableBuffer, RyBytes};

use crate::compression::PyCompression;

struct GzEncoderCache {
    level: flate2::Compression,
    encoder: flate2::Compress,
}

impl GzEncoderCache {
    fn new(level: flate2::Compression) -> Self {
        Self {
            level,
            encoder: flate2::Compress::new_gzip(level, 15),
        }
    }
}

type GzEncoderCacheRef = cell::RefCell<Option<GzEncoderCache>>;
thread_local! {
    // reuse the deflate state (~256kb); this is a trick I used somewhere else
    // in my life but I am not sure if it actually helps? i imagine it would
    // IIRC: I was doing this w/ zstd somewhere else
    static GZ_ENCODER: GzEncoderCacheRef = const { cell::RefCell::new(None) };
}

/// computes the initial capacity for the compressed output based on the input
/// data length
fn compress_capacity(len: usize) -> usize {
    if len <= 64 * 1024 {
        len + len / 1000 + 64
    } else {
        len / 4
    }
}

#[expect(clippy::cast_possible_truncation)]
fn gzip_compress_impl(data: &[u8], level: flate2::Compression) -> PyResult<Vec<u8>> {
    GZ_ENCODER.with_borrow_mut(|cached| {
        let encoder = match cached {
            Some(cache) if cache.level == level => {
                cache.encoder.reset();
                &mut cache.encoder
            }
            _ => &mut cached.insert(GzEncoderCache::new(level)).encoder,
        };
        let capacity = compress_capacity(data.len());
        let mut out = Vec::with_capacity(capacity);
        while encoder
            .compress_vec(
                &data[encoder.total_in() as usize..],
                &mut out,
                flate2::FlushCompress::Finish,
            )
            .map_err(io::Error::other)?
            != flate2::Status::StreamEnd
        {
            out.reserve(out.capacity());
        }
        // SAFETY: gzip header is ALWAYS (afaik) at least 10 bytes long
        #[expect(unsafe_code)]
        unsafe {
            *out.get_unchecked_mut(9) = 255;
        };
        out.shrink_to_fit();
        Ok(out)
    })
}

/// decodes the first gzip member (any trailing bytes are ignored)
#[expect(clippy::cast_possible_truncation)]
fn gzip_decompress_impl(data: &[u8]) -> PyResult<Vec<u8>> {
    // gzip trailer ends w/ the decompressed size % 2^32
    let size_hint_u32 = data.last_chunk::<4>().map_or(0, |n| u32::from_le_bytes(*n));
    let size_hint = usize::try_from(size_hint_u32)
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "invalid size hint"))?;
    let mut out = Vec::with_capacity(size_hint.min(data.len().saturating_mul(1032)).max(64));
    let mut gz_decoder = flate2::Decompress::new_gzip(15);
    loop {
        let status = gz_decoder
            .decompress_vec(
                &data[gz_decoder.total_in() as usize..],
                &mut out,
                flate2::FlushDecompress::Finish,
            )
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;
        if status == flate2::Status::StreamEnd {
            return Ok(out);
        }
        if out.len() < out.capacity() {
            // room left? eof?
            return Err(
                io::Error::new(io::ErrorKind::UnexpectedEof, "unexpected end of file").into(),
            );
        }
        out.reserve(out.capacity());
    }
}

#[expect(clippy::needless_pass_by_value)]
#[pyfunction]
#[pyo3(
    signature = (data, /, level = PyCompression::default()),
    text_signature = "(data, /, level=6)"
)]
pub fn gzip_compress(
    py: Python<'_>,
    data: ReadableBuffer,
    level: PyCompression,
) -> PyResult<RyBytes> {
    let bin: &[u8] = data.as_ref();
    py.detach(|| gzip_compress_impl(bin, level.into_inner()))
        .map(RyBytes::from)
}

#[expect(clippy::needless_pass_by_value)]
#[pyfunction]
#[pyo3(signature = (data, /))]
pub fn gzip_decompress(py: Python<'_>, data: ReadableBuffer) -> PyResult<RyBytes> {
    let bin: &[u8] = data.as_ref();
    py.detach(|| gzip_decompress_impl(bin)).map(RyBytes::from)
}

#[expect(clippy::needless_pass_by_value)]
#[pyfunction]
#[pyo3(signature = (data, /))]
#[must_use]
pub fn is_gzip(data: ReadableBuffer) -> bool {
    let bin: &[u8] = data.as_ref();
    bin.starts_with(b"\x1F\x8B")
}

// ============================================================================
// DEPRECATED ~ remove in v0.0.109
// ============================================================================

#[pyfunction]
#[pyo3(
    signature = (data, quality = PyCompression::default()),
    text_signature = "(data, quality=6)",
    warn(
        message = "`gzip_encode` is deprecated; use `gzip_compress` instead [removal: v0.0.109]",
        category = pyo3::exceptions::PyDeprecationWarning
    )
)]
pub fn gzip_encode(
    py: Python<'_>,
    data: ReadableBuffer,
    quality: PyCompression,
) -> PyResult<RyBytes> {
    gzip_compress(py, data, quality)
}

#[pyfunction]
#[pyo3(warn(
    message = "`gzip_decode` is deprecated; use `gzip_decompress` instead [removal: v0.0.109]",
    category = pyo3::exceptions::PyDeprecationWarning
))]
pub fn gzip_decode(py: Python<'_>, data: ReadableBuffer) -> PyResult<RyBytes> {
    gzip_decompress(py, data)
}

#[pyfunction]
#[pyo3(
    signature = (data, quality = PyCompression::default()),
    text_signature = "(data, quality=6)",
    warn(
        message = "`gzip` is deprecated; use `gzip_compress` instead [removal: v0.0.109]",
        category = pyo3::exceptions::PyDeprecationWarning
    )
)]
pub fn gzip(py: Python<'_>, data: ReadableBuffer, quality: PyCompression) -> PyResult<RyBytes> {
    gzip_compress(py, data, quality)
}

#[pyfunction]
#[pyo3(warn(
    message = "`gunzip` is deprecated; use `gzip_decompress` instead [removal: v0.0.109]",
    category = pyo3::exceptions::PyDeprecationWarning
))]
pub fn gunzip(py: Python<'_>, data: ReadableBuffer) -> PyResult<RyBytes> {
    gzip_decompress(py, data)
}

#[pyfunction]
#[pyo3(warn(
    message = "`is_gzipped` is deprecated; use `is_gzip` instead [removal: v0.0.109]",
    category = pyo3::exceptions::PyDeprecationWarning
))]
#[must_use]
pub fn is_gzipped(data: ReadableBuffer) -> bool {
    is_gzip(data)
}
