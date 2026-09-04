use pyo3::IntoPyObjectExt;
use pyo3::exceptions::{PyRecursionError, PyTypeError};
use pyo3::prelude::*;
use ryo3_bytes::RyBytes;
use ryo3_core::macros::py_not_implemented_err;
use ryo3_serde::PyAnySerializer;

use super::ser;

fn map_serde_json_err<E: std::fmt::Display>(e: E) -> PyErr {
    if e.to_string().starts_with("recursion") {
        PyRecursionError::new_err("Recursion limit reached")
    } else {
        PyTypeError::new_err(format!("Failed to serialize: {e}"))
    }
}

#[derive(Clone, Copy, Debug, Default)]
struct JsonOptionsV2 {
    fmt: bool,
    sort_keys: bool,
    append_newline: bool,
}

#[derive(Debug, Default)]
struct JsonSerializerV2<'py> {
    default: Option<&'py Bound<'py, PyAny>>,
    opts: JsonOptionsV2,
}

impl<'py> JsonSerializerV2<'py> {
    fn new(default: Option<&'py Bound<'py, PyAny>>, options: JsonOptionsV2) -> PyResult<Self> {
        let slf = JsonSerializerV2 {
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

    pub(crate) fn serialize_to_vec(&self, obj: &Bound<'py, PyAny>) -> PyResult<Vec<u8>> {
        let s = PyAnySerializer::new_json(obj.as_borrowed(), self.default);
        let mut bytes: Vec<u8> = Vec::with_capacity(4096);
        if self.opts.sort_keys {
            return py_not_implemented_err!("tbd");
        } else {
            write_json_v2(&mut bytes, &s, self.opts.fmt)?;
        }

        if self.opts.append_newline {
            bytes.push(b'\n');
        }
        Ok(bytes)
    }
}

fn write_json_v2<T: serde_core::Serialize>(
    bytes: &mut Vec<u8>,
    value: &T,
    fmt: bool,
) -> PyResult<()> {
    if fmt {
        ser::to_writer_pretty(bytes, value)
    } else {
        ser::to_writer(bytes, value)
    }
    .map_err(map_serde_json_err)
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
    let serializer = JsonSerializerV2::new(
        default,
        JsonOptionsV2 {
            fmt,
            sort_keys,
            append_newline,
        },
    )?;
    serializer.serialize_to_vec(obj).map(|v| {
        if pybytes {
            pyo3::types::PyBytes::new(py, &v).into_bound_py_any(py)
        } else {
            RyBytes::from(v).into_bound_py_any(py)
        }
    })?
}
