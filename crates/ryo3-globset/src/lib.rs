#![doc = include_str!("../README.md")]
mod constants;
mod globster;
mod py_glob;
mod py_globset;
mod py_globster;
mod traits;
use std::str::FromStr;
mod options;

pub use constants::DEFAULT_BACKSLASH_ESCAPE;
use py_globster::GlobsterPatterns;
use pyo3::prelude::*;

pub use crate::py_glob::PyGlob;
pub use crate::py_globset::PyGlobSet;
pub use crate::py_globster::PyGlobster;

#[derive(FromPyObject)]
pub enum GlobsterLike {
    Glob(PyGlob),
    GlobSet(PyGlobSet),
    Globster(PyGlobster),
    Str(String),
    Strings(Vec<String>),
}

impl TryFrom<&GlobsterLike> for PyGlobster {
    type Error = PyErr;

    fn try_from(globster_like: &GlobsterLike) -> PyResult<Self> {
        match globster_like {
            GlobsterLike::Glob(glob) => Ok(glob.globster()),
            GlobsterLike::GlobSet(globset) => Ok(globset.globster()),
            GlobsterLike::Globster(globster) => Ok(globster.clone()),
            GlobsterLike::Strings(patterns) => Self::try_from(patterns.clone()),
            GlobsterLike::Str(s) => Self::from_str(s),
        }
    }
}

#[pyfunction]
#[pyo3(
    name = "globster",
    signature = (
        *patterns,
        case_insensitive = false,
        literal_separator = false,
        backslash_escape = DEFAULT_BACKSLASH_ESCAPE
    )
)]
fn py_globster_fn(
    patterns: GlobsterPatterns,
    case_insensitive: bool,
    literal_separator: bool,
    backslash_escape: bool,
) -> PyResult<PyGlobster> {
    PyGlobster::from_pattern_args(
        patterns,
        case_insensitive,
        literal_separator,
        backslash_escape,
    )
}

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(py_globster_fn, m)?)?;
    m.add_class::<PyGlob>()?;
    m.add_class::<PyGlobSet>()?;
    m.add_class::<PyGlobster>()?;
    Ok(())
}
