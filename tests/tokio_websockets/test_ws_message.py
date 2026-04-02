from __future__ import annotations

import dataclasses
import pickle

import pytest

import ry


@dataclasses.dataclass
class _WsMessageReprTestCase:
    message: ry.WsMessage
    expected_repr: str


_WS_MSG_TEST_CASES: list[_WsMessageReprTestCase] = [
    _WsMessageReprTestCase(
        message=ry.WsMessage.text("hello"),
        expected_repr='WsMessage.text("hello")',
    ),
    _WsMessageReprTestCase(
        message=ry.WsMessage.binary(b"a-message"),
        expected_repr='WsMessage.binary(b"a-message")',
    ),
    _WsMessageReprTestCase(
        message=ry.WsMessage.ping(b"ping"),
        expected_repr='WsMessage.ping(b"ping")',
    ),
    _WsMessageReprTestCase(
        message=ry.WsMessage.pong(b"pong"),
        expected_repr='WsMessage.pong(b"pong")',
    ),
    _WsMessageReprTestCase(
        message=ry.WsMessage.close(),
        expected_repr='WsMessage.close(code=1000, reason="")',
    ),
    _WsMessageReprTestCase(
        message=ry.WsMessage.close(1001),
        expected_repr='WsMessage.close(code=1001, reason="")',
    ),
    _WsMessageReprTestCase(
        message=ry.WsMessage.close(1001, "thingy"),
        expected_repr='WsMessage.close(code=1001, reason="thingy")',
    ),
    _WsMessageReprTestCase(
        message=ry.WsMessage.close(1000, "normal closure"),
        expected_repr='WsMessage.close(code=1000, reason="normal closure")',
    ),
]


@pytest.mark.parametrize("data", _WS_MSG_TEST_CASES)
def test_ws_message_repr(data: _WsMessageReprTestCase) -> None:
    assert repr(data.message) == data.expected_repr
    # also test that eval(repr(msg)) == msg for good measure
    assert (
        eval(
            repr(data.message),
            {
                "WsMessage": ry.WsMessage,
            },
        )
        == data.message
    )


@pytest.mark.parametrize("data", _WS_MSG_TEST_CASES)
def test_ws_message_pickle(data: _WsMessageReprTestCase) -> None:
    pickled = pickle.dumps(data.message)
    unpickled = pickle.loads(pickled)
    assert unpickled == data.message


class TestWsMessageClose:
    def test_close_default(self) -> None:
        m = ry.WsMessage.close()
        assert m.kind == "close"
        assert m.is_close is True
        assert m.code == 1000
        assert m.reason == ""
        assert bytes(m) == b"\x03\xe8"

    def test_close_msg(self) -> None:
        m1 = ry.WsMessage.close(1000, "normal closure")
        assert m1.kind == "close"
        assert m1.is_close is True
        assert m1.code == 1000
        assert m1.reason == "normal closure"
        assert bytes(m1) == b"\x03\xe8normal closure"

    def test_close_msg_no_reason(self) -> None:
        m2 = ry.WsMessage.close(1001)
        assert m2.kind == "close"
        assert m2.is_close is True
        assert m2.code == 1001
        assert m2.reason == ""
        assert bytes(m2) == b"\x03\xe9"

    def test_close_msg_too_long_reason(self) -> None:
        with pytest.raises(
            ValueError, match="close reason exceeds the websocket limit of 123 byte"
        ):
            ry.WsMessage.close(1000, "reason-too-long: " + "x" * 123)

    def test_close_msg_reserved_code(self) -> None:
        with pytest.raises(
            ValueError, match="close code 1005 is reserved and cannot be sent"
        ):
            ry.WsMessage.close(1005, "reserved code")


class TestWsMessageBinary:
    def test_binary_msg(self) -> None:
        msg = ry.WsMessage.binary(b"\x00\x01\x02")
        assert msg.kind == "binary"
        assert msg.is_binary is True
        assert bytes(msg) == b"\x00\x01\x02"
        assert msg.payload == b"\x00\x01\x02"
        assert msg.data == b"\x00\x01\x02"

    def test_binary_msg_buffer_protocol_read(self) -> None:
        msg = ry.WsMessage.binary(b"das-bytes")
        assert msg.kind == "binary"
        assert msg.is_binary is True
        assert bytes(msg) == b"das-bytes"
        assert msg.payload == b"das-bytes"
        assert msg.data == b"das-bytes"
        memview = memoryview(b"das-bytes")
        assert memview.tobytes() == b"das-bytes"


class TestWsMessageText:
    def test_text_msg(self) -> None:
        msg = ry.WsMessage.text("hello")
        assert msg.kind == "text"
        assert msg.is_text is True
        assert bytes(msg) == b"hello"
        assert msg.payload == b"hello"
        assert msg.data == "hello"


class TestWsMessagePingPong:
    def test_ping_msg(self) -> None:
        msg = ry.WsMessage.ping(b"ping")
        assert msg.kind == "ping"
        assert msg.is_ping is True
        assert bytes(msg) == b"ping"
        assert msg.payload == b"ping"

    def test_pong_msg(self) -> None:
        msg = ry.WsMessage.pong(b"pong")
        assert msg.kind == "pong"
        assert msg.is_pong is True
        assert bytes(msg) == b"pong"
        assert msg.payload == b"pong"

    def test_ws_msg_ping_err(self) -> None:
        with pytest.raises(
            ValueError, match="ping-payload exceeds the websocket limit of 125 byte"
        ):
            ry.WsMessage.ping(b"ping-too-long: " + b"x" * 125)

    def test_ws_msg_pong_err(self) -> None:
        with pytest.raises(
            ValueError, match="pong-payload exceeds the websocket limit of 125 byte"
        ):
            ry.WsMessage.pong(b"pong-too-long: " + b"x" * 125)
