use pyo3::{prelude::*, types::PyString};
use ryo3_core::{py_type_err, py_value_err};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum PyWebSocketMessageKind {
    Text,
    Binary,
    Close,
    Ping,
    Pong,
}

impl PyWebSocketMessageKind {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::Text => "text",
            Self::Binary => "binary",
            Self::Close => "close",
            Self::Ping => "ping",
            Self::Pong => "pong",
        }
    }
}

const WS_MESSAGE_KIND_TYPE_ERR_MSG: &str =
    "Expected a string for message kind (options: 'text', 'binary', 'close', 'ping', 'pong')";
impl<'py> FromPyObject<'_, 'py> for PyWebSocketMessageKind {
    type Error = PyErr;
    fn extract(ob: Borrowed<'_, 'py, PyAny>) -> PyResult<Self> {
        if let Ok(s) = ob.cast_exact::<PyString>() {
            let s = s.to_str()?;
            match s {
                "text" => Ok(Self::Text),
                "binary" => Ok(Self::Binary),
                "close" => Ok(Self::Close),
                "ping" => Ok(Self::Ping),
                "pong" => Ok(Self::Pong),
                _ => py_value_err!(
                    "Invalid message kind: {s} (options: 'text', 'binary', 'close', 'ping', 'pong')"
                ),
            }
        } else {
            py_type_err!("{WS_MESSAGE_KIND_TYPE_ERR_MSG}")
        }
    }
}

impl<'py> IntoPyObject<'py> for PyWebSocketMessageKind {
    type Target = PyString;
    type Output = Borrowed<'py, 'py, Self::Target>;
    type Error = std::convert::Infallible;

    #[inline]
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        let s = match self {
            Self::Text => pyo3::intern!(py, "text"),
            Self::Binary => pyo3::intern!(py, "binary"),
            Self::Close => pyo3::intern!(py, "close"),
            Self::Ping => pyo3::intern!(py, "ping"),
            Self::Pong => pyo3::intern!(py, "pong"),
        }
        .as_borrowed();
        Ok(s)
    }
}
