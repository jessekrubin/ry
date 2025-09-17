use crate::types::{Base, Style};
use pyo3::exceptions::{PyOverflowError, PyTypeError, PyValueError};
use pyo3::prelude::*;
use pyo3::pyclass::CompareOp;
use pyo3::types::PyTuple;
use std::ops::{Neg, Not};

#[derive(Debug, Clone, Copy)]
#[pyclass(name = "Size", frozen)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
pub struct PySize(size::Size);

impl From<size::Size> for PySize {
    fn from(size: size::Size) -> Self {
        Self(size)
    }
}

impl From<i64> for PySize {
    fn from(size: i64) -> Self {
        Self(size::Size::from_const(size))
    }
}

#[expect(clippy::trivially_copy_pass_by_ref)]
#[pymethods]
impl PySize {
    #[new]
    fn py_new(size: i64) -> Self {
        Self(size::Size::from_const(size))
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

    #[staticmethod]
    fn parse(size: &str) -> PyResult<Self> {
        match size::Size::from_str(size) {
            Ok(s) => Ok(Self(s)),
            Err(e) => Err(PyValueError::new_err(e.to_string())),
        }
    }

    #[staticmethod]
    fn from_str(size: &str) -> PyResult<Self> {
        match size::Size::from_str(size) {
            Ok(s) => Ok(Self(s)),
            Err(e) => Err(PyValueError::new_err(e.to_string())),
        }
    }

    fn __abs__(&self) -> Self {
        Self(size::Size::from_const(self.0.bytes().abs()))
    }

    fn __neg__(&self) -> Self {
        Self(size::Size::from_const(self.0.bytes().neg()))
    }

    fn __pos__(&self) -> Self {
        self.__abs__()
    }

    fn __invert__(&self) -> Self {
        Self(size::Size::from_const(self.0.bytes().not()))
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
    #[staticmethod]
    fn from_bytes(size: PySizeIntermediate) -> Self {
        Self(size::Size::from_bytes(size.float64()))
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
    #[staticmethod]
    fn from_eb(size: PySizeIntermediate) -> Self {
        Self(size::Size::from_eb(size.float64()))
    }

    #[expect(clippy::needless_pass_by_value)]
    #[staticmethod]
    fn from_eib(size: PySizeIntermediate) -> Self {
        Self(size::Size::from_eib(size.float64()))
    }

    #[expect(clippy::needless_pass_by_value)]
    #[staticmethod]
    fn from_exabytes(size: PySizeIntermediate) -> Self {
        Self(size::Size::from_exabytes(size.float64()))
    }

    #[expect(clippy::needless_pass_by_value)]
    #[staticmethod]
    fn from_exbibytes(size: PySizeIntermediate) -> Self {
        Self(size::Size::from_exbibytes(size.float64()))
    }

    #[expect(clippy::needless_pass_by_value)]
    #[staticmethod]
    fn from_gb(size: PySizeIntermediate) -> Self {
        Self(size::Size::from_gb(size.float64()))
    }

    #[expect(clippy::needless_pass_by_value)]
    #[staticmethod]
    fn from_gib(size: PySizeIntermediate) -> Self {
        Self(size::Size::from_gib(size.float64()))
    }

    #[expect(clippy::needless_pass_by_value)]
    #[staticmethod]
    fn from_gibibytes(size: PySizeIntermediate) -> Self {
        Self(size::Size::from_gibibytes(size.float64()))
    }

    #[expect(clippy::needless_pass_by_value)]
    #[staticmethod]
    fn from_gigabytes(size: PySizeIntermediate) -> Self {
        Self(size::Size::from_gigabytes(size.float64()))
    }

    #[expect(clippy::needless_pass_by_value)]
    #[staticmethod]
    fn from_kb(size: PySizeIntermediate) -> Self {
        Self(size::Size::from_kb(size.float64()))
    }

    #[expect(clippy::needless_pass_by_value)]
    #[staticmethod]
    fn from_kib(size: PySizeIntermediate) -> Self {
        Self(size::Size::from_kib(size.float64()))
    }

    #[expect(clippy::needless_pass_by_value)]
    #[staticmethod]
    fn from_kibibytes(size: PySizeIntermediate) -> Self {
        Self(size::Size::from_kibibytes(size.float64()))
    }

    #[expect(clippy::needless_pass_by_value)]
    #[staticmethod]
    fn from_kilobytes(size: PySizeIntermediate) -> Self {
        Self(size::Size::from_kilobytes(size.float64()))
    }

    #[expect(clippy::needless_pass_by_value)]
    #[staticmethod]
    fn from_mb(size: PySizeIntermediate) -> Self {
        Self(size::Size::from_mb(size.float64()))
    }

    #[expect(clippy::needless_pass_by_value)]
    #[staticmethod]
    fn from_mebibytes(size: PySizeIntermediate) -> Self {
        Self(size::Size::from_mebibytes(size.float64()))
    }

    #[expect(clippy::needless_pass_by_value)]
    #[staticmethod]
    fn from_megabytes(size: PySizeIntermediate) -> Self {
        Self(size::Size::from_megabytes(size.float64()))
    }

    #[expect(clippy::needless_pass_by_value)]
    #[staticmethod]
    fn from_mib(size: PySizeIntermediate) -> Self {
        Self(size::Size::from_mib(size.float64()))
    }

    #[expect(clippy::needless_pass_by_value)]
    #[staticmethod]
    fn from_pb(size: PySizeIntermediate) -> Self {
        Self(size::Size::from_pb(size.float64()))
    }

    #[expect(clippy::needless_pass_by_value)]
    #[staticmethod]
    fn from_pebibytes(size: PySizeIntermediate) -> Self {
        Self(size::Size::from_pebibytes(size.float64()))
    }

    #[expect(clippy::needless_pass_by_value)]
    #[staticmethod]
    fn from_petabytes(size: PySizeIntermediate) -> Self {
        Self(size::Size::from_petabytes(size.float64()))
    }

    #[expect(clippy::needless_pass_by_value)]
    #[staticmethod]
    fn from_pib(size: PySizeIntermediate) -> Self {
        Self(size::Size::from_pib(size.float64()))
    }

    #[expect(clippy::needless_pass_by_value)]
    #[staticmethod]
    fn from_tb(size: PySizeIntermediate) -> Self {
        Self(size::Size::from_tb(size.float64()))
    }

    #[expect(clippy::needless_pass_by_value)]
    #[staticmethod]
    fn from_tebibytes(size: PySizeIntermediate) -> Self {
        Self(size::Size::from_tebibytes(size.float64()))
    }

    #[expect(clippy::needless_pass_by_value)]
    #[staticmethod]
    fn from_terabytes(size: PySizeIntermediate) -> Self {
        Self(size::Size::from_terabytes(size.float64()))
    }

    #[expect(clippy::needless_pass_by_value)]
    #[staticmethod]
    fn from_tib(size: PySizeIntermediate) -> Self {
        Self(size::Size::from_tib(size.float64()))
    }
}

#[derive(Debug, Clone)]
struct SizeWrapper(size::Size);

impl FromPyObject<'_> for SizeWrapper {
    fn extract_bound(ob: &Bound<'_, PyAny>) -> PyResult<Self> {
        if let Ok(s) = ob.cast::<PySize>() {
            let pysize = s.extract::<PySize>()?;
            Ok(Self(pysize.0))
        } else if let Ok(i) = ob.extract::<i64>() {
            Ok(Self(size::Size::from_const(i)))
        } else if let Ok(f) = ob.extract::<f64>() {
            Ok(Self(size::Size::from_bytes(f)))
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
            Self::Float64(f) => *f,
            Self::Int64(i) => *i as f64,
            Self::U64(u) => *u as f64,
        }
    }
}

#[derive(Debug, Clone, Copy, FromPyObject)]
enum PySizeArithmetic {
    Size(PySize),
    Int64(i64),
    U64(u64),
    Float64(f64), // must make float last
}
