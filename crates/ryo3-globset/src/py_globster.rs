use std::path::Path;
use std::str::FromStr;

use pyo3::prelude::*;
use pyo3::types::{PyString, PyTuple, PyTupleMethods};
use ryo3_core::PyCastExactOpt;
use ryo3_core::macros::{py_value_err, py_value_error};
use ryo3_core::types::PathLike;

use crate::globster::{Globster, GlobsterMatcher, Rule};
use crate::options::{DEFAULT_BACKSLASH_ESCAPE, GlobOptions};
use crate::py_args::VarArgs;
use crate::{PyGlob, PyGlobSet, options};

#[derive(Clone, Debug)]
pub(crate) enum GlobSource {
    Glob(PyGlob),
    Pattern(String),
}

#[pyclass(name = "Globster", frozen, immutable_type, skip_from_py_object)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
#[derive(Clone, Debug)]
pub struct PyGlobster(pub Globster, pub(crate) Vec<GlobSource>);

impl FromStr for PyGlobster {
    type Err = PyErr;

    fn from_str(pattern: &str) -> PyResult<Self> {
        Self::from_patterns(vec![pattern.to_string()], GlobOptions::new())
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
        *args,
        case_insensitive = false,
        literal_separator = false,
        backslash_escape = DEFAULT_BACKSLASH_ESCAPE
    )
)]
pub(crate) fn py_globster_fn(
    args: VarArgs<'_, '_>,
    case_insensitive: bool,
    literal_separator: bool,
    backslash_escape: bool,
) -> PyResult<PyGlobster> {
    PyGlobster::from_varargs(
        args,
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
            *args,
            case_insensitive = false,
            literal_separator = false,
            backslash_escape = DEFAULT_BACKSLASH_ESCAPE
        )
    )]
    pub(crate) fn py_new(
        args: VarArgs<'_, '_>,
        case_insensitive: bool,
        literal_separator: bool,
        backslash_escape: bool,
    ) -> PyResult<Self> {
        let options = options::GlobOptions::new()
            .case_insensitive(case_insensitive)
            .literal_separator(literal_separator)
            .backslash_escape(backslash_escape);
        Self::from_varargs(args, options)
    }

    fn __repr__(&self) -> String {
        format!("{self}")
    }

    fn __getnewargs__<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyTuple>> {
        let items = self
            .1
            .iter()
            .map(|source| -> PyResult<Bound<'py, PyAny>> {
                match source {
                    GlobSource::Glob(glob) => {
                        Bound::new(py, glob.clone()).map(pyo3::Bound::into_any)
                    }
                    GlobSource::Pattern(s) => Ok(PyString::new(py, s).into_any()),
                }
            })
            .collect::<PyResult<Vec<_>>>()?;
        PyTuple::new(py, items)
    }

    fn __len__(&self) -> usize {
        self.0.len()
    }

    fn is_empty(&self) -> bool {
        self.0.is_empty()
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
        PyTuple::new(py, self.0.patterns.iter())
    }
}

impl PyGlobster {
    pub(crate) fn from_varargs(args: VarArgs<'_, '_>, options: GlobOptions) -> PyResult<Self> {
        let mut builder = GlobsterBuilder::new();
        for arg in args.into_inner().iter_borrowed() {
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
                return Err(py_value_error!(
                    "Invalid pattern argument: expected str, Glob, GlobSet, Globster, or list of str, got {arg:?}"
                ));
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
        let default = GlobOptions::new();
        write!(f, "Globster(")?;
        for (i, source) in self.1.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            match source {
                GlobSource::Glob(glob) if glob.options == default => {
                    write!(f, "\"{}\"", glob.pattern)?;
                }
                GlobSource::Glob(glob) => {
                    write!(f, "{glob}")?;
                }
                GlobSource::Pattern(s) => {
                    write!(f, "\"{s}\"")?;
                }
            }
        }
        write!(f, ")")
    }
}

struct GlobsterBuilder {
    current_negative: Option<bool>,
    current_globs: Vec<globset::Glob>,
    rules: Vec<Rule>,
    patterns: Vec<String>,
    sources: Vec<GlobSource>,
}

impl GlobsterBuilder {
    fn new() -> Self {
        Self {
            current_negative: None,
            current_globs: Vec::new(),
            rules: Vec::new(),
            patterns: Vec::new(),
            sources: Vec::new(),
        }
    }

    fn push_pattern(&mut self, pattern: String, options: GlobOptions) -> PyResult<()> {
        if pattern.starts_with("!!") {
            return py_value_err!("Double negation is not allowed");
        }
        let (negative, glob_str) = match pattern.strip_prefix('!') {
            Some(stripped) => (true, stripped),
            None => (false, pattern.as_str()),
        };
        if glob_str.is_empty() {
            return py_value_err!("Empty pattern");
        }
        let glob = options
            .build(glob_str)
            .map_err(|e| py_value_error!("{e}"))?;
        if self.current_negative != Some(negative) {
            self.flush_group()?;
            self.current_negative = Some(negative);
        }
        self.current_globs.push(glob);
        self.patterns.push(pattern.clone());
        self.sources.push(GlobSource::Pattern(pattern));
        Ok(())
    }

    fn push_pyglob(&mut self, pyglob: PyGlob) -> PyResult<()> {
        if self.current_negative != Some(false) {
            self.flush_group()?;
            self.current_negative = Some(false);
        }
        self.current_globs.push(pyglob.glob.clone());
        self.patterns.push(pyglob.pattern.clone());
        self.sources.push(GlobSource::Glob(pyglob));
        Ok(())
    }

    fn push_globset(&mut self, globset: &PyGlobSet) -> PyResult<()> {
        self.flush_group()?;
        self.patterns
            .extend(globset.globs.iter().map(|g| g.pattern.clone()));
        self.sources
            .extend(globset.globs.iter().cloned().map(GlobSource::Glob));
        self.rules.push(Rule::from(globset.globset.clone()));
        Ok(())
    }

    fn push_globster(&mut self, globster: &PyGlobster) -> PyResult<()> {
        self.flush_group()?;
        self.patterns.extend(globster.0.patterns.iter().cloned());
        self.sources.extend(globster.1.iter().cloned());
        match &globster.0.matcher {
            GlobsterMatcher::Empty => {}
            GlobsterMatcher::Set(gs) => self.rules.push(Rule::from(gs.clone())),
            GlobsterMatcher::Rules(rules) => self.rules.extend(rules.iter().cloned()),
        }
        Ok(())
    }

    fn flush_group(&mut self) -> PyResult<()> {
        if let Some(negative) = self.current_negative.take() {
            let globs = std::mem::take(&mut self.current_globs);
            let rule = Rule::from_globs(negative, globs)
                .map_err(|e| py_value_error!("Error building globset: {e}"))?;
            self.rules.push(rule);
        }
        Ok(())
    }

    fn finish(mut self) -> PyResult<PyGlobster> {
        self.flush_group()?;
        let matcher = match self.rules.len() {
            0 => GlobsterMatcher::Empty,
            1 if !self.rules[0].negative => GlobsterMatcher::Set(self.rules.remove(0).globset),
            _ => GlobsterMatcher::Rules(self.rules),
        };
        Ok(PyGlobster(
            Globster {
                matcher,
                patterns: self.patterns,
            },
            self.sources,
        ))
    }
}
