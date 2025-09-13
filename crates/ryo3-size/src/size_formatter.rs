use crate::types::{Base, Style};
use pyo3::prelude::*;
use pyo3::types::PyTuple;
use pyo3::{IntoPyObjectExt, intern};

#[pyclass(name = "SizeFormatter", frozen)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
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
        let base = base.unwrap_or_default();
        let style = style.unwrap_or_default();
        let formatter = size::fmt::SizeFormatter::new()
            .with_base(base.0)
            .with_style(style.0);
        Self {
            formatter,
            base,
            style,
        }
    }

    fn __eq__(&self, other: &Self) -> bool {
        self.base == other.base && self.style == other.style
    }

    fn __getnewargs__<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyTuple>> {
        let base = match self.base.0 {
            size::fmt::Base::Base2 => Some(2),
            size::fmt::Base::Base10 => Some(10),
            _ => None,
        };
        let style = match self.style.0 {
            size::fmt::Style::Default => intern!(py, "default").into_bound_py_any(py)?,
            size::fmt::Style::Abbreviated => intern!(py, "abbreviated").into_bound_py_any(py)?,
            size::fmt::Style::AbbreviatedLowercase => {
                intern!(py, "abbreviated_lowercase").into_bound_py_any(py)?
            }
            size::fmt::Style::Full => intern!(py, "full").into_bound_py_any(py)?,
            size::fmt::Style::FullLowercase => {
                intern!(py, "full_lowercase").into_bound_py_any(py)?
            }
            _ => py.None().into_bound_py_any(py)?,
        };
        PyTuple::new(
            py,
            &[base.into_bound_py_any(py)?, style.into_bound_py_any(py)?],
        )
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
