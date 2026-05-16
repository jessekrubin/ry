#[macro_use]
mod py_errs;

#[macro_export]
macro_rules! any_repr {
    ($obj:expr) => {{
        let typ = $obj.get_type();
        let name = pyo3::types::PyTypeMethods::fully_qualified_name(&typ)
            .unwrap_or_else(|_| pyo3::types::PyString::new($obj.py(), "unknown"));
        match $obj.repr() {
            Ok(repr) => format!("{repr} ({name})"),
            Err(_) => name.to_string(),
        }
    }};
}

/// Macro that mimics ye-old rust `todo!()` macro producing a
/// `PyNotImplementedError` as opposed to panic-ing (at the disco).
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

/// Macro that mimics ye-old rust `todo!()` macro producing a
/// `PyNotImplementedError` wrapped in `Err(...)`
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

/// Macro to define a function that returns an interned Python string.
#[macro_export]
macro_rules! py_intern_fn {
    ($name:ident, $lit:literal) => {
        #[doc = concat!("Intern for the string `", $lit, "`; `pyo3::intern!(py, \"", $lit, "\")`")]
        pub(crate) fn $name(py: Python<'_>) -> &Bound<'_, pyo3::types::PyString> {
            pyo3::intern!(py, $lit)
        }
    };

    ($name:ident) => {
        #[doc = concat!("Intern for the string `", stringify!($name), "`; `pyo3::intern!(py, \"", stringify!($name), "\")`")]
        pub(crate) fn $name(py: Python<'_>) -> &Bound<'_, pyo3::types::PyString> {
            pyo3::intern!(py, stringify!($name))
        }
    };
}

#[cfg(test)]
mod tests {
    use pyo3::exceptions::{PyRuntimeError, PyValueError};
    use pyo3::{PyErr, Python};

    const CONST_MSG: &str = "const message";
    macro_rules! py_errstr {
        ($py:expr, $err:expr) => {
            $err.value($py).to_string()
        };
    }

    fn with_python(f: impl for<'py> FnOnce(Python<'py>)) {
        Python::initialize();
        Python::attach(f);
    }

    #[test]
    fn py_value_error_accepts_const_path() {
        with_python(|py| {
            let err = py_value_error!(CONST_MSG);
            assert!(err.is_instance_of::<PyValueError>(py));
            assert_eq!(py_errstr!(py, err), CONST_MSG);
        });
    }

    #[test]
    fn py_value_err_accepts_ident() {
        with_python(|py| {
            let msg = "local message";
            let result: Result<(), PyErr> = py_value_err!(msg);
            let err = result.unwrap_err();
            assert!(err.is_instance_of::<PyValueError>(py));
            assert_eq!(py_errstr!(py, err), msg);
        });
    }

    #[test]
    fn py_runtime_error_still_formats_string_literals() {
        with_python(|py| {
            let suffix = "details";
            let err = py_runtime_error!("runtime: {suffix}");
            assert!(err.is_instance_of::<PyRuntimeError>(py));
            assert_eq!(py_errstr!(py, err), "runtime: details");
        });
    }

    #[test]
    fn py_value_error_still_formats_single_interpolation_literal() {
        with_python(|py| {
            let msg = "interpolated";
            let err = py_value_error!("{msg}");
            assert!(err.is_instance_of::<PyValueError>(py));
            assert_eq!(py_errstr!(py, err), msg);
        });
    }
}
