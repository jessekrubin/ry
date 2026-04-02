use futures_util::stream::{SplitSink, SplitStream};
use pyo3::{prelude::*, pybacked::PyBackedStr};
use ryo3_bytes::PyBytes as RyBytes;
use ryo3_core::{py_type_err, py_value_err, py_value_error};
use ryo3_url::UrlLike;
use tokio::net::TcpStream;
use tokio_websockets::{MaybeTlsStream, Message, WebSocketStream};

use crate::constants::WS_MSG_CLOSE_REASON_MAX_LEN;

pub(crate) type TokioWs = WebSocketStream<MaybeTlsStream<TcpStream>>;
pub(crate) type TokioWsWrite = SplitSink<TokioWs, Message>;
pub(crate) type TokioWsRead = SplitStream<TokioWs>;

// ============================================================================
// URI-like
// ============================================================================
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) struct UriLike(http::Uri);

impl From<UriLike> for http::Uri {
    fn from(u: UriLike) -> Self {
        u.0
    }
}

impl<'py> FromPyObject<'_, 'py> for UriLike {
    type Error = PyErr;
    fn extract(obj: Borrowed<'_, 'py, PyAny>) -> PyResult<Self> {
        let url = obj.extract::<UrlLike>()?;
        url.0
            .as_str()
            .parse::<http::Uri>()
            .map(Self)
            .map_err(|err| py_value_error!("invalid websocket uri: {err}"))
    }
}
// ============================================================================
// PyWsCloseCode
// ============================================================================

/// Python wrapper around `tokio_websockets::CloseCode`
pub(crate) struct PyWsCloseCode(pub(crate) tokio_websockets::CloseCode);

impl PyWsCloseCode {
    pub(crate) const NORMAL_CLOSURE: Self = Self(tokio_websockets::CloseCode::NORMAL_CLOSURE);

    pub(crate) fn is_reserved(&self) -> bool {
        self.0.is_reserved()
    }
}

impl<'py> FromPyObject<'_, 'py> for PyWsCloseCode {
    type Error = PyErr;

    fn extract(obj: Borrowed<'_, 'py, PyAny>) -> PyResult<Self> {
        let code = obj.extract::<u16>()?;
        tokio_websockets::CloseCode::try_from(code)
            .map(Self::from)
            .map_err(|_| py_value_error!("invalid websocket close code: {code}"))
    }
}

// ============================================================================
// PyWsCloseReason
// ============================================================================
/// Close reason new-type to enforce 123 byte limit
#[derive(Default)]
pub(crate) struct PyWsCloseReason(pub(crate) String);

impl<'py> FromPyObject<'_, 'py> for PyWsCloseReason {
    type Error = PyErr;

    fn extract(obj: Borrowed<'_, 'py, PyAny>) -> PyResult<Self> {
        if let Ok(s) = obj.extract::<PyBackedStr>() {
            let s = s.to_string();
            if s.len() > WS_MSG_CLOSE_REASON_MAX_LEN {
                py_value_err!("close reason exceeds the websocket limit of 123 bytes")
            } else {
                Ok(Self(s))
            }
        } else if let Ok(bytes) = obj.extract::<RyBytes>() {
            let bytes = bytes.as_slice();
            if bytes.len() > WS_MSG_CLOSE_REASON_MAX_LEN {
                py_value_err!("close reason exceeds the websocket limit of 123 bytes")
            } else {
                let s = std::str::from_utf8(bytes)
                    .map_err(|_| py_value_error!("close reason must be valid UTF-8"))?;
                Ok(Self(s.to_owned()))
            }
        } else {
            py_type_err!("close reason must be a string or bytes-like object")
        }
    }
}
