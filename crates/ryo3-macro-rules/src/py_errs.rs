//! Macros for creating python errors
//!
//! py_*_error! creates a `PyErr`
//!
//! py_*_err! creates a `Result<_, PyErr>`
//!
//! # NOTES
//!
//! - use `format_args!` (2026-05-15): switching to use `std::format_args!`
//!   where i was using `format!()` allows checking if the thing is a simple
//!   string literal and avoid having to sploop out a new string (allocating)
//!   and instead just use the string literal directly.
//!

#[macro_export]
macro_rules! py_io_error {
    () => {
        ::pyo3::exceptions::PyIOError::new_err("io error")
    };
    ($msg:ident $(,)?) => {
        ::pyo3::exceptions::PyIOError::new_err($msg)
    };
    ($($arg:tt)+) => {
        {
            let args = ::std::format_args!($($arg)+);
            match args.as_str() {
                Some(s) => ::pyo3::exceptions::PyIOError::new_err(s),
                None => ::pyo3::exceptions::PyIOError::new_err(args.to_string()),
            }
        }
    };
}

#[macro_export]
macro_rules! py_io_err {
    () => {
        ::std::result::Result::Err(::pyo3::exceptions::PyIOError::new_err("io error"))
    };
    ($msg:ident $(,)?) => {
        ::std::result::Result::Err(::pyo3::exceptions::PyIOError::new_err($msg))
    };
    ($($arg:tt)+) => {
        {
            let args = ::std::format_args!($($arg)+);
            ::std::result::Result::Err(match args.as_str() {
                Some(s) => ::pyo3::exceptions::PyIOError::new_err(s),
                None => ::pyo3::exceptions::PyIOError::new_err(args.to_string()),
            })
        }
    };
}

/// Macro that returns a `pyo3::exceptions::PyKeyError`
///
/// # EXAMPLE
/// ```
/// use pyo3::prelude::*;
/// use ryo3_macro_rules::py_key_error;
///
/// fn example(py: Python<'_>) -> PyResult<()> {
///     Err(py_key_error!("this is a key error"))
/// }
/// ```
#[macro_export]
macro_rules! py_key_error {
    () => {
        ::pyo3::exceptions::PyKeyError::new_err("key error")
    };
    ($msg:ident $(,)?) => {
        ::pyo3::exceptions::PyKeyError::new_err($msg)
    };
    ($($arg:tt)+) => {
        {
            let args = ::std::format_args!($($arg)+);
            match args.as_str() {
                Some(s) => ::pyo3::exceptions::PyKeyError::new_err(s),
                None => ::pyo3::exceptions::PyKeyError::new_err(args.to_string()),
            }
        }
    };
}

#[macro_export]
macro_rules! py_key_err {
    () => {
        ::std::result::Result::Err(::pyo3::exceptions::PyKeyError::new_err("key error"))
    };
    ($msg:ident $(,)?) => {
        ::std::result::Result::Err(::pyo3::exceptions::PyKeyError::new_err($msg))
    };
    ($($arg:tt)+) => {
        {
            let args = ::std::format_args!($($arg)+);
            ::std::result::Result::Err(match args.as_str() {
                Some(s) => ::pyo3::exceptions::PyKeyError::new_err(s),
                None => ::pyo3::exceptions::PyKeyError::new_err(args.to_string()),
            })
        }
    };
}

// NotImplementedError
#[macro_export]
macro_rules! py_not_implemented_error {
    () => {
        ::pyo3::exceptions::PyNotImplementedError::new_err("not implemented")
    };
    ($msg:ident $(,)?) => {
        ::pyo3::exceptions::PyNotImplementedError::new_err($msg)
    };
    ($($arg:tt)+) => {
        {
            let args = ::std::format_args!($($arg)+);
            match args.as_str() {
                Some(s) => ::pyo3::exceptions::PyNotImplementedError::new_err(s),
                None => ::pyo3::exceptions::PyNotImplementedError::new_err(args.to_string()),
            }
        }
    };
}

