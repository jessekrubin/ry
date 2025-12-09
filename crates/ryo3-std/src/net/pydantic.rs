use super::{PySocketAddr, PySocketAddrV4, PySocketAddrV6};
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyTuple};
use ryo3_pydantic::{GetPydanticCoreSchemaCls, interns};

macro_rules! impl_get_pydantic_core_schema_cls_for_type {
    ($rytype:ty) => {
        impl GetPydanticCoreSchemaCls for $rytype {
            fn get_pydantic_core_schema<'py>(
                cls: &pyo3::Bound<'py, pyo3::types::PyType>,
                source: &pyo3::Bound<'py, pyo3::PyAny>,
                _handler: &pyo3::Bound<'py, pyo3::PyAny>,
            ) -> pyo3::PyResult<pyo3::Bound<'py, pyo3::PyAny>> {
                let py = source.py();
                let core_schema = ryo3_pydantic::core_schema(py)?;
                let schema = core_schema.call_method(interns::str_schema(py), (), None)?;
                let validation_fn = cls.getattr(interns::_pydantic_validate(py))?;
                let args = PyTuple::new(py, vec![&validation_fn, &schema])?;
                let string_serialization_schema =
                    core_schema.call_method(interns::to_string_ser_schema(py), (), None)?;
                let serialization_kwargs = PyDict::new(py);
                serialization_kwargs
                    .set_item(interns::serialization(py), &string_serialization_schema)?;
                core_schema.call_method(
                    interns::no_info_wrap_validator_function(py),
                    args,
                    Some(&serialization_kwargs),
                )
            }
        }
    };
}

impl_get_pydantic_core_schema_cls_for_type!(PySocketAddrV4);
impl_get_pydantic_core_schema_cls_for_type!(PySocketAddrV6);
impl_get_pydantic_core_schema_cls_for_type!(PySocketAddr);
