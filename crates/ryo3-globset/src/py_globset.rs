use ::globset::GlobSetBuilder;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::{PyTuple, PyTupleMethods};
use pyo3::{Borrowed, FromPyObject, PyAny, PyErr};
use ryo3_core::PyCastExactOpt;
use ryo3_core::types::PathLike;

use crate::globster::Globster;
use crate::traits::{PyGlobPatterns, PyGlobPatternsString};
use crate::{DEFAULT_BACKSLASH_ESCAPE, PyGlob, PyGlobster};

#[pyclass(name = "GlobSet", frozen, immutable_type, from_py_object)]
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
        patterns: GlobSetPatterns,
        case_insensitive: bool,
        literal_separator: bool,
        backslash_escape: bool,
    ) -> PyResult<Self> {
        let mut globset_builder = GlobSetBuilder::new();
        let elements = patterns.into_elements();
        let mut patterns = Vec::new();
        let mut globs = Vec::new();

        for element in elements {
            let glob = match element {
                GlobSetPatternElement::Glob(glob) => glob,
                GlobSetPatternElement::Pattern(pattern) => PyGlob::from_pattern(
                    pattern,
                    case_insensitive,
                    literal_separator,
                    backslash_escape,
                )?,
            };
            patterns.push(glob.pattern.clone());
            globset_builder.add(glob.glob.clone());
            globs.push(glob);
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

enum GlobSetPatternArg<'a, 'py> {
    Glob(Borrowed<'a, 'py, PyGlob>),
    Pattern(String),
    Patterns(Vec<String>),
}

impl<'a, 'py> FromPyObject<'a, 'py> for GlobSetPatternArg<'a, 'py> {
    type Error = PyErr;

    fn extract(obj: Borrowed<'a, 'py, PyAny>) -> PyResult<Self> {
        if let Some(glob) = obj.cast_exact_opt::<PyGlob>() {
            return Ok(Self::Glob(glob));
        }
        if let Ok(pattern) = obj.extract::<String>() {
            return Ok(Self::Pattern(pattern));
        }
        if let Ok(patterns) = obj.extract::<Vec<String>>() {
            return Ok(Self::Patterns(patterns));
        }
        Err(PyValueError::new_err(format!(
            "Invalid pattern argument: expected str, Glob, or list of str, got {obj:?}"
        )))
    }
}

enum GlobSetPatternElement {
    Glob(PyGlob),
    Pattern(String),
}

struct GlobSetPatterns(Vec<GlobSetPatternElement>);

impl GlobSetPatterns {
    fn into_elements(self) -> Vec<GlobSetPatternElement> {
        self.0
    }
}

impl<'py> FromPyObject<'_, 'py> for GlobSetPatterns {
    type Error = PyErr;

    fn extract(obj: Borrowed<'_, 'py, PyAny>) -> PyResult<Self> {
        let args = obj.cast_exact::<PyTuple>()?;
        let mut elements = Vec::new();

        for idx in 0..args.len() {
            let arg = args.get_item(idx)?;
            match GlobSetPatternArg::extract(arg.as_borrowed())? {
                GlobSetPatternArg::Glob(glob) => {
                    elements.push(GlobSetPatternElement::Glob(glob.get().clone()))
                }
                GlobSetPatternArg::Pattern(pattern) => {
                    elements.push(GlobSetPatternElement::Pattern(pattern))
                }
                GlobSetPatternArg::Patterns(patterns) => {
                    elements.extend(patterns.into_iter().map(GlobSetPatternElement::Pattern));
                }
            }
        }

        Ok(Self(elements))
    }
}