#[macro_export]
macro_rules! py_not_implemented_err {
    () => {
        ::std::result::Result::Err(::pyo3::exceptions::PyNotImplementedError::new_err("not implemented"))
    };
    ($msg:ident $(,)?) => {
        ::std::result::Result::Err(::pyo3::exceptions::PyNotImplementedError::new_err($msg))
    };
    ($($arg:tt)+) => {
        {
            let args = ::std::format_args!($($arg)+);
            ::std::result::Result::Err(match args.as_str() {
                Some(s) => ::pyo3::exceptions::PyNotImplementedError::new_err(s),
                None => ::pyo3::exceptions::PyNotImplementedError::new_err(args.to_string()),
            })
        }
    };
}

#[macro_export]
macro_rules! py_overflow_error {
    () => {
        ::pyo3::exceptions::PyOverflowError::new_err("overflow error")
    };
    ($msg:ident $(,)?) => {
        ::pyo3::exceptions::PyOverflowError::new_err($msg)
    };
    ($($arg:tt)+) => {
        {
            let args = ::std::format_args!($($arg)+);
            match args.as_str() {
                Some(s) => ::pyo3::exceptions::PyOverflowError::new_err(s),
                None => ::pyo3::exceptions::PyOverflowError::new_err(args.to_string()),
            }
        }
    };
}

#[macro_export]
macro_rules! py_overflow_err {
    () => {
        ::std::result::Result::Err(::pyo3::exceptions::PyOverflowError::new_err("overflow error"))
    };
    ($msg:ident $(,)?) => {
        ::std::result::Result::Err(::pyo3::exceptions::PyOverflowError::new_err($msg))
    };
    ($($arg:tt)+) => {
        {
            let args = ::std::format_args!($($arg)+);
            ::std::result::Result::Err(match args.as_str() {
                Some(s) => ::pyo3::exceptions::PyOverflowError::new_err(s),
                None => ::pyo3::exceptions::PyOverflowError::new_err(args.to_string()),
            })
        }
    };
}

#[macro_export]
macro_rules! py_runtime_error {
    () => {
        ::pyo3::exceptions::PyRuntimeError::new_err("runtime error")
    };
    ($msg:ident $(,)?) => {
        ::pyo3::exceptions::PyRuntimeError::new_err($msg)
    };
    ($($arg:tt)+) => {
        {
            let args = ::std::format_args!($($arg)+);
            match args.as_str() {
                Some(s) => ::pyo3::exceptions::PyRuntimeError::new_err(s),
                None => ::pyo3::exceptions::PyRuntimeError::new_err(args.to_string()),
            }
        }
    };
}

#[macro_export]
macro_rules! py_runtime_err {
    () => {
        ::std::result::Result::Err(::pyo3::exceptions::PyRuntimeError::new_err("runtime error"))
    };
    ($msg:ident $(,)?) => {
        ::std::result::Result::Err(::pyo3::exceptions::PyRuntimeError::new_err($msg))
    };
    ($($arg:tt)+) => {
        {
            let args = ::std::format_args!($($arg)+);
            ::std::result::Result::Err(match args.as_str() {
                Some(s) => ::pyo3::exceptions::PyRuntimeError::new_err(s),
                None => ::pyo3::exceptions::PyRuntimeError::new_err(args.to_string()),
            })
        }
    };
}

#[macro_export]
macro_rules! py_stop_async_iteration_err {
    () => {
        ::std::result::Result::Err(::pyo3::exceptions::PyStopAsyncIteration::new_err("stop async iteration"))
    };
    ($msg:ident $(,)?) => {
        ::std::result::Result::Err(::pyo3::exceptions::PyStopAsyncIteration::new_err($msg))
    };
    ($($arg:tt)+) => {
        {
            let args = ::std::format_args!($($arg)+);
            ::std::result::Result::Err(match args.as_str() {
                Some(s) => ::pyo3::exceptions::PyStopAsyncIteration::new_err(s),
                None => ::pyo3::exceptions::PyStopAsyncIteration::new_err(args.to_string()),
            })
        }
    };
}

