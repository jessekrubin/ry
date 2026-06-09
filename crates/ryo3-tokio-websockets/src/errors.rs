use pyo3::prelude::*;
use ryo3_core::macros::{py_runtime_error, py_value_error};
use tokio_websockets::Error as WsError;

// TODO - CUSTOM WS ERROR
// #[derive(Debug)]
// #[pyclass(extends=PyException, module="ry.ryo3", name="WebSocketError", frozen, immutable_type, skip_from_py_object)]
// pub struct PyWebSocketError {
//     pub(crate) inner: WsError,
// }

pub(crate) fn map_ws_err(err: WsError) -> PyErr {
    match err {
        WsError::AlreadyClosed => py_runtime_error!("websocket is already closed"),
        WsError::CannotResolveHost => py_runtime_error!("cannot resolve host"),
        // should never happen w this impl for py
        WsError::NoUriConfigured => py_runtime_error!("no URI configured for client connection"),

        WsError::Io(ioe) => ioe.into(),
        WsError::Protocol(proto_err) => py_value_error!("websocket protocol error: {proto_err}"),
        WsError::UnsupportedScheme => {
            py_value_error!("unsupported URI scheme (expected 'ws' or 'wss')")
        }

        _ => py_runtime_error!("websocket error: {err}"),
    }
}
