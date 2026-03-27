use crate::py_message::{PyPingPayload, PyPongPayload};
use crate::{PyMessageLike, PyWebSocketMessage};
use tokio_websockets::Message;

impl From<Message> for PyWebSocketMessage {
    fn from(value: Message) -> Self {
        Self(value)
    }
}

impl From<PyWebSocketMessage> for Message {
    fn from(value: PyWebSocketMessage) -> Self {
        value.0
    }
}
impl From<PyMessageLike> for Message {
    fn from(value: PyMessageLike) -> Self {
        match value {
            PyMessageLike::Message(msg) => msg.0,
            PyMessageLike::Text(text) => Message::text(text.to_string()),
            PyMessageLike::Bytes(bytes) => Message::binary(bytes.into_inner()),
        }
    }
}

impl From<PyPingPayload> for PyWebSocketMessage {
    fn from(value: PyPingPayload) -> Self {
        Self(value.into_inner())
    }
}

impl From<PyPongPayload> for PyWebSocketMessage {
    fn from(value: PyPongPayload) -> Self {
        Self(value.into_inner())
    }
}

impl From<PyPingPayload> for Message {
    fn from(value: PyPingPayload) -> Self {
        value.into_inner()
    }
}

impl From<PyPongPayload> for Message {
    fn from(value: PyPongPayload) -> Self {
        value.into_inner()
    }
}
