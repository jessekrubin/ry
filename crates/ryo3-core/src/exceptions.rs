use pyo3::create_exception;
use pyo3::exceptions::{PyAssertionError, PyBaseException, PyRuntimeError};
use pyo3::prelude::*;
use pyo3::pybacked::PyBackedStr;

create_exception!(
    ry.ryo3,
    PanicException,
    PyBaseException,
    "python-side exception for panicing from python"
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

#[pyfunction(name = "unreachable")]
fn py_unreachable(msg: Option<PyBackedStr>) -> PyResult<()> {
    if let Some(msg) = msg {
        Err(UnreachableError::new_err(msg))
    } else {
        Err(UnreachableError::new_err("unreachable code path reached"))
    }
}

#[pyfunction(name = "panic")]
fn py_panic(msg: Option<PyBackedStr>) -> PyResult<()> {
    if let Some(msg) = msg {
        Err(PanicException::new_err(msg))
    } else {
        Err(PanicException::new_err("panic"))
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
    pymod_add_exception!(PanicException);
    m.add_function(wrap_pyfunction!(py_unreachable, m)?)?;
    m.add_function(wrap_pyfunction!(py_panic, m)?)?;
    Ok(())
}