#[macro_export]
macro_rules! py_stop_async_iteration_error {
    () => {
        ::pyo3::exceptions::PyStopAsyncIteration::new_err("stop async iteration")
    };
    ($msg:ident $(,)?) => {
        ::pyo3::exceptions::PyStopAsyncIteration::new_err($msg)
    };
    ($($arg:tt)+) => {
        {
            let args = ::std::format_args!($($arg)+);
            match args.as_str() {
                Some(s) => ::pyo3::exceptions::PyStopAsyncIteration::new_err(s),
                None => ::pyo3::exceptions::PyStopAsyncIteration::new_err(args.to_string()),
            }
        }
    };
}

#[macro_export]
macro_rules! py_type_error {
    () => {
        ::pyo3::exceptions::PyTypeError::new_err("type error")
    };
    ($msg:ident $(,)?) => {
        ::pyo3::exceptions::PyTypeError::new_err($msg)
    };
    ($($arg:tt)+) => {
        {
            let args = ::std::format_args!($($arg)+);
            match args.as_str() {
                Some(s) => ::pyo3::exceptions::PyTypeError::new_err(s),
                None => ::pyo3::exceptions::PyTypeError::new_err(args.to_string()),
            }
        }
    };
}

#[macro_export]
macro_rules! py_type_err {
    () => {
        ::std::result::Result::Err(::pyo3::exceptions::PyTypeError::new_err("type error"))
    };
    ($msg:ident $(,)?) => {
        ::std::result::Result::Err(::pyo3::exceptions::PyTypeError::new_err($msg))
    };
    ($($arg:tt)+) => {
        {
            let args = ::std::format_args!($($arg)+);
            ::std::result::Result::Err(match args.as_str() {
                Some(s) => ::pyo3::exceptions::PyTypeError::new_err(s),
                None => ::pyo3::exceptions::PyTypeError::new_err(args.to_string()),
            })
        }
    };
}

#[macro_export]
macro_rules! py_value_error {
    () => {
        ::pyo3::exceptions::PyValueError::new_err("value error")
    };
    ($msg:ident $(,)?) => {
        ::pyo3::exceptions::PyValueError::new_err($msg)
    };
    ($($arg:tt)+) => {
        {
            let args = ::std::format_args!($($arg)+);
            match args.as_str() {
                Some(s) => ::pyo3::exceptions::PyValueError::new_err(s),
                None => ::pyo3::exceptions::PyValueError::new_err(args.to_string()),
            }
        }
    };
}

#[macro_export]
macro_rules! py_value_err {
    () => {
        ::std::result::Result::Err(::pyo3::exceptions::PyValueError::new_err("value error"))
    };
    ($msg:ident $(,)?) => {
        ::std::result::Result::Err(::pyo3::exceptions::PyValueError::new_err($msg))
    };
    ($($arg:tt)+) => {
        {
            let args = ::std::format_args!($($arg)+);
            ::std::result::Result::Err(match args.as_str() {
                Some(s) => ::pyo3::exceptions::PyValueError::new_err(s),
                None => ::pyo3::exceptions::PyValueError::new_err(args.to_string()),
            })
        }
    };
}

#[macro_export]
macro_rules! py_zero_division_error {
    () => {
        ::pyo3::exceptions::PyZeroDivisionError::new_err("division by zero")
    };
    ($msg:ident $(,)?) => {
        ::pyo3::exceptions::PyZeroDivisionError::new_err($msg)
    };
    ($($arg:tt)+) => {
        {
            let args = ::std::format_args!($($arg)+);
            match args.as_str() {
                Some(s) => ::pyo3::exceptions::PyZeroDivisionError::new_err(s),
                None => ::pyo3::exceptions::PyZeroDivisionError::new_err(args.to_string()),
            }
        }
    };
}

#[macro_export]
macro_rules! py_zero_division_err {
    () => {
        ::std::result::Result::Err(::pyo3::exceptions::PyZeroDivisionError::new_err("division by zero"))
    };
    ($msg:ident $(,)?) => {
        ::std::result::Result::Err(::pyo3::exceptions::PyZeroDivisionError::new_err($msg))
    };
    ($($arg:tt)+) => {
        {
            let args = ::std::format_args!($($arg)+);
            ::std::result::Result::Err(match args.as_str() {
                Some(s) => ::pyo3::exceptions::PyZeroDivisionError::new_err(s),
                None => ::pyo3::exceptions::PyZeroDivisionError::new_err(args.to_string()),
            })
        }
    };
}
