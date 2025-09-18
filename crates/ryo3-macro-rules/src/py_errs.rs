//! Macros for creating python errors
//!
//! py_*_error! creates a `PyErr`
//!
//! py_*_err! creates a `Result<_, PyErr>`

#[macro_export]
macro_rules! py_type_error {
    () => {
        ::pyo3::exceptions::PyTypeError::new_err("type error")
    };
    ($($arg:tt)+) => {
        ::pyo3::exceptions::PyTypeError::new_err(::std::format!($($arg)+))
    };
}

#[macro_export]
macro_rules! py_type_err {
    () => {
        ::std::result::Result::Err(::pyo3::exceptions::PyTypeError::new_err("type error"))
    };
    ($($arg:tt)+) => {
        ::std::result::Result::Err(::pyo3::exceptions::PyTypeError::new_err(::std::format!($($arg)+)))
    };
}

#[macro_export]
macro_rules! py_overflow_error {
    () => {
        ::pyo3::exceptions::PyOverflowError::new_err("overflow error")
    };
    ($($arg:tt)+) => {
        ::pyo3::exceptions::PyOverflowError::new_err(::std::format!($($arg)+))
    };
}

#[macro_export]
macro_rules! py_overflow_err {
    () => {
        ::std::result::Result::Err(::pyo3::exceptions::PyOverflowError::new_err("overflow error"))
    };
    ($($arg:tt)+) => {
        ::std::result::Result::Err(::pyo3::exceptions::PyOverflowError::new_err(::std::format!($($arg)+)))
    };
}

#[macro_export]
macro_rules! py_value_error {
    () => {
        ::pyo3::exceptions::PyValueError::new_err("value error")
    };
    ($($arg:tt)+) => {
        ::pyo3::exceptions::PyValueError::new_err(::std::format!($($arg)+))
    };
}

#[macro_export]
macro_rules! py_value_err {
    () => {
        ::std::result::Result::Err(::pyo3::exceptions::PyValueError::new_err("value error"))
    };
    ($($arg:tt)+) => {
        ::std::result::Result::Err(::pyo3::exceptions::PyValueError::new_err(::std::format!($($arg)+)))
    };
}

#[macro_export]
macro_rules! py_zero_division_error {
    () => {
        ::pyo3::exceptions::PyZeroDivisionError::new_err("division by zero")
    };
    ($($arg:tt)+) => {
        ::pyo3::exceptions::PyZeroDivisionError::new_err(::std::format!($($arg)+))
    };
}

#[macro_export]
macro_rules! py_zero_division_err {
    () => {
        ::std::result::Result::Err(::pyo3::exceptions::PyZeroDivisionError::new_err("division by zero"))
    };
    ($($arg:tt)+) => {
        ::std::result::Result::Err(::pyo3::exceptions::PyZeroDivisionError::new_err(::std::format!($($arg)+)))
    };
}
