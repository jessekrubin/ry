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

#[expect(clippy::struct_excessive_bools)]
struct JsonSerializer<'py> {
    default: Option<&'py Bound<'py, PyAny>>,

    fmt: bool,
    sort_keys: bool,
    append_newline: bool,
    pybytes: bool,
}

impl<'py> JsonSerializer<'py> {
    #[expect(clippy::fn_params_excessive_bools)]
    fn new(
        default: Option<&'py Bound<'py, PyAny>>,
        fmt: bool,
        sort_keys: bool,
        append_newline: bool,
        pybytes: bool,
    ) -> PyResult<Self> {
        let slf = JsonSerializer {
            default,
            fmt,
            sort_keys,
            append_newline,
            pybytes,
        };
        slf.check_default()?;
        Ok(slf)
    }

    fn check_default(&self) -> PyResult<()> {
        if let Some(default) = self.default {
            if !default.is_callable() {
                let type_str = default
                    .get_type()
                    .name()
                    .map(|name| name.to_string())
                    .unwrap_or("unknown-type".to_string());
                return Err(PyTypeError::new_err(format!(
                    "'{type_str}' is not callable",
                )));
            }
        }
        Ok(())
    }

    fn serialize(&self, py: Python<'py>, obj: &Bound<'py, PyAny>) -> PyResult<Bound<'py, PyAny>> {
        let s = SerializePyAny::new(obj, self.default);
        let mut bytes: Vec<u8> = Vec::with_capacity(4096);
        if self.sort_keys {
            // TODO: This is a very hacky way of handling sorting the keys...
            //       ideally this would be part of the serialization process
            //       I think
            let value = serde_json::to_value(&s).map_err(map_serde_json_err)?;
            if self.fmt {
                serde_json::to_writer_pretty(&mut bytes, &value).map_err(map_serde_json_err)?;
            } else {
                serde_json::to_writer(&mut bytes, &value).map_err(map_serde_json_err)?;
            }
        } else {
            // 4k seeeems is a reasonable default size for JSON serialization?
            if self.fmt {
                serde_json::to_writer_pretty(&mut bytes, &s).map_err(map_serde_json_err)?;
            } else {
                serde_json::to_writer(&mut bytes, &s).map_err(map_serde_json_err)?;
            }
        }

        if self.append_newline {
            bytes.push(b'\n');
        }
        if self.pybytes {
            pyo3::types::PyBytes::new(py, &bytes).into_bound_py_any(py)
        } else {
            ryo3_bytes::PyBytes::from(bytes).into_bound_py_any(py)
        }
    }
}

// MACRO TO CREATE THE STRINGIFY FUNCTION (USED TO CREATE "ALIASES" eg `stringify`/`loads`)
macro_rules! stringify_fn {
    ($name:ident) => {
        #[pyfunction(
            signature = (obj, *, default = None, fmt = false, sort_keys = false, append_newline = false, pybytes = false)
        )]
        pub fn $name<'py>(
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
                fmt,
                sort_keys,
                append_newline,
                pybytes,
            )?;
            serializer.serialize(py, obj)
        }
    };
}

stringify_fn!(stringify);
stringify_fn!(dumps);
