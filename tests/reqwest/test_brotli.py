from __future__ import annotations

import asyncio
import contextlib
import json
import typing as t

import pytest

import ry

from .conftest import big_body

if t.TYPE_CHECKING:
    from collections.abc import AsyncGenerator, Awaitable, Callable

    from typing_extensions import TypeIs

    from ry.ryo3._reqwest import BlockingResponse, ServerSentEvent

    from .conftest import ReqtestServer

pytestmark = pytest.mark.anyio

Client = ry.Client | ry.BlockingClient
Response: t.TypeAlias = "ry.Response | BlockingResponse"


def _is_async(response: Response) -> TypeIs[ry.Response]:
    return isinstance(response, ry.Response)


async def _get(client: Client, url: str, **kwargs: t.Any) -> Response:
    if isinstance(client, ry.BlockingClient):
        return await asyncio.to_thread(client.get, url, **kwargs)
    return await client.get(url, **kwargs)


async def _bytes(response: Response) -> bytes:
    if _is_async(response):
        return bytes(await response.bytes())
    return bytes(await asyncio.to_thread(response.bytes))


@pytest.mark.parametrize(
    "mode",
    ["valid", "boundary", "empty", "wire-empty", "legacy", "trailing", "redirect"],
)
async def test_decode(
    server: ReqtestServer, client_cls: type[Client], mode: str
) -> None:
    response = await _get(client_cls(), f"{server.url}brotli/{mode}")
    expected = b"" if mode in ("empty", "wire-empty") else big_body(200_000)
    if mode == "legacy":
        expected = b"XXXXXXXXXXYYYYYYYYYY"
    elif mode == "boundary":
        expected = big_body(64 * 1024)
    assert response.status == 200
    assert response.content_encoding is None
    assert response.content_length is None
    assert "content-encoding" not in response.headers
    assert "content-length" not in response.headers
    assert response.headers["x-brotli-test"] == "preserved"
    assert response.version == "HTTP/1.1"
    assert response.remote_addr is not None
    assert str(response.url).endswith(
        "/brotli/valid" if mode == "redirect" else f"/brotli/{mode}"
    )
    assert await _bytes(response) == expected


@pytest.mark.parametrize("mode", ["valid", "unknown"])
async def test_passthrough(
    server: ReqtestServer, client_cls: type[Client], mode: str
) -> None:
    response = await _get(client_cls(brotli=False), f"{server.url}brotli/{mode}")
    expected = bytes(ry.brotli_compress(big_body(200_000)))
    assert response.content_encoding == ("unknown" if mode == "unknown" else "br")
    assert response.content_length == len(expected)
    assert await _bytes(response) == expected


async def test_unknown_encoding(
    server: ReqtestServer, client_cls: type[Client]
) -> None:
    response = await _get(client_cls(), f"{server.url}brotli/unknown")
    assert response.content_encoding == "unknown"
    assert await _bytes(response) == ry.brotli_compress(big_body(200_000))


@pytest.mark.parametrize("mode", ["invalid", "truncated"])
async def test_decode_errors(
    server: ReqtestServer, client_cls: type[Client], mode: str
) -> None:
    response = await _get(client_cls(), f"{server.url}brotli/{mode}")
    with pytest.raises(ry.ReqwestError) as caught:
        await _bytes(response)
    assert caught.value.is_body()
    assert not caught.value.is_timeout()


@pytest.mark.parametrize(
    ("flags", "expected"),
    [
        ((True, True, True, True), "zstd,gzip,deflate,br"),
        ((False, True, False, False), "br"),
        ((True, False, False, False), "gzip"),
        ((False, False, True, True), "zstd,deflate"),
        ((False, False, False, False), None),
    ],
)
async def test_negotiation(
    server: ReqtestServer,
    client_cls: type[Client],
    flags: tuple[bool, bool, bool, bool],
    expected: str | None,
) -> None:
    gzip, brotli, deflate, zstd = flags
    client = client_cls(gzip=gzip, brotli=brotli, deflate=deflate, zstd=zstd)
    response = await _get(client, f"{server.url}echo")
    headers = json.loads(await _bytes(response))["headers"]
    assert headers.get("accept-encoding") == expected


@pytest.mark.parametrize("default", [False, True])
async def test_custom_accept_encoding(
    server: ReqtestServer, client_cls: type[Client], *, default: bool
) -> None:
    client = (
        client_cls(headers={"accept-encoding": "identity"}) if default else client_cls()
    )
    kwargs = (
        {}
        if default
        else {"headers": {"accept-encoding": "identity", "range": "bytes=0-9"}}
    )
    response = await _get(client, f"{server.url}echo", **kwargs)
    assert (
        json.loads(await _bytes(response))["headers"]["accept-encoding"] == "identity"
    )
    # Per-request headers still override a client's defaults.
    response = await _get(
        client, f"{server.url}echo", headers={"accept-encoding": "br"}
    )
    assert json.loads(await _bytes(response))["headers"]["accept-encoding"] == "br"


async def test_stream_and_sse(server: ReqtestServer, client_cls: type[Client]) -> None:
    response = await _get(client_cls(), f"{server.url}brotli/valid")
    chunks: list[ry.Bytes]
    if _is_async(response):
        chunks = [chunk async for chunk in response.stream()]
    else:
        chunks = await asyncio.to_thread(list, response.stream())
    assert b"".join(map(bytes, chunks)) == big_body(200_000)
    response = await _get(client_cls(), f"{server.url}brotli/sse")
    events: list[ServerSentEvent]
    if _is_async(response):
        events = [event async for event in response.sse()]
    else:
        events = await asyncio.to_thread(list, response.sse())
    assert [event.data for event in events] == ["hello", "world"]


