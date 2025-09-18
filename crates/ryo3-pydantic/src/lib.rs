#![doc = include_str!("../README.md")]
use pyo3::prelude::*;
use pyo3::sync::PyOnceLock;
use pyo3::types::PyType;
use pyo3::{PyAny, PyResult};

static CORE_SCHEMA: PyOnceLock<Py<PyModule>> = PyOnceLock::new();
pub fn core_schema(py: Python<'_>) -> PyResult<&Bound<'_, PyModule>> {
    CORE_SCHEMA.import(py, "pydantic_core", "core_schema")
}

pub trait GetPydanticCoreSchemaCls {
    fn get_pydantic_core_schema<'py>(
        cls: &Bound<'py, PyType>,
        source: &Bound<'py, PyAny>,
        _handler: &Bound<'py, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>>;
}

pub mod interns {
    /// String interns that are obviously unpaid and over worked...
    ///
    /// classic millenials... nobody wants to be a string intern anymore...
    ///
    /// This keeps the string-interning in one place so we dont have multiple
    /// pydantic based interns who are milling around, presumably not getting
    /// me the coffee I asked for.
    use pyo3::prelude::*;

    macro_rules! unpaid_intern {
        ($name:ident, $lit:expr) => {
            pub fn $name(py: Python<'_>) -> &Bound<'_, pyo3::types::PyString> {
                pyo3::intern!(py, $lit)
            }
        };

        ($name:ident) => {
            pub fn $name(py: Python<'_>) -> &Bound<'_, pyo3::types::PyString> {
                pyo3::intern!(py, stringify!($name))
            }
        };
    }

    unpaid_intern!(_pydantic_validate);
    unpaid_intern!(_pydantic_validate_strict);
    unpaid_intern!(json_unless_none, "json-unless-none");
    unpaid_intern!(no_info_wrap_validator_function);
    unpaid_intern!(no_info_plain_validator_function);
    unpaid_intern!(to_string_ser_schema);
    unpaid_intern!(lax_or_strict_schema);
    unpaid_intern!(when_used);
    // kwargs
    unpaid_intern!(serialization);
    unpaid_intern!(min_length);
    unpaid_intern!(max_length);
    unpaid_intern!(pattern);
    // schemas
    unpaid_intern!(str_schema);
    unpaid_intern!(bytes_schema);
    unpaid_intern!(union_schema);
    unpaid_intern!(is_instance_schema);
    // datetime schemas
    unpaid_intern!(timedelta_schema);
    unpaid_intern!(datetime_schema);
    unpaid_intern!(date_schema);
    unpaid_intern!(time_schema);
}
