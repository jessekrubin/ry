use jiter::{JsonErrorType, LinePosition};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

#[derive(Debug)]
#[pyclass(extends=PyValueError, name="JSONDecodeError", immutable_type, skip_from_py_object)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
pub struct RyJSONDecodeError {
    err: jiter::JsonError,
    data: Option<ryo3_bytes::Bytes>,
    position: Option<jiter::LinePosition>,
}

// class JSONDecodeError(ValueError):
//     msg: str
//     doc: str
//     pos: int
//     lineno: int
//     colno: int
//     def __init__(self, msg: str, doc: str, pos: int) -> None: ...
#[pymethods]
impl RyJSONDecodeError {
    #[new]
    fn py_new(msg: &str, doc: PyJsonDoc, pos: usize) -> PyResult<()> {
        Err(pyo3::exceptions::PyNotImplementedError::new_err(
            "not implemented",
        ))
    }

    #[getter]
    fn doc(&self) -> Option<ryo3_bytes::RyBytes> {
        if let Some(b) = self.data.as_ref() {
            Some(b.clone().into())
        } else {
            None
        }
        // self.data.map(|b| b.clone().into())
    }

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

    #[getter]
    fn pos<'py>(&self) -> usize {
        self.err.index
    }
    // fn lineno(&mut self) -> Option<usize> {
    //     self.position.map(|p| p.line)
    // }

    // fn colno(&self) -> Option<usize> {
    //     self.position.map(|p| p.column)
    // }
}
// msg
// The unformatted error message.

// doc
// The JSON document being parsed.

// pos
// The start index of doc where parsing failed.

// lineno
// The line corresponding to pos.

// colno
// The column corresponding to pos.

struct PyJsonErrorType<'a>(&'a JsonErrorType);

impl PyJsonErrorType<'_> {
    fn as_str(&self) -> &'static str {
        match self.0 {
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
            JsonErrorType::LoneLeadingSurrogateInHexEscape => {
                "lone-leading-surrogate-in-hex-escape"
            }
            JsonErrorType::TrailingComma => "trailing-comma",
            JsonErrorType::TrailingCharacters => "trailing-characters",
            JsonErrorType::UnexpectedEndOfHexEscape => "unexpected-end-of-hex-escape",
            JsonErrorType::RecursionLimitExceeded => "recursion-limit-exceeded",
        }
    }

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

impl<'py> IntoPyObject<'py> for &PyJsonErrorType<'_> {
    type Target = pyo3::types::PyString;
    type Output = Borrowed<'py, 'py, Self::Target>;
    type Error = std::convert::Infallible;

    #[inline]
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        Ok(self.as_pystr(py).as_borrowed())
    }
}

// impl<'py> IntoPyObject<'py> for PyHttpMethod {
//     type Target = PyString;
//     type Output = Borrowed<'py, 'py, Self::Target>;
//     type Error = PyErr;

//     fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
//         (&self).into_pyobject(py)
//     }
// }

enum PyJsonDoc {
    Bytes(ryo3_bytes::Bytes),
    PyString(pyo3::types::PyString),
    PyBytes(pyo3::types::PyBytes),
}

impl<'a, 'py> FromPyObject<'a, 'py> for PyJsonDoc {
    type Error = PyErr;

    fn extract(ob: Borrowed<'a, 'py, pyo3::PyAny>) -> PyResult<Self> {
        todo!()
    }
}
