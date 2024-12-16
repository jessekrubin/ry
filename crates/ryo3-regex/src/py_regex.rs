use pyo3::prelude::*;
use regex::{Regex, RegexBuilder};
use std::borrow::Borrow;

#[pyclass(name = "Regex", frozen, module = "ryo3")]
#[derive(Clone, Debug)]
pub struct PyRegex(pub Regex);

impl From<Regex> for PyRegex {
    fn from(r: Regex) -> Self {
        PyRegex(r)
    }
}

impl TryFrom<RegexBuilder> for PyRegex {
    type Error = PyErr;

    fn try_from(rb: RegexBuilder) -> Result<Self, Self::Error> {
        rb.build().map(PyRegex::from).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Invalid regex: {}", e))
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
#[pymethods]
impl PyRegex {
    #[new]
    fn py_new(pattern: &str) -> PyResult<Self> {
        let r = Regex::new(pattern).map(PyRegex::from).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Invalid regex: {}", e))
        })?;
        Ok(r)
    }

    fn __str__(&self) -> String {
        format!("Regex('{}')", self.0)
    }
    fn __repr__(&self) -> String {
        format!("Regex('{}')", self.0)
    }

    fn is_match(&self, text: &str) -> bool {
        self.0.is_match(text)
    }

    fn find(&self, text: &str) -> Option<(usize, usize)> {
        self.0.find(text).map(|m| (m.start(), m.end()))
    }

    fn findall(&self, text: &str) -> Vec<(usize, usize)> {
        self.0
            .find_iter(text)
            .map(|m| (m.start(), m.end()))
            .collect()
    }

    fn replace(&self, text: &str, replace: &str) -> String {
        self.0.replace(text, replace).to_string()
    }
}

impl Borrow<Regex> for PyRegex {
    fn borrow(&self) -> &Regex {
        &self.0
    }
}
