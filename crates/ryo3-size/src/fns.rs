use crate::types::{Base, Style};
use pyo3::prelude::*;

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
    signature = (n, *, base = Base::default(), style = Style::default()),
    text_signature = "(n, *, base=2, style='default')"
)]
pub fn fmt_size(n: i64, base: Base, style: Style) -> String {
    let formatter = size::fmt::SizeFormatter::new()
        .with_base(base.0)
        .with_style(style.0);
    formatter.format(n)
}
