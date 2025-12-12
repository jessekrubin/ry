use pyo3::IntoPyObjectExt;
use pyo3::exceptions::{PyRecursionError, PyTypeError};
use pyo3::prelude::*;
use ryo3_serde::SerializePyAny;

fn map_serde_json_err<E: std::fmt::Display>(e: E) -> PyErr {
    if e.to_string().starts_with("recursion") {
        PyRecursionError::new_err("Recursion limit reached")
    } else {
        PyTypeError::new_err(format!("Failed to serialize: {e}"))
    }
}

struct JsonOptions {
    fmt: bool,
    sort_keys: bool,
    append_newline: bool,
}

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

    pub(crate) fn serialize_to_vec(&self, obj: &Bound<'py, PyAny>) -> PyResult<Vec<u8>> {
        let s = SerializePyAny::new(obj, self.default);
        let mut bytes: Vec<u8> = Vec::with_capacity(4096);
        if self.opts.sort_keys {
            // TODO: This is a very hacky way of handling sorting the keys...
            //       ideally this would be part of the serialization process
            //       I think
            let value = serde_json::to_value(&s).map_err(map_serde_json_err)?;
            if self.opts.fmt {
                serde_json::to_writer_pretty(&mut bytes, &value).map_err(map_serde_json_err)?;
            } else {
                serde_json::to_writer(&mut bytes, &value).map_err(map_serde_json_err)?;
            }
        } else {
            // 4k seeeems is a reasonable default size for JSON serialization?
            if self.opts.fmt {
                serde_json::to_writer_pretty(&mut bytes, &s).map_err(map_serde_json_err)?;
            } else {
                serde_json::to_writer(&mut bytes, &s).map_err(map_serde_json_err)?;
            }
        }

        if self.opts.append_newline {
            bytes.push(b'\n');
        }
        Ok(bytes)
    }

    #[expect(clippy::unused_self)]
    fn serialize_unsafe(
        &self,
        _py: Python<'py>,
        _obj: &Bound<'py, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        Err(pyo3::exceptions::PyNotImplementedError::new_err(
            "Unsafe serialization is not implemented yet",
        ))
    }
}

// **retired**
// MACRO TO CREATE THE STRINGIFY FUNCTION (USED TO CREATE "ALIASES" eg `stringify`/`dumps`)
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

#[expect(clippy::fn_params_excessive_bools)]
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
    let serializer = JsonSerializer::new(
        default,
        JsonOptions {
            fmt,
            sort_keys,
            append_newline,
        },
    )?;
    serializer.serialize_to_vec(obj).map(|v| {
        if pybytes {
            pyo3::types::PyBytes::new(py, &v).into_bound_py_any(py)
        } else {
            ryo3_bytes::PyBytes::from(v).into_bound_py_any(py)
        }
    })?
}

#[expect(clippy::fn_params_excessive_bools)]
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
#[expect(unused_variables, clippy::fn_params_excessive_bools)]
pub(crate) fn stringify_unsafe<'py>(
    py: Python<'py>,
    obj: &Bound<'py, PyAny>,
    default: Option<&'py Bound<'py, PyAny>>,
    fmt: bool,
    sort_keys: bool,
    append_newline: bool,
    pybytes: bool,
) -> PyResult<Bound<'py, PyAny>> {
    let serializer = JsonSerializer::new(
        default,
        JsonOptions {
            fmt,
            sort_keys,
            append_newline,
        },
    )?;
    serializer.serialize_unsafe(py, obj)
}
