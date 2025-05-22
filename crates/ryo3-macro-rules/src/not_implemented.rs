#[macro_export]
macro_rules! err_py_not_impl {
    () => {
        Err(::pyo3::exceptions::PyNotImplementedError::new_err(
            "Not implemented",
        ))
    };
    ($msg:expr) => {
        Err(::pyo3::exceptions::PyNotImplementedError::new_err($msg))
    };
}

/// Return a `PyResult` with a `PyNotImplementedError` with the message "Not implemented (yet)".
///
/// Marker for functions that are not implemented yet.
#[macro_export]
macro_rules! err_py_not_impl_yet {
    () => {
        Err(::pyo3::exceptions::PyNotImplementedError::new_err(
            "Not implemented (yet)",
        ))
    };
}
