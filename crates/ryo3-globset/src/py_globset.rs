use ::globset::GlobSetBuilder;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::{PyTuple, PyTupleMethods};
use pyo3::{Borrowed, FromPyObject, PyAny, PyErr};
use ryo3_core::PyCastExactOpt;
use ryo3_core::types::PathLike;

use crate::globster::Globster;
use crate::options::{DEFAULT_BACKSLASH_ESCAPE, GlobOptions};
use crate::traits::{PyGlobPatterns, PyGlobPatternsString};
use crate::{PyGlob, PyGlobster};

#[pyclass(name = "GlobSet", frozen, immutable_type, skip_from_py_object)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
#[derive(Clone, Debug)]
pub struct PyGlobSet {
    pub(crate) globset: globset::GlobSet,
    pub(crate) patterns: Vec<String>,
    pub(crate) globs: Vec<PyGlob>,
}

impl PyGlobPatternsString for PyGlobSet {}

impl PyGlobPatterns for PyGlobSet {
    fn patterns_ref(&self) -> &Vec<String> {
        &self.patterns
    }
}

#[pymethods]
impl PyGlobSet {
    #[new]
    #[pyo3(
        signature = (
            *patterns,
            case_insensitive = false,
            literal_separator = false,
            backslash_escape = DEFAULT_BACKSLASH_ESCAPE
        )
    )]
    fn py_new(
        patterns: GlobSetPatterns<'_, '_>,
        case_insensitive: bool,
        literal_separator: bool,
        backslash_escape: bool,
    ) -> PyResult<Self> {
        let options = GlobOptions::new()
            .case_insensitive(case_insensitive)
            .literal_separator(literal_separator)
            .backslash_escape(backslash_escape);

        let args = patterns.into_inner();
        let mut globset_builder = GlobSetBuilder::new();
        let mut patterns = Vec::new();
        let mut globs: Vec<PyGlob> = Vec::new();

        for arg in args.iter_borrowed() {
            if let Some(glob) = arg.cast_exact_opt::<PyGlob>() {
                let glob = glob.get().clone();
                patterns.push(glob.pattern.clone());
                globset_builder.add(glob.glob.clone());
                globs.push(glob);
            } else if let Ok(pattern) = arg.extract::<String>() {
                let glob = PyGlob::from_pattern(pattern, options)?;
                patterns.push(glob.pattern.clone());
                globset_builder.add(glob.glob.clone());
                globs.push(glob);
            } else if let Ok(arg_patterns) = arg.extract::<Vec<String>>() {
                for pattern in arg_patterns {
                    let glob = PyGlob::from_pattern(pattern, options)?;
                    patterns.push(glob.pattern.clone());
                    globset_builder.add(glob.glob.clone());
                    globs.push(glob);
                }
            } else {
                return Err(PyValueError::new_err(format!(
                    "Invalid pattern argument: expected str, Glob, or list of str, got {arg:?}"
                )));
            }
        }
        let gs = globset_builder
            .build()
            .map_err(|e| PyValueError::new_err(format!("Error building globset: {e}")))?;
        Ok(Self {
            patterns,
            globset: gs,
            globs,
        })
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
    fn pyprop_patterns<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, pyo3::types::PyTuple>> {
        pyo3::types::PyTuple::new(py, self.patterns.clone())
    }

    pub(crate) fn globster(&self) -> PyGlobster {
        PyGlobster(
            Globster::from_globset(self.patterns.clone(), self.globset.clone()),
            self.globs.clone(),
        )
    }
}

impl std::fmt::Display for PyGlobSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let tuple_str = self.patterns_string();
        write!(f, "GlobSet({tuple_str})")
    }
}

struct GlobSetPatterns<'a, 'py>(Borrowed<'a, 'py, PyTuple>);

impl<'a, 'py> GlobSetPatterns<'a, 'py> {
    fn into_inner(self) -> Borrowed<'a, 'py, PyTuple> {
        self.0
    }
}

impl<'a, 'py> FromPyObject<'a, 'py> for GlobSetPatterns<'a, 'py> {
    type Error = PyErr;

    fn extract(obj: Borrowed<'a, 'py, PyAny>) -> PyResult<Self> {
        Ok(Self(obj.cast_exact()?))
    }
}
