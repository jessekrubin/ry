use crate::types::{Base, Style};
use pyo3::prelude::*;

#[pyfunction]
pub fn parse_size(input: &str) -> PyResult<i64> {
    size::Size::from_str(input)
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(format!("{e}")))
        .map(|s| s.bytes())
}

/// Format a size of bytes into a human-readable string.
#[must_use]
#[pyfunction]
#[pyo3(signature = (n, *, base = None, style = None))]
pub fn fmt_size(n: i64, base: Option<Base>, style: Option<Style>) -> String {
    let formatter = size::fmt::SizeFormatter::new()
        .with_base(base.unwrap_or_default().0)
        .with_style(style.unwrap_or_default().0);
    formatter.format(n)
}
