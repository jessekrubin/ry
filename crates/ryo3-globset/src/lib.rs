#![doc = include_str!("../README.md")]
mod globster;
mod py_glob;
mod py_globset;
mod py_globster;
mod traits;
use std::str::FromStr;

mod options;

use pyo3::prelude::*;

pub use crate::py_glob::PyGlob;
pub use crate::py_globset::PyGlobSet;
pub use crate::py_globster::PyGlobster;

pub enum GlobsterLike<'a, 'py> {
    Glob(Borrowed<'a, 'py, PyGlob>),
    GlobSet(Borrowed<'a, 'py, PyGlobSet>),
    Globster(Borrowed<'a, 'py, PyGlobster>),
    Str(String),
    Strings(Vec<String>),
}

impl<'a, 'py> FromPyObject<'a, 'py> for GlobsterLike<'a, 'py> {
    type Error = PyErr;
    fn extract(ob: Borrowed<'a, 'py, PyAny>) -> PyResult<Self> {
        use ryo3_core::PyCastExactOpt;
        if let Some(glob) = ob.cast_exact_opt::<PyGlob>() {
            Ok(Self::Glob(glob))
        } else if let Some(globset) = ob.cast_exact_opt::<PyGlobSet>() {
            Ok(Self::GlobSet(globset))
        } else if let Some(globster) = ob.cast_exact_opt::<PyGlobster>() {
            Ok(Self::Globster(globster))
        } else if let Ok(s) = ob.extract::<String>() {
            Ok(Self::Str(s))
        } else if let Ok(patterns) = ob.extract::<Vec<String>>() {
            Ok(Self::Strings(patterns))
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                "Expected a Glob, GlobSet, Globster, str, or list of str",
            ))
        }
    }
}

impl<'a, 'py> TryFrom<&GlobsterLike<'a, 'py>> for PyGlobster {
    type Error = PyErr;

    fn try_from(globster_like: &GlobsterLike<'a, 'py>) -> PyResult<Self> {
        match globster_like {
            GlobsterLike::Glob(glob) => Ok(glob.get().globster()),
            GlobsterLike::GlobSet(globset) => Ok(globset.get().globster()),
            GlobsterLike::Globster(globster) => Ok(globster.get().clone()),
            GlobsterLike::Strings(patterns) => Self::try_from(patterns.clone()),
            GlobsterLike::Str(s) => Self::from_str(s),
        }
    }
}

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(crate::py_globster::py_globster_fn, m)?)?;
    m.add_class::<PyGlob>()?;
    m.add_class::<PyGlobSet>()?;
    m.add_class::<PyGlobster>()?;
    Ok(())
}
