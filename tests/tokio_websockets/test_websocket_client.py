from __future__ import annotations

import typing as t

import pytest

import ry

if t.TYPE_CHECKING:
    from .conftest import WsTestServer


async def test_ws_ctx_manager_handshake_stuff(websocket_server: WsTestServer) -> None:
    ws = ry.WebSocket(
        websocket_server.url("/echo?mode=roundtrip"),
        headers={"x-client-header": "roundtrip"},
    )

    assert repr(ws) == (
        f"WebSocket(uri={websocket_server.url('/echo?mode=roundtrip')}, open=false)"
    )
    assert bool(ws) is False
    assert ws.uri == websocket_server.url("/echo?mode=roundtrip")
    assert ws.closed is True
    assert ws.open is False
    assert ws.ready_state == 3
    assert ws.status is None
    assert ws.headers is None

    async with ws:
        assert bool(ws) is True
        assert ws.open is True
        assert ws.closed is False
        assert ws.ready_state == 1
        assert ws.status == ry.HttpStatus(101)
        assert ws.headers is not None
        assert ws.headers["x-ws-path"] == "/echo"
        assert ws.headers["x-ws-query"] == "mode=roundtrip"
        assert ws.headers["x-seen-client-header"] == "roundtrip"

        await ws.send("howdy")
        text_msg = await ws.recv()
        assert text_msg.kind == "text"
        assert text_msg.is_text is True
        assert text_msg.data == "howdy"

        await ws.send(b"\x00\x01\x02")
        binary_msg = await ws.receive()
        assert binary_msg.kind == "binary"
        assert binary_msg.is_binary is True
        assert bytes(binary_msg) == b"\x00\x01\x02"

        await ws.ping(b"pi")
        pong_msg = await ws.recv()
        assert pong_msg.kind == "pong"
        assert pong_msg.is_pong is True
        assert bytes(pong_msg) == b"pi"

    assert ws.closed is True
    assert ws.open is False
    assert ws.ready_state == 3
    assert ws.status == ry.HttpStatus(101)

    assert websocket_server.requests[-1].path == "/echo"
    assert websocket_server.requests[-1].query == "mode=roundtrip"
    assert websocket_server.requests[-1].headers["x-client-header"] == "roundtrip"


async def test_websocket_recv_reports_server_messages_and_close_frame(
    websocket_server: WsTestServer,
) -> None:
    ws = ry.websocket(websocket_server.url("/push-close"))
    await ws

    assert ws.open is True
    assert ws.status == ry.HttpStatus(101)

    text_msg = await ws.recv()
    binary_msg = await ws.recv()
    close_msg = await ws.recv()

    assert text_msg.kind == "text"
    assert text_msg.data == "alpha"
    assert binary_msg.kind == "binary"
    assert bytes(binary_msg) == b"\x00\x01\x02"
    assert close_msg.kind == "close"
    assert close_msg.code == 1000
    assert close_msg.reason == "done"

    assert ws.closed is True
    assert ws.open is False
    assert ws.ready_state == 3


async def test_websocket_send_requires_an_open_connection(
    websocket_server: WsTestServer,
) -> None:
    ws = ry.WebSocket(websocket_server.url("/echo"))

    with pytest.raises(RuntimeError, match="WebSocket not connected"):
        await ws.send("before-connect")


@pytest.mark.xfail(
    raises=RuntimeError,
    reason="__anext__ currently raises RuntimeError after the close frame instead of stopping iteration",
)
async def test_websocket_async_iteration_stops_cleanly_after_close(
    websocket_server: WsTestServer,
) -> None:
    ws = ry.websocket(websocket_server.url("/push-close"))
    await ws

    messages = [message async for message in ws]

    assert [message.kind for message in messages] == ["text", "binary", "close"]


@pytest.mark.parametrize(
    ("close_timeout", "err_type", "err_match"),
    [
        (
            -1.0,
            ValueError,
            "timeout must be a positive-finite-number of seconds",
        ),
        (
            float("inf"),
            ValueError,
            "timeout must be a positive-finite-number of seconds",
        ),
        (
            "invalid-str",
            TypeError,
            "argument 'close_timeout': timeout must be a Duration | datetime.timedelta | positive number of seconds",
        ),
    ],
)
def test_websocket_rejects_invalid_close_timeout(
    close_timeout: t.Any, err_type: type[BaseException], err_match: str
) -> None:
    with pytest.raises(err_type, match=err_match):
        ry.WebSocket("ws://example.com", close_timeout=close_timeout)
