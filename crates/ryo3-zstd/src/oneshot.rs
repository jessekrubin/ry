use pyo3::prelude::*;
use ryo3_bytes::{PyBytes as RyBytes, ReadableBuffer};

use crate::compression_level::PyCompressionLevel;

fn rs_zstd_compress_oneshot(data: &[u8], level: PyCompressionLevel) -> PyResult<Vec<u8>> {
    ::zstd::stream::encode_all(data, level.into()).map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("zstd-encode-error: {e:?}"))
    })
}

fn rs_zstd_decode_one_shot(data: &[u8]) -> PyResult<Vec<u8>> {
    ::zstd::stream::decode_all(data).map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("zstd-decode-error: {e:?}"))
    })
}

pub(crate) fn py_decode(py: Python<'_>, data: &ReadableBuffer) -> PyResult<RyBytes> {
    let slice = data.as_slice();
    py.detach(|| rs_zstd_decode_one_shot(slice)).map(Into::into)
}

pub(crate) fn py_encode(
    py: Python<'_>,
    data: &ReadableBuffer,
    level: PyCompressionLevel,
) -> PyResult<RyBytes> {
    let slice = data.as_slice();
    let encoded = py.detach(|| rs_zstd_compress_oneshot(slice, level))?;
    Ok(encoded.into())
}

macro_rules! zstd_decode_pyfunction {
    ($func_name:ident) => {
        #[pyfunction(signature = (data))]
        pub fn $func_name(py: Python<'_>, data: ReadableBuffer) -> PyResult<RyBytes> {
            py_decode(py, &data)
        }
    };
}

macro_rules! zstd_encode_pyfunction {
    ($func_name:ident) => {
        #[pyfunction(signature = (data, level = PyCompressionLevel::default()), text_signature = "(data, level=3)")]
        pub fn $func_name(
            py: Python<'_>, data: ReadableBuffer, level: PyCompressionLevel
        ) -> PyResult<RyBytes> {
            py_encode(py, &data, level)
        }
    };
}

// encoding/compression functions
zstd_encode_pyfunction!(compress);
zstd_encode_pyfunction!(zstd);
zstd_encode_pyfunction!(zstd_compress);
zstd_encode_pyfunction!(zstd_encode);

// decoding/decompression functions
zstd_decode_pyfunction!(decode);
zstd_decode_pyfunction!(decompress);
zstd_decode_pyfunction!(unzstd);
zstd_decode_pyfunction!(zstd_decode);
zstd_decode_pyfunction!(zstd_decompress);

#[expect(clippy::needless_pass_by_value, reason = "pyo3-extraction")]
#[pyfunction]
pub fn is_zstd(data: ReadableBuffer) -> PyResult<bool> {
    Ok(data.as_slice().starts_with(b"\x28\xB5\x2F\xFD"))
}
