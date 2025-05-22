use crate::types::{Base, Style};
use pyo3::exceptions::{PyOverflowError, PyTypeError, PyValueError};
use pyo3::prelude::*;
use pyo3::pyclass::CompareOp;
use pyo3::types::{PyTuple, PyType};
use std::ops::{Neg, Not};

#[derive(Debug, Clone)]
#[pyclass(name = "Size", module = "ry.ryo3", frozen)]
pub struct PySize(size::Size);

impl From<size::Size> for PySize {
    fn from(size: size::Size) -> Self {
        PySize(size)
    }
}

impl From<i64> for PySize {
    fn from(size: i64) -> Self {
        PySize(size::Size::from_bytes(size))
    }
}

#[pymethods]
impl PySize {
    #[new]
    fn py_new(size: i64) -> Self {
        PySize(size::Size::from_bytes(size))
    }

    fn __getnewargs__<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyTuple>> {
        PyTuple::new(py, vec![self.0.bytes()])
    }

    fn __int__(&self) -> i64 {
        self.0.bytes()
    }

    fn __str__(&self) -> String {
        format!("{}", self.0)
    }

    fn __repr__(&self) -> String {
        format!("Size({})", self.0.bytes())
    }

    fn __hash__(&self) -> u64 {
        // can just use the actual number converted to u64
        u64::from_ne_bytes(self.0.bytes().to_ne_bytes())
    }

    fn __bool__(&self) -> bool {
        self.0.bytes() != 0
    }

    #[getter]
    fn bytes(&self) -> i64 {
        self.0.bytes()
    }

    #[expect(clippy::needless_pass_by_value)]
    fn __richcmp__(&self, other: SizeWrapper, op: CompareOp) -> bool {
        match op {
            CompareOp::Eq => self.0 == other.0,
            CompareOp::Ne => self.0 != other.0,
            CompareOp::Lt => self.0 < other.0,
            CompareOp::Le => self.0 <= other.0,
            CompareOp::Gt => self.0 > other.0,
            CompareOp::Ge => self.0 >= other.0,
        }
    }

    #[pyo3(signature = (*, base = None, style = None))]
    fn format(&self, base: Option<Base>, style: Option<Style>) -> String {
        self.0
            .format()
            .with_base(base.unwrap_or_default().0)
            .with_style(style.unwrap_or_default().0)
            .to_string()
    }

    #[classmethod]
    fn parse(_cls: &Bound<'_, PyType>, size: &str) -> PyResult<Self> {
        match size::Size::from_str(size) {
            Ok(s) => Ok(PySize(s)),
            Err(e) => Err(PyValueError::new_err(e.to_string())),
        }
    }

    #[classmethod]
    fn from_str(_cls: &Bound<'_, PyType>, size: &str) -> PyResult<Self> {
        match size::Size::from_str(size) {
            Ok(s) => Ok(PySize(s)),
            Err(e) => Err(PyValueError::new_err(e.to_string())),
        }
    }

    fn __abs__(&self) -> Self {
        PySize(size::Size::from_const(self.0.bytes().abs()))
    }

    fn __neg__(&self) -> Self {
        PySize(size::Size::from_const(self.0.bytes().neg()))
    }

    fn __pos__(&self) -> Self {
        self.__abs__()
    }

    fn __invert__(&self) -> Self {
        PySize(size::Size::from_const(self.0.bytes().not()))
    }

    #[expect(clippy::needless_pass_by_value)]
    fn __add__(&self, other: SizeWrapper) -> PyResult<Self> {
        self.0
            .bytes()
            .checked_add(other.0.bytes())
            .map(Self::from)
            .ok_or_else(|| PyValueError::new_err("Overflow"))
    }

    #[expect(clippy::needless_pass_by_value)]
    fn __sub__(&self, other: SizeWrapper) -> PyResult<Self> {
        self.0
            .bytes()
            .checked_sub(other.0.bytes())
            .map(Self::from)
            .ok_or_else(|| PyValueError::new_err("Overflow"))
    }

    #[expect(clippy::cast_precision_loss, clippy::cast_possible_truncation)]
    fn __mul__(&self, other: PySizeArithmetic) -> PyResult<Self> {
        let base = self.0.bytes();

        if self.0.bytes() == 0 {
            return Ok(Self::from(0));
        }
        let result_bytes: Option<i64> = match other {
            PySizeArithmetic::Size(s) => base.checked_mul(s.0.bytes()),

            PySizeArithmetic::Int64(i) => base.checked_mul(i),

            PySizeArithmetic::U64(u) => {
                let lhs = i128::from(base);
                let rhs = i128::from(u);

                let product = lhs
                    .checked_mul(rhs)
                    .ok_or_else(|| PyOverflowError::new_err("Overflow in Size * u64"))?;
                if (i128::from(i64::MIN)..=i128::from(i64::MAX)).contains(&product) {
                    product.try_into().ok()
                } else {
                    None
                }
            }

            PySizeArithmetic::Float64(f) => {
                if !(f.is_finite()) {
                    return Err(PyOverflowError::new_err(
                        "Cannot multiply Size by NaN or infinite float",
                    ));
                }
                let result_f64 = (base as f64) * f;
                if !result_f64.is_finite() {
                    return Err(PyOverflowError::new_err("Overflow in Size * float"));
                }
                let rounded = result_f64.round();
                if rounded < i64::MIN as f64 || rounded > i64::MAX as f64 {
                    None
                } else {
                    let result = rounded as i64;
                    Some(result)
                }
            }
        };
        result_bytes
            .ok_or_else(|| PyOverflowError::new_err("Overflow in Size * Size"))
            .map(Self::from)
    }

