use std::ops::{Neg, Not};
use std::str::FromStr;

use pyo3::prelude::*;
use pyo3::pyclass::CompareOp;
use pyo3::types::{PyFloat, PyInt, PyTuple};
use ryo3_core::macros::{py_overflow_err, py_overflow_error, py_type_error};
use ryo3_core::{
    PyAsciiString, PyNotImplementedOr, FromPyOperand, PyOperandArg, PyRichCmp, pynumber_check,
};

use crate::types::{PyBase, PyStyle};

#[derive(Debug, Clone, Copy)]
#[pyclass(name = "Size", frozen)]
#[cfg_attr(any(not(Py_LIMITED_API), Py_3_14), pyo3(immutable_type))]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
pub struct RySize(size::Size);

impl From<size::Size> for RySize {
    fn from(size: size::Size) -> Self {
        Self(size)
    }
}

impl From<i64> for RySize {
    fn from(size: i64) -> Self {
        Self::from_const(size)
    }
}

impl RySize {
    const fn from_const(size: i64) -> Self {
        Self(size::Size::from_const(size))
    }

    fn py_try_add(self, rhs: PySizeOperand) -> PyResult<Self> {
        self.0
            .bytes()
            .checked_add(rhs.bytes_for_add_sub())
            .map(Self::from)
            .ok_or_else(|| py_overflow_error!("overflow"))
    }

    fn py_try_sub(self, rhs: PySizeOperand) -> PyResult<Self> {
        self.0
            .bytes()
            .checked_sub(rhs.bytes_for_add_sub())
            .map(Self::from)
            .ok_or_else(|| py_overflow_error!("overflow"))
    }

