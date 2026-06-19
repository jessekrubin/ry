use std::path::Path;
use std::str::FromStr;

use pyo3::exceptions::PyValueError;
use pyo3::types::{PyTuple, PyTupleMethods};
use pyo3::{Borrowed, Bound, FromPyObject, PyAny, PyErr, PyResult, Python, pyclass, pymethods};
use ryo3_core::PyCastExactOpt;
use ryo3_core::types::PathLike;

use crate::globster::{Globster, GlobsterStrategy};
use crate::traits::{PyGlobPatterns, PyGlobPatternsString};
use crate::{DEFAULT_BACKSLASH_ESCAPE, PyGlob, PyGlobSet};

enum GlobsterPatternArg<'a, 'py> {
    Glob(Borrowed<'a, 'py, PyGlob>),
    GlobSet(Borrowed<'a, 'py, PyGlobSet>),
    Globster(Borrowed<'a, 'py, PyGlobster>),
    Pattern(String),
    Patterns(Vec<String>),
}

impl<'a, 'py> FromPyObject<'a, 'py> for GlobsterPatternArg<'a, 'py> {
    type Error = PyErr;

    fn extract(obj: Borrowed<'a, 'py, PyAny>) -> PyResult<Self> {
        if let Some(glob) = obj.cast_exact_opt::<PyGlob>() {
            return Ok(Self::Glob(glob));
        }
        if let Some(globset) = obj.cast_exact_opt::<PyGlobSet>() {
            return Ok(Self::GlobSet(globset));
        }
        if let Some(globster) = obj.cast_exact_opt::<PyGlobster>() {
            return Ok(Self::Globster(globster));
        }
        if let Ok(glob) = obj.extract::<String>() {
            return Ok(Self::Pattern(glob));
        }
        if let Ok(patterns) = obj.extract::<Vec<String>>() {
            return Ok(Self::Patterns(patterns));
        }
        Err(PyValueError::new_err(format!(
            "Invalid pattern argument: expected str, Glob, GlobSet, Globster, or list of str, got {obj:?}"
        )))
    }
}
enum GlobsterPatternElement {
    Pattern(String),
    Strategy {
        patterns: Vec<String>,
        globs: Vec<PyGlob>,
        strategy: GlobsterStrategy,
    },
}

pub(crate) struct GlobsterPatterns(Vec<GlobsterPatternElement>);

impl GlobsterPatterns {
    fn into_elements(self) -> Vec<GlobsterPatternElement> {
        self.0
    }
}

impl<'py> FromPyObject<'_, 'py> for GlobsterPatterns {
    type Error = PyErr;

    fn extract(obj: Borrowed<'_, 'py, PyAny>) -> PyResult<Self> {
        let args = obj.cast_exact::<PyTuple>()?;
        let mut elements = Vec::new();

        for idx in 0..args.len() {
            let arg = args.get_item(idx)?;
            match GlobsterPatternArg::extract(arg.as_borrowed())? {
                GlobsterPatternArg::Glob(glob) => {
                    let glob = glob.get();
                    let strategy = if glob.negative {
                        GlobsterStrategy::SingleNegative(glob.matcher.clone())
                    } else {
                        GlobsterStrategy::SinglePositive(glob.matcher.clone())
                    };
                    elements.push(GlobsterPatternElement::Strategy {
                        patterns: vec![glob.pattern.clone()],
                        globs: vec![glob.clone()],
                        strategy,
                    });
                }
                GlobsterPatternArg::GlobSet(globset) => {
                    let globset = globset.get();
                    elements.push(GlobsterPatternElement::Strategy {
                        patterns: globset.patterns.clone(),
                        globs: globset.globs.clone(),
                        strategy: GlobsterStrategy::MultiPositive(globset.globset.clone()),
                    });
                }
                GlobsterPatternArg::Globster(globster) => {
                    let globster = globster.get();
                    elements.push(GlobsterPatternElement::Strategy {
                        patterns: globster.0.patterns.clone(),
                        globs: globster.1.clone(),
                        strategy: globster.0.strategy.clone(),
                    });
                }
                GlobsterPatternArg::Pattern(pattern) => {
                    elements.push(GlobsterPatternElement::Pattern(pattern));
                }
                GlobsterPatternArg::Patterns(patterns) => {
                    elements.extend(patterns.into_iter().map(GlobsterPatternElement::Pattern));
                }
            }
        }

        Ok(Self(elements))
    }
}

