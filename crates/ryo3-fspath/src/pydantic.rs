use pyo3::prelude::*;
use pyo3::types::PyDict;
use ryo3_pydantic::{GetPydanticCoreSchemaCls, GetPydanticJsonSchemaCls, interns};

use crate::PyFsPath;

impl GetPydanticCoreSchemaCls for PyFsPath {
    fn get_pydantic_core_schema<'py>(
        cls: &Bound<'py, pyo3::types::PyType>,
        source: &Bound<'py, PyAny>,
        _handler: &Bound<'py, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let py = source.py();
        let core_schema = ryo3_pydantic::core_schema(py)?;
        let str_schema = core_schema.call_method(interns::str_schema(py), (), None)?;
        let validation_fn = cls.getattr(interns::_pydantic_validate(py))?;
        let string_serialization_schema =
            core_schema.call_method(interns::to_string_ser_schema(py), (), None)?;
        let plain_validator_kwargs = PyDict::new(py);
        plain_validator_kwargs.set_item(interns::json_schema_input_schema(py), &str_schema)?;
        plain_validator_kwargs
            .set_item(interns::serialization(py), &string_serialization_schema)?;
        core_schema.call_method(
            interns::no_info_plain_validator_function(py),
            (&validation_fn,),
            Some(&plain_validator_kwargs),
        )
    }
}

impl GetPydanticJsonSchemaCls for PyFsPath {
    fn get_pydantic_json_schema<'py>(
        _cls: &Bound<'py, pyo3::types::PyType>,
        core_schema: &Bound<'py, PyAny>,
        handler: &Bound<'py, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let py = core_schema.py();
        let json_schema = handler.call1((core_schema,))?;
        json_schema.set_item(interns::format(py), "path")?;
        Ok(json_schema)
    }
}
