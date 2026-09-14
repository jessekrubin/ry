use std::io;

use pyo3::IntoPyObjectExt;
use pyo3::prelude::*;
use pyo3::types::PyString;
use ryo3_bytes::{ReadableBuffer, RyBytes};
use ryo3_core::PyCastExactOpt;
use ryo3_core::macros::{py_type_err, py_value_error};
use serde_json::{Deserializer, Serializer};

fn minify_json<W: io::Write>(input: &[u8], output: W) -> Result<(), serde_json::Error> {
    let mut de = Deserializer::from_slice(input);
    let mut ser = Serializer::new(output);
    serde_transcode::transcode(&mut de, &mut ser)
}

fn py_minify_json<W: io::Write>(input: &[u8], output: W) -> PyResult<()> {
    minify_json(input, output).map_err(|e| py_value_error!("Failed to minify JSON: {e}"))
}

#[pyfunction(signature = (buf, /, *, append_newline = false))]
pub(crate) fn minify<'py>(
    buf: &'py Bound<'py, PyAny>,
    append_newline: bool,
) -> PyResult<JsonFragmentOutput> {
    if let Some(s) = buf.cast_exact_opt::<PyString>() {
        // py-string
        let json_str = s.to_string();
        let json_bytes = json_str.as_bytes();
        let mut output = Vec::with_capacity(json_bytes.len() / 8);
        py_minify_json(json_bytes, &mut output)?;
        if append_newline {
            output.push(b'\n');
        }
        #[expect(unsafe_code, reason = "wenodis: serde-json does it's job")]
        let s = unsafe { String::from_utf8_unchecked(output) };
        Ok(JsonFragmentOutput::String(s))
    } else if let Ok(pybytes) = buf.extract::<ReadableBuffer>() {
        let json_bytes = pybytes.as_ref();
        let mut output = Vec::with_capacity(json_bytes.len() / 8);
        py_minify_json(json_bytes, &mut output)?;
        if append_newline {
            output.push(b'\n');
        }
        match pybytes {
            ReadableBuffer::PyBytes(_) => Ok(JsonFragmentOutput::PyBytes(output)),
            _ => Ok(JsonFragmentOutput::RyBytes(output)),
        }
    } else {
        py_type_err!("Expected bytes-like object, str or buffer-protocol object")
    }
}

fn indent2_json<W: io::Write>(input: &[u8], output: W) -> Result<(), serde_json::Error> {
    let mut de = Deserializer::from_slice(input);
    let mut ser = Serializer::pretty(output);
    serde_transcode::transcode(&mut de, &mut ser)
}

fn py_indent2_json<W: io::Write>(input: &[u8], output: W) -> PyResult<()> {
    indent2_json(input, output).map_err(|e| py_value_error!("Failed to format JSON: {e}"))
}

#[pyfunction(signature = (buf, /, *, append_newline = false))]
pub(crate) fn fmt<'py>(
    buf: &'py Bound<'py, PyAny>,
    append_newline: bool,
) -> PyResult<JsonFragmentOutput> {
    if let Some(s) = buf.cast_exact_opt::<PyString>() {
        // py-string
        let json_str = s.to_string();
        let json_bytes = json_str.as_bytes();
        let mut output = Vec::with_capacity(json_bytes.len());
        py_indent2_json(json_bytes, &mut output)?;
        if append_newline {
            output.push(b'\n');
        }
        #[expect(unsafe_code, reason = "wenodis: serde-json does it's job")]
        let s = unsafe { String::from_utf8_unchecked(output) };
        Ok(JsonFragmentOutput::String(s))
    } else if let Ok(rb) = buf.extract::<ReadableBuffer>() {
        let json_bytes = rb.as_ref();
        let mut output = Vec::with_capacity(json_bytes.len());
        py_indent2_json(json_bytes, &mut output)?;
        if append_newline {
            output.push(b'\n');
        }
        match rb {
            ReadableBuffer::PyBytes(_) => Ok(JsonFragmentOutput::PyBytes(output)),
            _ => Ok(JsonFragmentOutput::RyBytes(output)),
        }
    } else {
        py_type_err!("Expected bytes-like object, str or buffer-protocol object")
    }
}

pub(crate) enum JsonFragmentOutput {
    String(String),
    PyBytes(Vec<u8>),
    RyBytes(Vec<u8>),
}

impl<'py> IntoPyObject<'py> for JsonFragmentOutput {
    type Target = PyAny;

    type Output = Bound<'py, Self::Target>;

    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        match self {
            JsonFragmentOutput::String(s) => s.into_bound_py_any(py),
            JsonFragmentOutput::PyBytes(b) => {
                pyo3::types::PyBytes::new(py, &b).into_bound_py_any(py)
            }
            JsonFragmentOutput::RyBytes(b) => RyBytes::from(b).into_bound_py_any(py),
        }
    }
}
