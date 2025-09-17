#![doc = include_str!("../README.md")]
mod globster;

pub use crate::globster::PyGlobster;
use ::globset::{Glob, GlobBuilder, GlobSetBuilder};
use globster::Globster;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use ryo3_core::types::{PathLike, StringOrStrings};
use std::str::FromStr;

/// Default value for the `literal_separator` parameter.
const DEFAULT_BACKSLASH_ESCAPE: bool = cfg!(windows);

#[pyclass(name = "Glob", frozen)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
#[derive(Clone, Debug)]
pub struct PyGlob {
    pattern: String,
    glob: Glob,
    matcher: globset::GlobMatcher,
    negative: bool,
}

trait PyGlobPatterns {
    fn patterns_ref(&self) -> &Vec<String>;
}

// trait for thingy that implements PyGlobPatterns
trait PyGlobPatternsString: PyGlobPatterns {
    fn patterns_string(&self) -> String {
        let inner_str = self
            .patterns_ref()
            .iter()
            .map(|s| format!("\"{s}\""))
            .collect::<Vec<_>>()
            .join(", ");
        format!("[{inner_str}]")
    }
}

#[pymethods]
impl PyGlob {
    #[new]
    #[pyo3(
        signature = (pattern, /, *, case_insensitive=None, literal_separator=None, backslash_escape=None)
    )]
    fn py_new(
        pattern: String,
        case_insensitive: Option<bool>,
        literal_separator: Option<bool>,
        backslash_escape: Option<bool>,
    ) -> PyResult<Self> {
        let negative = pattern.starts_with('!');
        let mut glob_builder = GlobBuilder::new(&pattern);
        glob_builder
            .backslash_escape(backslash_escape.unwrap_or(DEFAULT_BACKSLASH_ESCAPE))
            .literal_separator(literal_separator.unwrap_or(false))
            .case_insensitive(case_insensitive.unwrap_or(false));
        let glob = glob_builder.build();
        match glob {
            Ok(glob) => {
                let matcher = glob.compile_matcher();
                Ok(Self {
                    pattern,
                    glob,
                    matcher,
                    negative,
                })
            }
            Err(e) => Err(PyValueError::new_err(e.to_string())),
        }
    }

    fn __invert__(&self) -> Self {
        Self {
            pattern: self.pattern.clone(),
            glob: self.glob.clone(),
            matcher: self.matcher.clone(),
            negative: !self.negative,
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

    fn __str__(&self) -> String {
        format!("Glob(\"{}\")", self.pattern)
    }

    fn __repr__(&self) -> String {
        self.__str__()
    }

    #[getter]
    fn regex(&self) -> String {
        self.glob.regex().to_string()
    }

    fn globset(&self) -> PyResult<PyGlobSet> {
        let gs = GlobSetBuilder::new()
            .add(self.glob.clone())
            .build()
            .map_err(|e| PyValueError::new_err(e.to_string()))?;
        Ok(PyGlobSet {
            patterns: vec![self.pattern.clone()],
            globset: gs,
        })
    }

    fn globster(&self) -> PyResult<PyGlobster> {
        let globset = GlobSetBuilder::new()
            .add(self.glob.clone())
            .build()
            .map_err(|e| PyValueError::new_err(e.to_string()))?;
        Ok(PyGlobster(Globster {
            patterns: vec![self.pattern.clone()],
            globset: Some(globset),
            nglobset: None,
            length: 1,
        }))
    }
}

#[pyclass(name = "GlobSet", frozen)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
#[derive(Clone, Debug)]
pub struct PyGlobSet {
    globset: globset::GlobSet,
    patterns: Vec<String>,
}

#[pymethods]
impl PyGlobSet {
    #[new]
    #[pyo3(
        signature = (patterns, /, *, case_insensitive=None, literal_separator=None, backslash_escape=None)
    )]
    fn py_new(
        patterns: StringOrStrings,
        case_insensitive: Option<bool>,
        literal_separator: Option<bool>,
        backslash_escape: Option<bool>,
    ) -> PyResult<Self> {
        let mut globset_builder = GlobSetBuilder::new();
        let case_insensitive = case_insensitive.unwrap_or(false);
        let literal_separator = literal_separator.unwrap_or(false);
        let backslash_escape = backslash_escape.unwrap_or(DEFAULT_BACKSLASH_ESCAPE);
        let patterns = Vec::from(patterns);

        {
            for pattern in &patterns {
                if pattern.is_empty() {
                    return Err(PyValueError::new_err("Empty pattern"));
                }
                if pattern.starts_with("!!") {
                    return Err(PyValueError::new_err("Double negation is not allowed"));
                }
                let g = GlobBuilder::new(pattern)
                    .case_insensitive(case_insensitive)
                    .literal_separator(literal_separator)
                    .backslash_escape(backslash_escape)
                    .build()
                    .map_err(|e| PyValueError::new_err(e.to_string()))?;
                globset_builder.add(g);
            }
        }
        let gs = globset_builder
            .build()
            .map_err(|e| PyValueError::new_err(format!("Error building globset: {e}")))?;
        Ok(Self {
            patterns,
            globset: gs,
        })
    }

    fn __str__(&self) -> String {
        let tuple_str = self.patterns_string();
        format!("GlobSet({tuple_str})")
    }

    fn __repr__(&self) -> String {
        self.__str__()
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
    fn patterns(&self) -> Vec<String> {
        self.patterns.clone()
    }

    fn globster(&self) -> PyGlobster {
        PyGlobster(Globster {
            patterns: self.patterns.clone(),
            globset: Some(self.globset.clone()),
            nglobset: None,
            length: self.patterns.len(),
        })
    }
}

