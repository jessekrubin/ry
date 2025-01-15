use crate::{PyGlobPatternsString, DEFAULT_BACKSLASH_ESCAPE};
use globset::{GlobBuilder, GlobSetBuilder};
use pyo3::exceptions::PyValueError;
use pyo3::types::PyTuple;
use pyo3::{pyclass, pymethods, Bound, PyErr, PyResult, Python};
use ryo3_types::PathLike;
use std::str::FromStr;

#[derive(Clone, Debug)]
pub struct Globster {
    pub globset: Option<globset::GlobSet>,
    pub nglobset: Option<globset::GlobSet>,
    pub patterns: Vec<String>,
    pub length: usize,
}

#[pyclass(name = "Globster", frozen, module = "ryo3")]
#[derive(Clone, Debug)]
pub struct PyGlobster(pub Globster);

impl FromStr for PyGlobster {
    type Err = PyErr;

    fn from_str(pattern: &str) -> PyResult<Self> {
        let patterns = vec![pattern.to_string()];
        PyGlobster::py_new(patterns, None, None, None)
    }
}

impl TryFrom<Vec<String>> for PyGlobster {
    type Error = PyErr;

    fn try_from(patterns: Vec<String>) -> PyResult<Self> {
        PyGlobster::py_new(patterns, None, None, None)
    }
}

#[pymethods]
impl PyGlobster {
    #[new]
    #[pyo3(
        signature = (patterns, /, *, case_insensitive=None, literal_separator=None, backslash_escape=None)
    )]
    pub fn py_new(
        patterns: Vec<String>,
        case_insensitive: Option<bool>,
        literal_separator: Option<bool>,
        backslash_escape: Option<bool>,
    ) -> PyResult<Self> {
        let mut globset_builder = GlobSetBuilder::new();
        let mut nglobset_builder = GlobSetBuilder::new();
        let case_insensitive = case_insensitive.unwrap_or(false);
        let literal_separator = literal_separator.unwrap_or(false);
        let backslash_escape = backslash_escape.unwrap_or(DEFAULT_BACKSLASH_ESCAPE);
        let mut positive_patterns: Vec<String> = vec![];
        let mut negative_patterns: Vec<String> = vec![];

        for pattern in &patterns {
            if pattern.is_empty() {
                return Err(PyValueError::new_err("Empty pattern"));
            }
            if pattern.starts_with("!!") {
                return Err(PyValueError::new_err("Double negation is not allowed"));
            }
            if pattern.starts_with('!') {
                negative_patterns.push(pattern.clone());
            } else {
                positive_patterns.push(pattern.clone());
            }
        }

        {
            for pattern in &positive_patterns {
                let g = GlobBuilder::new(pattern)
                    .case_insensitive(case_insensitive)
                    .literal_separator(literal_separator)
                    .backslash_escape(backslash_escape)
                    .build()
                    .map_err(|e| PyValueError::new_err(e.to_string()))?;
                globset_builder.add(g);
            }
        }
        {
            for pattern in &negative_patterns {
                let g = GlobBuilder::new(pattern)
                    .case_insensitive(case_insensitive)
                    .literal_separator(literal_separator)
                    .backslash_escape(backslash_escape)
                    .build()
                    .map_err(|e| PyValueError::new_err(e.to_string()))?;
                nglobset_builder.add(g);
            }
        }
        let gs = globset_builder
            .build()
            .map_err(|e| PyValueError::new_err(format!("Error building globset: {e}")))?;
        let ngs = nglobset_builder
            .build()
            .map_err(|e| PyValueError::new_err(format!("Error building globset: {e}")))?;
        let globster = Globster {
            patterns,
            globset: Option::from(gs),
            nglobset: Option::from(ngs),
            length: positive_patterns.len() + negative_patterns.len(),
        };
        Ok(Self(globster))
    }

    fn __str__(&self) -> String {
        let tuple_str = self.patterns_string();
        format!("Globster({tuple_str})")
    }
    fn __repr__(&self) -> String {
        self.__str__()
    }

    fn __len__(&self) -> usize {
        self.0.length
    }

    fn is_empty(&self) -> bool {
        self.0.length == 0
    }

    pub fn is_match_str(&self, path: &str) -> bool {
        match (&self.0.globset, &self.0.nglobset) {
            (Some(gs), Some(ngs)) => gs.is_match(path) && !ngs.is_match(path),
            (Some(gs), None) => gs.is_match(path),
            (None, Some(ngs)) => !ngs.is_match(path),
            _ => false,
        }
    }

    pub fn is_match(&self, path: PathLike) -> bool {
        match (&self.0.globset, &self.0.nglobset) {
            (Some(gs), Some(ngs)) => {
                // let path = path.to_string();
                gs.is_match(&path) && !ngs.is_match(&path)
            }
            (Some(gs), None) => gs.is_match(&path),
            (None, Some(ngs)) => !ngs.is_match(&path),
            _ => false,
        }
    }

    fn __call__(&self, path: PathLike) -> bool {
        self.is_match(path)
    }

    #[getter]
    fn patterns<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyTuple>> {
        let patterns = self.0.patterns.clone();
        PyTuple::new(py, patterns)
    }
}
