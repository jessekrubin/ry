//! Options for Python regex compilation
//!
//! Based on the `regex::RegexBuilder` options.
//!
//! REF: <https://docs.rs/regex/latest/regex/struct.RegexBuilder.html>
//!
use pyo3::{IntoPyObjectExt, prelude::*};

#[expect(clippy::struct_excessive_bools)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct PyRegexOptions {
    pub(crate) case_insensitive: bool,
    pub(crate) crlf: bool,
    pub(crate) dot_matches_new_line: bool,
    pub(crate) ignore_whitespace: bool,
    pub(crate) line_terminator: u8,
    pub(crate) multi_line: bool,
    pub(crate) octal: bool,
    pub(crate) size_limit: Option<usize>,
    pub(crate) swap_greed: bool,
    pub(crate) unicode: bool,
    // TODO: add when needed
    // pub(crate) dfa_size_limit: Option<usize>
    // pub(crate) nest_limit: Option<usize>,
}

impl Default for PyRegexOptions {
    fn default() -> Self {
        Self {
            case_insensitive: false,
            crlf: false,
            dot_matches_new_line: false,
            ignore_whitespace: false,
            line_terminator: b'\n',
            multi_line: false,
            octal: false,
            size_limit: None,
            swap_greed: false,
            unicode: true,
        }
    }
}

impl PyRegexOptions {
    pub(crate) fn is_default(&self) -> bool {
        *self == Self::default()
    }

    pub(crate) fn as_pydict<'py>(
        &self,
        py: Python<'py>,
    ) -> PyResult<Bound<'py, pyo3::types::PyDict>> {
        let dict = pyo3::types::PyDict::new(py);
        dict.set_item(pyo3::intern!(py, "case_insensitive"), self.case_insensitive)?;
        dict.set_item(pyo3::intern!(py, "crlf"), self.crlf)?;
        dict.set_item(
            pyo3::intern!(py, "dot_matches_new_line"),
            self.dot_matches_new_line,
        )?;
        dict.set_item(
            pyo3::intern!(py, "ignore_whitespace"),
            self.ignore_whitespace,
        )?;

        let py_byte = ryo3_core::types::Byte::from(self.line_terminator).into_bound_py_any(py)?;
        dict.set_item(pyo3::intern!(py, "line_terminator"), py_byte)?;

        dict.set_item(pyo3::intern!(py, "multi_line"), self.multi_line)?;
        dict.set_item(pyo3::intern!(py, "octal"), self.octal)?;
        if let Some(size_limit) = self.size_limit {
            dict.set_item(pyo3::intern!(py, "size_limit"), size_limit)?;
        } else {
            dict.set_item(pyo3::intern!(py, "size_limit"), py.None())?;
        }
        dict.set_item(pyo3::intern!(py, "swap_greed"), self.swap_greed)?;
        dict.set_item(pyo3::intern!(py, "unicode"), self.unicode)?;
        Ok(dict)
    }

    pub(crate) fn write_regex_kwargs(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_default() {
            return Ok(());
        }
        if self.case_insensitive {
            // always write at least one option
            write!(f, ", case_insensitive=True")?;
        }
        if self.crlf {
            write!(f, ", crlf=True")?;
        }
        if self.dot_matches_new_line {
            write!(f, ", dot_matches_new_line=True")?;
        }
        if self.ignore_whitespace {
            write!(f, ", ignore_whitespace=True")?;
        }
        if self.line_terminator != b'\n' {
            let b = ryo3_core::types::Byte::from(self.line_terminator);
            write!(f, ", line_terminator={b:?}")?;
        }
        if self.multi_line {
            write!(f, ", multi_line=True")?;
        }
        if self.octal {
            write!(f, ", octal=True")?;
        }
        if let Some(size_limit) = self.size_limit {
            write!(f, ", size_limit={size_limit}")?;
        }
        if self.swap_greed {
            write!(f, ", swap_greed=True")?;
        }
        if !self.unicode {
            write!(f, ", unicode=False")?;
        }
        Ok(())
    }

    pub(crate) fn build_pattern(&self, pattern: &str) -> PyResult<regex::Regex> {
        let mut builder = regex::RegexBuilder::new(pattern);
        let mut builder = builder
            .case_insensitive(self.case_insensitive)
            .crlf(self.crlf)
            .dot_matches_new_line(self.dot_matches_new_line)
            .ignore_whitespace(self.ignore_whitespace)
            .line_terminator(self.line_terminator)
            .multi_line(self.multi_line)
            .octal(self.octal)
            .swap_greed(self.swap_greed)
            .unicode(self.unicode);
        if let Some(size_limit) = self.size_limit {
            builder = builder.size_limit(size_limit);
        }
        builder.build().map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Invalid regex: {e}"))
        })
    }
}
