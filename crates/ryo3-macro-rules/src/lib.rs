mod not_implemented;
#[macro_use]
mod py_errs;

#[macro_export]
macro_rules! any_repr {
    ($obj:expr) => {{
        let typ = $obj.get_type();
        let name = typ
            .fully_qualified_name()
            .unwrap_or_else(|_| pyo3::types::PyString::new($obj.py(), "unknown"));
        match $obj.repr() {
            Ok(repr) => format!("{repr} ({name})"),
            Err(_) => name.to_string(),
        }
    }};
}

/// Macro that mimics ye-old rust `todo!()` macro producing a `PyNotImplementedError`
/// as opposed to panic-ing (at the disco).
#[macro_export]
macro_rules! pytodo_pyerr {
    () => {
        ::pyo3::exceptions::PyNotImplementedError::new_err("not yet implemented")
    };
    ($($arg:tt)+) => {
        ::pyo3::exceptions::PyNotImplementedError::new_err(
            ::std::format!($($arg)+)
        )
    };
}

/// Macro that mimics ye-old rust `todo!()` macro producing a `PyNotImplementedError`
/// wrapped in `Err(...)`
#[macro_export]
macro_rules! pytodo_err {
    () => {
        ::core::result::Result::Err(
            ::pyo3::exceptions::PyNotImplementedError::new_err("not yet implemented")
        )
    };
    ($($arg:tt)+) => {
        ::core::result::Result::Err(
            ::pyo3::exceptions::PyNotImplementedError::new_err(::std::format!($($arg)+))
        )
    };
}

/// Macro that returns a `PyResult::Err` with a `PyNotImplementedError`
///
/// GOTTA use this in functions that return `PyResult<T>`
#[macro_export]
macro_rules! pytodo {
    () => {
        return $crate::pytodo_err!();
    };
    ($($arg:tt)+) => {
        return $crate::pytodo_err!($($arg)+);
    };
}
