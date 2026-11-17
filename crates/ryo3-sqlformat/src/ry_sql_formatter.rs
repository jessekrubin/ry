use pyo3::IntoPyObjectExt;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyTuple};
use sqlformat::QueryParams;

use crate::RySqlfmtQueryParams;
use crate::py_types::{PyDialect, PyIndent, PyQueryParamsLike};

#[derive(Debug, Clone, PartialEq)]
struct PySqlFormatterOptions {
    indent: PyIndent,
    uppercase: Option<bool>,
    lines_between_queries: u8,
    ignore_case_convert: Option<Vec<String>>,
    inline: bool,
    max_inline_block: usize,
    max_inline_arguments: Option<usize>,
    max_inline_top_level: Option<usize>,
    joins_as_top_level: bool,
    dialect: PyDialect,
}

#[derive(Clone, PartialEq)]
#[pyclass(name = "SqlFormatter", frozen, immutable_type, eq)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
pub struct RySqlFormatter(PySqlFormatterOptions);

impl RySqlFormatter {
    fn ignore_case_convert(&self) -> Option<Vec<&str>> {
        self.0
            .ignore_case_convert
            .as_ref()
            .map(|v| v.iter().map(String::as_str).collect())
    }
}

#[pymethods]
impl RySqlFormatter {
    #[new]
    #[pyo3(
        signature = (
            *,
            indent = PyIndent::default(),
            uppercase = None,
            lines_between_queries = 1,
            ignore_case_convert = None,
            inline = false,
            max_inline_block = 50,
            max_inline_arguments = None,
            max_inline_top_level = None,
            joins_as_top_level = false,
            dialect = PyDialect::default()
        ),
        text_signature = "(*, indent=2, uppercase=None, lines_between_queries=1, ignore_case_convert=None, inline=False, max_inline_block=50, max_inline_arguments=None, max_inline_top_level=None, joins_as_top_level=False, dialect='generic')"
    )]
    #[expect(clippy::too_many_arguments, reason = "python kwargs")]
    fn py_new(
        indent: PyIndent,
        uppercase: Option<bool>,
        lines_between_queries: u8,
        ignore_case_convert: Option<Vec<String>>,
        inline: bool,
        max_inline_block: usize,
        max_inline_arguments: Option<usize>,
        max_inline_top_level: Option<usize>,
        joins_as_top_level: bool,
        dialect: PyDialect,
    ) -> Self {
        Self(PySqlFormatterOptions {
            indent,
            uppercase,
            lines_between_queries,
            ignore_case_convert: ignore_case_convert.filter(|v| !v.is_empty()),
            inline,
            max_inline_block,
            max_inline_arguments,
            max_inline_top_level,
            joins_as_top_level,
            dialect,
        })
    }

    fn __repr__(&self) -> String {
        format!("{self:?}")
    }

    fn __getnewargs_ex__<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyTuple>> {
        let args = PyTuple::empty(py).into_bound_py_any(py)?;
        let kwargs = self.to_dict(py)?.into_bound_py_any(py)?;
        PyTuple::new(py, vec![args, kwargs])
    }

    fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        let dict = PyDict::new(py);
        dict.set_item(pyo3::intern!(py, "indent"), self.0.indent)?;
        dict.set_item(pyo3::intern!(py, "uppercase"), self.0.uppercase)?;
        dict.set_item(
            pyo3::intern!(py, "lines_between_queries"),
            self.0.lines_between_queries,
        )?;
        let ignore_case_convert_py_str = pyo3::intern!(py, "ignore_case_convert");
        if let Some(v) = &self.0.ignore_case_convert {
            let pylist = pyo3::types::PyList::new(py, v)?;
            dict.set_item(ignore_case_convert_py_str, pylist)?;
        } else {
            dict.set_item(ignore_case_convert_py_str, py.None())?;
        }
        dict.set_item(pyo3::intern!(py, "inline"), self.0.inline)?;
        dict.set_item(
            pyo3::intern!(py, "max_inline_block"),
            self.0.max_inline_block,
        )?;
        dict.set_item(
            pyo3::intern!(py, "max_inline_arguments"),
            self.0.max_inline_arguments,
        )?;
        dict.set_item(
            pyo3::intern!(py, "max_inline_top_level"),
            self.0.max_inline_top_level,
        )?;
        dict.set_item(
            pyo3::intern!(py, "joins_as_top_level"),
            self.0.joins_as_top_level,
        )?;
        dict.set_item(pyo3::intern!(py, "dialect"), self.0.dialect)?;
        Ok(dict)
    }

    #[pyo3(signature = (sql, params = None))]
    fn fmt(&self, sql: &str, params: Option<PyQueryParamsLike>) -> PyResult<String> {
        let opts = sqlformat::FormatOptions {
            indent: self.0.indent.into(),
            uppercase: self.0.uppercase,
            lines_between_queries: self.0.lines_between_queries,
            ignore_case_convert: self.ignore_case_convert(),
            inline: self.0.inline,
            max_inline_block: self.0.max_inline_block,
            max_inline_arguments: self.0.max_inline_arguments,
            max_inline_top_level: self.0.max_inline_top_level,
            joins_as_top_level: self.0.joins_as_top_level,
            dialect: self.0.dialect.into(),
        };
        if let Some(p) = params {
            if let PyQueryParamsLike::PyQueryParams(p) = p {
                Ok(sqlformat::format(sql, &p.0, &opts))
            } else {
                let py_params = RySqlfmtQueryParams::py_new(Some(p))?;
                Ok(sqlformat::format(sql, &py_params.0, &opts))
            }
        } else {
            let nada = QueryParams::None;
            Ok(sqlformat::format(sql, &nada, &opts))
        }
    }

    #[pyo3(signature = (sql, params = None))]
    fn __call__(&self, sql: &str, params: Option<PyQueryParamsLike>) -> PyResult<String> {
        self.fmt(sql, params)
    }
}

impl std::fmt::Debug for RySqlFormatter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO: exclude defaults from output

        // indent: PyIndent,
        // uppercase: Option<bool>,
        // lines_between_queries: u8,
        // ignore_case_convert: Option<Vec<String>>,
        // inline: bool,
        // max_inline_block: usize,
        // max_inline_arguments: Option<usize>,
        // max_inline_top_level: Option<usize>,
        // joins_as_top_level: bool,
        // dialect: PyDialect,
        write!(f, "SqlFormatter(")?;
        write!(f, "indent={:?}, ", self.0.indent)?;
        if let Some(uc) = self.0.uppercase {
            if uc {
                write!(f, "uppercase=True, ")?;
            } else {
                write!(f, "uppercase=False, ")?;
            }
        }
        write!(
            f,
            "lines_between_queries={}, ",
            self.0.lines_between_queries
        )?;
        if let Some(v) = &self.0.ignore_case_convert
            && !v.is_empty()
        {
            write!(f, "ignore_case_convert={v:?}, ")?;
        }
        if self.0.inline {
            write!(f, "inline=True, ")?;
        } else {
            write!(f, "inline=False, ")?;
        }
        write!(f, "max_inline_block={}, ", self.0.max_inline_block)?;
        if let Some(v) = self.0.max_inline_arguments {
            write!(f, "max_inline_arguments={v}, ")?;
        }
        if let Some(v) = self.0.max_inline_top_level {
            write!(f, "max_inline_top_level={v}, ")?;
        }
        if self.0.joins_as_top_level {
            write!(f, "joins_as_top_level=True, ")?;
        } else {
            write!(f, "joins_as_top_level=False, ")?;
        }

        write!(f, "dialect=\"{}\"", self.0.dialect)?;
        f.write_str(")")
    }
}
