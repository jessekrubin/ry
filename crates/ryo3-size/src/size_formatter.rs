use pyo3::IntoPyObjectExt;
use pyo3::prelude::*;
use pyo3::types::PyTuple;
use ryo3_core::PyAsciiString;

use crate::types::{PyBase, PyStyle};

#[pyclass(name = "SizeFormatter", frozen, immutable_type, skip_from_py_object)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
pub struct PySizeFormatter {
    formatter: size::fmt::SizeFormatter,
    base: PyBase,
    style: PyStyle,
}

impl PySizeFormatter {
    fn new(base: PyBase, style: PyStyle) -> Self {
        let formatter = size::fmt::SizeFormatter::new()
            .with_base(base.into())
            .with_style(style.0);
        Self {
            formatter,
            base,
            style,
        }
    }
}

#[pymethods]
impl PySizeFormatter {
    #[new]
    #[pyo3(
        signature = (base = PyBase::default(), style = PyStyle::default()),
        text_signature = "(base=2, style='default')"
    )]
    fn py_new(base: PyBase, style: PyStyle) -> Self {
        Self::new(base, style)
    }

    fn __eq__(&self, other: &Self) -> bool {
        self.base == other.base && self.style == other.style
    }

    fn __getnewargs__<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyTuple>> {
        PyTuple::new(
            py,
            &[
                self.base.into_bound_py_any(py)?,
                self.style.into_bound_py_any(py)?,
            ],
        )
    }

    #[getter]
    fn style(&self) -> &PyStyle {
        &self.style
    }

    #[getter]
    fn base(&self) -> &PyBase {
        &self.base
    }

    fn format(&self, n: i64) -> PyAsciiString {
        self.formatter.format(n).into()
    }

    fn __repr__(&self) -> PyAsciiString {
        format!("{self}").into()
    }

    fn __call__(&self, n: i64) -> PyAsciiString {
        self.formatter.format(n).into()
    }

    fn with_base(&self, base: PyBase) -> Self {
        Self::from((base, self.style))
    }

    fn with_style(&self, style: PyStyle) -> Self {
        Self::from((self.base, style))
    }
}

impl From<(PyBase, PyStyle)> for PySizeFormatter {
    fn from((base, style): (PyBase, PyStyle)) -> Self {
        Self::py_new(base, style)
    }
}

impl std::fmt::Display for PySizeFormatter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "SizeFormatter(base={}, style='{}')",
            self.base, self.style
        )
    }
}
