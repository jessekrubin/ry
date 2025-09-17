use pyo3::prelude::*;

use pyo3::types::{PyAnyMethods, PyDict, PyType};
use pyo3::{Bound, intern};

pub(crate) fn dataclass_fields<'a, 'py>(obj: &'a Bound<'py, PyAny>) -> Option<Bound<'py, PyDict>>
where
    'py: 'a,
{
    obj.getattr(intern!(obj.py(), "__dataclass_fields__")) // PyResult<Bound<PyAny>>
        .ok()? // Option<Bound<PyAny>>
        .cast_into::<PyDict>() // PyResult<Bound<PyDict>>
        .ok() // Option<Bound<PyDict>>
}

// Modified from pydantic's `is_dataclass` function
// ty pydantic team: https://github.com/pydantic/pydantic-core/blob/e0bc980764ec5d5f59c7d451948df937b5a1921f/src/serializers/ob_type.rs#L342
pub(crate) fn is_dataclass(value: &Bound<'_, PyAny>) -> bool {
    value
        .hasattr(intern!(value.py(), "__dataclass_fields__"))
        .unwrap_or(false)
        && !value.is_instance_of::<PyType>()
}
