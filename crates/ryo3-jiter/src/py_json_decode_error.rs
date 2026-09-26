use jiter::{JsonError, JsonErrorType, LinePosition};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

/// jiter based `JSONDecodeError` that is (mostly) compatible with python's
/// stdlib `json.JSONDecodeError`
///
/// REF: <https://github.com/python/typeshed/blob/249fa03490c26bf2aa83c8d94136ff33635588c0/stdlib/json/decoder.pyi#L7>
///
/// ```python
/// class JSONDecodeError(ValueError):
///     msg: str
///     doc: str
///     pos: int
///     lineno: int
///     colno: int
///     def __init__(self, msg: str, doc: str, pos: int) -> None: ...
/// ```
///
/// Differences from `json.JSONDecodeError`:
/// - NOT a subclass of `json.JSONDecodeError` (it is a `ValueError` subclass)
/// - `doc` is `ry.Bytes` (or `None`)
/// - `pos`/`colno` are *byte* indexes (not char indexes)
#[derive(Debug)]
#[pyclass(extends=PyValueError, name="JSONDecodeError", frozen, immutable_type, skip_from_py_object)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
pub struct RyJSONDecodeError {
    err: JsonError,
    position: LinePosition,
    data: Option<ryo3_bytes::Bytes>,
}

impl RyJSONDecodeError {
    #[must_use]
    pub fn new(json_data: &[u8], err: JsonError, data: Option<ryo3_bytes::Bytes>) -> Self {
        let position = err.get_position(json_data);
        Self {
            err,
            position,
            data,
        }
    }

    /// Create the python exception object and wrap it in a `PyErr`
    #[must_use]
    pub fn into_pyerr(self, py: Python<'_>) -> PyErr {
        match Bound::new(py, self) {
            Ok(e) => PyErr::from_value(e.into_any()),
            Err(e) => e,
        }
    }

    fn message(&self) -> String {
        format!(
            "{}: line {} column {} (char {})",
            self.err.error_type, self.position.line, self.position.column, self.err.index
        )
    }
}

#[pymethods]
impl RyJSONDecodeError {
    #[new]
    #[expect(unused_variables)]
    fn py_new(msg: &str, doc: &Bound<'_, PyAny>, pos: usize) -> PyResult<Self> {
        Err(pyo3::exceptions::PyNotImplementedError::new_err(
            "not implemented",
        ))
    }

    fn __str__(&self) -> String {
        self.message()
    }

    fn __repr__(&self) -> String {
        format!("JSONDecodeError({:?})", self.message())
    }

    /// The JSON document being parsed.
    #[getter]
    fn doc(&self) -> Option<ryo3_bytes::RyBytes> {
        self.data.as_ref().map(|b| b.clone().into())
    }

    /// The unformatted error message.
    #[getter]
    fn msg(&self) -> String {
        self.err.error_type.to_string()
    }

    #[getter]
    fn kind<'py>(&self, py: Python<'py>) -> Borrowed<'py, 'py, pyo3::types::PyString> {
        PyJsonErrorType(&self.err.error_type)
            .as_pystr(py)
            .as_borrowed()
    }

    /// The start (byte) index of doc where parsing failed.
    #[getter]
    fn pos(&self) -> usize {
        self.err.index
    }

    /// The line corresponding to pos.
    #[getter]
    fn lineno(&self) -> usize {
        self.position.line
    }

    /// The column corresponding to pos.
    #[getter]
    fn colno(&self) -> usize {
        self.position.column
    }
}

impl From<RyJSONDecodeError> for pyo3::PyErr {
    fn from(e: RyJSONDecodeError) -> Self {
        Python::attach(|py| e.into_pyerr(py))
    }
}

struct PyJsonErrorType<'a>(&'a JsonErrorType);

impl<'py> IntoPyObject<'py> for &PyJsonErrorType<'_> {
    type Target = pyo3::types::PyString;
    type Output = Borrowed<'py, 'py, Self::Target>;
    type Error = std::convert::Infallible;

    #[inline]
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        Ok(self.as_pystr(py).as_borrowed())
    }
}

