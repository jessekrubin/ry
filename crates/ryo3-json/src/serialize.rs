use pyo3::IntoPyObjectExt;
use pyo3::exceptions::{PyRecursionError, PyTypeError};
use pyo3::prelude::*;
use ryo3_bytes::RyBytes;
use ryo3_serde::PyAnySerializer;

const DEFAULT_CAPACITY: usize = 4096;

fn map_serde_json_err<E: std::fmt::Display>(e: E) -> PyErr {
    if e.to_string().starts_with("recursion") {
        PyRecursionError::new_err("Recursion limit reached")
    } else {
        PyTypeError::new_err(format!("Failed to serialize: {e}"))
    }
}

type JsonSerOpt = u8;
const JSON_SER_FMT: JsonSerOpt = 1 << 0;
const JSON_SER_SORT_KEYS: JsonSerOpt = 1 << 1;
const JSON_SER_APPEND_NEWLINE: JsonSerOpt = 1 << 2;

#[derive(Clone, Copy, Debug, Default)]
struct JsonOptions(JsonSerOpt);

impl JsonOptions {
    #[inline]
    const fn new() -> Self {
        Self(0)
    }

    #[inline]
    const fn with_sort_keys(self, sort_keys: bool) -> Self {
        if sort_keys {
            Self(self.0 | JSON_SER_SORT_KEYS)
        } else {
            self
        }
    }

    #[inline]
    const fn with_append_newline(self, append_newline: bool) -> Self {
        if append_newline {
            Self(self.0 | JSON_SER_APPEND_NEWLINE)
        } else {
            self
        }
    }

    #[inline]
    const fn with_fmt(self, fmt: bool) -> Self {
        if fmt {
            Self(self.0 | JSON_SER_FMT)
        } else {
            self
        }
    }

    #[inline]
    const fn fmt(self) -> bool {
        self.0 & JSON_SER_FMT != 0
    }

    #[inline]
    const fn sort_keys(self) -> bool {
        self.0 & JSON_SER_SORT_KEYS != 0
    }

    #[inline]
    const fn append_newline(self) -> bool {
        self.0 & JSON_SER_APPEND_NEWLINE != 0
    }
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

    pub(crate) fn serialize_to_vec(&self, obj: Borrowed<'_, '_, PyAny>) -> PyResult<Vec<u8>> {
        let s = PyAnySerializer::new_json(obj.as_borrowed(), self.default);
        let mut bytes: Vec<u8> = Vec::with_capacity(DEFAULT_CAPACITY);
        if self.opts.sort_keys() {
            // TODO: This is a very hacky way of handling sorting the keys...
            //       ideally this would be part of the serialization process
            //       I think
            let value = serde_json::to_value(&s).map_err(map_serde_json_err)?;
            if self.opts.fmt() {
                serde_json::to_writer_pretty(&mut bytes, &value).map_err(map_serde_json_err)?;
            } else {
                serde_json::to_writer(&mut bytes, &value).map_err(map_serde_json_err)?;
            }
        } else {
            // 4k seeeems is a reasonable default size for JSON serialization?
            if self.opts.fmt() {
                serde_json::to_writer_pretty(&mut bytes, &s).map_err(map_serde_json_err)?;
            } else {
                serde_json::to_writer(&mut bytes, &s).map_err(map_serde_json_err)?;
            }
        }

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

pub fn to_vec(obj: Borrowed<'_, '_, PyAny>) -> PyResult<Vec<u8>> {
    JsonSerializer::new_no_default(JsonOptions(0)).serialize_to_vec(obj)
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
