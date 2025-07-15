use crate::compression_level::PyCompressionLevel;
use pyo3::IntoPyObjectExt;
use pyo3::prelude::*;

fn rs_zstd_compress_oneshot(data: &[u8], level: Option<PyCompressionLevel>) -> PyResult<Vec<u8>> {
    ::zstd::stream::encode_all(data, level.unwrap_or_default().into()).map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("zstd-encode-error: {e:?}"))
    })
}

fn rs_zstd_decode_one_shot(data: &[u8]) -> PyResult<Vec<u8>> {
    ::zstd::stream::decode_all(data).map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("zstd-decode-error: {e:?}"))
    })
}

pub(crate) fn py_decode<'py>(
    py: Python<'py>,
    data: &Bound<'py, PyAny>,
) -> PyResult<Bound<'py, PyAny>> {
    let slice = ryo3_bytes::extract_bytes_ref(data)?;
    let decoded = rs_zstd_decode_one_shot(slice)?;
    ryo3_bytes::PyBytes::from(decoded).into_bound_py_any(py)
    // if pybytes {
    //     let a =pyo3::types::PyBytes::new(py, &decoded).into_py(py);
    //     Ok(a.into_bound_py_any(py)?)
    // } else{
    //     let a = ryo3_bytes::PyBytes::from(decoded) .into_py(py);
    //     Ok(a.into_bound_py_any(py)?)
    // }
}

pub(crate) fn py_encode<'py>(
    py: Python<'py>,
    data: &Bound<'py, PyAny>,
    level: Option<PyCompressionLevel>,
) -> PyResult<Bound<'py, PyAny>> {
    let slice = ryo3_bytes::extract_bytes_ref(data)?;
    let encoded = rs_zstd_compress_oneshot(slice, level)?;
    ryo3_bytes::PyBytes::from(encoded).into_bound_py_any(py)
}

macro_rules! zstd_decode_pyfunction {
    ($func_name:ident) => {
        #[pyfunction(signature = (data))]
        pub fn $func_name<'py>(
            py: Python<'py>,
            data: &Bound<'py, PyAny>,
        ) -> PyResult<Bound<'py, PyAny>> {
            py_decode(py, data)
        }
    };
}

macro_rules! zstd_encode_pyfunction {
    ($func_name:ident) => {
        #[pyfunction(signature = (data, level = PyCompressionLevel::default()))]
        pub fn $func_name<'py>(
            py: Python<'py>,
            data: &Bound<'py, PyAny>,
            level: Option<PyCompressionLevel>,
        ) -> PyResult<Bound<'py, PyAny>> {
            py_encode(py, data, level)
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

#[pyfunction]
pub fn is_zstd(data: &Bound<'_, PyAny>) -> PyResult<bool> {
    let slice = ryo3_bytes::extract_bytes_ref(data)?;
    Ok(slice.starts_with(b"\x28\xB5\x2F\xFD"))
}
