use crate::py_regex_options::PyRegexOptions;
use pyo3::prelude::*;
use regex::{Regex, RegexBuilder};
use std::borrow::{Borrow, Cow};

#[pyclass(name = "Regex", module = "ry.ryo3", frozen)]
#[derive(Clone, Debug)]
pub struct PyRegex {
    pub re: std::sync::Arc<Regex>,
    pub options: Option<PyRegexOptions>,
}

impl TryFrom<&str> for PyRegex {
    type Error = PyErr;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Regex::new(s)
            .map(|re| PyRegex {
                re: std::sync::Arc::new(re),
                options: None,
            })
            .map_err(|e| {
                PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Invalid regex: {e}"))
            })
    }
}

impl From<Regex> for PyRegex {
    fn from(re: Regex) -> Self {
        PyRegex {
            re: std::sync::Arc::new(re),
            options: None,
        }
    }
}

impl TryFrom<RegexBuilder> for PyRegex {
    type Error = PyErr;

    fn try_from(rb: RegexBuilder) -> Result<Self, Self::Error> {
        rb.build()
            .map(|re| PyRegex {
                re: std::sync::Arc::new(re),
                options: None,
            })
            .map_err(|e| {
                PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Invalid regex: {e}"))
            })
    }
}

//
// build
// case_insensitive -- default false
// crlf -- default false
// dot_matches_new_line -- default false
// ignore_whitespace -- default false
// line_terminator - default '\n'
// multi_line -- default false
// octal -- default false
// size_limit -- default none
// swap_greed -- default false
// unicode -- default true
// -------NOT SUPPORTED---------
// dfa_size_limit
// nest_limit -- idk

fn get_line_terminator_u8(line_terminator: Option<&str>) -> PyResult<u8> {
    match line_terminator {
        Some(lt) => {
            if lt.len() != 1 {
                return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                    "line_terminator must be a single byte",
                ));
            }
            Ok(lt.as_bytes()[0])
        }
        None => Ok(b'\n'),
    }
}

#[pymethods]
impl PyRegex {
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
        pattern: &str,
        // kwargs
        case_insensitive: bool,
        crlf: bool,
        dot_matches_new_line: bool,
        ignore_whitespace: bool,
        line_terminator: Option<&str>,
        multi_line: bool,
        octal: bool,
        size_limit: Option<usize>,
        swap_greed: bool,
        unicode: bool,
    ) -> PyResult<Self> {
        let mut builder = RegexBuilder::new(pattern);

        // fill in the bools
        let mut builder = builder
            .case_insensitive(case_insensitive)
            .crlf(crlf)
            .dot_matches_new_line(dot_matches_new_line)
            .ignore_whitespace(ignore_whitespace)
            .multi_line(multi_line)
            .octal(octal)
            .swap_greed(swap_greed)
            .unicode(unicode);

        let line_terminator_u8: u8 = get_line_terminator_u8(line_terminator)?;
        builder = builder.line_terminator(line_terminator_u8);

        if let Some(size_limit) = size_limit {
            builder.size_limit(size_limit);
        }
        builder.build().map(PyRegex::from).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Invalid regex: {e}"))
        })
    }

    fn __repr__(&self) -> String {
        format!("Regex('{}')", self.re)
    }

    fn __eq__(&self, other: &PyRegex) -> bool {
        self.re.as_str() == other.re.as_str()
    }

    fn is_match(&self, text: &str) -> bool {
        self.re.is_match(text)
    }

    fn find(&self, text: &str) -> Option<(usize, usize)> {
        self.re.find(text).map(|m| (m.start(), m.end()))
    }

    fn find_all(&self, text: &str) -> Vec<(usize, usize)> {
        self.re
            .find_iter(text)
            .map(|m| (m.start(), m.end()))
            .collect()
    }

    fn findall(&self, text: &str) -> Vec<(usize, usize)> {
        self.find_all(text)
    }

    fn replace<'py>(&self, text: &'py str, replace: &str) -> Cow<'py, str> {
        self.re.replace(text, replace)
    }

    fn replace_all(&self, text: &str, replace: &str) -> String {
        self.re.replace_all(text, replace).to_string()
    }

    fn split(&self, text: &str) -> Vec<String> {
        self.re
            .split(text)
            .map(std::string::ToString::to_string)
            .collect()
    }

    fn splitn(&self, text: &str, n: usize) -> Vec<String> {
        self.re
            .splitn(text, n)
            .map(std::string::ToString::to_string)
            .collect()
    }
}

impl Borrow<Regex> for PyRegex {
    fn borrow(&self) -> &Regex {
        &self.re
    }
}
