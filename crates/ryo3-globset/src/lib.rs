use ::globset::{Glob, GlobBuilder, GlobSetBuilder};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

/// Default value for the `literal_separator` parameter.
const DEFAULT_BACKSLASH_ESCAPE: bool = cfg!(windows);

#[pyclass(name = "Glob", frozen, module = "ryo3")]
#[derive(Clone, Debug)]
struct PyGlob {
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

    fn is_match(&self, path: &str) -> bool {
        self.matcher.is_match(path) ^ self.negative
    }

    fn __call__(&self, path: &str) -> bool {
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
}

#[pyclass(name = "GlobSet", frozen, module = "ryo3")]
#[derive(Clone, Debug)]
struct PyGlobSet {
    globset: globset::GlobSet,
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

    fn is_match(&self, path: &str) -> bool {
        self.globset.is_match(path)
    }

    fn matches(&self, path: &str) -> Vec<usize> {
        self.globset.matches(path)
    }

    fn __call__(&self, path: &str) -> bool {
        self.is_match(path)
    }

    #[getter]
    fn patterns(&self) -> Vec<String> {
        self.patterns.clone()
    }
}

#[pyclass(name = "Globster", frozen, module = "ryo3")]
#[derive(Clone, Debug)]
struct Globster {
    globset: Option<globset::GlobSet>,
    nglobset: Option<globset::GlobSet>,
    patterns: Vec<String>,
    length: usize,
}

#[pymethods]
impl Globster {
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
        Ok(Self {
            patterns,
            globset: Option::from(gs),
            nglobset: Option::from(ngs),
            length: positive_patterns.len() + negative_patterns.len(),
        })
    }

    fn __str__(&self) -> String {
        let tuple_str = self.patterns_string();
        format!("Globster({tuple_str})")
    }
    fn __repr__(&self) -> String {
        self.__str__()
    }

    fn __len__(&self) -> usize {
        self.length
    }

    fn is_empty(&self) -> bool {
        self.length == 0
    }

    fn is_match(&self, path: &str) -> bool {
        match (&self.globset, &self.nglobset) {
            (Some(gs), Some(ngs)) => gs.is_match(path) && !ngs.is_match(path),
            (Some(gs), None) => gs.is_match(path),
            (None, Some(ngs)) => !ngs.is_match(path),
            _ => false,
        }
    }

    fn __call__(&self, path: &str) -> bool {
        self.is_match(path)
    }

    #[getter]
    fn patterns(&self) -> Vec<String> {
        self.patterns.clone()
    }
}

impl PyGlobPatterns for PyGlobSet {
    fn patterns_ref(&self) -> &Vec<String> {
        &self.patterns
    }
}

impl PyGlobPatterns for Globster {
    fn patterns_ref(&self) -> &Vec<String> {
        &self.patterns
    }
}

impl PyGlobPatternsString for PyGlobSet {}
impl PyGlobPatternsString for Globster {}

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

#[pyfunction]
#[pyo3(
    signature = (patterns, /, *, case_insensitive=None, literal_separator=None, backslash_escape=None)
)]
fn globs(
    patterns: Vec<String>,
    case_insensitive: Option<bool>,
    literal_separator: Option<bool>,
    backslash_escape: Option<bool>,
) -> PyResult<Globster> {
    Globster::__new__(
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
    m.add_class::<Globster>()?;
    Ok(())
}
