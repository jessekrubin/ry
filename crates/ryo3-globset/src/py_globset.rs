use ::globset::GlobSetBuilder;
use pyo3::prelude::*;
use pyo3::types::{PyTuple, PyTupleMethods};
use ryo3_core::PyCastExactOpt;
use ryo3_core::macros::{py_value_err, py_value_error};
use ryo3_core::types::PathLike;

use crate::globster::Globster;
use crate::options::{DEFAULT_BACKSLASH_ESCAPE, GlobOptions};
use crate::py_args::VarArgs;
use crate::py_globster::GlobSource;
use crate::{PyGlob, PyGlobster};

#[pyclass(name = "GlobSet", frozen, immutable_type, skip_from_py_object)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
#[derive(Clone, Debug)]
pub struct PyGlobSet {
    pub(crate) globset: globset::GlobSet,
    pub(crate) globs: Vec<PyGlob>,
}

#[pymethods]
impl PyGlobSet {
    #[new]
    #[pyo3(
        signature = (
            *args,
            case_insensitive = false,
            literal_separator = false,
            backslash_escape = DEFAULT_BACKSLASH_ESCAPE
        )
    )]
    fn py_new(
        args: VarArgs<'_, '_>,
        case_insensitive: bool,
        literal_separator: bool,
        backslash_escape: bool,
    ) -> PyResult<Self> {
        let options = GlobOptions::new()
            .case_insensitive(case_insensitive)
            .literal_separator(literal_separator)
            .backslash_escape(backslash_escape);

        let mut globset_builder = GlobSetBuilder::new();
        let mut globs: Vec<PyGlob> = Vec::new();

        for arg in args.into_inner().iter_borrowed() {
            if let Some(glob) = arg.cast_exact_opt::<PyGlob>() {
                let glob = glob.get().clone();
                globset_builder.add(glob.glob.clone());
                globs.push(glob);
            } else if let Ok(pattern) = arg.extract::<String>() {
                let glob = PyGlob::from_pattern(pattern, options)?;
                globset_builder.add(glob.glob.clone());
                globs.push(glob);
            } else if let Ok(arg_patterns) = arg.extract::<Vec<String>>() {
                for pattern in arg_patterns {
                    let glob = PyGlob::from_pattern(pattern, options)?;
                    globset_builder.add(glob.glob.clone());
                    globs.push(glob);
                }
            } else {
                return py_value_err!(
                    "Invalid pattern argument: expected str, Glob, or list of str, got {arg:?}"
                );
            }
        }
        let gs = globset_builder
            .build()
            .map_err(|e| py_value_error!("Error building globset: {e}"))?;
        Ok(Self { globset: gs, globs })
    }

    fn __repr__(&self) -> String {
        format!("{self}")
    }

    fn __getnewargs__<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyTuple>> {
        PyTuple::new(py, self.globs.clone())
    }

    fn __len__(&self) -> usize {
        self.globset.len()
    }

    fn is_empty(&self) -> bool {
        self.globset.is_empty()
    }

    #[must_use]
    fn is_match_str(&self, path: &str) -> bool {
        self.globset.is_match(path)
    }

    #[must_use]
    fn is_match(&self, path: PathLike) -> bool {
        self.globset.is_match(path)
    }

    fn __call__(&self, path: PathLike) -> bool {
        self.is_match(path)
    }

    fn matches(&self, path: &str) -> Vec<usize> {
        self.globset.matches(path)
    }

    #[getter]
    #[pyo3(name = "patterns")]
    fn pyprop_patterns<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyTuple>> {
        PyTuple::new(py, self.globs.iter().map(|g| &g.pattern))
    }

    pub(crate) fn globster(&self) -> PyGlobster {
        let patterns: Vec<String> = self.globs.iter().map(|g| g.pattern.clone()).collect();
        PyGlobster(
            Globster::from_globset(patterns, self.globset.clone()),
            self.globs.iter().cloned().map(GlobSource::Glob).collect(),
        )
    }
}

impl std::fmt::Display for PyGlobSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let default = GlobOptions::new();
        write!(f, "GlobSet(")?;
        for (i, glob) in self.globs.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            if glob.options == default {
                write!(f, "\"{}\"", glob.pattern)?;
            } else {
                write!(f, "{glob}")?;
            }
        }
        write!(f, ")")
    }
}
