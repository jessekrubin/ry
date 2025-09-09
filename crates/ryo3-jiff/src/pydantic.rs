use crate::{RyDate, RyDateTime, RySignedDuration, RySpan, RyTime, RyZoned};
use pyo3::prelude::PyAnyMethods;
use pyo3::types::{PyDict, PyTuple, PyType};
use pyo3::{intern, Bound, PyAny, PyResult};
use ryo3_pydantic::GetPydanticCoreSchemaCls;

impl GetPydanticCoreSchemaCls for RyDate {
    fn get_pydantic_core_schema<'py>(
        cls: &Bound<'py, PyType>,
        source: &Bound<'py, PyAny>,
        _handler: &Bound<'py, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let py = source.py();
        let pydantic_core = py.import(intern!(py, "pydantic_core"))?;
        let core_schema = pydantic_core.getattr(intern!(py, "core_schema"))?;
        let date_schema = core_schema.call_method(intern!(py, "date_schema"), (), None)?;
        let validation_fn = cls.getattr(intern!(py, "_pydantic_parse"))?;
        let args = PyTuple::new(py, vec![&validation_fn, &date_schema])?;
        let string_serialization_schema =
            core_schema.call_method(intern!(py, "to_string_ser_schema"), (), None)?;
        let serialization_kwargs = PyDict::new(py);
        serialization_kwargs
            .set_item(intern!(py, "serialization"), &string_serialization_schema)?;
        // serialization_kwargs.set_item(intern!(py, "when_used"), intern!(py, "json-unless-none"))?;
        // string_serialization_schema.call_method(intern!(py, "update"), (serialization_kwargs,), None)?;
        core_schema.call_method(
            intern!(py, "no_info_wrap_validator_function"),
            args,
            Some(&serialization_kwargs),
        )
    }
}
impl GetPydanticCoreSchemaCls for RyDateTime {
    fn get_pydantic_core_schema<'py>(
        cls: &Bound<'py, PyType>,
        source: &Bound<'py, PyAny>,
        _handler: &Bound<'py, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let py = source.py();
        let pydantic_core = py.import(intern!(py, "pydantic_core"))?;
        let core_schema = pydantic_core.getattr(intern!(py, "core_schema"))?;
        let datetime_schema = core_schema.call_method(intern!(py, "datetime_schema"), (), None)?;
        let validation_fn = cls.getattr(intern!(py, "_pydantic_parse"))?;
        let args = PyTuple::new(py, vec![&validation_fn, &datetime_schema])?;
        let string_serialization_schema =
            core_schema.call_method(intern!(py, "to_string_ser_schema"), (), None)?;
        let serialization_kwargs = PyDict::new(py);
        serialization_kwargs
            .set_item(intern!(py, "serialization"), &string_serialization_schema)?;
        // serialization_kwargs.set_item(intern!(py, "when_used"), intern!(py, "json-unless-none"))?;
        // string_serialization_schema.call_method(intern!(py, "update"), (serialization_kwargs,), None)?;
        core_schema.call_method(
            intern!(py, "no_info_wrap_validator_function"),
            args,
            Some(&serialization_kwargs),
        )
    }
}

