use crate::types::{Base, Style};
use pyo3::prelude::*;

#[pyclass(name = "SizeFormatter", module = "ry")]
pub struct PySizeFormatter {
    formatter: size::fmt::SizeFormatter,
    base: Base,
    style: Style,
}

#[pymethods]
impl PySizeFormatter {
    #[new]
    #[pyo3(signature = (base = None, style = None))]
    fn py_new(base: Option<Base>, style: Option<Style>) -> Self {
        // fn py_new(base: Option<u8>, style: Option<&str>) -> Self {
        let base = base.unwrap_or_default();
        let style = style.unwrap_or_default();

        let formatter = size::fmt::SizeFormatter::new()
            .with_base(base.0)
            .with_style(style.0);
        PySizeFormatter {
            formatter,
            base,
            style,
        }
    }

    fn format(&self, n: i64) -> String {
        self.formatter.format(n)
    }

    fn __repr__(&self) -> String {
        format!("SizeFormatter(base: {}, style: {})", self.base, self.style)
    }

    fn __call__(&self, n: i64) -> String {
        self.formatter.format(n)
    }
}
