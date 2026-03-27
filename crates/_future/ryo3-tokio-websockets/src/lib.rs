#![doc = include_str!("../README.md")]
mod from;
mod py_client;
mod py_message;
mod util;
use crate::py_client::{PyWebSocket, websocket};
use crate::py_message::{PyMessageLike, PyWebSocketMessage};
use pyo3::prelude::*;
mod constants;

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyWebSocketMessage>()?;
    m.add_class::<PyWebSocket>()?;
    m.add_function(wrap_pyfunction!(websocket, m)?)?;
    Ok(())
}
