use pyo3::prelude::*;
use pyo3::types::PyString;
use ryo3_bytes::PyBytes as RyBytes;
use ryo3_core::{py_type_err, py_value_error};
use serde_json::Deserializer;
use serde_json::Serializer;
use std::io;

fn minify_json<R: io::Read, W: io::Write>(input: R, output: W) -> Result<(), serde_json::Error> {
    let mut de = Deserializer::from_reader(input);
    let mut ser = Serializer::new(output);
    serde_transcode::transcode(&mut de, &mut ser)
}

fn py_minify_json<R: io::Read, W: io::Write>(input: R, output: W) -> PyResult<()> {
    minify_json(input, output).map_err(|e| py_value_error!("Failed to minify JSON: {e}"))
}

#[pyfunction(signature = (buf, /))]
pub(crate) fn minify<'py>(buf: &'py Bound<'py, PyAny>) -> PyResult<RyBytes> {
    if let Ok(s) = buf.cast::<PyString>() {
        // py-string
        let json_str = s.to_string();
        let json_bytes = json_str.as_bytes();
        let mut output = Vec::with_capacity(json_bytes.len());
        py_minify_json(json_bytes, &mut output)?;
        Ok(RyBytes::from(output))
    } else if let Ok(pybytes) = buf.cast::<pyo3::types::PyBytes>() {
        // py bytes
        let json_bytes = pybytes.as_bytes();
        let mut output = Vec::with_capacity(json_bytes.len());
        py_minify_json(json_bytes, &mut output)?;
        Ok(RyBytes::from(output))
    } else if let Ok(custom) = buf.cast::<RyBytes>() {
        // rs-bytes instance
        let borrowed = custom.borrow();
        let json_bytes = borrowed.as_slice();
        let mut output = Vec::with_capacity(json_bytes.len());
        py_minify_json(json_bytes, &mut output)?;
        Ok(RyBytes::from(output))
    } else if let Ok(bytes_extracted) = buf.extract::<RyBytes>() {
        // buffer protocol extract via rs-bytes
        let json_bytes = bytes_extracted.as_slice();
        let mut output = Vec::with_capacity(json_bytes.len());
        py_minify_json(json_bytes, &mut output)?;
        Ok(RyBytes::from(output))
    } else {
        py_type_err!("Expected bytes-like object, str or buffer-protocol object")
    }
}

fn indent2_json<R: io::Read, W: io::Write>(input: R, output: W) -> Result<(), serde_json::Error> {
    let mut de = Deserializer::from_reader(input);
    let mut ser = Serializer::pretty(output);
    serde_transcode::transcode(&mut de, &mut ser)
}

fn py_indent2_json<R: io::Read, W: io::Write>(input: R, output: W) -> PyResult<()> {
    indent2_json(input, output).map_err(|e| py_value_error!("Failed to format JSON: {e}"))
}

#[pyfunction(signature = (buf, /))]
pub(crate) fn fmt<'py>(buf: &'py Bound<'py, PyAny>) -> PyResult<RyBytes> {
    if let Ok(s) = buf.cast::<PyString>() {
        // py-string
        let json_str = s.to_string();
        let json_bytes = json_str.as_bytes();
        let mut output = Vec::with_capacity(json_bytes.len());
        py_indent2_json(json_bytes, &mut output)?;
        Ok(RyBytes::from(output))
    } else if let Ok(pybytes) = buf.cast::<pyo3::types::PyBytes>() {
        // py bytes
        let json_bytes = pybytes.as_bytes();
        let mut output = Vec::with_capacity(json_bytes.len());
        py_indent2_json(json_bytes, &mut output)?;
        Ok(RyBytes::from(output))
    } else if let Ok(custom) = buf.cast::<RyBytes>() {
        // rs-bytes instance
        let borrowed = custom.borrow();
        let json_bytes = borrowed.as_slice();
        let mut output = Vec::with_capacity(json_bytes.len());
        py_indent2_json(json_bytes, &mut output)?;
        Ok(RyBytes::from(output))
    } else if let Ok(bytes_extracted) = buf.extract::<RyBytes>() {
        // buffer protocol extract via rs-bytes
        let json_bytes = bytes_extracted.as_slice();
        let mut output = Vec::with_capacity(json_bytes.len());
        py_indent2_json(json_bytes, &mut output)?;
        Ok(RyBytes::from(output))
    } else {
        py_type_err!("Expected bytes-like object, str or buffer-protocol object")
    }
}
