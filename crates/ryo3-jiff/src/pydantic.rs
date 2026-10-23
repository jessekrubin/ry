use pyo3::prelude::PyAnyMethods;
use pyo3::types::{PyDict, PyType};
use pyo3::{Bound, PyAny, PyResult};
use ryo3_pydantic::{GetPydanticCoreSchemaCls, GetPydanticJsonSchemaCls, interns};

use crate::{
    RyDate, RyDateTime, RyISOWeekDate, RyOffset, RySignedDuration, RySpan, RyTime, RyTimeZone,
    RyTimestamp, RyZoned,
};

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
        let string_serialization_schema =
            core_schema.call_method(interns::to_string_ser_schema(py), (), None)?;
        let plain_validator_kwargs = PyDict::new(py);
        plain_validator_kwargs.set_item(interns::json_schema_input_schema(py), &date_schema)?;
        plain_validator_kwargs
            .set_item(interns::serialization(py), &string_serialization_schema)?;
        core_schema.call_method(
            interns::no_info_plain_validator_function(py),
            (&validation_fn,),
            Some(&plain_validator_kwargs),
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

impl GetPydanticCoreSchemaCls for RyTime {
    fn get_pydantic_core_schema<'py>(
        cls: &Bound<'py, PyType>,
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

impl GetPydanticCoreSchemaCls for RyTimestamp {
    fn get_pydantic_core_schema<'py>(
        cls: &Bound<'py, PyType>,
        source: &Bound<'py, PyAny>,
        _handler: &Bound<'py, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let py = source.py();
        let core_schema = ryo3_pydantic::core_schema(py)?;

        // Maybe it should be a not str_schema? idk? really not sure if it should be str
        // or datetime
        let datetime_schema = core_schema.call_method(interns::datetime_schema(py), (), None)?;
        let validation_fn = cls.getattr(interns::_pydantic_validate(py))?;
        let string_serialization_schema =
            core_schema.call_method(interns::to_string_ser_schema(py), (), None)?;
        let plain_validator_kwargs = PyDict::new(py);
        plain_validator_kwargs.set_item(interns::json_schema_input_schema(py), &datetime_schema)?;
        plain_validator_kwargs
            .set_item(interns::serialization(py), &string_serialization_schema)?;
        core_schema.call_method(
            interns::no_info_plain_validator_function(py),
            (&validation_fn,),
            Some(&plain_validator_kwargs),
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
        let string_serialization_schema =
            core_schema.call_method(interns::to_string_ser_schema(py), (), None)?;
        let plain_validator_kwargs = PyDict::new(py);
        plain_validator_kwargs
            .set_item(interns::json_schema_input_schema(py), &timedelta_schema)?;
        plain_validator_kwargs
            .set_item(interns::serialization(py), &string_serialization_schema)?;
        core_schema.call_method(
            interns::no_info_plain_validator_function(py),
            (&validation_fn,),
            Some(&plain_validator_kwargs),
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
//             core_schema.call_method(pyo3::intern!(py, "timedelta_schema"), (), None)?;
//         let validation_fn = cls.getattr(pyo3::intern!(py, "_pydantic_validate"))?;
//         let args = PyTuple::new(py, vec![&validation_fn, &timedelta_schema])?;
//         let string_serialization_schema =
//             core_schema.call_method(pyo3::intern!(py, "to_string_ser_schema"), (), None)?;
//         let serialization_kwargs = PyDict::new(py);
//         serialization_kwargs
//             .set_item(pyo3::intern!(py, "serialization"), &string_serialization_schema)?;
//         // serialization_kwargs.set_item(pyo3::intern!(py, "when_used"), pyo3::intern!(py, "json-unless-none"))?;
//         // string_serialization_schema.call_method(pyo3::intern!(py, "update"), (serialization_kwargs,), None)?;
//         core_schema.call_method(
//             pyo3::intern!(py, "no_info_wrap_validator_function"),
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
        let string_serialization_schema =
            core_schema.call_method(interns::to_string_ser_schema(py), (), None)?;
        let plain_validator_kwargs = PyDict::new(py);
        plain_validator_kwargs
            .set_item(interns::json_schema_input_schema(py), &timedelta_schema)?;
        plain_validator_kwargs
            .set_item(interns::serialization(py), &string_serialization_schema)?;
        core_schema.call_method(
            interns::no_info_plain_validator_function(py),
            (&validation_fn,),
            Some(&plain_validator_kwargs),
        )
    }
}

impl GetPydanticCoreSchemaCls for RyISOWeekDate {
    fn get_pydantic_core_schema<'py>(
        cls: &pyo3::Bound<'py, pyo3::types::PyType>,
        source: &pyo3::Bound<'py, pyo3::PyAny>,
        _handler: &pyo3::Bound<'py, pyo3::PyAny>,
    ) -> pyo3::PyResult<pyo3::Bound<'py, pyo3::PyAny>> {
        let py = source.py();
        let core_schema = ryo3_pydantic::core_schema(py)?;
        let schema = core_schema.call_method(interns::str_schema(py), (), None)?;
        let validation_fn = cls.getattr(interns::_pydantic_validate(py))?;
        let string_serialization_schema =
            core_schema.call_method(interns::to_string_ser_schema(py), (), None)?;
        let plain_validator_kwargs = PyDict::new(py);
        plain_validator_kwargs.set_item("json_schema_input_schema", &schema)?;
        plain_validator_kwargs
            .set_item(interns::serialization(py), &string_serialization_schema)?;
        core_schema.call_method(
            interns::no_info_plain_validator_function(py),
            (&validation_fn,),
            Some(&plain_validator_kwargs),
        )
    }
}

impl GetPydanticCoreSchemaCls for RyOffset {
    fn get_pydantic_core_schema<'py>(
        cls: &pyo3::Bound<'py, pyo3::types::PyType>,
        source: &pyo3::Bound<'py, pyo3::PyAny>,
        _handler: &pyo3::Bound<'py, pyo3::PyAny>,
    ) -> pyo3::PyResult<pyo3::Bound<'py, pyo3::PyAny>> {
        let py = source.py();
        let core_schema = ryo3_pydantic::core_schema(py)?;
        let schema = core_schema.call_method(interns::str_schema(py), (), None)?;
        let validation_fn = cls.getattr(interns::_pydantic_validate(py))?;
        let string_serialization_schema =
            core_schema.call_method(interns::to_string_ser_schema(py), (), None)?;
        let plain_validator_kwargs = PyDict::new(py);
        plain_validator_kwargs.set_item(interns::json_schema_input_schema(py), &schema)?;
        plain_validator_kwargs
            .set_item(interns::serialization(py), &string_serialization_schema)?;
        core_schema.call_method(
            interns::no_info_plain_validator_function(py),
            (&validation_fn,),
            Some(&plain_validator_kwargs),
        )
    }
}

