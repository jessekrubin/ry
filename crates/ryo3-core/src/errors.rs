use pyo3::create_exception;
use pyo3::exceptions::{PyAssertionError, PyBaseException, PyRuntimeError};
use pyo3::prelude::*;
use pyo3::pybacked::PyBackedStr;

create_exception!(
    ry.ryo3,
    PanicError,
    PyBaseException,
    "panic == fatal python error"
);

create_exception!(
    ry.ryo3,
    FeatureNotEnabledError,
    PyRuntimeError,
    "Error indicating a feature was not enabled in the build"
);

create_exception!(
    ry.ryo3,
    UnreachableError,
    PyAssertionError,
    "unreachable code path reached"
);

#[pyfunction(name = "unreachable", signature = (msg = None))]
fn py_unreachable(msg: Option<PyBackedStr>) -> PyResult<()> {
    if let Some(msg) = msg {
        Err(UnreachableError::new_err(msg))
    } else {
        Err(UnreachableError::new_err("unreachable"))
    }
}

#[pyfunction(name = "panic", signature = (msg = None))]
fn py_panic(msg: Option<PyBackedStr>) -> PyResult<()> {
    if let Some(msg) = msg {
        Err(PanicError::new_err(msg))
    } else {
        Err(PanicError::new_err("panic"))
    }
}

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    macro_rules! pymod_add_exception {
        ($e:ident) => {
            m.add(stringify!($e), m.py().get_type::<$e>())?;
        };
    }
    pymod_add_exception!(FeatureNotEnabledError);
    pymod_add_exception!(UnreachableError);
    pymod_add_exception!(PanicError);
    m.add_function(wrap_pyfunction!(py_unreachable, m)?)?;
    m.add_function(wrap_pyfunction!(py_panic, m)?)?;
    Ok(())
}
