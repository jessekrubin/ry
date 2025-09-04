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
