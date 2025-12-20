//! Macros for creating python errors
//!
//! py_*_error! creates a `PyErr`
//!
//! py_*_err! creates a `Result<_, PyErr>`

#[macro_export]
macro_rules! py_io_error {
    () => {
        ::pyo3::exceptions::PyIOError::new_err("io error")
    };
    ($($arg:tt)+) => {
        ::pyo3::exceptions::PyIOError::new_err(::std::format!($($arg)+))
    };
}

#[macro_export]
macro_rules! py_io_err {
    () => {
        ::std::result::Result::Err(::pyo3::exceptions::PyIOError::new_err("io error"))
    };
    ($($arg:tt)+) => {
        ::std::result::Result::Err(::pyo3::exceptions::PyIOError::new_err(::std::format!($($arg)+)))
    };
}

#[macro_export]
macro_rules! py_key_error {
    () => {
        ::pyo3::exceptions::PyKeyError::new_err("key error")
    };
    ($($arg:tt)+) => {
        ::pyo3::exceptions::PyKeyError::new_err(::std::format!($($arg)+))
    };
}

#[macro_export]
macro_rules! py_key_err {
    () => {
        ::std::result::Result::Err(::pyo3::exceptions::PyKeyError::new_err("key error"))
    };
    ($($arg:tt)+) => {
        ::std::result::Result::Err(::pyo3::exceptions::PyKeyError::new_err(::std::format!($($arg)+)))
    };
}

// NotImplementedError
#[macro_export]
macro_rules! py_not_implemented_error {
    () => {
        ::pyo3::exceptions::PyNotImplementedError::new_err("not implemented")
    };
    ($($arg:tt)+) => {
        ::pyo3::exceptions::PyNotImplementedError::new_err(::std::format!($($arg)+))
    };
}

#[macro_export]
macro_rules! py_not_implemented_err {
    () => {
        ::std::result::Result::Err(::pyo3::exceptions::PyNotImplementedError::new_err("not implemented"))
    };
    ($($arg:tt)+) => {
        ::std::result::Result::Err(::pyo3::exceptions::PyNotImplementedError::new_err(::std::format!($($arg)+)))
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
macro_rules! py_runtime_error {
    () => {
        ::pyo3::exceptions::PyRuntimeError::new_err("runtime error")
    };
    ($($arg:tt)+) => {
        ::pyo3::exceptions::PyRuntimeError::new_err(::std::format!($($arg)+))
    };
}

#[macro_export]
macro_rules! py_runtime_err {
    () => {
        ::std::result::Result::Err(::pyo3::exceptions::PyRuntimeError::new_err("runtime error"))
    };
    ($($arg:tt)+) => {
        ::std::result::Result::Err(::pyo3::exceptions::PyRuntimeError::new_err(::std::format!($($arg)+)))
    };
}

#[macro_export]
macro_rules! py_stop_async_iteration_err {
    () => {
        ::std::result::Result::Err(::pyo3::exceptions::PyStopAsyncIteration::new_err("stop async iteration"))
    };
    ($($arg:tt)+) => {
        ::std::result::Result::Err(::pyo3::exceptions::PyStopAsyncIteration::new_err(::std::format!($($arg)+)))
    };
}

#[macro_export]
macro_rules! py_stop_async_iteration_erorr {
    () => {
        ::pyo3::exceptions::PyStopAsyncIteration::new_err("stop async iteration")
    };
    ($($arg:tt)+) => {
        ::pyo3::exceptions::PyStopAsyncIteration::new_err(::std::format!($($arg)+))
    };
}

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
