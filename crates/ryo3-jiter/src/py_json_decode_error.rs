use jiter::{JsonError, JsonErrorType, LinePosition};
use pyo3::IntoPyObjectExt;
use pyo3::exceptions::{PyTypeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::PyTuple;
use ryo3_bytes::ReadableBuffer;

/// `json.JSONDecodeError`-ish error; NOT a subclass of it, `doc` is
/// `Bytes | None` and `pos`/`colno` are byte offsets
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
#[pyclass(extends=PyValueError, name="JSONDecodeError", frozen, immutable_type, skip_from_py_object)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
pub struct RyJSONDecodeError {
    err: JsonError,
    position: LinePosition,
    data: Option<ryo3_bytes::Bytes>,
}

impl RyJSONDecodeError {
    #[must_use]
    pub fn new(json_data: &[u8], err: JsonError) -> Self {
        // no `doc` in slice parsing context
        let position = err.get_position(json_data);
        Self {
            err,
            position,
            data: None,
        }
    }

    /// Create the python exception object and wrap it in a `PyErr`
    #[must_use]
    pub fn into_pyerr(self, py: Python<'_>) -> PyErr {
        let e = match Bound::new(py, self) {
            Ok(e) => e,
            Err(err) => return err,
        };
        if let Err(err) = Self::set_args(&e) {
            return err;
        }
        PyErr::from_value(e.into_any())
    }

    /// `args` is `(message,)` like ye old `json.JSONDecodeError`
    fn set_args(slf: &Bound<'_, Self>) -> PyResult<()> {
        slf.setattr(pyo3::intern!(slf.py(), "args"), (slf.get().message(),))
    }

    fn message(&self) -> String {
        format!(
            "{}: line {} column {} (char {})",
            self.err.error_type, self.position.line, self.position.column, self.err.index
        )
    }
}

impl From<(JsonError, ryo3_bytes::Bytes)> for RyJSONDecodeError {
    fn from(value: (JsonError, ryo3_bytes::Bytes)) -> Self {
        let (err, data) = value;
        let position = err.get_position(&data);
        Self {
            err,
            position,
            data: Some(data),
        }
    }
}

#[pymethods]
impl RyJSONDecodeError {
    #[new]
    #[pyo3(signature = (kind, doc = None, pos = 0, lineno = None, colno = None))]
    fn py_new(
        kind: PyJsonErrorTypeArg,
        doc: Option<ReadableBuffer>,
        pos: usize,
        lineno: Option<usize>,
        colno: Option<usize>,
    ) -> Self {
        let data = doc.map(|d| d.to_bytes());
        let err = JsonError {
            error_type: kind.0,
            index: pos,
        };
        let position = match (lineno, colno, &data) {
            (Some(line), Some(column), _) => LinePosition { line, column },
            (_, _, Some(d)) => err.get_position(d),
            _ => LinePosition {
                line: 1,
                column: pos + 1,
            },
        };
        Self {
            err,
            position,
            data,
        }
    }

    // `BaseException.__init__` errs w/ kwargs and sets `args` to the ctor args
    #[pyo3(signature = (*_args, **_kwargs))]
    fn __init__(
        slf: &Bound<'_, Self>,
        _args: &Bound<'_, PyTuple>,
        _kwargs: Option<&Bound<'_, pyo3::types::PyDict>>,
    ) -> PyResult<()> {
        Self::set_args(slf)
    }

    fn __str__(&self) -> String {
        self.message()
    }

    // no `doc`; could be an entire response body
    fn __repr__(&self) -> String {
        format!(
            "JSONDecodeError({}, pos={}, lineno={}, colno={})",
            PyJsonErrorTypeRef(&self.err.error_type),
            self.err.index,
            self.position.line,
            self.position.column
        )
    }

    // `BaseException.__reduce__` rebuilds from `args` (the message)
    fn __reduce__<'py>(
        slf: &Bound<'py, Self>,
    ) -> PyResult<(Bound<'py, pyo3::types::PyType>, Bound<'py, PyTuple>)> {
        let e = slf.get();
        let args = (
            &PyJsonErrorTypeRef(&e.err.error_type),
            e.doc(),
            e.pos(),
            e.lineno(),
            e.colno(),
        )
            .into_pyobject(slf.py())?;
        Ok((slf.get_type(), args))
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

    #[getter]
    fn kind<'py>(&self, py: Python<'py>) -> Borrowed<'py, 'py, pyo3::types::PyString> {
        PyJsonErrorTypeRef(&self.err.error_type)
            .kind_pystr(py)
            .as_borrowed()
    }
}

impl From<RyJSONDecodeError> for pyo3::PyErr {
    fn from(e: RyJSONDecodeError) -> Self {
        Python::attach(|py| e.into_pyerr(py))
    }
}

struct PyJsonErrorTypeArg(JsonErrorType);
struct PyJsonErrorTypeRef<'a>(&'a JsonErrorType);

impl From<JsonErrorType> for PyJsonErrorTypeArg {
    fn from(value: JsonErrorType) -> Self {
        Self(value)
    }
}

impl<'py> IntoPyObject<'py> for &PyJsonErrorTypeRef<'_> {
    type Target = pyo3::types::PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    #[inline]
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        // `(kind, detail)` for kinds that carry an extra string
        match &self.0 {
            JsonErrorType::DuplicateKey(s) | JsonErrorType::InternalError(s) => {
                let kind = self.kind_pystr(py);
                let msg = s.into_pyobject(py)?;
                (kind, msg).into_bound_py_any(py)
            }
            _ => self.kind_pystr(py).into_bound_py_any(py),
        }
    }
}

