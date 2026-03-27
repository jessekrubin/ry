use pyo3::prelude::*;
use ryo3_core::{py_runtime_error, py_value_error};
use tokio_websockets::Error;
pub(crate) fn map_ws_err(err: Error) -> PyErr {
    match err {
        Error::AlreadyClosed => py_runtime_error!("websocket is already closed"),
        Error::CannotResolveHost => py_runtime_error!("cannot resolve host"),
        // should never happen w this impl for py
        Error::NoUriConfigured => py_runtime_error!("no URI configured for client connection"),

        Error::Io(ioe) => ioe.into(),
        Error::Protocol(proto_err) => py_value_error!("websocket protocol error: {proto_err}"),
        Error::UnsupportedScheme => {
            py_value_error!("unsupported URI scheme (expected 'ws' or 'wss')")
        }

        _ => py_runtime_error!("websocket error: {err}"),
    }
}
