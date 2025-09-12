use crate::{RyDate, RyDateTime, RySignedDuration, RySpan, RyTime, RyTimestamp, RyZoned};
use pyo3::prelude::PyAnyMethods;
use pyo3::types::{PyDict, PyTuple, PyType};
use pyo3::{Bound, PyAny, PyResult};
use ryo3_pydantic::{GetPydanticCoreSchemaCls, interns};

impl GetPydanticCoreSchemaCls for RyDate {
    fn get_pydantic_core_schema<'py>(
        cls: &Bound<'py, PyType>,
        source: &Bound<'py, PyAny>,
        _handler: &Bound<'py, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let py = source.py();
        let core_schema = ryo3_pydantic::core_schema(py)?;
        let date_schema = core_schema.call_method(interns::date_schema(py), (), None)?;
        let validation_fn = cls.getattr(interns::_pydantic_validate(py))?;
        let args = PyTuple::new(py, vec![&validation_fn, &date_schema])?;
        let string_serialization_schema =
            core_schema.call_method(interns::to_string_ser_schema(py), (), None)?;
        let serialization_kwargs = PyDict::new(py);
        serialization_kwargs.set_item(interns::serialization(py), &string_serialization_schema)?;
        core_schema.call_method(
            interns::no_info_wrap_validator_function(py),
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
        let core_schema = ryo3_pydantic::core_schema(py)?;
        let datetime_schema = core_schema.call_method(interns::datetime_schema(py), (), None)?;
        let validation_fn = cls.getattr(interns::_pydantic_validate(py))?;
        let args = PyTuple::new(py, vec![&validation_fn, &datetime_schema])?;
        let string_serialization_schema =
            core_schema.call_method(interns::to_string_ser_schema(py), (), None)?;
        let serialization_kwargs = PyDict::new(py);
        serialization_kwargs.set_item(interns::serialization(py), &string_serialization_schema)?;
        core_schema.call_method(
            interns::no_info_wrap_validator_function(py),
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
        let core_schema = ryo3_pydantic::core_schema(py)?;
        let time_schema = core_schema.call_method(interns::time_schema(py), (), None)?;
        let validation_fn = cls.getattr(interns::_pydantic_validate(py))?;
        let args = PyTuple::new(py, vec![&validation_fn, &time_schema])?;
        let string_serialization_schema =
            core_schema.call_method(interns::to_string_ser_schema(py), (), None)?;
        let serialization_kwargs = PyDict::new(py);
        serialization_kwargs.set_item(interns::serialization(py), &string_serialization_schema)?;
        core_schema.call_method(
            interns::no_info_wrap_validator_function(py),
            args,
            Some(&serialization_kwargs),
        )
    }
}

impl GetPydanticCoreSchemaCls for RyTimestamp {
    fn get_pydantic_core_schema<'py>(
        cls: &Bound<'py, PyType>,
        source: &Bound<'py, PyAny>,
        _handler: &Bound<'py, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let py = source.py();
        let core_schema = ryo3_pydantic::core_schema(py)?;

