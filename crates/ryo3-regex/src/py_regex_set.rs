use crate::py_regex_options::PyRegexOptions;
use pyo3::{IntoPyObjectExt, prelude::*, pybacked::PyBackedStr};
use regex::{Regex, RegexBuilder, RegexSet, RegexSetBuilder};
use std::borrow::{Borrow, Cow};

#[pyclass(name = "RegexSet", frozen)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
#[derive(Clone, Debug)]
pub struct PyRegexSet {
    pub re: std::sync::Arc<RegexSet>,
    pub options: Option<PyRegexOptions>,
}

impl From<RegexSet> for PyRegexSet {
    fn from(re: RegexSet) -> Self {
        Self {
            re: std::sync::Arc::new(re),
            options: None,
        }
    }
}

impl TryFrom<RegexSetBuilder> for PyRegexSet {
    type Error = PyErr;

    fn try_from(rb: RegexSetBuilder) -> Result<Self, Self::Error> {
        rb.build()
            .map(|re| Self {
                re: std::sync::Arc::new(re),
                options: None,
            })
            .map_err(|e| {
                PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Invalid regex: {e}"))
            })
    }
}

#[pymethods]
impl PyRegexSet {
    #[new]
    #[pyo3(signature = (
        pattern,
        *,
        case_insensitive = false,
        crlf = false,
        dot_matches_new_line = false,
        ignore_whitespace = false,
        line_terminator = None,
        multi_line = false,
        octal = false,
        size_limit = None,
        swap_greed = false,
        unicode = true
    ))]
    #[expect(clippy::too_many_arguments)]
    #[expect(clippy::fn_params_excessive_bools)]
    fn py_new(
        pattern: Vec<PyBackedStr>,
        // kwargs
        case_insensitive: bool,
        crlf: bool,
        dot_matches_new_line: bool,
        ignore_whitespace: bool,
        line_terminator: Option<ryo3_core::types::Byte>,
        multi_line: bool,
        octal: bool,
        size_limit: Option<usize>,
        swap_greed: bool,
        unicode: bool,
    ) -> PyResult<Self> {
        // let line_terminator_u8: u8 = get_line_terminator_u8(line_terminator)?;
        let options = PyRegexOptions {
            case_insensitive,
            crlf,
            dot_matches_new_line,
            ignore_whitespace,
            line_terminator: line_terminator.map_or(b'\n', |lt| *lt),
            multi_line,
            octal,
            size_limit,
            swap_greed,
            unicode,
        };

        let re = options.build_patterns(pattern)?;
        Ok(Self {
            re: std::sync::Arc::new(re),
            options: Some(options),
        })
    }

    fn __getnewargs_ex__<'py>(
        &self,
        py: Python<'py>,
    ) -> PyResult<Bound<'py, pyo3::types::PyTuple>> {
        let kwargs = if let Some(opts) = &self.options {
            opts.as_pydict(py)?.into_bound_py_any(py)?
        } else {
            pyo3::types::PyDict::new(py).into_bound_py_any(py)?
        };

        let args = pyo3::types::PyTuple::new(
            py,
            [
                // self.re.as_str().into_bound_py_any(py)?
                self.re.patterns(),
            ],
        )?;
        pyo3::types::PyTuple::new(py, &[args.into_bound_py_any(py)?, kwargs])
    }

    fn __repr__(&self) -> String {
        format!("{self}")
    }

    fn __eq__(&self, other: &Self) -> bool {
        self.re.patterns() == other.re.patterns() && self.options == other.options
    }

    fn __ne__(&self, other: &Self) -> bool {
        !self.__eq__(other)
    }

    /// Returns true if and only if there is a match for the regex anywhere in the haystack given.
    ///
    /// It is recommended to use this method if all you need to do is test
    /// whether a match exists, since the underlying matching engine may be
    /// able to do less work.
    fn is_match(&self, text: &str) -> bool {
        self.re.is_match(text)
    }

    /// Returns true if and only if there is a match for the regex anywhere in the haystack given.
    ///
    /// It is recommended to use this method if all you need to do is test
    /// whether a match exists, since the underlying matching engine may be
    /// able to do less work.
    fn test(&self, text: &str) -> bool {
        self.re.is_match(text)
    }
}

impl Borrow<RegexSet> for PyRegexSet {
    fn borrow(&self) -> &RegexSet {
        &self.re
    }
}

impl std::fmt::Display for PyRegexSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(opts) = &self.options {
            write!(f, "RegexSet({:?}", self.re.patterns())?;
            opts.write_regex_kwargs(f)?;
            write!(f, ")")
        } else {
            write!(f, "RegexSet({:?}", self.re.patterns())
        }
    }
}
