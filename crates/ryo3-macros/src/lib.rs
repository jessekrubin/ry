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
