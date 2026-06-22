use std::path::Path;
use std::str::FromStr;

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::{PyTuple, PyTupleMethods};
use ryo3_core::PyCastExactOpt;
use ryo3_core::types::PathLike;

use crate::globster::{Globster, GlobsterStrategy, GlobsterStrategyElement};
use crate::options::{DEFAULT_BACKSLASH_ESCAPE, GlobOptions};
use crate::traits::{PyGlobPatterns, PyGlobPatternsString};
use crate::{PyGlob, PyGlobSet, options};

#[pyclass(name = "Globster", frozen, immutable_type, skip_from_py_object)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
#[derive(Clone, Debug)]
pub struct PyGlobster(pub Globster, pub(crate) Vec<PyGlob>);

impl PyGlobPatternsString for PyGlobster {}

impl FromStr for PyGlobster {
    type Err = PyErr;

    fn from_str(pattern: &str) -> PyResult<Self> {
        let patterns = vec![pattern.to_string()];
        Self::from_patterns(patterns, GlobOptions::new())
    }
}

impl TryFrom<Vec<String>> for PyGlobster {
    type Error = PyErr;

    fn try_from(patterns: Vec<String>) -> PyResult<Self> {
        Self::from_patterns(patterns, GlobOptions::new())
    }
}

impl PyGlobster {
    pub fn is_match<P: AsRef<Path>>(&self, path: P) -> bool {
        self.0.is_match_path(path.as_ref())
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
pub(crate) fn py_globster_fn(
    patterns: GlobsterPatterns<'_, '_>,
    case_insensitive: bool,
    literal_separator: bool,
    backslash_escape: bool,
) -> PyResult<PyGlobster> {
    PyGlobster::from_pattern_args(
        patterns,
        options::GlobOptions::new()
            .case_insensitive(case_insensitive)
            .literal_separator(literal_separator)
            .backslash_escape(backslash_escape),
    )
}

#[pymethods]
impl PyGlobster {
    #[new]
    #[pyo3(
        signature = (
            *patterns,
            case_insensitive = false,
            literal_separator = false,
            backslash_escape = DEFAULT_BACKSLASH_ESCAPE
        )
    )]
    pub(crate) fn py_new(
        patterns: GlobsterPatterns<'_, '_>,
        case_insensitive: bool,
        literal_separator: bool,
        backslash_escape: bool,
    ) -> PyResult<Self> {
        let options = options::GlobOptions::new()
            .case_insensitive(case_insensitive)
            .literal_separator(literal_separator)
            .backslash_escape(backslash_escape);
        Self::from_pattern_args(patterns, options)
    }

    fn __repr__(&self) -> String {
        format!("{self}")
    }

    fn __getnewargs__<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyTuple>> {
        PyTuple::new(py, self.1.clone())
    }

    fn __len__(&self) -> usize {
        self.0.length
    }

    fn is_empty(&self) -> bool {
        self.0.length == 0
    }
    #[must_use]
    fn is_match_str(&self, path: &str) -> bool {
        self.0.is_match_str(path)
    }

    #[expect(clippy::needless_pass_by_value)]
    #[pyo3(name = "is_match")]
    #[must_use]
    fn py_is_match(&self, path: PathLike) -> bool {
        self.0.is_match_path(path.as_ref())
    }

    fn __call__(&self, path: PathLike) -> bool {
        self.py_is_match(path)
    }

    #[getter]
    fn patterns<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyTuple>> {
        let patterns = self.0.patterns.clone();
        PyTuple::new(py, patterns)
    }
}

impl PyGlobster {
    pub(crate) fn from_pattern_args(
        patterns: GlobsterPatterns<'_, '_>,
        options: GlobOptions,
    ) -> PyResult<Self> {
        let args = patterns.into_inner().to_owned();
        let mut builder = GlobsterBuilder::new();

        for idx in 0..args.len() {
            let arg = args.get_borrowed_item(idx)?;
            if let Some(glob) = arg.cast_exact_opt::<PyGlob>() {
                builder.push_pyglob(glob.get().clone())?;
            } else if let Some(globset) = arg.cast_exact_opt::<PyGlobSet>() {
                builder.push_globset(globset.get())?;
            } else if let Some(globster) = arg.cast_exact_opt::<Self>() {
                builder.push_globster(globster.get())?;
            } else if let Ok(pattern) = arg.extract::<String>() {
                builder.push_pattern(pattern, options)?;
            } else if let Ok(patterns) = arg.extract::<Vec<String>>() {
                for pattern in patterns {
                    builder.push_pattern(pattern, options)?;
                }
            } else {
                return Err(PyValueError::new_err(format!(
                    "Invalid pattern argument: expected str, Glob, GlobSet, Globster, or list of str, got {arg:?}"
                )));
            }
        }

        builder.finish()
    }

