use jiter::serde::JiterDeserializer;
use pyo3::prelude::*;
use pyo3::types::PyString;
use ryo3_bytes::{ReadableBuffer, RyBytes};
use ryo3_core::PyCastExactOpt;
use ryo3_core::macros::{py_type_err, py_value_error};

use crate::experimental::{JsonFormat, JsonFormatCompact, JsonFormatPretty, Serializer};

fn transcode_json<F: JsonFormat>(input: &[u8], fmt: F) -> Result<Vec<u8>, String> {
    let mut de = JiterDeserializer::new(input);
    let mut ser = Serializer::with_capacity(input.len(), fmt);
    serde_transcode::transcode(&mut de, &mut ser).map_err(|e| e.to_string())?;
    de.finish().map_err(|e| e.description(input))?;
    Ok(ser.into_inner())
}

fn py_minify_json(input: &[u8]) -> PyResult<Vec<u8>> {
    transcode_json(input, JsonFormatCompact)
        .map_err(|e| py_value_error!("Failed to minify JSON: {e}"))
}

#[pyfunction(signature = (buf, /))]
pub(crate) fn minify<'py>(buf: &'py Bound<'py, PyAny>) -> PyResult<RyBytes> {
    if let Some(s) = buf.cast_exact_opt::<PyString>() {
        // py-string
        let json_str = s.to_string();
        let output = py_minify_json(json_str.as_bytes())?;
        Ok(RyBytes::from(output))
    } else if let Ok(pybytes) = buf.extract::<ReadableBuffer>() {
        let output = py_minify_json(pybytes.as_ref())?;
        Ok(RyBytes::from(output))
    } else {
        py_type_err!("Expected bytes-like object, str or buffer-protocol object")
    }
}

fn py_indent2_json(input: &[u8]) -> PyResult<Vec<u8>> {
    transcode_json(input, JsonFormatPretty::<2>::new())
        .map_err(|e| py_value_error!("Failed to format JSON: {e}"))
}

#[pyfunction(signature = (buf, /))]
pub(crate) fn fmt<'py>(buf: &'py Bound<'py, PyAny>) -> PyResult<RyBytes> {
    if let Some(s) = buf.cast_exact_opt::<PyString>() {
        // py-string
        let json_str = s.to_string();
        let output = py_indent2_json(json_str.as_bytes())?;
        Ok(RyBytes::from(output))
    } else if let Ok(pybytes) = buf.extract::<ReadableBuffer>() {
        let output = py_indent2_json(pybytes.as_ref())?;
        Ok(RyBytes::from(output))
    } else {
        py_type_err!("Expected bytes-like object, str or buffer-protocol object")
    }
}
