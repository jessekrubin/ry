use pyo3::exceptions::{PyRecursionError, PyTypeError};
use pyo3::prelude::*;
use pyo3::{IntoPyObjectExt, PyTypeInfo};
use ryo3_bytes::RyBytes;
use ryo3_serde::{JsonTarget, PyAnySerializer};

use crate::ser::{self, JsonSerError};
use crate::ser_opts::JsonOptions;

fn map_ser_err(e: &JsonSerError) -> PyErr {
    if e.to_string() == ryo3_serde::RECURSION_ERR_MSG {
        PyRecursionError::new_err("Recursion limit reached")
    } else {
        PyTypeError::new_err(format!("Failed to serialize: {e}"))
    }
}

pub fn py_to_vec(obj: Borrowed<'_, '_, PyAny>) -> PyResult<Vec<u8>> {
    let s = PyAnySerializer::new_json(obj, None);
    ser::to_vec(&s).map_err(|e| map_ser_err(&e))
}

#[derive(Debug, Default)]
struct JsonSerializer<'py> {
    default: Option<&'py Bound<'py, PyAny>>,
    opts: JsonOptions,
}

impl<'py> JsonSerializer<'py> {
    fn new(default: Option<&'py Bound<'py, PyAny>>, options: JsonOptions) -> PyResult<Self> {
        let slf = JsonSerializer {
            default,
            opts: options,
        };
        slf.check_default()?;
        Ok(slf)
    }

    fn check_default(&self) -> PyResult<()> {
        if let Some(default) = self.default
            && !default.is_callable()
        {
            let type_str = default
                .get_type()
                .name()
                .map_or_else(|_| "unknown-type".to_string(), |name| name.to_string());
            return Err(PyTypeError::new_err(format!(
                "'{type_str}' is not callable",
            )));
        }
        Ok(())
    }

    fn to_vec<T: serde_core::Serialize>(&self, s: &T) -> Result<Vec<u8>, JsonSerError> {
        if self.opts.fmt() {
            ser::to_vec_pretty(s)
        } else {
            ser::to_vec(s)
        }
    }

    pub(crate) fn serialize_to_vec(&self, obj: Borrowed<'_, '_, PyAny>) -> PyResult<Vec<u8>> {
        let mut bytes = if self.opts.sort_keys() {
            let s = PyAnySerializer::<JsonTarget<true>>::with_target(obj, self.default);
            self.to_vec(&s)
        } else {
            let s = PyAnySerializer::new_json(obj, self.default);
            self.to_vec(&s)
        }
        .map_err(|e| map_ser_err(&e))?;

        if self.opts.append_newline() {
            bytes.push(b'\n');
        }
        Ok(bytes)
    }
}

// **retired**
// MACRO TO CREATE THE STRINGIFY FUNCTION (USED TO CREATE "ALIASES" eg
// `stringify`/`dumps`)
// ```rust
// macro_rules! stringify_fn {
//     ($name:ident) => {
//         #[pyfunction(
//             signature = (obj,*, default = None, fmt = false, sort_keys = false, append_newline = false, pybytes = false))
//         ]
//         pub fn $name<'py>(
//             py: Python<'py>,
//             obj: &Bound<'py, PyAny>,
//             default: Option<&'py Bound<'py, PyAny>>,
//             fmt: bool,
//             sort_keys: bool,
//             append_newline: bool,
//             pybytes: bool,
//         ) -> PyResult<Bound<'py, PyAny>> {
//             let serializer = JsonSerializer::new(
//                 default,
//                 JsonOptions {
//                     fmt,
//                     sort_keys,
//                     append_newline,
//                     pybytes,
//                 },
//             )?;
//             serializer.serialize(py, obj)
//         }
//     };
// }
// stringify_fn!(stringify);
// stringify_fn!(dumps);
// ```

/// stringify w/ into output type
pub(crate) fn stringify_impl<'py>(
    py: Python<'py>,
    obj: &Bound<'py, PyAny>,
    default: Option<&'py Bound<'py, PyAny>>,
    fmt: bool,
    sort_keys: bool,
    append_newline: bool,
    dtype: PyJsonDtype,
) -> PyResult<Bound<'py, PyAny>> {
    let opts = JsonOptions::new()
        .with_fmt(fmt)
        .with_sort_keys(sort_keys)
        .with_append_newline(append_newline);
    let serializer = JsonSerializer::new(default, opts)?;
    serializer.serialize_to_vec(obj.as_borrowed()).map(|v| {
        if dtype.is_pybytes() {
            pyo3::types::PyBytes::new(py, &v).into_bound_py_any(py)
        } else {
            RyBytes::from(v).into_bound_py_any(py)
        }
    })?
}

