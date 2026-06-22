use ::globset::{Glob, GlobSetBuilder};
use pyo3::IntoPyObjectExt;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyString, PyTuple};
use ryo3_core::types::PathLike;

use crate::globster::Globster;
use crate::options::{DEFAULT_BACKSLASH_ESCAPE, GlobOptions};
use crate::{PyGlobSet, PyGlobster};

#[pyclass(name = "Glob", frozen, immutable_type, skip_from_py_object)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
#[derive(Clone, Debug)]
pub struct PyGlob {
    pub(crate) pattern: String,
    pub(crate) glob: Glob,
    pub(crate) matcher: globset::GlobMatcher,
    pub(crate) negative: bool,
    pub(crate) options: GlobOptions,
}

#[pymethods]
impl PyGlob {
    #[new]
    #[pyo3(
        signature = (
            pattern,
            /, *,
            case_insensitive = false,
            literal_separator = false,
            backslash_escape = DEFAULT_BACKSLASH_ESCAPE
        )
    )]
    fn py_new(
        pattern: String,
        case_insensitive: bool,
        literal_separator: bool,
        backslash_escape: bool,
    ) -> PyResult<Self> {
        let options = GlobOptions::new()
            .case_insensitive(case_insensitive)
            .literal_separator(literal_separator)
            .backslash_escape(backslash_escape);
        Self::from_pattern(pattern, options)
    }

    fn __invert__(&self) -> Self {
        Self {
            pattern: self.pattern.clone(),
            glob: self.glob.clone(),
            matcher: self.matcher.clone(),
            negative: !self.negative,
            options: self.options,
        }
    }

    #[must_use]
    fn is_match_str(&self, path: &str) -> bool {
        self.matcher.is_match(path) ^ self.negative
    }

    #[must_use]
    fn is_match(&self, path: PathLike) -> bool {
        self.matcher.is_match(path) ^ self.negative
    }

    fn __call__(&self, path: PathLike) -> bool {
        self.is_match(path)
    }

    fn __repr__(&self) -> String {
        format!("{self}")
    }

    fn __getnewargs_ex__<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyTuple>> {
        let pattern_str = PyString::new(py, &self.pattern);
        let args = PyTuple::new(py, [pattern_str])?.into_bound_py_any(py)?;
        let kwargs = PyDict::new(py);
        kwargs.set_item(
            pyo3::intern!(py, "case_insensitive"),
            self.options.is_case_insensitive(),
        )?;
        kwargs.set_item(
            pyo3::intern!(py, "literal_separator"),
            self.options.is_literal_separator(),
        )?;
        kwargs.set_item(
            pyo3::intern!(py, "backslash_escape"),
            self.options.is_backslash_escape(),
        )?;
        PyTuple::new(py, [args, kwargs.into_bound_py_any(py)?])
    }

    #[getter]
    fn regex(&self) -> String {
        self.glob.regex().to_string()
    }

    pub(crate) fn globset(&self) -> PyResult<PyGlobSet> {
        let gs = GlobSetBuilder::new()
            .add(self.glob.clone())
            .build()
            .map_err(|e| PyValueError::new_err(e.to_string()))?;
        Ok(PyGlobSet {
            patterns: vec![self.pattern.clone()],
            globset: gs,
            globs: vec![self.clone()],
        })
    }

    pub(crate) fn globster(&self) -> PyGlobster {
        PyGlobster(
            Globster::from_positive_glob(self.pattern.clone(), &self.glob),
            vec![self.clone()],
        )
    }

    pub(crate) fn __eq__(&self, other: &Self) -> bool {
        self.glob.regex() == other.glob.regex()
            && self.negative == other.negative
            && self.options == other.options
    }

    pub(crate) fn __neq__(&self, other: &Self) -> bool {
        !self.__eq__(other)
    }
}

impl PyGlob {
    pub(crate) fn from_pattern(pattern: String, options: GlobOptions) -> PyResult<Self> {
        if pattern.starts_with("!!") {
            return Err(PyValueError::new_err("Double negation is not allowed"));
        }
        let (negative, glob_pattern) = match pattern.strip_prefix('!') {
            Some(stripped) => (true, stripped),
            None => (false, pattern.as_str()),
        };
        if glob_pattern.is_empty() {
            return Err(PyValueError::new_err("Empty pattern"));
        }
        let glob = options.build(glob_pattern);
        match glob {
            Ok(glob) => {
                let matcher = glob.compile_matcher();
                Ok(Self {
                    pattern,
                    glob,
                    matcher,
                    negative,
                    options,
                })
            }
            Err(e) => Err(PyValueError::new_err(e.to_string())),
        }
    }
}

impl std::fmt::Display for PyGlob {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Glob(\"{}\")", self.pattern)
    }
}
