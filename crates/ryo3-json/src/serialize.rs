use pyo3::IntoPyObjectExt;
use pyo3::exceptions::{PyRecursionError, PyTypeError};
use pyo3::prelude::*;
use ryo3_bytes::RyBytes;
use ryo3_serde::{JsonTarget, PyAnySerializer, SerializeTarget};

fn map_serde_json_err<E: std::fmt::Display>(e: E) -> PyErr {
    if e.to_string().starts_with("recursion") {
        PyRecursionError::new_err("Recursion limit reached")
    } else {
        PyTypeError::new_err(format!("Failed to serialize: {e}"))
    }
}

#[derive(Clone, Copy, Debug, Default)]
struct JsonOptions {
    fmt: bool,
    sort_keys: bool,
    append_newline: bool,
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

    fn new_no_default(options: JsonOptions) -> Self {
        JsonSerializer {
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

    pub(crate) fn serialize_to_vec(&self, obj: &Bound<'py, PyAny>) -> PyResult<Vec<u8>> {
        let mut bytes: Vec<u8> = Vec::with_capacity(4096);

        if self.opts.sort_keys {
            let s = PyAnySerializer::new_with_target(
                obj.as_borrowed(),
                self.default,
                JsonTarget::<true>,
            );
            self.write_json(&mut bytes, &s)?;
        } else {
            let s = PyAnySerializer::new_with_target(
                obj.as_borrowed(),
                self.default,
                JsonTarget::<false>,
            );
            self.write_json(&mut bytes, &s)?;
        }

        if self.opts.append_newline {
            bytes.push(b'\n');
        }
        Ok(bytes)
    }

    fn write_json<T: SerializeTarget>(
        &self,
        bytes: &mut Vec<u8>,
        serializer: &PyAnySerializer<'_, 'py, T>,
    ) -> PyResult<()> {
        if self.opts.fmt {
            serde_json::to_writer_pretty(bytes, serializer).map_err(map_serde_json_err)
        } else {
            serde_json::to_writer(bytes, serializer).map_err(map_serde_json_err)
        }
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
            RyBytes::from(v).into_bound_py_any(py)
        }
    })?
}

pub fn to_vec(obj: &Bound<'_, PyAny>) -> PyResult<Vec<u8>> {
    JsonSerializer::new_no_default(JsonOptions {
        fmt: false,
        sort_keys: false,
        append_newline: false,
    })
    .serialize_to_vec(obj)
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
