use pyo3::{create_exception, exceptions::PyRuntimeError};

create_exception!(
    ry.ryo3,
    FeatureNotEnabledError,
    PyRuntimeError,
    "Error indicating a feature was not enabled in the build"
);