impl GetPydanticCoreSchemaCls for RyTimeZone {
    fn get_pydantic_core_schema<'py>(
        cls: &pyo3::Bound<'py, pyo3::types::PyType>,
        source: &pyo3::Bound<'py, pyo3::PyAny>,
        _handler: &pyo3::Bound<'py, pyo3::PyAny>,
    ) -> pyo3::PyResult<pyo3::Bound<'py, pyo3::PyAny>> {
        let py = source.py();
        let core_schema = ryo3_pydantic::core_schema(py)?;
        let schema = core_schema.call_method(interns::str_schema(py), (), None)?;
        let validation_fn = cls.getattr(interns::_pydantic_validate(py))?;
        let string_serialization_schema =
            core_schema.call_method(interns::to_string_ser_schema(py), (), None)?;
        let plain_validator_kwargs = PyDict::new(py);
        plain_validator_kwargs.set_item(interns::json_schema_input_schema(py), &schema)?;
        plain_validator_kwargs
            .set_item(interns::serialization(py), &string_serialization_schema)?;
        core_schema.call_method(
            interns::no_info_plain_validator_function(py),
            (&validation_fn,),
            Some(&plain_validator_kwargs),
        )
    }
}

// ---------------------------------------------------------------------------
// JSON schema overrides
//
// JSON Schema's `date-time`/`time` require an offset (RFC 3339) so the
// civil + zoned types can't use them (i (jesse) was getting ajv errs)
//
// REF(s):
// - <https://spec.openapis.org/registry/format/> (`date-time-local`/`time-local`)
// - <https://www.rfc-editor.org/info/rfc9557/> (zoned `[America/New_York]` suffix)
// ---------------------------------------------------------------------------

impl GetPydanticJsonSchemaCls for RyDateTime {
    fn get_pydantic_json_schema<'py>(
        _cls: &Bound<'py, PyType>,
        core_schema: &Bound<'py, PyAny>,
        handler: &Bound<'py, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let py = core_schema.py();
        let json_schema = handler.call1((core_schema,))?;
        json_schema.set_item(interns::format(py), "date-time-local")?;
        Ok(json_schema)
    }
}

impl GetPydanticJsonSchemaCls for RyTime {
    fn get_pydantic_json_schema<'py>(
        _cls: &Bound<'py, PyType>,
        core_schema: &Bound<'py, PyAny>,
        handler: &Bound<'py, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let py = core_schema.py();
        let json_schema = handler.call1((core_schema,))?;
        json_schema.set_item(interns::format(py), "time-local")?;
        Ok(json_schema)
    }
}

impl GetPydanticJsonSchemaCls for RyZoned {
    fn get_pydantic_json_schema<'py>(
        _cls: &Bound<'py, PyType>,
        core_schema: &Bound<'py, PyAny>,
        handler: &Bound<'py, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        // no released json-schema format for RFC 9557 (offset + `[tz]`) yet
        let json_schema = handler.call1((core_schema,))?;
        json_schema.set_item(
            "description",
            "RFC 9557 zoned date-time (RFC 3339 w/ offset + IANA time zone suffix)",
        )?;
        json_schema.set_item("examples", ["2024-01-15T09:30:00-05:00[America/New_York]"])?;
        Ok(json_schema)
    }
}
