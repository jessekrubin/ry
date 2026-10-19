use pyo3::prelude::*;
use ryo3_core::PyAsciiString;

use crate::types::{PyBase, PyStyle};

#[pyfunction]
pub fn parse_size(s: &str) -> PyResult<i64> {
    size::Size::from_str(s)
        .map_err(|_| pyo3::exceptions::PyValueError::new_err("parse size error"))
        .map(|s| s.bytes())
}

/// Format a size of bytes into a human-readable string.
#[must_use]
#[pyfunction]
#[pyo3(
    signature = (n, *, base = PyBase::default(), style = PyStyle::default(), scale = None),
    text_signature = "(n, *, base=2, style='default', scale=None)"
)]
pub fn fmt_size(n: i64, base: PyBase, style: PyStyle, scale: Option<u8>) -> PyAsciiString {
    size::fmt::SizeFormatter::new()
        .with_base(base.into())
        .with_style(style.into())
        .with_scale(scale.map(usize::from))
        .format(n)
        .into()
}
