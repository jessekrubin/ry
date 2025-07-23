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
        let mut s = if self.fmt {
            toml::to_string_pretty(&s).map_err(map_serde_json_err)
        } else {
            toml::to_string(&s).map_err(map_serde_json_err)
        }?;

        if self.append_newline {
            s.push('\n');
        }

        let pyany = s.into_bound_py_any(py)?;
        Ok(pyany)
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

stringify_fn!(stringify_toml);
stringify_fn!(dumps_toml);
