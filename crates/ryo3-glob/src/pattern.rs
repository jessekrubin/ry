use glob::MatchOptions;
use pyo3::types::{PyDict, PyString, PyTuple};
use pyo3::{IntoPyObjectExt, prelude::*};
use ryo3_macro_rules::{py_type_err, py_value_error};
use std::path::PathBuf;

#[pyclass(name = "Pattern", frozen)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
#[derive(Clone)]
pub struct PyPattern {
    pattern: glob::Pattern,
    options: MatchOptions,
}

impl PyPattern {
    fn build_options_with_overrides(
        &self,
        case_sensitive: Option<bool>,
        require_literal_separator: Option<bool>,
        require_literal_leading_dot: Option<bool>,
    ) -> MatchOptions {
        MatchOptions {
            case_sensitive: case_sensitive.unwrap_or(self.options.case_sensitive),
            require_literal_separator: require_literal_separator
                .unwrap_or(self.options.require_literal_separator),
            require_literal_leading_dot: require_literal_leading_dot
                .unwrap_or(self.options.require_literal_leading_dot),
        }
    }
}

impl From<glob::Pattern> for PyPattern {
    fn from(value: glob::Pattern) -> Self {
        Self {
            pattern: value,
            options: MatchOptions {
                case_sensitive: true,
                require_literal_separator: false,
                require_literal_leading_dot: false,
            },
        }
    }
}

#[pymethods]
impl PyPattern {
    #[new]
    #[pyo3(
        signature = (
            pattern,
            *,
            case_sensitive = true,
            require_literal_separator = false,
            require_literal_leading_dot = false
        )
    )]
    fn py_new(
        pattern: &str,
        case_sensitive: bool,
        require_literal_separator: bool,
        require_literal_leading_dot: bool,
    ) -> PyResult<Self> {
        let p = glob::Pattern::new(pattern)
            .map(Self::from)
            .map_err(|e| py_value_error!("{e}"))?;
        Ok(Self {
            pattern: p.pattern,
            options: MatchOptions {
                case_sensitive,
                require_literal_separator,
                require_literal_leading_dot,
            },
        })
    }

    #[pyo3(
        signature = (
            ob,
            *,
            case_sensitive = None,
            require_literal_separator = None,
            require_literal_leading_dot = None
        )
    )]
    fn __call__(
        &self,
        ob: &Bound<'_, PyAny>,
        case_sensitive: Option<bool>,
        require_literal_separator: Option<bool>,
        require_literal_leading_dot: Option<bool>,
    ) -> PyResult<bool> {
        let opts = self.build_options_with_overrides(
            case_sensitive,
            require_literal_separator,
            require_literal_leading_dot,
        );
        // if string...
        if let Ok(s) = ob.extract::<&str>() {
            Ok(self.pattern.matches_with(s, opts))
        } else if let Ok(path) = ob.extract::<PathBuf>() {
            Ok(self.pattern.matches_path_with(&path, opts))
        } else {
            py_type_err!("Pattern() takes either a string or a pathlike object",)
        }
    }

    #[getter]
    fn pattern(&self) -> String {
        self.pattern.to_string()
    }

    fn __repr__(&self) -> String {
        format!("{self:?}")
    }

    #[staticmethod]
    fn escape(pattern: &str) -> String {
        glob::Pattern::escape(pattern)
    }

    fn matches(&self, path: &str) -> bool {
        self.pattern.matches(path)
    }

    #[expect(clippy::needless_pass_by_value)]
    fn matches_path(&self, path: PathBuf) -> bool {
        self.pattern.matches_path(&path)
    }

    #[pyo3(
        signature = (
            s,
            *,
            case_sensitive = None,
            require_literal_separator = None,
            require_literal_leading_dot = None
        )
    )]
    fn matches_with(
        &self,
        s: &str,
        case_sensitive: Option<bool>,
        require_literal_separator: Option<bool>,
        require_literal_leading_dot: Option<bool>,
    ) -> bool {
        self.pattern.matches_with(
            s,
            self.build_options_with_overrides(
                case_sensitive,
                require_literal_separator,
                require_literal_leading_dot,
            ),
        )
    }

    #[pyo3(
        signature = (
            path,
            *,
            case_sensitive = None,
            require_literal_separator = None,
            require_literal_leading_dot = None
        )
    )]
    #[expect(clippy::needless_pass_by_value)]
    fn matches_path_with(
        &self,
        path: PathBuf,
        case_sensitive: Option<bool>,
        require_literal_separator: Option<bool>,
        require_literal_leading_dot: Option<bool>,
    ) -> bool {
        self.pattern.matches_path_with(
            &path,
            self.build_options_with_overrides(
                case_sensitive,
                require_literal_separator,
                require_literal_leading_dot,
            ),
        )
    }

    fn __getnewargs_ex__<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyTuple>> {
        let pattern_str = PyString::new(py, &self.pattern.to_string());
        let args = PyTuple::new(py, [pattern_str])?.into_bound_py_any(py)?;
        let kwargs = PyDict::new(py);
        kwargs.set_item(
            pyo3::intern!(py, "case_sensitive"),
            self.options.case_sensitive,
        )?;
        kwargs.set_item(
            pyo3::intern!(py, "require_literal_separator"),
            self.options.require_literal_separator,
        )?;
        kwargs.set_item(
            pyo3::intern!(py, "require_literal_leading_dot"),
            self.options.require_literal_leading_dot,
        )?;
        PyTuple::new(py, [args, kwargs.into_bound_py_any(py)?])
    }

    fn __eq__(&self, other: &Self) -> bool {
        self.pattern == other.pattern && self.options == other.options
    }

    fn __ne__(&self, other: &Self) -> bool {
        !self.__eq__(other)
    }

    fn __hash__(&self) -> u64 {
        use std::hash::{Hash, Hasher};
        let mut hasher = std::hash::DefaultHasher::new();
        self.pattern.hash(&mut hasher);
        self.options.hash(&mut hasher);
        hasher.finish()
    }
}

impl std::fmt::Debug for PyPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Pattern(\"{}\"", self.pattern)?;
        if !self.options.case_sensitive {
            write!(f, ", case_sensitive=False")?;
        }
        if self.options.require_literal_separator {
            write!(f, ", require_literal_separator=True")?;
        }
        if self.options.require_literal_leading_dot {
            write!(f, ", require_literal_leading_dot=True")?;
        }
        write!(f, ")")
    }
}