    pub(crate) fn from_patterns(patterns: Vec<String>, options: GlobOptions) -> PyResult<Self> {
        let mut builder = GlobsterBuilder::new();

        for pattern in patterns {
            builder.push_pattern(pattern, options)?;
        }

        builder.finish()
    }
}

impl std::fmt::Display for PyGlobster {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let tuple_str = self.patterns_string();
        write!(f, "Globster({tuple_str})")
    }
}

impl PyGlobPatterns for PyGlobster {
    fn patterns_ref(&self) -> &Vec<String> {
        &self.0.patterns
    }
}

struct GlobsterBuilder {
    strategies: Vec<GlobsterStrategyElement>,
    current_negative: Option<bool>,
    current_globs: Vec<globset::Glob>,
    patterns: Vec<String>,
    globs: Vec<PyGlob>,
}

impl GlobsterBuilder {
    fn new() -> Self {
        Self {
            strategies: Vec::new(),
            current_negative: None,
            current_globs: Vec::new(),
            patterns: Vec::new(),
            globs: Vec::new(),
        }
    }

    fn push_pattern(&mut self, pattern: String, options: GlobOptions) -> PyResult<()> {
        self.push_pyglob(PyGlob::from_pattern(pattern, options)?)
    }

    fn push_pyglob(&mut self, pyglob: PyGlob) -> PyResult<()> {
        let negative = pyglob.negative;

        if self.current_negative != Some(negative) {
            self.flush_group()?;
            self.current_negative = Some(negative);
        }

        self.current_globs.push(pyglob.glob.clone());
        self.patterns.push(pyglob.pattern.clone());
        self.globs.push(pyglob);
        Ok(())
    }

    fn push_globset(&mut self, globset: &PyGlobSet) -> PyResult<()> {
        self.flush_group()?;
        self.patterns.extend(globset.patterns.clone());
        self.globs.extend(globset.globs.clone());
        self.strategies
            .push(GlobsterStrategyElement::Set(globset.globset.clone()));
        Ok(())
    }

    fn push_globster(&mut self, globster: &PyGlobster) -> PyResult<()> {
        self.flush_group()?;
        self.patterns.extend(globster.0.patterns.clone());
        self.globs.extend(globster.1.clone());
        match &globster.0.strategy {
            GlobsterStrategy::Empty => {}
            GlobsterStrategy::One(strategy) => self.strategies.push(strategy.clone()),
            GlobsterStrategy::Ignore(strategies) => self.strategies.extend(strategies.clone()),
        }
        Ok(())
    }

    fn flush_group(&mut self) -> PyResult<()> {
        if let Some(negative) = self.current_negative.take() {
            let globs = std::mem::take(&mut self.current_globs);
            self.strategies
                .push(GlobsterStrategyElement::from_globs(negative, globs)?);
        }
        Ok(())
    }

    fn finish(mut self) -> PyResult<PyGlobster> {
        self.flush_group()?;

        let strategy = match self.strategies.len() {
            0 => GlobsterStrategy::Empty,
            1 => GlobsterStrategy::One(self.strategies.remove(0)),
            _ => GlobsterStrategy::Ignore(self.strategies),
        };

        Ok(PyGlobster(
            Globster {
                length: self.patterns.len(),
                patterns: self.patterns,
                strategy,
            },
            self.globs,
        ))
    }
}

pub(crate) struct GlobsterPatterns<'a, 'py>(Borrowed<'a, 'py, PyTuple>);

impl<'a, 'py> GlobsterPatterns<'a, 'py> {
    fn into_inner(self) -> Borrowed<'a, 'py, PyTuple> {
        self.0
    }
}

impl<'a, 'py> FromPyObject<'a, 'py> for GlobsterPatterns<'a, 'py> {
    type Error = PyErr;

    fn extract(obj: Borrowed<'a, 'py, PyAny>) -> PyResult<Self> {
        Ok(Self(obj.cast_exact()?))
    }
}
