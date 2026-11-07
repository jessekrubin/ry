#![doc = include_str!("../README.md")]
mod enums;
mod errors;
mod from;
mod ry_message;
mod ry_websocket;
mod types;
use pyo3::prelude::*;

use crate::enums::PyWebSocketMessageKind;
use crate::ry_message::{PyMessageLike, PyPingPayload, PyPongPayload, RyWsMessage};
use crate::ry_websocket::{RyWebSocket, websocket};
mod constants;

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<RyWsMessage>()?;
    m.add_class::<RyWebSocket>()?;
    m.add_function(wrap_pyfunction!(websocket, m)?)?;
    Ok(())
}