    fn __rmul__(&self, other: PySizeArithmetic) -> PyResult<Self> {
        self.__mul__(other)
    }

    #[expect(clippy::needless_pass_by_value)]
    #[classmethod]
    fn from_bytes(_cls: &Bound<'_, PyType>, size: PySizeIntermediate) -> Self {
        PySize(size::Size::from_bytes(size.float64()))
    }

    // ========================================================================
    // CLASS METHODS ~ generated by moi in python repl and lost into the ether
    // ========================================================================
    // size::Size::from_eb
    // size::Size::from_eib
    // size::Size::from_exabytes
    // size::Size::from_exbibytes
    // size::Size::from_gb
    // size::Size::from_gib
    // size::Size::from_gibibytes
    // size::Size::from_gigabytes
    // size::Size::from_kb
    // size::Size::from_kib
    // size::Size::from_kibibytes
    // size::Size::from_kilobytes
    // size::Size::from_mb
    // size::Size::from_mebibytes
    // size::Size::from_megabytes
    // size::Size::from_mib
    // size::Size::from_pb
    // size::Size::from_pebibytes
    // size::Size::from_petabytes
    // size::Size::from_pib
    // size::Size::from_str
    // size::Size::from_tb
    // size::Size::from_tebibytes
    // size::Size::from_terabytes
    // size::Size::from_tib
    // ========================================================================

    #[expect(clippy::needless_pass_by_value)]
    #[classmethod]
    fn from_eb(_cls: &Bound<'_, PyType>, size: PySizeIntermediate) -> Self {
        PySize(size::Size::from_eb(size.float64()))
    }

    #[expect(clippy::needless_pass_by_value)]
    #[classmethod]
    fn from_eib(_cls: &Bound<'_, PyType>, size: PySizeIntermediate) -> Self {
        PySize(size::Size::from_eib(size.float64()))
    }

    #[expect(clippy::needless_pass_by_value)]
    #[classmethod]
    fn from_exabytes(_cls: &Bound<'_, PyType>, size: PySizeIntermediate) -> Self {
        PySize(size::Size::from_exabytes(size.float64()))
    }

    #[expect(clippy::needless_pass_by_value)]
    #[classmethod]
    fn from_exbibytes(_cls: &Bound<'_, PyType>, size: PySizeIntermediate) -> Self {
        PySize(size::Size::from_exbibytes(size.float64()))
    }

    #[expect(clippy::needless_pass_by_value)]
    #[classmethod]
    fn from_gb(_cls: &Bound<'_, PyType>, size: PySizeIntermediate) -> Self {
        PySize(size::Size::from_gb(size.float64()))
    }

    #[expect(clippy::needless_pass_by_value)]
    #[classmethod]
    fn from_gib(_cls: &Bound<'_, PyType>, size: PySizeIntermediate) -> Self {
        PySize(size::Size::from_gib(size.float64()))
    }

    #[expect(clippy::needless_pass_by_value)]
    #[classmethod]
    fn from_gibibytes(_cls: &Bound<'_, PyType>, size: PySizeIntermediate) -> Self {
        PySize(size::Size::from_gibibytes(size.float64()))
    }

    #[expect(clippy::needless_pass_by_value)]
    #[classmethod]
    fn from_gigabytes(_cls: &Bound<'_, PyType>, size: PySizeIntermediate) -> Self {
        PySize(size::Size::from_gigabytes(size.float64()))
    }

    #[expect(clippy::needless_pass_by_value)]
    #[classmethod]
    fn from_kb(_cls: &Bound<'_, PyType>, size: PySizeIntermediate) -> Self {
        PySize(size::Size::from_kb(size.float64()))
    }

    #[expect(clippy::needless_pass_by_value)]
    #[classmethod]
    fn from_kib(_cls: &Bound<'_, PyType>, size: PySizeIntermediate) -> Self {
        PySize(size::Size::from_kib(size.float64()))
    }

    #[expect(clippy::needless_pass_by_value)]
    #[classmethod]
    fn from_kibibytes(_cls: &Bound<'_, PyType>, size: PySizeIntermediate) -> Self {
        PySize(size::Size::from_kibibytes(size.float64()))
    }

