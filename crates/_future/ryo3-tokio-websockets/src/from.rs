use tokio_websockets::{CloseCode, Message};

use crate::{
    PyMessageLike, PyPingPayload, PyPongPayload, PyWebSocketMessageKind, PyWsMessage,
    py_websocket::WebSocketReadyState, types::PyWsCloseCode,
};

impl From<Message> for PyWsMessage {
    fn from(value: Message) -> Self {
        Self(value)
    }
}

impl From<PyWsMessage> for Message {
    fn from(value: PyWsMessage) -> Self {
        value.0
    }
}
impl From<PyMessageLike> for Message {
    fn from(value: PyMessageLike) -> Self {
        match value {
            PyMessageLike::Message(msg) => msg.0,
            // boom opt trick use `bytes::Bytes::from_owner` to avoid copy
            // TODO: bypass the internal message utf8 check bc python should guarantee the text is valid utf8
            PyMessageLike::Text(text) => Self::text(bytes::Bytes::from_owner(text)),
            PyMessageLike::Bytes(bytes) => Self::binary(bytes.into_inner()),
        }
    }
}

impl From<PyPingPayload> for PyWsMessage {
    fn from(value: PyPingPayload) -> Self {
        Self(value.into_inner())
    }
}

impl From<PyPongPayload> for PyWsMessage {
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

impl From<&Message> for PyWebSocketMessageKind {
    fn from(message: &Message) -> Self {
        if message.is_text() {
            Self::Text
        } else if message.is_binary() {
            Self::Binary
        } else if message.is_close() {
            Self::Close
        } else if message.is_ping() {
            Self::Ping
        } else if message.is_pong() {
            Self::Pong
        } else {
            unreachable!()
        }
    }
}

impl From<&PyWsMessage> for PyWebSocketMessageKind {
    fn from(message: &PyWsMessage) -> Self {
        (&message.0).into()
    }
}

impl From<CloseCode> for PyWsCloseCode {
    fn from(value: CloseCode) -> Self {
        Self(value)
    }
}

impl From<WebSocketReadyState> for u8 {
    #[inline]
    fn from(state: WebSocketReadyState) -> Self {
        match state {
            WebSocketReadyState::Connecting => 0,
            WebSocketReadyState::Open => 1,
            WebSocketReadyState::Closing => 2,
            WebSocketReadyState::Closed => 3,
        }
    }
}
