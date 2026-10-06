use jiter::serde::JiterDeserializer;
use pyo3::prelude::*;
use pyo3::types::PyString;
use ryo3_bytes::{ReadableBuffer, RyBytes};
use ryo3_core::PyCastExactOpt;
use ryo3_core::macros::{py_type_err, py_value_error};

use crate::ser::Serializer;

fn minify_json(input: &[u8]) -> Result<Vec<u8>, String> {
    let mut de = JiterDeserializer::new(input);
    // allocate some cap, and  for add newline just in case its already minified...
    let capacity = (input.len() / 8).saturating_add(1);
    let mut ser = Serializer::compact_with_capacity(capacity);
    serde_transcode::transcode(&mut de, &mut ser).map_err(|e| e.to_string())?;
    de.finish().map_err(|e| e.description(input))?;
    Ok(ser.into_inner())
}

fn py_minify_json(input: &[u8]) -> PyResult<Vec<u8>> {
    minify_json(input).map_err(|e| py_value_error!("Failed to minify JSON: {e}"))
}

#[pyfunction(signature = (buf, /, append_newline = false))]
pub(crate) fn minify<'py>(buf: &'py Bound<'py, PyAny>, append_newline: bool) -> PyResult<RyBytes> {
    if let Some(s) = buf.cast_exact_opt::<PyString>() {
        // py-string
        let json_str = s.to_string();
        let mut output = py_minify_json(json_str.as_bytes())?;
        if append_newline {
            output.push(b'\n');
        }
        Ok(RyBytes::from(output))
    } else if let Ok(pybytes) = buf.extract::<ReadableBuffer>() {
        let mut output = py_minify_json(pybytes.as_ref())?;
        if append_newline {
            output.push(b'\n');
        }
        Ok(RyBytes::from(output))
    } else {
        py_type_err!("Expected bytes-like object, str or buffer-protocol object")
    }
}

fn indent2_json(input: &[u8]) -> Result<Vec<u8>, String> {
    let mut de = JiterDeserializer::new(input);
    let mut ser = Serializer::pretty_with_capacity(input.len());
    serde_transcode::transcode(&mut de, &mut ser).map_err(|e| e.to_string())?;
    de.finish().map_err(|e| e.description(input))?;
    Ok(ser.into_inner())
}

fn py_indent2_json(input: &[u8]) -> PyResult<Vec<u8>> {
    indent2_json(input).map_err(|e| py_value_error!("Failed to format JSON: {e}"))
}

#[pyfunction(signature = (buf, /, append_newline = false))]
pub(crate) fn fmt<'py>(buf: &'py Bound<'py, PyAny>, append_newline: bool) -> PyResult<RyBytes> {
    if let Some(s) = buf.cast_exact_opt::<PyString>() {
        // py-string
        let json_str = s.to_string();
        let mut output = py_indent2_json(json_str.as_bytes())?;
        if append_newline {
            output.push(b'\n');
        }
        Ok(RyBytes::from(output))
    } else if let Ok(pybytes) = buf.extract::<ReadableBuffer>() {
        let mut output = py_indent2_json(pybytes.as_ref())?;
        if append_newline {
            output.push(b'\n');
        }
        Ok(RyBytes::from(output))
    } else {
        py_type_err!("Expected bytes-like object, str or buffer-protocol object")
    }
}
