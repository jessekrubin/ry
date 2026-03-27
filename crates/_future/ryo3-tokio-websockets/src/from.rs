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
