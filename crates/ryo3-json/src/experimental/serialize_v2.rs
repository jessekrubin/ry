use pyo3::IntoPyObjectExt;
use pyo3::exceptions::{PyRecursionError, PyTypeError};
use pyo3::prelude::*;
use ryo3_bytes::RyBytes;
use ryo3_core::macros::py_not_implemented_err;
use ryo3_serde::PyAnySerializer;

use super::ser;
use crate::ser_opts::JsonOptions;

fn map_serde_json_err<E: std::fmt::Display>(e: E) -> PyErr {
    if e.to_string().starts_with("recursion") {
        PyRecursionError::new_err("Recursion limit reached")
    } else {
        PyTypeError::new_err(format!("Failed to serialize: {e}"))
    }
}

#[derive(Debug, Default)]
struct JsonSerializerV2<'py> {
    default: Option<&'py Bound<'py, PyAny>>,
    opts: JsonOptions,
}

impl<'py> JsonSerializerV2<'py> {
    fn new(default: Option<&'py Bound<'py, PyAny>>, options: JsonOptions) -> PyResult<Self> {
        let slf = JsonSerializerV2 {
            default,
            opts: options,
        };
        slf.check_default()?;
        Ok(slf)
    }

    fn new_no_default(options: JsonOptions) -> Self {
        JsonSerializerV2 {
            default: None,
            opts: options,
        }
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

    pub(crate) fn serialize_to_vec(&self, obj: Borrowed<'_, '_, PyAny>) -> PyResult<Vec<u8>> {
        let s = PyAnySerializer::new_json(obj, self.default);
        if self.opts.sort_keys() {
            return py_not_implemented_err!("tbd");
        }

        let mut bytes = {
            if self.opts.fmt() {
                ser::to_vec_pretty(&s)
            } else {
                ser::to_vec(&s)
            }
        }
        .map_err(map_serde_json_err)?;

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
pub(crate) fn stringify_v2<'py>(
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
    let serializer = if let Some(default) = default {
        JsonSerializerV2::new(Some(default), opts)?
    } else {
        JsonSerializerV2::new_no_default(opts)
    };
    serializer.serialize_to_vec(obj.as_borrowed()).map(|v| {
        if pybytes {
            pyo3::types::PyBytes::new(py, &v).into_bound_py_any(py)
        } else {
            RyBytes::from(v).into_bound_py_any(py)
        }
    })?
}