impl PyJsonErrorType<'_> {
    // pub const fn as_str(&self) -> &'static str {
    //     match self.0 {
    //         JsonErrorType::FloatExpectingInt => "float-expecting-int",
    //         JsonErrorType::DuplicateKey(_) => "duplicate-key",
    //         JsonErrorType::InternalError(_) => "internal-error",
    //         JsonErrorType::EofWhileParsingList => "eof-while-parsing-list",
    //         JsonErrorType::EofWhileParsingObject => "eof-while-parsing-object",
    //         JsonErrorType::EofWhileParsingString => "eof-while-parsing-string",
    //         JsonErrorType::EofWhileParsingValue => "eof-while-parsing-value",
    //         JsonErrorType::ExpectedColon => "expected-colon",
    //         JsonErrorType::ExpectedListCommaOrEnd => "expected-list-comma-or-end",
    //         JsonErrorType::ExpectedObjectCommaOrEnd => "expected-object-comma-or-end",
    //         JsonErrorType::ExpectedSomeIdent => "expected-some-ident",
    //         JsonErrorType::ExpectedSomeValue => "expected-some-value",
    //         JsonErrorType::InvalidEscape => "invalid-escape",
    //         JsonErrorType::InvalidNumber => "invalid-number",
    //         JsonErrorType::NumberOutOfRange => "number-out-of-range",
    //         JsonErrorType::InvalidUnicodeCodePoint => "invalid-unicode-code-point",
    //         JsonErrorType::ControlCharacterWhileParsingString => {
    //             "control-character-while-parsing-string"
    //         }
    //         JsonErrorType::KeyMustBeAString => "key-must-be-a-string",
    //         JsonErrorType::LoneLeadingSurrogateInHexEscape => {
    //             "lone-leading-surrogate-in-hex-escape"
    //         }
    //         JsonErrorType::TrailingComma => "trailing-comma",
    //         JsonErrorType::TrailingCharacters => "trailing-characters",
    //         JsonErrorType::UnexpectedEndOfHexEscape => "unexpected-end-of-hex-escape",
    //         JsonErrorType::RecursionLimitExceeded => "recursion-limit-exceeded",
    //     }
    // }

    fn as_pystr<'py>(&self, py: Python<'py>) -> &'py Bound<'py, pyo3::types::PyString> {
        match self.0 {
            JsonErrorType::FloatExpectingInt => pyo3::intern!(py, "float-expecting-int"),
            JsonErrorType::DuplicateKey(_) => pyo3::intern!(py, "duplicate-key"),
            JsonErrorType::InternalError(_) => pyo3::intern!(py, "internal-error"),
            JsonErrorType::EofWhileParsingList => pyo3::intern!(py, "eof-while-parsing-list"),
            JsonErrorType::EofWhileParsingObject => pyo3::intern!(py, "eof-while-parsing-object"),
            JsonErrorType::EofWhileParsingString => pyo3::intern!(py, "eof-while-parsing-string"),
            JsonErrorType::EofWhileParsingValue => pyo3::intern!(py, "eof-while-parsing-value"),
            JsonErrorType::ExpectedColon => pyo3::intern!(py, "expected-colon"),
            JsonErrorType::ExpectedListCommaOrEnd => {
                pyo3::intern!(py, "expected-list-comma-or-end")
            }
            JsonErrorType::ExpectedObjectCommaOrEnd => {
                pyo3::intern!(py, "expected-object-comma-or-end")
            }
            JsonErrorType::ExpectedSomeIdent => pyo3::intern!(py, "expected-some-ident"),
            JsonErrorType::ExpectedSomeValue => pyo3::intern!(py, "expected-some-value"),
            JsonErrorType::InvalidEscape => pyo3::intern!(py, "invalid-escape"),
            JsonErrorType::InvalidNumber => pyo3::intern!(py, "invalid-number"),
            JsonErrorType::NumberOutOfRange => pyo3::intern!(py, "number-out-of-range"),
            JsonErrorType::InvalidUnicodeCodePoint => {
                pyo3::intern!(py, "invalid-unicode-code-point")
            }
            JsonErrorType::ControlCharacterWhileParsingString => {
                pyo3::intern!(py, "control-character-while-parsing-string")
            }
            JsonErrorType::KeyMustBeAString => pyo3::intern!(py, "key-must-be-a-string"),
            JsonErrorType::LoneLeadingSurrogateInHexEscape => {
                pyo3::intern!(py, "lone-leading-surrogate-in-hex-escape")
            }
            JsonErrorType::TrailingComma => pyo3::intern!(py, "trailing-comma"),
            JsonErrorType::TrailingCharacters => pyo3::intern!(py, "trailing-characters"),
            JsonErrorType::UnexpectedEndOfHexEscape => {
                pyo3::intern!(py, "unexpected-end-of-hex-escape")
            }
            JsonErrorType::RecursionLimitExceeded => pyo3::intern!(py, "recursion-limit-exceeded"),
        }
    }
}