#[pyclass(name = "Globster", frozen, immutable_type, from_py_object)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
#[derive(Clone, Debug)]
pub struct PyGlobster(pub Globster, pub(crate) Vec<PyGlob>);

impl PyGlobPatternsString for PyGlobster {}

impl FromStr for PyGlobster {
    type Err = PyErr;

    fn from_str(pattern: &str) -> PyResult<Self> {
        let patterns = vec![pattern.to_string()];
        Self::from_patterns(patterns, false, false, DEFAULT_BACKSLASH_ESCAPE)
    }
}

impl TryFrom<Vec<String>> for PyGlobster {
    type Error = PyErr;

    fn try_from(patterns: Vec<String>) -> PyResult<Self> {
        Self::from_patterns(patterns, false, false, DEFAULT_BACKSLASH_ESCAPE)
    }
}

impl PyGlobster {
    pub fn is_match<P: AsRef<Path>>(&self, path: P) -> bool {
        self.0.is_match_path(path.as_ref())
    }
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
        patterns: GlobsterPatterns,
        case_insensitive: bool,
        literal_separator: bool,
        backslash_escape: bool,
    ) -> PyResult<Self> {
        Self::from_pattern_elements(
            patterns.into_elements(),
            case_insensitive,
            literal_separator,
            backslash_escape,
        )
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
        patterns: GlobsterPatterns,
        case_insensitive: bool,
        literal_separator: bool,
        backslash_escape: bool,
    ) -> PyResult<Self> {
        Self::from_pattern_elements(
            patterns.into_elements(),
            case_insensitive,
            literal_separator,
            backslash_escape,
        )
    }

    pub(crate) fn from_patterns(
        patterns: Vec<String>,
        case_insensitive: bool,
        literal_separator: bool,
        backslash_escape: bool,
    ) -> PyResult<Self> {
        let elements = patterns
            .into_iter()
            .map(GlobsterPatternElement::Pattern)
            .collect();
        Self::from_pattern_elements(
            elements,
            case_insensitive,
            literal_separator,
            backslash_escape,
        )
    }

    fn from_pattern_elements(
        elements: Vec<GlobsterPatternElement>,
        case_insensitive: bool,
        literal_separator: bool,
        backslash_escape: bool,
    ) -> PyResult<Self> {
        let mut strategies = Vec::new();
        let mut current_group: Option<PendingGlobsterGroup> = None;
        let mut patterns = Vec::new();
        let mut globs = Vec::new();

        for element in elements {
            let pyglob = match element {
                GlobsterPatternElement::Pattern(pattern) => pattern,
                GlobsterPatternElement::Strategy {
                    patterns: matcher_patterns,
                    globs: matcher_globs,
                    strategy,
                } => {
                    if let Some(group) = current_group.take() {
                        strategies.push(group.into_strategy()?);
                    }
                    patterns.extend(matcher_patterns);
                    globs.extend(matcher_globs);
                    strategies.push(strategy);
                    continue;
                }
            };
            let pyglob = PyGlob::from_pattern(
                pyglob,
                case_insensitive,
                literal_separator,
                backslash_escape,
            )?;
            let negative = pyglob.negative;

            match &mut current_group {
                Some(group) if group.negative == negative => {
                    group.globs.push(pyglob.clone());
                }
                _ => {
                    if let Some(group) = current_group.take() {
                        strategies.push(group.into_strategy()?);
                    }
                    let mut group = PendingGlobsterGroup::new(negative);
                    group.globs.push(pyglob.clone());
                    current_group = Some(group);
                }
            }
            patterns.push(pyglob.pattern.clone());
            globs.push(pyglob);
        }

        if let Some(group) = current_group {
            strategies.push(group.into_strategy()?);
        }

        let strategy = match strategies.len() {
            0 => GlobsterStrategy::Empty,
            1 => strategies.remove(0),
            _ => GlobsterStrategy::Ordered(strategies),
        };

        Ok(Self(
            Globster {
                length: patterns.len(),
                patterns,
                strategy,
            },
            globs,
        ))
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

struct PendingGlobsterGroup {
    negative: bool,
    globs: Vec<PyGlob>,
}

impl PendingGlobsterGroup {
    fn new(negative: bool) -> Self {
        Self {
            negative,
            globs: Vec::new(),
        }
    }

    fn into_strategy(self) -> PyResult<GlobsterStrategy> {
        let globs: Vec<globset::Glob> = self.globs.into_iter().map(|glob| glob.glob).collect();
        GlobsterStrategy::from_globs(self.negative, globs)
    }
}
