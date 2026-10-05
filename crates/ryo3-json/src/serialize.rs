use pyo3::IntoPyObjectExt;
use pyo3::exceptions::{PyRecursionError, PyTypeError};
use pyo3::prelude::*;
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

#[expect(clippy::fn_params_excessive_bools, reason = "python kwargs")]
#[pyfunction(
    signature=(
        obj,
        *,
        default = None,
        fmt = false,
        sort_keys = false,
        append_newline = false,
        pybytes = false
    )
)]
pub fn stringify<'py>(
    py: Python<'py>,
    obj: &Bound<'py, PyAny>,
    default: Option<&'py Bound<'py, PyAny>>,
    fmt: bool,
    sort_keys: bool,
    append_newline: bool,
    pybytes: bool,
) -> PyResult<Bound<'py, PyAny>> {
    let opts = JsonOptions::new()
        .with_fmt(fmt)
        .with_sort_keys(sort_keys)
        .with_append_newline(append_newline);
    let serializer = JsonSerializer::new(default, opts)?;
    serializer.serialize_to_vec(obj.as_borrowed()).map(|v| {
        if pybytes {
            pyo3::types::PyBytes::new(py, &v).into_bound_py_any(py)
        } else {
            RyBytes::from(v).into_bound_py_any(py)
        }
    })?
}

#[expect(clippy::fn_params_excessive_bools, reason = "python kwargs")]
#[pyfunction(
    signature=(
        obj,
        *,
        default = None,
        fmt = false,
        sort_keys = false,
        append_newline = false,
        pybytes = false
    )
)]
pub fn dumps<'py>(
    py: Python<'py>,
    obj: &Bound<'py, PyAny>,
    default: Option<&'py Bound<'py, PyAny>>,
    fmt: bool,
    sort_keys: bool,
    append_newline: bool,
    pybytes: bool,
) -> PyResult<Bound<'py, PyAny>> {
    stringify(py, obj, default, fmt, sort_keys, append_newline, pybytes)
}