impl PyGlobPatterns for PyGlobSet {
    fn patterns_ref(&self) -> &Vec<String> {
        &self.patterns
    }
}

impl PyGlobPatterns for PyGlobster {
    fn patterns_ref(&self) -> &Vec<String> {
        &self.0.patterns
    }
}

impl PyGlobPatternsString for PyGlobSet {}
impl PyGlobPatternsString for PyGlobster {}

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
            GlobsterLike::Glob(glob) => glob.globster(),
            GlobsterLike::GlobSet(globset) => Ok(globset.globster()),
            GlobsterLike::Globster(globster) => Ok(globster.clone()),
            GlobsterLike::Strings(patterns) => Self::try_from(patterns.clone()),
            GlobsterLike::Str(s) => Self::from_str(s),
        }
    }
}

// ============================================================================
// NOTE: This has been removed/commented out because it conflicts with the
//       `glob` function in the `ryo3-glob` crate
//
// #[pyfunction]
// #[pyo3(
//     signature = (pattern, /, *, case_insensitive=None, literal_separator=None, backslash_escape=None)
// )]
// fn glob(
//     pattern: String,
//     case_insensitive: Option<bool>,
//     literal_separator: Option<bool>,
//     backslash_escape: Option<bool>,
// ) -> PyResult<PyGlob> {
//     PyGlob::py_new(
//         pattern,
//         case_insensitive,
//         literal_separator,
//         backslash_escape,
//     )
// }

#[pyfunction]
#[pyo3(
    name = "globster",
    signature = (patterns, /, *, case_insensitive=None, literal_separator=None, backslash_escape=None)
)]
fn py_globster(
    patterns: StringOrStrings,
    case_insensitive: Option<bool>,
    literal_separator: Option<bool>,
    backslash_escape: Option<bool>,
) -> PyResult<PyGlobster> {
    let patterns = Vec::from(patterns);
    PyGlobster::py_new(
        patterns,
        case_insensitive,
        literal_separator,
        backslash_escape,
    )
}

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(py_globster, m)?)?;
    m.add_class::<PyGlob>()?;
    m.add_class::<PyGlobSet>()?;
    m.add_class::<PyGlobster>()?;
    Ok(())
}
