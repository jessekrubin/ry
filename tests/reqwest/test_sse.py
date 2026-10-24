from __future__ import annotations

import typing as t

import pytest

import ry

if t.TYPE_CHECKING:
    from ry.ryo3 import ServerSentEvent

    from .conftest import ReqtestServer

EXPECTED = [
    ("greeting", "howdy", "1"),
    ("message", '{"a": 1}', "1"),
    ("message", "multi\nline", "2"),
]


def _tuples(events: list[ServerSentEvent]) -> list[tuple[str, str, str | None]]:
    return [(e.event, e.data, e.id) for e in events]


class TestSseAsync:
    @pytest.mark.anyio
    async def test_events(self, server: ReqtestServer) -> None:
        response = await ry.Client().get(str(server.url) + "sse")
        stream = response.sse()
        events = [e async for e in stream]
        assert _tuples(events) == EXPECTED
        assert events[1].json() == {"a": 1}
        assert events[1].bytes == b'{"a": 1}'
        assert isinstance(events[1].bytes, ry.Bytes)
        assert stream.last_event_id == "2"
        assert stream.retry == 1500

    @pytest.mark.anyio
    async def test_ctx_mgr_close(self, server: ReqtestServer) -> None:
        response = await ry.Client().get(str(server.url) + "sse")
        async with response.sse() as stream:
            first = await anext(stream)
            assert first.event == "greeting"
        with pytest.raises(StopAsyncIteration):
            await anext(stream)

    @pytest.mark.anyio
    async def test_reconnect_w_last_event_id(self, server: ReqtestServer) -> None:
        client = ry.Client()
        url = str(server.url) + "sse"
        stream = (await client.get(url)).sse()
        _ = [e async for e in stream]
        assert stream.last_event_id is not None
        response = await client.get(
            url, headers={"Last-Event-ID": stream.last_event_id}
        )
        resumed = await anext(response.sse())
        assert (resumed.event, resumed.data) == ("resumed", "2")

    @pytest.mark.anyio
    async def test_consumed(self, server: ReqtestServer) -> None:
        response = await ry.Client().get(str(server.url) + "sse")
        _stream = response.sse()
        with pytest.raises(ValueError, match="already consumed"):
            response.sse()

    @pytest.mark.anyio
    async def test_max_event_size(self, server: ReqtestServer) -> None:
        response = await ry.Client().get(str(server.url) + "sse")
        stream = response.sse(max_event_size=8)
        assert (await anext(stream)).data == "howdy"
        assert (await anext(stream)).data == '{"a": 1}'
        # "multi\nline" is 10b
        with pytest.raises(ValueError, match="max_event_size"):
            await anext(stream)
        with pytest.raises(ValueError, match="max_event_size"):
            await anext(stream)
        with pytest.raises(StopAsyncIteration):
            await anext(stream)

    @pytest.mark.anyio
    async def test_buffered_body(self, server: ReqtestServer) -> None:
        response = await ry.Client().get(str(server.url) + "howdy")
        assert [e async for e in response.sse()] == []


class TestSseBlocking:
    def test_events(self, server: ReqtestServer) -> None:
        response = ry.BlockingClient().get(str(server.url) + "sse")
        stream = response.sse()
        events = list(stream)
        assert _tuples(events) == EXPECTED
        assert stream.last_event_id == "2"
        assert stream.retry == 1500

    def test_ctx_mgr_close(self, server: ReqtestServer) -> None:
        response = ry.BlockingClient().get(str(server.url) + "sse")
        with response.sse() as stream:
            assert next(stream).event == "greeting"
        with pytest.raises(StopIteration):
            next(stream)


def test_event_repr_eq(server: ReqtestServer) -> None:
    response = ry.BlockingClient().get(str(server.url) + "sse")
    a = next(response.sse())
    b = next(ry.BlockingClient().get(str(server.url) + "sse").sse())
    assert a == b
    assert repr(a) == ('ServerSentEvent(event="greeting", data="howdy", id="1")')