impl<'a, 'py> FromPyObject<'a, 'py> for PyJsonErrorTypeArg {
    type Error = PyErr;

    fn extract(obj: Borrowed<'a, 'py, PyAny>) -> Result<Self, Self::Error> {
        if let Ok(s) = obj.extract::<&str>() {
            match s {
                "duplicate-key" => Ok(JsonErrorType::DuplicateKey("???".to_owned())),
                "internal-error" => Ok(JsonErrorType::InternalError("???".to_owned())),
                "float-expecting-int" => Ok(JsonErrorType::FloatExpectingInt),
                "eof-while-parsing-list" => Ok(JsonErrorType::EofWhileParsingList),
                "eof-while-parsing-object" => Ok(JsonErrorType::EofWhileParsingObject),
                "eof-while-parsing-string" => Ok(JsonErrorType::EofWhileParsingString),
                "eof-while-parsing-value" => Ok(JsonErrorType::EofWhileParsingValue),
                "expected-colon" => Ok(JsonErrorType::ExpectedColon),
                "expected-list-comma-or-end" => Ok(JsonErrorType::ExpectedListCommaOrEnd),
                "expected-object-comma-or-end" => Ok(JsonErrorType::ExpectedObjectCommaOrEnd),
                "expected-some-ident" => Ok(JsonErrorType::ExpectedSomeIdent),
                "expected-some-value" => Ok(JsonErrorType::ExpectedSomeValue),
                "invalid-escape" => Ok(JsonErrorType::InvalidEscape),
                "invalid-number" => Ok(JsonErrorType::InvalidNumber),
                "number-out-of-range" => Ok(JsonErrorType::NumberOutOfRange),
                "invalid-unicode-code-point" => Ok(JsonErrorType::InvalidUnicodeCodePoint),
                "control-character-while-parsing-string" => {
                    Ok(JsonErrorType::ControlCharacterWhileParsingString)
                }
                "key-must-be-a-string" => Ok(JsonErrorType::KeyMustBeAString),
                "lone-leading-surrogate-in-hex-escape" => {
                    Ok(JsonErrorType::LoneLeadingSurrogateInHexEscape)
                }
                "trailing-comma" => Ok(JsonErrorType::TrailingComma),
                "trailing-characters" => Ok(JsonErrorType::TrailingCharacters),
                "unexpected-end-of-hex-escape" => Ok(JsonErrorType::UnexpectedEndOfHexEscape),
                "recursion-limit-exceeded" => Ok(JsonErrorType::RecursionLimitExceeded),
                _ => Err(PyTypeError::new_err("Unknown JSON error type")),
            }
            .map(Self)
        } else if let Ok((e_type, s_msg)) = obj.extract::<(&str, String)>() {
            match e_type {
                "duplicate-key" => Ok(JsonErrorType::DuplicateKey(s_msg).into()),
                "internal-error" => Ok(JsonErrorType::InternalError(s_msg).into()),
                _ => Err(PyTypeError::new_err("Unknown JSON error type")),
            }
        } else {
            Err(PyTypeError::new_err("Expected a string"))
        }
    }
}

impl std::fmt::Display for PyJsonErrorTypeRef<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            JsonErrorType::DuplicateKey(s) | JsonErrorType::InternalError(s) => {
                write!(f, "(\"{}\", {s:?})", self.kind_str())
            }
            _ => write!(f, "\"{}\"", self.kind_str()),
        }
    }
}

const fn json_error_type_kebab(err: &JsonErrorType) -> &'static str {
    match err {
        JsonErrorType::FloatExpectingInt => "float-expecting-int",
        JsonErrorType::DuplicateKey(_) => "duplicate-key",
        JsonErrorType::InternalError(_) => "internal-error",
        JsonErrorType::EofWhileParsingList => "eof-while-parsing-list",
        JsonErrorType::EofWhileParsingObject => "eof-while-parsing-object",
        JsonErrorType::EofWhileParsingString => "eof-while-parsing-string",
        JsonErrorType::EofWhileParsingValue => "eof-while-parsing-value",
        JsonErrorType::ExpectedColon => "expected-colon",
        JsonErrorType::ExpectedListCommaOrEnd => "expected-list-comma-or-end",
        JsonErrorType::ExpectedObjectCommaOrEnd => "expected-object-comma-or-end",
        JsonErrorType::ExpectedSomeIdent => "expected-some-ident",
        JsonErrorType::ExpectedSomeValue => "expected-some-value",
        JsonErrorType::InvalidEscape => "invalid-escape",
        JsonErrorType::InvalidNumber => "invalid-number",
        JsonErrorType::NumberOutOfRange => "number-out-of-range",
        JsonErrorType::InvalidUnicodeCodePoint => "invalid-unicode-code-point",
        JsonErrorType::ControlCharacterWhileParsingString => {
            "control-character-while-parsing-string"
        }
        JsonErrorType::KeyMustBeAString => "key-must-be-a-string",
        JsonErrorType::LoneLeadingSurrogateInHexEscape => "lone-leading-surrogate-in-hex-escape",
        JsonErrorType::TrailingComma => "trailing-comma",
        JsonErrorType::TrailingCharacters => "trailing-characters",
        JsonErrorType::UnexpectedEndOfHexEscape => "unexpected-end-of-hex-escape",
        JsonErrorType::RecursionLimitExceeded => "recursion-limit-exceeded",
    }
}

impl PyJsonErrorTypeRef<'_> {
    fn kind_str(&self) -> &'static str {
        json_error_type_kebab(self.0)
    }

    fn kind_pystr<'py>(&self, py: Python<'py>) -> &'py Bound<'py, pyo3::types::PyString> {
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
