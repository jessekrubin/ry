use http::Uri;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use ryo3_bytes::PyBytes as RyBytes;
use ryo3_url::UrlLike;
use tokio_websockets::{CloseCode, Error};
pub(crate) fn map_ws_err(err: Error) -> PyErr {
    PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(err.to_string())
}

pub(crate) fn parse_uri(url: UrlLike) -> PyResult<Uri> {
    url.0
        .as_str()
        .parse::<Uri>()
        .map_err(|err| PyValueError::new_err(format!("invalid websocket uri: {err}")))
}

pub(crate) fn validate_close_reason(
    code: Option<u16>,
    reason: &str,
) -> PyResult<Option<CloseCode>> {
    if reason.len() > 123 {
        return Err(PyValueError::new_err(
            "close reason exceeds the websocket limit of 123 bytes",
        ));
    }
    if !reason.is_empty() && code.is_none() {
        return Err(PyValueError::new_err(
            "a close reason requires a close code",
        ));
    }
    code.map(|code| {
        CloseCode::try_from(code)
            .map_err(|_| PyValueError::new_err(format!("invalid websocket close code: {code}")))
    })
    .transpose()
}

pub(crate) fn validate_control_payload_len(payload: &RyBytes, kind: &str) -> PyResult<()> {
    if payload.as_slice().len() > 125 {
        Err(PyValueError::new_err(format!(
            "{kind} payload exceeds the websocket limit of 125 bytes"
        )))
    } else {
        Ok(())
    }
}
