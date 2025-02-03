/// Return a `PyValueError` with formatted string
#[macro_export]
macro_rules! py_value_error {
    ($($arg:tt)*) => {
        ::pyo3::PyErr::new::<pyo3::exceptions::PyValueError, _>( format!($($arg)*) )
    };
}
