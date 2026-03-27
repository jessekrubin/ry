use bytes::Bytes;
use futures_util::{
    SinkExt, StreamExt,
    stream::{SplitSink, SplitStream},
};
use crate::PyWebSocketMessage;
use http::Uri;
use pyo3::exceptions::{PyEOFError, PyStopAsyncIteration, PyValueError};
use pyo3::{IntoPyObjectExt, prelude::*, pybacked::PyBackedStr, types::PyModule};
use ryo3_bytes::PyBytes as RyBytes;
use ryo3_http::{PyHeaders, PyHeadersLike};
use ryo3_url::UrlLike;
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use tokio_websockets::{
    ClientBuilder, CloseCode, Config, Error, Limits, MaybeTlsStream, Message, WebSocketStream,
};


impl From<Message> for PyWebSocketMessage {
    fn from(value: Message) -> Self {
        Self { inner: value }
    }
}

impl From<PyWebSocketMessage> for Message {
    fn from(value: PyWebSocketMessage) -> Self {
        value.inner
    }
}
