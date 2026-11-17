#![doc = include_str!("../README.md")]
mod globster;
mod py_args;
mod ry_glob;
mod ry_globset;
mod ry_globster;
use std::str::FromStr;

mod options;

use pyo3::prelude::*;

pub use crate::ry_glob::RyGlob;
pub use crate::ry_globset::RyGlobSet;
pub use crate::ry_globster::RyGlobster;

pub enum GlobsterLike<'a, 'py> {
    Glob(Borrowed<'a, 'py, RyGlob>),
    GlobSet(Borrowed<'a, 'py, RyGlobSet>),
    Globster(Borrowed<'a, 'py, RyGlobster>),
    Str(String),
    Strings(Vec<String>),
}

impl<'a, 'py> FromPyObject<'a, 'py> for GlobsterLike<'a, 'py> {
    type Error = PyErr;
    fn extract(ob: Borrowed<'a, 'py, PyAny>) -> PyResult<Self> {
        use ryo3_core::PyCastExactOpt;
        if let Some(glob) = ob.cast_exact_opt::<RyGlob>() {
            Ok(Self::Glob(glob))
        } else if let Some(globset) = ob.cast_exact_opt::<RyGlobSet>() {
            Ok(Self::GlobSet(globset))
        } else if let Some(globster) = ob.cast_exact_opt::<RyGlobster>() {
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

impl<'a, 'py> TryFrom<&GlobsterLike<'a, 'py>> for RyGlobster {
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
    m.add_function(wrap_pyfunction!(crate::ry_globster::py_globster_fn, m)?)?;
    m.add_class::<RyGlob>()?;
    m.add_class::<RyGlobSet>()?;
    m.add_class::<RyGlobster>()?;
    Ok(())
}
