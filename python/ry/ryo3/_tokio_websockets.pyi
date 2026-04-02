"""ryo3-tokio-websockets types"""

import sys
import typing as t
from collections.abc import Generator
from types import TracebackType

from ry import Bytes
from ry._types import Buffer
from ry.ryo3._http import Headers, HttpStatus
from ry.ryo3._url import URL

if sys.version_info >= (3, 12):
    from collections.abc import Buffer as Buffer
else:
    from typing_extensions import Buffer as Buffer

ReadyState: t.TypeAlias = t.Literal[0, 1, 2, 3]

class WebSocketKwargs(t.TypedDict, total=False):
    headers: Headers | dict[str, str] | None
    max_payload_len: int
    frame_size: int
    flush_threshold: int
    close_timeout: float | None

@t.final
class WsMessage(Buffer):
    @t.overload
    def __init__(self, kind: t.Literal["text"], data: str) -> None: ...
    @t.overload
    def __init__(self, kind: t.Literal["binary"], data: Buffer) -> None: ...
    @t.overload
    def __init__(
        self, kind: t.Literal["ping", "pong"], data: Buffer | None = None
    ) -> None: ...
    @t.overload
    def __init__(
        self,
        kind: t.Literal["close"],
        data: None = None,
        code: int = 1_000,
        reason: str | Buffer = "",
    ) -> None: ...
    @staticmethod
    def text(text: str) -> WsMessage:
        """Construct a new text message with the given text data.

        Args:
            text: the text data for the message

        Returns:
            text `WsMessage`
        """
    @staticmethod
    def binary(data: Buffer) -> WsMessage:
        """Construct a new binary message with the given data.

        Args:
            data: readable-buffer data for the message

        Returns:
            binary `WsMessage`
        """
    @staticmethod
    def ping(payload: Buffer | None = None) -> WsMessage:
        """Construct a new ping message with the given optional payload.

        Returns:
            ping `WsMessage`
        """
    @staticmethod
    def pong(payload: Buffer | None = None) -> WsMessage:
        """Construct a new pong message with the given optional payload.

        Returns:
            pong `WsMessage`
        """
    @staticmethod
    def close(code: int = 1_000, reason: str | Buffer | None = None) -> WsMessage:
        """Construct a new close message with the given close-code and reason"""
    @property
    def kind(self) -> t.Literal["text", "binary", "close", "ping", "pong"]:
        """Return the message kind as a string literal."""
    @property
    def is_text(self) -> bool:
        """Return `True` if this is a text message, `False` otherwise"""
    @property
    def is_binary(self) -> bool:
        """Return `True` if this is a binary message, `False` otherwise"""
    @property
    def is_close(self) -> bool:
        """Return `True` if this is a close message, `False` otherwise"""
    @property
    def is_ping(self) -> bool:
        """Return `True` if this is a ping message, `False` otherwise"""
    @property
    def is_pong(self) -> bool:
        """Return `True` if this is a pong message, `False` otherwise"""
    @property
    def data(self) -> str | Bytes:
        """Return the message data as a `str` for text messages or `Bytes` for binary messages"""
    @property
    def payload(self) -> Bytes:
        """Return the message payload as a `Bytes` object for any message kind"""
    @property
    def code(self) -> int | None:
        """Returns the close code as an integer if the close message"""
    @property
    def reason(self) -> str | None:
        """Returns the close reason as a string if the close message"""
    def __bytes__(self) -> bytes:
        """Return message payload as `builtins.bytes`"""

@t.final
class WebSocket:
    def __init__(
        self,
        uri: URL | str,
        *,
        headers: Headers | dict[str, str] | None = None,
        max_payload_len: int = 67_108_864,
        frame_size: int = 4_194_304,
        flush_threshold: int = 8_192,
        close_timeout: float | None = 10.0,
    ) -> None: ...
    @property
    def uri(self) -> str: ...
    @property
    def status(self) -> HttpStatus | None: ...
    @property
    def headers(self) -> Headers | None: ...
    @property
    def closed(self) -> bool: ...
    @property
    def open(self) -> bool: ...
    @property
    def read_state(self) -> ReadyState:
        """Return `WebSocket` ready-state (`0`=CONNECTING, `1`=OPEN, `2`=CLOSING, `3`=CLOSED).

        Based on the `WebSocket.readyState` property from the Web API:
        <https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/readyState>
        """
    @property
    def ready_state(self) -> ReadyState:
        """Alias for `read_state`."""
    def __bool__(self) -> bool:
        """Return `True` if the WebSocket is open, `False` otherwise."""
    def __await__(self) -> Generator[t.Any, t.Any, t.Self]: ...
    def __aiter__(self) -> t.Self: ...
    async def __anext__(self) -> WsMessage: ...
    async def __aenter__(self) -> t.Self: ...
    async def __aexit__(
        self,
        exc_type: type[BaseException] | None,
        exc_val: BaseException | None,
        exc_tb: TracebackType | None,
    ) -> None: ...
    async def recv(self) -> WsMessage: ...
    async def receive(self) -> WsMessage: ...
    async def send(self, message: WsMessage | str | Buffer) -> None: ...
    async def close(
        self, code: int = 1_000, reason: str | Buffer | None = None
    ) -> None:
        """Close the WebSocket connection.

        Args:
            code: Optional close code (default: `1000`=NORMAL_CLOSURE)
            reason: Optional close reason (max length: 123 bytes)
        """
    async def ping(self, payload: Buffer | None = None) -> None: ...
    async def pong(self, payload: Buffer | None = None) -> None: ...

def websocket(
    uri: URL | str,
    *,
    headers: Headers | dict[str, str] | None = None,
    close_timeout: float | None = 10,
    flush_threshold: int = 8_192,
    frame_size: int = 4_194_304,
    max_payload_len: int = 67_108_864,
) -> WebSocket: ...
