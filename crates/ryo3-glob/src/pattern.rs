use glob::MatchOptions;
use pyo3::prelude::*;
use pyo3::types::PyString;
use std::path::PathBuf;

#[pyclass(name = "Pattern", frozen)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
#[derive(Debug, Clone)]
pub struct PyPattern(pub(crate) glob::Pattern);

impl From<glob::Pattern> for PyPattern {
    fn from(value: glob::Pattern) -> Self {
        Self(value)
    }
}

#[pymethods]
impl PyPattern {
    #[new]
    fn py_new(pattern: &str) -> PyResult<Self> {
        glob::Pattern::new(pattern)
            .map(Self::from)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e}")))
    }

    #[pyo3(
        signature = (ob, *, case_sensitive = true, require_literal_separator = false, require_literal_leading_dot = false)
    )]
    fn __call__(
        &self,
        ob: &Bound<'_, PyAny>,
        case_sensitive: bool,
        require_literal_separator: bool,
        require_literal_leading_dot: bool,
    ) -> PyResult<bool> {
        let use_match_with =
            !case_sensitive || require_literal_separator || require_literal_leading_dot;

        // if string...
        if let Ok(s) = ob.cast::<PyString>()?.to_str() {
            if use_match_with {
                Ok(self.0.matches_with(
                    s,
                    MatchOptions {
                        case_sensitive,
                        require_literal_separator,
                        require_literal_leading_dot,
                    },
                ))
            } else {
                Ok(self.0.matches(s))
            }
        } else if let Ok(path) = ob.extract::<PathBuf>() {
            if use_match_with {
                Ok(self.0.matches_path_with(
                    &path,
                    MatchOptions {
                        case_sensitive,
                        require_literal_separator,
                        require_literal_leading_dot,
                    },
                ))
            } else {
                Ok(self.0.matches_path(&path))
            }
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "Pattern() takes either a string or a pathlike object",
            ))
        }
    }

    #[getter]
    fn pattern(&self) -> String {
        self.0.to_string()
    }

    fn __str__(&self) -> String {
        self.__repr__()
    }

    fn __repr__(&self) -> String {
        format!("Pattern(\"{}\")", self.0)
    }

    #[staticmethod]
    fn escape(pattern: &str) -> String {
        glob::Pattern::escape(pattern)
    }

    fn matches(&self, path: &str) -> bool {
        self.0.matches(path)
    }

    #[expect(clippy::needless_pass_by_value)]
    fn matches_path(&self, path: PathBuf) -> bool {
        self.0.matches_path(&path)
    }

    #[pyo3(
        signature = (s, *, case_sensitive = true, require_literal_separator = false, require_literal_leading_dot = false)
    )]
    fn matches_with(
        &self,
        s: &str,
        case_sensitive: bool,
        require_literal_separator: bool,
        require_literal_leading_dot: bool,
    ) -> bool {
        self.0.matches_with(
            s,
            MatchOptions {
                case_sensitive,
                require_literal_separator,
                require_literal_leading_dot,
            },
        )
    }

    #[pyo3(
        signature = (path, *, case_sensitive = true, require_literal_separator = false, require_literal_leading_dot = false)
    )]
    #[expect(clippy::needless_pass_by_value)]
    fn matches_path_with(
        &self,
        path: PathBuf,
        case_sensitive: bool,
        require_literal_separator: bool,
        require_literal_leading_dot: bool,
    ) -> bool {
        self.0.matches_path_with(
            &path,
            MatchOptions {
                case_sensitive,
                require_literal_separator,
                require_literal_leading_dot,
            },
        )
    }
}