impl GetPydanticCoreSchemaCls for RyTime {
    fn get_pydantic_core_schema<'py>(
        cls: &Bound<'py, PyType>,
        source: &Bound<'py, PyAny>,
        _handler: &Bound<'py, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let py = source.py();
        let pydantic_core = py.import(intern!(py, "pydantic_core"))?;
        let core_schema = pydantic_core.getattr(intern!(py, "core_schema"))?;
        let datetime_schema = core_schema.call_method(intern!(py, "datetime_schema"), (), None)?;
        let validation_fn = cls.getattr(intern!(py, "_pydantic_parse"))?;
        let args = PyTuple::new(py, vec![&validation_fn, &datetime_schema])?;
        let string_serialization_schema =
            core_schema.call_method(intern!(py, "to_string_ser_schema"), (), None)?;
        let serialization_kwargs = PyDict::new(py);
        serialization_kwargs
            .set_item(intern!(py, "serialization"), &string_serialization_schema)?;
        // serialization_kwargs.set_item(intern!(py, "when_used"), intern!(py, "json-unless-none"))?;
        // string_serialization_schema.call_method(intern!(py, "update"), (serialization_kwargs,), None)?;
        core_schema.call_method(
            intern!(py, "no_info_wrap_validator_function"),
            args,
            Some(&serialization_kwargs),
        )
    }
}
impl GetPydanticCoreSchemaCls for RyZoned {
    fn get_pydantic_core_schema<'py>(
        cls: &Bound<'py, PyType>,
        source: &Bound<'py, PyAny>,
        _handler: &Bound<'py, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let py = source.py();
        let pydantic_core = py.import(intern!(py, "pydantic_core"))?;
        let core_schema = pydantic_core.getattr(intern!(py, "core_schema"))?;
        let datetime_schema = core_schema.call_method(intern!(py, "datetime_schema"), (), None)?;
        let validation_fn = cls.getattr(intern!(py, "_pydantic_parse"))?;
        let args = PyTuple::new(py, vec![&validation_fn, &datetime_schema])?;
        let string_serialization_schema =
            core_schema.call_method(intern!(py, "to_string_ser_schema"), (), None)?;
        let serialization_kwargs = PyDict::new(py);
        serialization_kwargs
            .set_item(intern!(py, "serialization"), &string_serialization_schema)?;
        // serialization_kwargs.set_item(intern!(py, "when_used"), intern!(py, "json-unless-none"))?;
        // string_serialization_schema.call_method(intern!(py, "update"), (serialization_kwargs,), None)?;
        core_schema.call_method(
            intern!(py, "no_info_wrap_validator_function"),
            args,
            Some(&serialization_kwargs),
        )
    }
}

impl GetPydanticCoreSchemaCls for RySpan {
    fn get_pydantic_core_schema<'py>(
        cls: &Bound<'py, PyType>,
        source: &Bound<'py, PyAny>,
        _handler: &Bound<'py, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let py = source.py();
        let pydantic_core = py.import(intern!(py, "pydantic_core"))?;
        let core_schema = pydantic_core.getattr(intern!(py, "core_schema"))?;
        let timedelta_schema =
            core_schema.call_method(intern!(py, "timedelta_schema"), (), None)?;
        let validation_fn = cls.getattr(intern!(py, "_pydantic_parse"))?;
        let args = PyTuple::new(py, vec![&validation_fn, &timedelta_schema])?;
        let string_serialization_schema =
            core_schema.call_method(intern!(py, "to_string_ser_schema"), (), None)?;
        let serialization_kwargs = PyDict::new(py);
        serialization_kwargs
            .set_item(intern!(py, "serialization"), &string_serialization_schema)?;
        // serialization_kwargs.set_item(intern!(py, "when_used"), intern!(py, "json-unless-none"))?;
        // string_serialization_schema.call_method(intern!(py, "update"), (serialization_kwargs,), None)?;
        core_schema.call_method(
            intern!(py, "no_info_wrap_validator_function"),
            args,
            Some(&serialization_kwargs),
        )
    }
}

impl GetPydanticCoreSchemaCls for RySignedDuration {
    fn get_pydantic_core_schema<'py>(
        cls: &Bound<'py, PyType>,
        source: &Bound<'py, PyAny>,
        _handler: &Bound<'py, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let py = source.py();
        let pydantic_core = py.import(intern!(py, "pydantic_core"))?;
        let core_schema = pydantic_core.getattr(intern!(py, "core_schema"))?;
        let timedelta_schema =
            core_schema.call_method(intern!(py, "timedelta_schema"), (), None)?;
        let validation_fn = cls.getattr(intern!(py, "_pydantic_parse"))?;
        let args = PyTuple::new(py, vec![&validation_fn, &timedelta_schema])?;
        let string_serialization_schema =
            core_schema.call_method(intern!(py, "to_string_ser_schema"), (), None)?;
        let serialization_kwargs = PyDict::new(py);
        serialization_kwargs
            .set_item(intern!(py, "serialization"), &string_serialization_schema)?;
        // serialization_kwargs.set_item(intern!(py, "when_used"), intern!(py, "json-unless-none"))?;
        // string_serialization_schema.call_method(intern!(py, "update"), (serialization_kwargs,), None)?;
        core_schema.call_method(
            intern!(py, "no_info_wrap_validator_function"),
            args,
            Some(&serialization_kwargs),
        )
    }
}
