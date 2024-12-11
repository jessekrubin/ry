use ::globset::{Glob, GlobBuilder, GlobSetBuilder};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyTuple;
use ryo3_types::PathLike;

/// Default value for the `literal_separator` parameter.
const DEFAULT_BACKSLASH_ESCAPE: bool = cfg!(windows);

#[pyclass(name = "Glob", frozen, module = "ryo3")]
#[derive(Clone, Debug)]
pub struct PyGlob {
    pattern: String,
    glob: Glob,
    matcher: ::globset::GlobMatcher,
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
    fn __new__(
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

    fn __invert__(&self) -> PyGlob {
        PyGlob {
            pattern: self.pattern.clone(),
            glob: self.glob.clone(),
            matcher: self.matcher.clone(),
            negative: !self.negative,
        }
    }

    pub fn is_match_str(&self, path: &str) -> bool {
        self.matcher.is_match(path) ^ self.negative
    }

    pub fn is_match(&self, path: PathLike) -> bool {
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

    fn globset(&self) -> PyGlobSet {
        PyGlobSet {
            patterns: vec![self.pattern.clone()],
            globset: GlobSetBuilder::new()
                .add(self.glob.clone())
                .build()
                .unwrap(),
        }
    }

    fn globster(&self) -> PyGlobster {
        PyGlobster(Globster {
            patterns: vec![self.pattern.clone()],
            globset: Some(
                GlobSetBuilder::new()
                    .add(self.glob.clone())
                    .build()
                    .unwrap(),
            ),
            nglobset: None,
            length: 1,
        })
    }
}

#[pyclass(name = "GlobSet", frozen, module = "ryo3")]
#[derive(Clone, Debug)]
pub struct PyGlobSet {
    globset: ::globset::GlobSet,
    patterns: Vec<String>,
}

#[pymethods]
impl PyGlobSet {
    #[new]
    #[pyo3(
        signature = (patterns, /, *, case_insensitive=None, literal_separator=None, backslash_escape=None)
    )]
    fn __new__(
        patterns: Vec<String>,
        case_insensitive: Option<bool>,
        literal_separator: Option<bool>,
        backslash_escape: Option<bool>,
    ) -> PyResult<Self> {
        let mut globset_builder = GlobSetBuilder::new();
        let case_insensitive = case_insensitive.unwrap_or(false);
        let literal_separator = literal_separator.unwrap_or(false);
        let backslash_escape = backslash_escape.unwrap_or(DEFAULT_BACKSLASH_ESCAPE);
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

    pub fn is_match_str(&self, path: &str) -> bool {
        self.globset.is_match(path)
    }

    pub fn is_match(&self, path: PathLike) -> bool {
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

#[derive(Clone, Debug)]
pub struct Globster {
    pub globset: Option<::globset::GlobSet>,
    pub nglobset: Option<::globset::GlobSet>,
    pub patterns: Vec<String>,
    pub length: usize,
}

#[pyclass(name = "Globster", frozen, module = "ryo3")]
#[derive(Clone, Debug)]
pub struct PyGlobster(Globster);

#[pymethods]
impl PyGlobster {
    #[new]
    #[pyo3(
        signature = (patterns, /, *, case_insensitive=None, literal_separator=None, backslash_escape=None)
    )]
    fn __new__(
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
}

impl From<GlobsterLike> for PyGlobster {
    fn from(globster_like: GlobsterLike) -> Self {
        match globster_like {
            GlobsterLike::Glob(glob) => glob.globster(),
            GlobsterLike::GlobSet(globset) => globset.globster(),
            GlobsterLike::Globster(globster) => globster,
        }
    }
}

#[pyfunction]
#[pyo3(
    signature = (pattern, /, *, case_insensitive=None, literal_separator=None, backslash_escape=None)
)]
fn glob(
    pattern: String,
    case_insensitive: Option<bool>,
    literal_separator: Option<bool>,
    backslash_escape: Option<bool>,
) -> PyResult<PyGlob> {
    PyGlob::__new__(
        pattern,
        case_insensitive,
        literal_separator,
        backslash_escape,
    )
}

// #[pyfunction]
// #[pyo3(
//     signature = (pattern, /, *, case_insensitive=None, literal_separator=None, backslash_escape=None)
// )]
// fn globset(
//     pattern: String,
//     case_insensitive: Option<bool>,
//     literal_separator: Option<bool>,
//     backslash_escape: Option<bool>,
// ) -> PyResult<PyGlob> {
//     PyGlob::__new__(
//         pattern,
//         case_insensitive,
//         literal_separator,
//         backslash_escape,
//     )
// }

#[pyfunction]
#[pyo3(
    signature = (patterns, /, *, case_insensitive=None, literal_separator=None, backslash_escape=None)
)]
fn globs(
    patterns: Vec<String>,
    case_insensitive: Option<bool>,
    literal_separator: Option<bool>,
    backslash_escape: Option<bool>,
) -> PyResult<PyGlobster> {
    PyGlobster::__new__(
        patterns,
        case_insensitive,
        literal_separator,
        backslash_escape,
    )
}

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(glob, m)?)?;
    m.add_function(wrap_pyfunction!(globs, m)?)?;
    m.add_class::<PyGlob>()?;
    m.add_class::<PyGlobSet>()?;
    m.add_class::<PyGlobster>()?;
    Ok(())
}