        // Maybe it should be a not str_schema? idk? really not sure if it should be str or datetime
        // let str_schema = core_schema.call_method(intern!(py, "str_schema"), (), None)?;
        let datetime_schema = core_schema.call_method(interns::datetime_schema(py), (), None)?;
        let validation_fn = cls.getattr(interns::_pydantic_validate(py))?;
        let args = PyTuple::new(py, vec![&validation_fn, &datetime_schema])?;
        let string_serialization_schema =
            core_schema.call_method(interns::to_string_ser_schema(py), (), None)?;
        let serialization_kwargs = PyDict::new(py);
        serialization_kwargs.set_item(interns::serialization(py), &string_serialization_schema)?;
        core_schema.call_method(
            interns::no_info_wrap_validator_function(py),
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
        let core_schema = ryo3_pydantic::core_schema(py)?;
        let datetime_schema = core_schema.call_method(interns::datetime_schema(py), (), None)?;
        let validation_fn = cls.getattr(interns::_pydantic_validate(py))?;
        let args = PyTuple::new(py, vec![&validation_fn, &datetime_schema])?;
        let string_serialization_schema =
            core_schema.call_method(interns::to_string_ser_schema(py), (), None)?;
        let serialization_kwargs = PyDict::new(py);
        serialization_kwargs.set_item(interns::serialization(py), &string_serialization_schema)?;
        core_schema.call_method(
            interns::no_info_wrap_validator_function(py),
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
        let core_schema = ryo3_pydantic::core_schema(py)?;
        let timedelta_schema = core_schema.call_method(interns::timedelta_schema(py), (), None)?;
        let validation_fn = cls.getattr(interns::_pydantic_validate(py))?;
        let args = PyTuple::new(py, vec![&validation_fn, &timedelta_schema])?;
        let string_serialization_schema =
            core_schema.call_method(interns::to_string_ser_schema(py), (), None)?;
        let serialization_kwargs = PyDict::new(py);
        serialization_kwargs.set_item(interns::serialization(py), &string_serialization_schema)?;
        core_schema.call_method(
            interns::no_info_wrap_validator_function(py),
            args,
            Some(&serialization_kwargs),
        )
    }
}
// ============================================================================
// FOR REFERENCE WITH OUT ALL THE INTERNS FLOATING AROUND THIS PRE INTERNS
// ============================================================================
// ```
// impl GetPydanticCoreSchemaCls for RySignedDuration {
//     fn get_pydantic_core_schema<'py>(
//         cls: &Bound<'py, PyType>,
//         source: &Bound<'py, PyAny>,
//         _handler: &Bound<'py, PyAny>,
//     ) -> PyResult<Bound<'py, PyAny>> {
//         let py = source.py();
//         let core_schema = ryo3_pydantic::core_schema(py)?;
//         let timedelta_schema =
//             core_schema.call_method(intern!(py, "timedelta_schema"), (), None)?;
//         let validation_fn = cls.getattr(intern!(py, "_pydantic_validate"))?;
//         let args = PyTuple::new(py, vec![&validation_fn, &timedelta_schema])?;
//         let string_serialization_schema =
//             core_schema.call_method(intern!(py, "to_string_ser_schema"), (), None)?;
//         let serialization_kwargs = PyDict::new(py);
//         serialization_kwargs
//             .set_item(intern!(py, "serialization"), &string_serialization_schema)?;
//         // serialization_kwargs.set_item(intern!(py, "when_used"), intern!(py, "json-unless-none"))?;
//         // string_serialization_schema.call_method(intern!(py, "update"), (serialization_kwargs,), None)?;
//         core_schema.call_method(
//             intern!(py, "no_info_wrap_validator_function"),
//             args,
//             Some(&serialization_kwargs),
//         )
//     }
// }
// ```

impl GetPydanticCoreSchemaCls for RySignedDuration {
    fn get_pydantic_core_schema<'py>(
        cls: &Bound<'py, PyType>,
        source: &Bound<'py, PyAny>,
        _handler: &Bound<'py, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let py = source.py();
        let core_schema = ryo3_pydantic::core_schema(py)?;
        let timedelta_schema = core_schema.call_method(interns::timedelta_schema(py), (), None)?;
        let validation_fn = cls.getattr(interns::_pydantic_validate(py))?;
        let args = PyTuple::new(py, vec![&validation_fn, &timedelta_schema])?;
        let string_serialization_schema =
            core_schema.call_method(interns::to_string_ser_schema(py), (), None)?;
        let serialization_kwargs = PyDict::new(py);
        serialization_kwargs.set_item(interns::serialization(py), &string_serialization_schema)?;
        core_schema.call_method(
            interns::no_info_wrap_validator_function(py),
            args,
            Some(&serialization_kwargs),
        )
    }
}
