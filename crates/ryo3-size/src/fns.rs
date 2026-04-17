use pyo3::prelude::*;
use ryo3_core::PyAsciiString;

use crate::types::{PyBase, PyStyle};

#[pyfunction]
pub fn parse_size(s: &str) -> PyResult<i64> {
    size::Size::from_str(s)
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(format!("{e}")))
        .map(|s| s.bytes())
}

/// Format a size of bytes into a human-readable string.
#[must_use]
#[pyfunction]
#[pyo3(
    signature = (n, *, base = PyBase::default(), style = PyStyle::default()),
    text_signature = "(n, *, base=2, style='default')"
)]
pub fn fmt_size(n: i64, base: PyBase, style: PyStyle) -> PyAsciiString {
    size::fmt::SizeFormatter::new()
        .with_base(base.0)
        .with_style(style.0)
        .format(n)
        .into()
}