    #[expect(clippy::cast_precision_loss, clippy::cast_possible_truncation)]
    fn py_try_mul(self, rhs: PySizeOperand) -> PyResult<Self> {
        let base = self.0.bytes();
        if base == 0 {
            return Ok(Self::from(0));
        }
        let result_bytes: Option<i64> = match rhs {
            PySizeOperand::Size(s) => base.checked_mul(s.bytes()),

            PySizeOperand::Int64(i) => base.checked_mul(i),
            PySizeOperand::UInt64(u) => {
                let product = i128::from(base)
                    .checked_mul(i128::from(u))
                    .ok_or_else(|| py_overflow_error!("overflow (size * u64)"))?;
                if (i128::from(i64::MIN)..=i128::from(i64::MAX)).contains(&product) {
                    product.try_into().ok()
                } else {
                    None
                }
            }
            PySizeOperand::Float64(f) => {
                if !(f.is_finite()) {
                    return py_overflow_err!("Cannot multiply Size by NaN or infinite float");
                }
                let result_f64 = (base as f64) * f;
                if !result_f64.is_finite() {
                    return py_overflow_err!("Overflow in Size * float");
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
            .map(Self::from)
            .ok_or_else(|| py_overflow_error!("Overflow in Size multiplication"))
    }

    /// compare as a python `int` (big ints, floats)
    fn py_int_richcmp(self, other: &Bound<'_, PyAny>, op: CompareOp) -> PyResult<PyRichCmp> {
        self.0
            .bytes()
            .into_pyobject(other.py())?
            .rich_compare(other, op)?
            .is_truthy()
            .map(PyRichCmp::from)
    }
}

#[expect(clippy::trivially_copy_pass_by_ref)]
#[pymethods]
impl RySize {
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

    fn __str__(&self) -> PyAsciiString {
        format!("{}", self.0).into()
    }

    fn __repr__(&self) -> PyAsciiString {
        format!("{self}").into()
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

    fn __richcmp__(&self, other: &Bound<'_, PyAny>, op: CompareOp) -> PyResult<PyRichCmp> {
        use ryo3_core::PyCastExactOpt;
        let other_bytes = if let Some(s) = other.cast_exact_opt::<Self>() {
            s.get().0.bytes()
        } else if other.is_instance_of::<PyInt>() {
            match other.extract::<i64>() {
                Ok(i) => i,
                // big int: let python do it (exact)
                Err(_) => return self.py_int_richcmp(other, op),
            }
        } else if other.is_instance_of::<PyFloat>() {
            // python int-vs-float compare is exact (no truncating the float)
            return self.py_int_richcmp(other, op);
        } else {
            return Ok(PyRichCmp::NotImplemented);
        };
        Ok(op.matches(self.0.bytes().cmp(&other_bytes)).into())
    }

    #[pyo3(
        signature = (*, base = PyBase::default(), style = PyStyle::default(), scale = None),
        text_signature = "(self, *, base=2, style='default', scale=None)"
    )]
    fn format(&self, base: PyBase, style: PyStyle, scale: Option<u8>) -> PyAsciiString {
        self.0
            .format()
            .with_base(base.into())
            .with_style(style.into())
            .with_scale(scale.map(usize::from))
            .to_string()
            .into()
    }

    #[staticmethod]
    #[pyo3(signature = (s, /))]
    fn from_str(s: ryo3_core::PyFromStrArg<Self>) -> Self {
        s.into_inner()
    }

    #[staticmethod]
    #[pyo3(signature = (value, /))]
    fn parse(value: ryo3_core::PyParseArg<Self>) -> Self {
        value.into_inner()
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

    fn __add__(&self, other: PySizeOperandArg) -> PyResult<PyNotImplementedOr<Self>> {
        other.try_map(|rhs| self.py_try_add(rhs))
    }

    fn __sub__(&self, other: PySizeOperandArg) -> PyResult<PyNotImplementedOr<Self>> {
        other.try_map(|rhs| self.py_try_sub(rhs))
    }

    fn __mul__(&self, other: PySizeOperandArg) -> PyResult<PyNotImplementedOr<Self>> {
        other.try_map(|rhs| self.py_try_mul(rhs))
    }

    fn __rmul__(&self, other: PySizeOperandArg) -> PyResult<PyNotImplementedOr<Self>> {
        other.try_map(|rhs| self.py_try_mul(rhs))
    }

    #[staticmethod]
    fn from_bytes(size: PySizeIntermediate) -> Self {
        size.into_bytes().into()
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

    #[staticmethod]
    fn from_eb(size: PySizeIntermediate) -> Self {
        size.into_eb().into()
    }

    #[staticmethod]
    fn from_eib(size: PySizeIntermediate) -> Self {
        size.into_eib().into()
    }

    #[staticmethod]
    fn from_exabytes(size: PySizeIntermediate) -> Self {
        size.into_exabytes().into()
    }

    #[staticmethod]
    fn from_exbibytes(size: PySizeIntermediate) -> Self {
        size.into_exbibytes().into()
    }

    #[staticmethod]
    fn from_gb(size: PySizeIntermediate) -> Self {
        size.into_gb().into()
    }

    #[staticmethod]
    fn from_gib(size: PySizeIntermediate) -> Self {
        size.into_gib().into()
    }

    #[staticmethod]
    fn from_gibibytes(size: PySizeIntermediate) -> Self {
        size.into_gibibytes().into()
    }

    #[staticmethod]
    fn from_gigabytes(size: PySizeIntermediate) -> Self {
        size.into_gigabytes().into()
    }

    #[staticmethod]
    fn from_kb(size: PySizeIntermediate) -> Self {
        size.into_kb().into()
    }

    #[staticmethod]
    fn from_kib(size: PySizeIntermediate) -> Self {
        size.into_kib().into()
    }

    #[staticmethod]
    fn from_kibibytes(size: PySizeIntermediate) -> Self {
        size.into_kibibytes().into()
    }

    #[staticmethod]
    fn from_kilobytes(size: PySizeIntermediate) -> Self {
        size.into_kilobytes().into()
    }

    #[staticmethod]
    fn from_mb(size: PySizeIntermediate) -> Self {
        size.into_mb().into()
    }

    #[staticmethod]
    fn from_mebibytes(size: PySizeIntermediate) -> Self {
        size.into_mebibytes().into()
    }

    #[staticmethod]
    fn from_megabytes(size: PySizeIntermediate) -> Self {
        size.into_megabytes().into()
    }

    #[staticmethod]
    fn from_mib(size: PySizeIntermediate) -> Self {
        size.into_mib().into()
    }

    #[staticmethod]
    fn from_pb(size: PySizeIntermediate) -> Self {
        size.into_pb().into()
    }

    #[staticmethod]
    fn from_pebibytes(size: PySizeIntermediate) -> Self {
        size.into_pebibytes().into()
    }

    #[staticmethod]
    fn from_petabytes(size: PySizeIntermediate) -> Self {
        size.into_petabytes().into()
    }

    #[staticmethod]
    fn from_pib(size: PySizeIntermediate) -> Self {
        size.into_pib().into()
    }

    #[staticmethod]
    fn from_tb(size: PySizeIntermediate) -> Self {
        size.into_tb().into()
    }

    #[staticmethod]
    fn from_tebibytes(size: PySizeIntermediate) -> Self {
        size.into_tebibytes().into()
    }

    #[staticmethod]
    fn from_terabytes(size: PySizeIntermediate) -> Self {
        size.into_terabytes().into()
    }

    #[staticmethod]
    fn from_tib(size: PySizeIntermediate) -> Self {
        size.into_tib().into()
    }

    // ========================================================================
    // CLASS ATTRIBUTES / CONSTS
    // ========================================================================

    #[classattr]
    #[expect(non_snake_case, reason = "python classattr")]
    fn MIN() -> Self {
        Self::from_const(i64::MIN)
    }

    #[classattr]
    #[expect(non_snake_case, reason = "python classattr")]
    fn MAX() -> Self {
        Self::from_const(i64::MAX)
    }

    #[classattr]
    #[expect(non_snake_case, reason = "python classattr")]
    fn ZERO() -> Self {
        Self::from_const(0)
    }

    /// Basic "byte" constant, used across all bases.
    #[classattr]
    #[pyo3(name = "BYTE")]
    #[expect(non_snake_case, reason = "python classattr")]
    fn CONST_BYTE() -> Self {
        Self::from_const(::size::consts::BYTE)
    }
    /// Base-10 "kilobyte" constant, equal to 1000 bytes.
    #[classattr]
    #[expect(non_snake_case, reason = "python classattr")]
    fn KILOBYTE() -> Self {
        Self::from_const(::size::consts::KILOBYTE)
    }
    /// Base-10 "megabyte" constant, equal to 1000 kilobytes.
    #[classattr]
    #[expect(non_snake_case, reason = "python classattr")]
    fn MEGABYTE() -> Self {
        Self::from_const(::size::consts::MEGABYTE)
    }
    /// Base-10 "gigabyte" constant, equal to 1000 megabytes.
    #[classattr]
    #[expect(non_snake_case, reason = "python classattr")]
    fn GIGABYTE() -> Self {
        Self::from_const(::size::consts::GIGABYTE)
    }

    /// Base-10 "terabyte" constant, equal to 1000 gigabytes.
    #[classattr]
    #[expect(non_snake_case, reason = "python classattr")]
    fn TERABYTE() -> Self {
        Self::from_const(::size::consts::TERABYTE)
    }
    /// Base-10 "petabyte" constant, equal to 1000 terabytes.
    #[classattr]
    #[expect(non_snake_case, reason = "python classattr")]
    fn PETABYTE() -> Self {
        Self::from_const(::size::consts::PETABYTE)
    }
    /// Base-10 "exabyte" constant, equal to 1000 petabytes.
    #[classattr]
    #[expect(non_snake_case, reason = "python classattr")]
    fn EXABYTE() -> Self {
        Self::from_const(::size::consts::EXABYTE)
    }

    /// Abbreviated "byte" constant. Identical to [`BYTE`].
    #[classattr]
    #[expect(non_snake_case, reason = "python classattr")]
    fn B() -> Self {
        Self::from_const(::size::consts::BYTE)
    }
    /// Abbreviated base-10 "kilobyte" constant, equal to 1000 bytes. Identical to [`KILOBYTE`].
    #[classattr]
    #[expect(non_snake_case, reason = "python classattr")]
    fn KB() -> Self {
        Self::from_const(::size::consts::KILOBYTE)
    }
    /// Abbreviated base-10 "megabyte" constant, equal to 1000 kilobytes. Identical to [`MEGABYTE`].
    #[classattr]
    #[expect(non_snake_case, reason = "python classattr")]
    fn MB() -> Self {
        Self::from_const(::size::consts::MEGABYTE)
    }
    /// Abbreviated base-10 "gigabyte" constant, equal to 1000 megabytes. Identical to [`GIGABYTE`].
    #[classattr]
    #[expect(non_snake_case, reason = "python classattr")]
    fn GB() -> Self {
        Self::from_const(::size::consts::GIGABYTE)
    }
    /// Abbreviated base-10 "terabyte" constant, equal to 1000 gigabytes. Identical to [`TERABYTE`].
    #[classattr]
    #[expect(non_snake_case, reason = "python classattr")]
    fn TB() -> Self {
        Self::from_const(::size::consts::TERABYTE)
    }
    /// Abbreviated base-10 "petabyte" constant, equal to 1000 terabytes. Identical to [`PETABYTE`].
    #[classattr]
    #[expect(non_snake_case, reason = "python classattr")]
    fn PB() -> Self {
        Self::from_const(::size::consts::PETABYTE)
    }
    /// Abbreviated base-10 "exabyte" constant, equal to 1000 petabytes. Identical to [`EXABYTE`].
    #[classattr]
    #[expect(non_snake_case, reason = "python classattr")]
    fn EB() -> Self {
        Self::from_const(::size::consts::EXABYTE)
    }

    /// Base-2 "kibibyte" constant, equal to 2^10 bytes.
    #[classattr]
    #[expect(non_snake_case, reason = "python classattr")]
    fn KIBIBYTE() -> Self {
        Self::from_const(::size::consts::KIBIBYTE)
    }
    /// Base-2 "mebibyte" constant, equal to 2^20 bytes.
    #[classattr]
    #[expect(non_snake_case, reason = "python classattr")]
    fn MEBIBYTE() -> Self {
        Self::from_const(::size::consts::MEBIBYTE)
    }
    /// Base-2 "gibibyte" constant, equal to 2^30 bytes.
    #[classattr]
    #[expect(non_snake_case, reason = "python classattr")]
    fn GIBIBYTE() -> Self {
        Self::from_const(::size::consts::GIBIBYTE)
    }
    /// Base-2 "tebibyte" constant, equal to 2^40 bytes.
    #[classattr]
    #[expect(non_snake_case, reason = "python classattr")]
    fn TEBIBYTE() -> Self {
        Self::from_const(::size::consts::TEBIBYTE)
    }
    /// Base-2 "pebibyte" constant, equal to 2^50 bytes.
    #[classattr]
    #[expect(non_snake_case, reason = "python classattr")]
    fn PEBIBYTE() -> Self {
        Self::from_const(::size::consts::PEBIBYTE)
    }
    /// Base-2 "exbibyte" constant, equal to 2^60 bytes.
    #[classattr]
    #[expect(non_snake_case, reason = "python classattr")]
    fn EXBIBYTE() -> Self {
        Self::from_const(::size::consts::EXBIBYTE)
    }

    /// Abbreviated base-2 "kibibyte" constant, equal to 1024 bytes. Identical to [`KIBIBYTE`].
    #[classattr]
    #[expect(non_snake_case, reason = "python classattr")]
    fn KIB() -> Self {
        Self::from_const(::size::consts::KIBIBYTE)
    }
    /// Abbreviated base-2 "mebibyte" constant, equal to 1024 kibibytes. Identical to [`MEBIBYTE`].
    #[classattr]
    #[expect(non_snake_case, reason = "python classattr")]
    fn MIB() -> Self {
        Self::from_const(::size::consts::MEBIBYTE)
    }
    /// Abbreviated base-2 "gibibyte" constant, equal to 1024 mebibytes. Identical to [`GIBIBYTE`].
    #[classattr]
    #[expect(non_snake_case, reason = "python classattr")]
    fn GIB() -> Self {
        Self::from_const(::size::consts::GIBIBYTE)
    }
    /// Abbreviated base-2 "tebibyte" constant, equal to 1024 gibibytes. Identical to [`TEBIBYTE`].
    #[classattr]
    #[expect(non_snake_case, reason = "python classattr")]
    fn TIB() -> Self {
        Self::from_const(::size::consts::TEBIBYTE)
    }
    /// Abbreviated base-2 "pebibyte" constant, equal to 1024 tebibytes. Identical to [`PEBIBYTE`].
    #[classattr]
    #[expect(non_snake_case, reason = "python classattr")]
    fn PIB() -> Self {
        Self::from_const(::size::consts::PEBIBYTE)
    }
    /// Abbreviated base-2 "exbibyte" constant, equal to 1024 pebibytes. Identical to [`EXBIBYTE`].
    #[classattr]
    #[expect(non_snake_case, reason = "python classattr")]
    fn EIB() -> Self {
        Self::from_const(::size::consts::EXBIBYTE)
    }
}

impl<'py> FromPyObject<'_, 'py> for RySize {
    type Error = pyo3::PyErr;

    fn extract(obj: Borrowed<'_, 'py, PyAny>) -> Result<Self, Self::Error> {
        obj.cast_exact::<Self>()
            .map(Self::from)
            .map_err(|_| py_type_error!("expected `Size`"))
    }
}

impl From<Borrowed<'_, '_, Self>> for RySize {
    fn from(value: Borrowed<'_, '_, Self>) -> Self {
        Self(value.get().0)
    }
}

impl FromStr for RySize {
    type Err = size::ParseSizeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let size = size::Size::from_str(s)?;
        Ok(Self(size))
    }
}

#[derive(Debug, Clone, FromPyObject)]
enum PySizeIntermediate {
    Int64(i64),
    UInt64(u64),
    Float64(f64),
}

// implement intermidate `into_` functions using `from_` functions on `size::Size`
macro_rules! impl_intermediate_to_size_fn {
    ($to_fnname:ident, $from_fnname:ident ) => {
        impl PySizeIntermediate {
            fn $to_fnname(self) -> size::Size {
                match self {
                    Self::Float64(f) => size::Size::$from_fnname(f),
                    Self::Int64(i) => size::Size::$from_fnname(i),
                    Self::UInt64(u) => size::Size::$from_fnname(u),
                }
            }
        }
    };
}

// impl_intermediate_to_size_fn!(to_eb, from_eb);
impl_intermediate_to_size_fn!(into_bytes, from_bytes);
impl_intermediate_to_size_fn!(into_kb, from_kb);
impl_intermediate_to_size_fn!(into_kib, from_kib);
impl_intermediate_to_size_fn!(into_kibibytes, from_kibibytes);
impl_intermediate_to_size_fn!(into_kilobytes, from_kilobytes);
impl_intermediate_to_size_fn!(into_mb, from_mb);
impl_intermediate_to_size_fn!(into_mebibytes, from_mebibytes);
impl_intermediate_to_size_fn!(into_megabytes, from_megabytes);
impl_intermediate_to_size_fn!(into_mib, from_mib);
impl_intermediate_to_size_fn!(into_gb, from_gb);
impl_intermediate_to_size_fn!(into_gib, from_gib);
impl_intermediate_to_size_fn!(into_gibibytes, from_gibibytes);
impl_intermediate_to_size_fn!(into_gigabytes, from_gigabytes);
impl_intermediate_to_size_fn!(into_tb, from_tb);
impl_intermediate_to_size_fn!(into_tebibytes, from_tebibytes);
impl_intermediate_to_size_fn!(into_terabytes, from_terabytes);
impl_intermediate_to_size_fn!(into_tib, from_tib);
impl_intermediate_to_size_fn!(into_pb, from_pb);
impl_intermediate_to_size_fn!(into_pebibytes, from_pebibytes);
impl_intermediate_to_size_fn!(into_petabytes, from_petabytes);
impl_intermediate_to_size_fn!(into_pib, from_pib);
impl_intermediate_to_size_fn!(into_eb, from_eb);
impl_intermediate_to_size_fn!(into_eib, from_eib);
impl_intermediate_to_size_fn!(into_exabytes, from_exabytes);
impl_intermediate_to_size_fn!(into_exbibytes, from_exbibytes);

/// rhs of `+`/`-`/`*`
#[derive(Debug, Clone, Copy)]
enum PySizeOperand {
    Size(size::Size),
    Int64(i64),
    UInt64(u64),
    Float64(f64),
}
type PySizeOperandArg = PyOperandArg<PySizeOperand>;

impl FromPyOperand for PySizeOperand {
    fn from_py_operand(ob: Borrowed<'_, '_, PyAny>) -> Option<Self> {
        use ryo3_core::PyCastExactOpt;
        if let Some(s) = ob.cast_exact_opt::<RySize>() {
            Some(Self::Size(s.get().0))
        } else if let Some(f) = ob.cast_exact_opt::<PyFloat>() {
            // exact float has no `__index__` so the int extracts can't succeed
            Some(Self::Float64(f.value()))
        } else if !pynumber_check(ob) {
            // not a number (None, str, ...): every extract below would fail
            None
        } else if let Ok(i) = ob.extract::<i64>() {
            Some(Self::Int64(i))
        } else if let Ok(u) = ob.extract::<u64>() {
            Some(Self::UInt64(u))
        } else {
            ob.extract::<f64>().ok().map(Self::Float64)
        }
    }
}

impl PySizeOperand {
    #[expect(clippy::cast_precision_loss)]
    fn bytes_for_add_sub(self) -> i64 {
        match self {
            Self::Size(s) => s.bytes(),
            Self::Int64(i) => i,
            Self::UInt64(u) => size::Size::from_bytes(u as f64).bytes(),
            Self::Float64(f) => size::Size::from_bytes(f).bytes(),
        }
    }
}

impl std::fmt::Display for RySize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Size({})", self.0.bytes())
    }
}