/// JSON dumps/stringify return type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PyJsonDtype {
    PyBytes,
    RyBytes,
}

impl PyJsonDtype {
    /// ptr compare against the type objects
    fn from_ptr(py: Python<'_>, ptr: *mut pyo3::ffi::PyObject) -> Option<Self> {
        if ptr == pyo3::types::PyBytes::type_object_raw(py).cast() {
            Some(Self::PyBytes)
        } else if ptr == RyBytes::type_object_raw(py).cast() {
            Some(Self::RyBytes)
        } else {
            None
        }
    }

    #[inline]
    fn is_pybytes(self) -> bool {
        matches!(self, Self::PyBytes)
    }
}

impl<'a, 'py> FromPyObject<'a, 'py> for PyJsonDtype {
    type Error = PyErr;

    fn extract(obj: Borrowed<'a, 'py, PyAny>) -> Result<Self, Self::Error> {
        Self::from_ptr(obj.py(), obj.as_ptr())
            .ok_or_else(|| PyTypeError::new_err("Invalid dtype (expected `bytes` or `Bytes`)"))
    }
}

#[cold]
fn warn_pybytes_deprecated(py: Python<'_>) -> PyResult<()> {
    PyErr::warn(
        py,
        &py.get_type::<pyo3::exceptions::PyDeprecationWarning>(),
        c"`pybytes` is deprecated; use `dtype=bytes` instead [removal: v0.0.109]",
        1,
    )?;
    Ok(())
}

/// DEPRECATED `pybytes` ~ remove in v0.0.109 (explicit `dtype=ry.Bytes` looks like the default)
fn stringify_kwargs_check(
    py: Python<'_>,
    dtype: PyJsonDtype,
    pybytes: Option<bool>,
) -> PyResult<PyJsonDtype> {
    match (dtype, pybytes) {
        (_, None) => Ok(dtype),
        (PyJsonDtype::PyBytes, Some(_)) => Err(PyTypeError::new_err(
            "`pybytes` and `dtype` are mutually exclusive; use `dtype` (`pybytes` is deprecated)",
        )),
        (PyJsonDtype::RyBytes, Some(pybytes)) => {
            warn_pybytes_deprecated(py)?;
            Ok(if pybytes {
                PyJsonDtype::PyBytes
            } else {
                PyJsonDtype::RyBytes
            })
        }
    }
}

#[expect(clippy::too_many_arguments, reason = "python kwargs")]
#[pyfunction(
    signature=(
        obj,
        *,
        default = None,
        fmt = false,
        sort_keys = false,
        append_newline = false,
        dtype = PyJsonDtype::RyBytes,
        pybytes = None
    )
)]
pub fn stringify<'py>(
    py: Python<'py>,
    obj: &Bound<'py, PyAny>,
    default: Option<&'py Bound<'py, PyAny>>,
    fmt: bool,
    sort_keys: bool,
    append_newline: bool,
    dtype: PyJsonDtype,
    pybytes: Option<bool>,
) -> PyResult<Bound<'py, PyAny>> {
    let dtype = stringify_kwargs_check(py, dtype, pybytes)?;
    stringify_impl(py, obj, default, fmt, sort_keys, append_newline, dtype)
}

#[expect(clippy::too_many_arguments, reason = "python kwargs")]
#[pyfunction(
    signature=(
        obj,
        *,
        default = None,
        fmt = false,
        sort_keys = false,
        append_newline = false,
        dtype = PyJsonDtype::RyBytes,
        pybytes = None
    )
)]
pub fn dumps<'py>(
    py: Python<'py>,
    obj: &Bound<'py, PyAny>,
    default: Option<&'py Bound<'py, PyAny>>,
    fmt: bool,
    sort_keys: bool,
    append_newline: bool,
    dtype: PyJsonDtype,
    pybytes: Option<bool>,
) -> PyResult<Bound<'py, PyAny>> {
    let dtype = stringify_kwargs_check(py, dtype, pybytes)?;
    stringify_impl(py, obj, default, fmt, sort_keys, append_newline, dtype)
}
