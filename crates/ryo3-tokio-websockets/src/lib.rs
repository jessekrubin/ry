#![doc = include_str!("../README.md")]
mod enums;
mod errors;
mod from;
mod py_message;
mod py_websocket;
mod types;
use pyo3::prelude::*;

use crate::enums::PyWebSocketMessageKind;
use crate::py_message::{PyMessageLike, PyPingPayload, PyPongPayload, PyWsMessage};
use crate::py_websocket::{PyWebSocket, websocket};
mod constants;

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyWsMessage>()?;
    m.add_class::<PyWebSocket>()?;
    m.add_function(wrap_pyfunction!(websocket, m)?)?;
    Ok(())
}
