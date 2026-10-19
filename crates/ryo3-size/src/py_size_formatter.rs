use pyo3::IntoPyObjectExt;
use pyo3::prelude::*;
use pyo3::types::PyTuple;
use ryo3_core::PyAsciiString;

use crate::types::{PyBase, PyStyle};

#[pyclass(name = "SizeFormatter", frozen, immutable_type)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
pub struct PySizeFormatter {
    formatter: size::fmt::SizeFormatter,
    base: PyBase,
    style: PyStyle,
    scale: Option<u8>,
}

impl PySizeFormatter {
    fn new(base: PyBase, style: PyStyle, scale: Option<u8>) -> Self {
        let formatter = size::fmt::SizeFormatter::new()
            .with_base(base.into())
            .with_style(style.into())
            .with_scale(scale.map(usize::from));
        Self {
            formatter,
            base,
            style,
            scale,
        }
    }
}

#[pymethods]
impl PySizeFormatter {
    #[new]
    #[pyo3(
        signature = (base = PyBase::default(), style = PyStyle::default(), scale = None),
        text_signature = "(base=2, style='default', scale=None)"
    )]
    fn py_new(base: PyBase, style: PyStyle, scale: Option<u8>) -> Self {
        Self::new(base, style, scale)
    }

    fn __eq__(&self, other: &Self) -> bool {
        self.base == other.base && self.style == other.style && self.scale == other.scale
    }

    fn __getnewargs__<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyTuple>> {
        PyTuple::new(
            py,
            &[
                self.base.into_bound_py_any(py)?,
                self.style.into_bound_py_any(py)?,
                self.scale.into_bound_py_any(py)?,
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

    #[getter]
    fn scale(&self) -> Option<u8> {
        self.scale
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
        Self::from((base, self.style, self.scale))
    }

    fn with_style(&self, style: PyStyle) -> Self {
        Self::from((self.base, style, self.scale))
    }

    fn with_scale(&self, scale: Option<u8>) -> Self {
        Self::from((self.base, self.style, scale))
    }
}

impl From<(PyBase, PyStyle, Option<u8>)> for PySizeFormatter {
    fn from((base, style, scale): (PyBase, PyStyle, Option<u8>)) -> Self {
        Self::py_new(base, style, scale)
    }
}

impl std::fmt::Display for PySizeFormatter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "SizeFormatter(base={}, style='{}', scale=",
            self.base, self.style
        )?;
        match self.scale {
            Some(scale) => write!(f, "{scale})"),
            None => f.write_str("None)"),
        }
    }
}
