from __future__ import annotations

import asyncio
import base64
import contextlib
import hashlib
import types
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
    assert ws.closed is False
    assert ws.open is False
    assert ws.ready_state == 0  # CONNECTING
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


async def test_close_never_opened(websocket_server: WsTestServer) -> None:
    ws = ry.websocket(websocket_server.url("/echo"))
    assert ws.ready_state == 0
    await ws.close()
    assert ws.ready_state == 3  # type: ignore[comparison-overlap]
    assert ws.closed
    assert websocket_server.requests == []  # never connected bummer


async def test_close_already_closed_is_noop(websocket_server: WsTestServer) -> None:
    async with ry.websocket(websocket_server.url("/echo")) as ws:
        pass
    assert ws.ready_state == 3
    await ws.close()
    await ws.close()
    assert ws.ready_state == 3


@pytest.mark.parametrize("read_close_frame", [True, False])
async def test_async_with_exit_after_server_closed(
    websocket_server: WsTestServer, *, read_close_frame: bool
) -> None:
    async with ry.websocket(websocket_server.url("/push-close")) as ws:
        if read_close_frame:
            kinds = [(await ws.recv()).kind for _ in range(3)]
            assert kinds == ["text", "binary", "close"]
            assert ws.ready_state == 3
        else:
            # server already closed; exit does the close w/o reading shit
            await asyncio.sleep(0.05)
            assert ws.ready_state == 1
    assert ws.ready_state == 3
    assert ws.closed


async def test_websocket_send_parse_json(websocket_server: WsTestServer) -> None:
    ws = ry.websocket(websocket_server.url("/echo?mode=roundtrip"))
    await ws

    await ws.send(ry.stringify({"data": 123}).decode())
    msg1 = await ws.recv()
    assert msg1.is_text
    assert msg1.data == '{"data":123}'
    assert msg1.json() == {"data": 123}


async def test_cancelled_recv_releases_underlying_conn_thingy(
    websocket_server: WsTestServer,
) -> None:
    """test that we dont get in bad state w/ cancel"""
    ws = ry.websocket(websocket_server.url("/echo"))
    await ws
    recv_task = asyncio.create_task(ws.recv())
    await asyncio.sleep(0)
    recv_task.cancel()

    with pytest.raises(asyncio.CancelledError):
        await recv_task

    await ws.send("still-open")
    msg = await asyncio.wait_for(ws.recv(), timeout=1)
    assert msg.data == "still-open"
    await ws.close()


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


_WS_GUID = b"258EAFA5-E914-47DA-95CA-C5AB0DC85B11"


@contextlib.asynccontextmanager
async def _raw_ws_server(*, handshake: bool) -> t.AsyncGenerator[str]:
    """Handshakes (or not), then ignores everything."""
    writers: list[asyncio.StreamWriter] = []

    async def handle(
        reader: asyncio.StreamReader, writer: asyncio.StreamWriter
    ) -> None:
        writers.append(writer)
        request = await reader.readuntil(b"\r\n\r\n")
        if handshake:
            key = next(
                line.split(b":", 1)[1].strip()
                for line in request.split(b"\r\n")
                if line.lower().startswith(b"sec-websocket-key:")
            )
            accept = base64.b64encode(hashlib.sha1(key + _WS_GUID).digest())  # noqa: S324
            writer.write(
                b"HTTP/1.1 101 Switching Protocols\r\n"
                b"Upgrade: websocket\r\nConnection: Upgrade\r\n"
                b"Sec-WebSocket-Accept: " + accept + b"\r\n\r\n"
            )
            await writer.drain()
        await asyncio.Event().wait()

    server = await asyncio.start_server(handle, "127.0.0.1", 0)
    port = server.sockets[0].getsockname()[1]
    try:
        yield f"ws://127.0.0.1:{port}/"
    finally:
        for writer in writers:
            writer.close()
        server.close()


class TestWebSocketCoroutines:
    """Coroutine shape + cancel behavior."""

    def test_qualnames(self) -> None:
        ws = ry.websocket("ws://127.0.0.1:1/")
        coros: dict[str, t.Any] = {
            "WebSocket.__await__": ws.__await__(),  # noqa: PLC2801
            "WebSocket.__aenter__": ws.__aenter__(),  # noqa: PLC2801
            "WebSocket.__aexit__": ws.__aexit__(None, None, None),  # noqa: PLC2801
            "WebSocket.__anext__": anext(ws),
            "WebSocket.recv": ws.recv(),
            "WebSocket.receive": ws.receive(),
            "WebSocket.send": ws.send("x"),
            "WebSocket.close": ws.close(),
            "WebSocket.ping": ws.ping(),
            "WebSocket.pong": ws.pong(),
        }
        for qualname, coro in coros.items():
            assert coro.__qualname__ == qualname
            assert asyncio.iscoroutine(coro)
            coro.close()
        assert ws.ready_state == 0  # never awaited

    async def test_yield_from_await_adapter(
        self, websocket_server: WsTestServer
    ) -> None:
        @types.coroutine
        def _yield_from(aw: t.Any) -> t.Generator[t.Any, t.Any, t.Any]:
            return (yield from aw.__await__())

        ws = ry.websocket(websocket_server.url("/echo"))
        assert await _yield_from(ws) is ws
        await _yield_from(ws.send("adapter"))
        msg = await _yield_from(ws.recv())
        assert msg.data == "adapter"
        await ws.close()

    async def test_failed_connect_is_closed(self) -> None:
        ws = ry.websocket("ws://127.0.0.1:1/")  # nothing listening
        with pytest.raises(OSError):  # noqa: PT011
            await ws
        assert ws.ready_state == 3

    async def test_cancelled_connect_changes_nothing(self) -> None:
        async with _raw_ws_server(handshake=False) as url:
            ws = ry.websocket(url)

            async def connect() -> None:
                await ws

            task = asyncio.create_task(connect())
            await asyncio.sleep(0.05)  # handshake in flight
            assert ws.ready_state == 0
            task.cancel()
            with pytest.raises(asyncio.CancelledError):
                await task
            assert ws.ready_state == 0

            # can retry
            with pytest.raises(TimeoutError):
                await asyncio.wait_for(connect(), timeout=0.1)
            assert ws.ready_state == 0

    async def test_close_runs_to_the_end_even_if_cancelled(self) -> None:
        async with _raw_ws_server(handshake=True) as url:
            ws = await ry.websocket(url, close_timeout=0.2)
            assert ws.ready_state == 1
            task = asyncio.create_task(ws.close())
            await asyncio.sleep(0.05)
            assert ws.ready_state == 2  # type: ignore[comparison-overlap]
            task.cancel()
            with pytest.raises(asyncio.CancelledError):
                await task
            assert ws.ready_state == 2  # STILLLLLLL closing
            for _ in range(100):
                if ws.ready_state == 3:
                    break
                await asyncio.sleep(0.01)
            assert ws.ready_state == 3  # close_timeout hit
            assert ws.closed

    async def test_cancelled_recv_keeps_socket_usable(
        self, websocket_server: WsTestServer
    ) -> None:
        async with ry.websocket(websocket_server.url("/echo")) as ws:
            for i in range(50):
                task = asyncio.create_task(ws.recv())
                await asyncio.sleep(0)
                task.cancel()
                await asyncio.gather(task, return_exceptions=True)
                await ws.send(f"msg-{i}")
                msg = await asyncio.wait_for(ws.recv(), timeout=1)
                assert msg.data == f"msg-{i}"