    #[expect(clippy::needless_pass_by_value)]
    #[classmethod]
    fn from_kilobytes(_cls: &Bound<'_, PyType>, size: PySizeIntermediate) -> Self {
        PySize(size::Size::from_kilobytes(size.float64()))
    }

    #[expect(clippy::needless_pass_by_value)]
    #[classmethod]
    fn from_mb(_cls: &Bound<'_, PyType>, size: PySizeIntermediate) -> Self {
        PySize(size::Size::from_mb(size.float64()))
    }

    #[expect(clippy::needless_pass_by_value)]
    #[classmethod]
    fn from_mebibytes(_cls: &Bound<'_, PyType>, size: PySizeIntermediate) -> Self {
        PySize(size::Size::from_mebibytes(size.float64()))
    }

    #[expect(clippy::needless_pass_by_value)]
    #[classmethod]
    fn from_megabytes(_cls: &Bound<'_, PyType>, size: PySizeIntermediate) -> Self {
        PySize(size::Size::from_megabytes(size.float64()))
    }

    #[expect(clippy::needless_pass_by_value)]
    #[classmethod]
    fn from_mib(_cls: &Bound<'_, PyType>, size: PySizeIntermediate) -> Self {
        PySize(size::Size::from_mib(size.float64()))
    }

    #[expect(clippy::needless_pass_by_value)]
    #[classmethod]
    fn from_pb(_cls: &Bound<'_, PyType>, size: PySizeIntermediate) -> Self {
        PySize(size::Size::from_pb(size.float64()))
    }

    #[expect(clippy::needless_pass_by_value)]
    #[classmethod]
    fn from_pebibytes(_cls: &Bound<'_, PyType>, size: PySizeIntermediate) -> Self {
        PySize(size::Size::from_pebibytes(size.float64()))
    }

    #[expect(clippy::needless_pass_by_value)]
    #[classmethod]
    fn from_petabytes(_cls: &Bound<'_, PyType>, size: PySizeIntermediate) -> Self {
        PySize(size::Size::from_petabytes(size.float64()))
    }

    #[expect(clippy::needless_pass_by_value)]
    #[classmethod]
    fn from_pib(_cls: &Bound<'_, PyType>, size: PySizeIntermediate) -> Self {
        PySize(size::Size::from_pib(size.float64()))
    }

    #[expect(clippy::needless_pass_by_value)]
    #[classmethod]
    fn from_tb(_cls: &Bound<'_, PyType>, size: PySizeIntermediate) -> Self {
        PySize(size::Size::from_tb(size.float64()))
    }

    #[expect(clippy::needless_pass_by_value)]
    #[classmethod]
    fn from_tebibytes(_cls: &Bound<'_, PyType>, size: PySizeIntermediate) -> Self {
        PySize(size::Size::from_tebibytes(size.float64()))
    }

    #[expect(clippy::needless_pass_by_value)]
    #[classmethod]
    fn from_terabytes(_cls: &Bound<'_, PyType>, size: PySizeIntermediate) -> Self {
        PySize(size::Size::from_terabytes(size.float64()))
    }

    #[expect(clippy::needless_pass_by_value)]
    #[classmethod]
    fn from_tib(_cls: &Bound<'_, PyType>, size: PySizeIntermediate) -> Self {
        PySize(size::Size::from_tib(size.float64()))
    }
}

#[derive(Debug, Clone)]
struct SizeWrapper(size::Size);

impl FromPyObject<'_> for SizeWrapper {
    fn extract_bound(ob: &Bound<'_, PyAny>) -> PyResult<Self> {
        if let Ok(s) = ob.downcast::<PySize>() {
            let pysize = s.extract::<PySize>()?;
            Ok(SizeWrapper(pysize.0))
        } else if let Ok(i) = ob.extract::<i64>() {
            Ok(SizeWrapper(size::Size::from_const(i)))
        } else if let Ok(f) = ob.extract::<f64>() {
            Ok(SizeWrapper(size::Size::from_bytes(f)))
        } else {
            Err(PyTypeError::new_err("Must be Size or i64"))
        }
    }
}

#[derive(Debug, Clone, FromPyObject)]
enum PySizeIntermediate {
    Float64(f64),
    Int64(i64),
    U64(u64),
}

impl PySizeIntermediate {
    #[expect(clippy::cast_precision_loss)]
    fn float64(&self) -> f64 {
        match self {
            PySizeIntermediate::Float64(f) => *f,
            PySizeIntermediate::Int64(i) => *i as f64,
            PySizeIntermediate::U64(u) => *u as f64,
        }
    }
}

#[derive(Debug, Clone, FromPyObject)]
enum PySizeArithmetic {
    Size(PySize),
    Float64(f64),
    Int64(i64),
    U64(u64),
}