async def test_head(server: ReqtestServer, client_cls: type[Client]) -> None:
    client = client_cls()
    url = f"{server.url}brotli/valid"
    if isinstance(client, ry.BlockingClient):
        response: Response = await asyncio.to_thread(client.head, url)
    else:
        response = await client.head(url)
    assert response.content_encoding is None
    assert response.content_length is None
    assert await _bytes(response) == b""


@contextlib.asynccontextmanager
async def _server(
    handler: Callable[[asyncio.StreamReader, asyncio.StreamWriter], Awaitable[None]],
) -> AsyncGenerator[str, None]:
    async with await asyncio.start_server(handler, "127.0.0.1", 0) as server:
        yield f"http://127.0.0.1:{server.sockets[0].getsockname()[1]}/"


async def test_incremental() -> None:
    data = big_body(500_000)
    encoded = bytes(ry.brotli_compress(data, 0))
    finish = asyncio.Event()
    closed = asyncio.Event()

    async def serve(reader: asyncio.StreamReader, writer: asyncio.StreamWriter) -> None:
        try:
            await reader.readuntil(b"\r\n\r\n")
            writer.write(
                b"HTTP/1.1 200 OK\r\ncontent-encoding: br\r\ntransfer-encoding: chunked\r\n\r\n"
            )
            prefix = encoded[:-1]
            writer.write(f"{len(prefix):x}\r\n".encode() + prefix + b"\r\n")
            await writer.drain()
            await finish.wait()
            writer.write(b"1\r\n" + encoded[-1:] + b"\r\n0\r\n\r\n")
            await writer.drain()
        finally:
            writer.close()
            await writer.wait_closed()
            closed.set()

    async with _server(serve) as url:
        try:
            response = await ry.Client().get(url)
            stream = response.stream()
            first = await asyncio.wait_for(anext(stream), 2)
            assert 0 < len(first) < len(data)
            finish.set()
            rest = [bytes(chunk) async for chunk in stream]
            assert bytes(first) + b"".join(rest) == data
        finally:
            finish.set()
            await asyncio.wait_for(closed.wait(), 2)


@pytest.mark.parametrize(
    ("mode", "complete"),
    [
        ("timeout", False),
        ("timeout", True),
        ("read_timeout", False),
        ("read_timeout", True),
        ("close", False),
    ],
)
async def test_stalled_body(mode: str, *, complete: bool) -> None:
    finish = asyncio.Event()
    closed = asyncio.Event()

    async def serve(reader: asyncio.StreamReader, writer: asyncio.StreamWriter) -> None:
        try:
            await reader.readuntil(b"\r\n\r\n")
            prefix = bytes(ry.brotli_compress(b"done")) if complete else b"\x0b"
            writer.write(
                b"HTTP/1.1 200 OK\r\ncontent-encoding: br\r\ntransfer-encoding: chunked\r\n\r\n"
            )
            writer.write(f"{len(prefix):x}\r\n".encode() + prefix + b"\r\n")
            await writer.drain()
            await finish.wait()
        finally:
            writer.close()
            await writer.wait_closed()
            closed.set()

    async with _server(serve) as url:
        try:
            duration = ry.Duration.from_secs_f64(0.1)
            if mode == "timeout":
                client = ry.Client(timeout=duration)
            elif mode == "read_timeout":
                client = ry.Client(read_timeout=duration)
            else:
                client = ry.Client()
            response = await client.get(url)
            if mode == "close":
                stream = response.stream()
                pending = asyncio.ensure_future(anext(stream))
                await asyncio.sleep(0.01)
                await stream.close()
                with pytest.raises(StopAsyncIteration):
                    await asyncio.wait_for(pending, 2)
            else:
                with pytest.raises(ry.ReqwestError) as caught:
                    await response.bytes()
                assert caught.value.is_timeout()
        finally:
            finish.set()
            await asyncio.wait_for(closed.wait(), 2)


async def test_keep_alive_with_trailing_data_and_trailers() -> None:
    connections = 0
    encoded = bytes(ry.brotli_compress(b"hello")) + b"trailing data"
    closed = asyncio.Event()

    async def serve(reader: asyncio.StreamReader, writer: asyncio.StreamWriter) -> None:
        nonlocal connections
        connections += 1
        try:
            for _ in range(2):
                await reader.readuntil(b"\r\n\r\n")
                writer.write(
                    b"HTTP/1.1 200 OK\r\ncontent-encoding: br\r\ntransfer-encoding: chunked\r\n\r\n"
                )
                writer.write(f"{len(encoded):x}\r\n".encode() + encoded + b"\r\n")
                writer.write(b"0\r\nx-test-trailer: value\r\n\r\n")
                await writer.drain()
        finally:
            writer.close()
            await writer.wait_closed()
            closed.set()

    async with _server(serve) as url:
        client = ry.Client()
        for _ in range(2):
            response = await asyncio.wait_for(client.get(url), 2)
            assert await asyncio.wait_for(response.bytes(), 2) == b"hello"
        assert connections == 1
        await asyncio.wait_for(closed.wait(), 2)
