from __future__ import annotations

import asyncio
import base64
import contextlib
import contextvars
import gc
import types
import typing as t
from typing import TYPE_CHECKING

import pytest

import ry

from .conftest import big_body

if TYPE_CHECKING:
    from ry._types import Buffer
    from ry.ryo3._http import HttpMethodLike
    from ry.ryo3._reqwest import RequestKwargs

    from .conftest import ReqtestServer

TClient: t.TypeAlias = ry.Client


@pytest.fixture(params=(ry.Client,))
def client_cls(request: pytest.FixtureRequest) -> type[TClient]:
    return t.cast("type[TClient]", request.param)


@pytest.fixture(params=(ry.Client,))
def client(request: pytest.FixtureRequest) -> TClient:
    return t.cast("TClient", request.param())


@contextlib.asynccontextmanager
async def _raw_server(
    handler: t.Callable[
        [asyncio.StreamReader, asyncio.StreamWriter], t.Awaitable[None]
    ],
) -> t.AsyncGenerator[str]:
    """raw asyncio server for hand-rolled http responses ~ yields the url"""
    async with await asyncio.start_server(handler, "127.0.0.1", 0) as srv:
        port = srv.sockets[0].getsockname()[1]
        yield f"http://127.0.0.1:{port}/"


async def _trun_body(
    reader: asyncio.StreamReader, writer: asyncio.StreamWriter
) -> None:
    """send a body shorter than its content-length then hang up"""
    await reader.readuntil(b"\r\n\r\n")
    writer.write(b"HTTP/1.1 200 OK\r\ncontent-length: 10\r\n\r\nshort")
    await writer.drain()
    writer.close()


@pytest.mark.anyio
async def test_get(server: ReqtestServer, client: TClient) -> None:
    url = server.url
    response = await client.get(str(url) + "howdy")
    assert not response.body_used
    assert response.status_code == 200
    assert response.version == "HTTP/1.1"
    res_text = await response.text()
    assert response.body_used
    assert res_text == '{"howdy": "partner"}'
    assert response.ok
    assert bool(response)
    assert f"{response!r}" == f"<Response [{response.status_code}; {response.url}]>"
    assert response.content_encoding is None
    if response.remote_addr is not None:
        assert isinstance(response.remote_addr, ry.SocketAddr)
    assert response.content_length == len(res_text)


@pytest.mark.anyio
async def test_bytes(server: ReqtestServer, client: TClient) -> None:
    url = server.url
    response = await client.get(str(url) + "howdy")
    assert response.status_code == 200
    assert response.version == "HTTP/1.1"
    res_bin = await response.bytes()
    assert res_bin == b'{"howdy": "partner"}'
    assert response.ok
    assert bool(response)


@pytest.mark.anyio
async def test_get_query(server: ReqtestServer, client: TClient) -> None:
    url = server.url
    query_params = {
        "dog": "dingo",
        "is-dingo": True,
        "bluey-fam-size": 4,
        "fraction-red-heelers": 2 / 4,
    }
    response = await client.fetch(str(url) + "howdy", query=query_params)
    assert response.status_code == 200
    assert response.version == "HTTP/1.1"
    assert not response.redirected
    assert response.status == 200
    assert response.status_text == "OK"
    assert response.status_code == ry.HttpStatus(200)
    res_text = await response.text()
    assert res_text == '{"howdy": "partner"}'

    expected_query = "dog=dingo&is-dingo=true&bluey-fam-size=4&fraction-red-heelers=0.5"
    assert response.url.query == expected_query

    assert isinstance(response.url.query_pairs, tuple)
    assert response.url.query_pairs == (
        ("dog", "dingo"),
        ("is-dingo", "true"),
        ("bluey-fam-size", "4"),
        ("fraction-red-heelers", "0.5"),
    )


@pytest.mark.parametrize(
    ("method", "options"),
    [
        ("get", {}),
        ("post", {}),
        ("put", {}),
        ("delete", {}),
        ("patch", {}),
        ("head", {}),
        ("options", {}),
        ("query", {}),
    ],
)
@pytest.mark.parametrize(
    "use_cls_callable",
    [True, False],
)
async def test_client_methods(
    server: ReqtestServer,
    method: t.Literal[
        "get",
        "post",
        "put",
        "delete",
        "patch",
        "head",
        "options",
        "query",
    ],
    options: RequestKwargs,
    client_cls: type[TClient],
    *,
    use_cls_callable: bool,
) -> None:
    """Test that headers are sent with the request and work good"""
    url = server.url
    client = client_cls()
    if use_cls_callable:
        response = await client(str(url) + "echo", method=method, **options)
    else:
        response = await getattr(client, method)(str(url) + "echo", **options)
    assert response.status_code == 200
    assert response.headers["x-request-method"] == method.upper()
    if method.lower() != "head":
        response_data = await response.json()
        assert response_data["method"].lower() == method


@pytest.mark.anyio
@pytest.mark.parametrize("method", ["post", "put", "patch", "delete", "query"])
@pytest.mark.parametrize(
    "form_data",
    [
        {
            "dog": "dingo",
            "is-dingo": True,
            "bluey-fam-size": 4,
            "fraction-red-heelers": 2 / 4,
        },
        [
            ("dog", "dingo"),
            ("is-dingo", True),
            ("bluey-fam-size", 4),
            ("fraction-red-heelers", 2 / 4),
        ],
    ],
)
async def test_form_data(
    server: ReqtestServer,
    client: TClient,
    method: HttpMethodLike,
    form_data: t.Any,
) -> None:
    url = server.url / "echo"
    form_data = {
        "dog": "dingo",
        "is-dingo": True,
        "bluey-fam-size": 4,
        "fraction-red-heelers": 2 / 4,
    }
    response = await client.fetch(
        url,
        method=method,
        form=form_data,
    )
    assert response.status_code == 200
    assert response.version == "HTTP/1.1"
    assert response.http_version == "HTTP/1.1"
    assert not response.redirected
    assert response.status == 200
    assert response.status_text == "OK"
    assert response.status_code == ry.HttpStatus(200)
    res_json = await response.json()

    expected_body = "dog=dingo&is-dingo=true&bluey-fam-size=4&fraction-red-heelers=0.5"
    assert res_json["body"] == expected_body


@pytest.mark.anyio
async def test_get_query_url_already_has_param(
    server: ReqtestServer, client: TClient
) -> None:
    url = server.url
    query_params = {
        "dog": "dingo",
        "is-dingo": True,
        "bluey-fam-size": 4,
        "fraction-red-heelers": 2 / 4,
    }
    response = await client.fetch(str(url) + "howdy?doggy=bruf", query=query_params)
    assert response.status_code == 200
    assert response.version == "HTTP/1.1"
    assert not response.redirected
    assert response.status == 200
    assert response.status_text == "OK"
    assert response.status_code == ry.HttpStatus(200)
    res_text = await response.text()
    assert res_text == '{"howdy": "partner"}'

    expected_query = (
        "doggy=bruf&dog=dingo&is-dingo=true&bluey-fam-size=4&fraction-red-heelers=0.5"
    )
    assert response.url.query == expected_query

    assert response.url.query_pairs == (
        ("doggy", "bruf"),
        ("dog", "dingo"),
        ("is-dingo", "true"),
        ("bluey-fam-size", "4"),
        ("fraction-red-heelers", "0.5"),
    )


@pytest.mark.anyio
async def test_get_url(server: ReqtestServer, client: TClient) -> None:
    url_str = str(server.url) + "howdy"
    url_obj = ry.URL(url_str)

    response = await client.get(url_obj)
    assert response.status_code == 200
    res_text = await response.text()
    assert res_text == '{"howdy": "partner"}'


@pytest.mark.anyio
async def test_get_json(server: ReqtestServer, client: TClient) -> None:
    url = server.url
    response = await client.get(str(url) + "howdy")
    assert response.status_code == 200
    res_json = await response.json()
    assert res_json == {"howdy": "partner"}
    headers = response.headers
    assert isinstance(headers, ry.Headers)
    assert headers["content-type"] == "application/json"
    headers_dict = dict(headers)
    assert headers_dict["content-type"] == "application/json"


class TestResponseJson:
    @pytest.mark.anyio
    async def test_get_json_broken_is_broken(
        self, server: ReqtestServer, client: TClient
    ) -> None:
        url = server.url / "broken-json"
        response = await client.get(url)
        with pytest.raises(
            ry.JSONDecodeError, match="EOF while parsing a string: line 1 column 153"
        ) as exc_info:
            _data = await response.json()
        assert exc_info.value.doc == ry.Bytes(
            b'{"dog":"dingo","is-dingo":true,"bluey-fam-size":4,"fraction-red-heelers":0.5,"activities":["screwing up the garden","barking at strangers for existing","'
        )

    @pytest.mark.anyio
    async def test_get_json_broken_is_broken_allow_partial(
        self, server: ReqtestServer, client: TClient
    ) -> None:
        url = server.url / "broken-json"
        response = await client.get(url)
        data = await response.json(partial_mode=True)
        expected = {
            "dog": "dingo",
            "is-dingo": True,
            "bluey-fam-size": 4,
            "fraction-red-heelers": 0.5,
            "activities": [
                "screwing up the garden",
                "barking at strangers for existing",
            ],
        }
        assert data == expected


class TestAuthHeaders:
    def _basic_auth_header(self, username: str, password: str | None) -> str:

        _password = password if password is not None else ""
        user_pass = f"{username}:{_password}"
        user_pass_bytes = user_pass.encode("utf-8")
        base64_bytes = base64.b64encode(user_pass_bytes)
        base64_str = base64_bytes.decode("utf-8")
        return f"Basic {base64_str}"

    @pytest.mark.anyio
    @pytest.mark.parametrize(
        "basic_auth",
        [
            ("supersecretuser", "solarwinds123"),
            ("supersecretuser", None),
        ],
    )
    async def test_basic_auth_headers(
        self, server: ReqtestServer, client: TClient, basic_auth: tuple[str, str | None]
    ) -> None:
        url = server.url / "echo"
        response = await client.get(url, basic_auth=basic_auth)
        assert response.status_code == 200
        res_json = await response.json()
        expected_auth_header = self._basic_auth_header(*basic_auth)
        assert res_json["headers"]["authorization"] == expected_auth_header

    @pytest.mark.anyio
    async def test_bearer_auth_headers(
        self, server: ReqtestServer, client: TClient
    ) -> None:
        url = server.url / "echo"
        token = "supersecrettoken"  # noqa: S105
        response = await client.get(url, bearer_auth=token)
        assert response.status_code == 200
        res_json = await response.json()
        expected_auth_header = f"Bearer {token}"
        assert res_json["headers"]["authorization"] == expected_auth_header


class TestStream:
    @pytest.mark.anyio
    async def test_get_bytes_stream(
        self, server: ReqtestServer, client: TClient
    ) -> None:
        url = server.url
        response = await client.get(str(url) + "long")

        expected = "".join([f"howdy partner {i}\n" for i in range(100)]).encode()
        parts = b""
        async for thing in response.bytes_stream():
            parts += thing
        assert parts == expected

    @pytest.mark.anyio
    async def test_stream_ready_chunk(
        self, server: ReqtestServer, client: TClient
    ) -> None:
        response = await client.get(str(server.url) + "long")
        response_stream = response.bytes_stream()

        # let the res eater get ahead of the thing
        parts = b""
        n_ready = 0
        while True:
            await asyncio.sleep(0.01)
            next_chunk = anext(response_stream)
            n_ready += not asyncio.isfuture(next_chunk)
            try:
                parts += await next_chunk
            except StopAsyncIteration:
                break
        expected = "".join([f"howdy partner {i}\n" for i in range(100)]).encode()
        assert parts == expected
        assert n_ready > 0

    @pytest.mark.anyio
    async def test_stream_end_raises_on_await(
        self, server: ReqtestServer, client: TClient
    ) -> None:
        response = await client.get(str(server.url) + "long")
        response_stream = response.bytes_stream()
        _ = [chunk async for chunk in response_stream]
        await asyncio.sleep(0.01)

        # `anext()` gotta raise only when awaiting
        next_chunk = anext(response_stream)
        with pytest.raises(StopAsyncIteration):
            await next_chunk

    @pytest.mark.anyio
    async def test_stream_error_raises_on_await(self, client: TClient) -> None:
        async with _raw_server(_trun_body) as url:
            response = await client.get(url)
            response_stream = response.bytes_stream()
            assert await anext(response_stream) == b"short"
            # only raise here on await
            await asyncio.sleep(0.1)
            next_chunk = anext(response_stream)
            with pytest.raises(ry.ReqwestError):
                await next_chunk

    @pytest.mark.anyio
    async def test_stream_anext_cancel(self, client: TClient) -> None:
        send_second = asyncio.Event()
        second_sent = asyncio.Event()

        async def _two_chunks(
            reader: asyncio.StreamReader, writer: asyncio.StreamWriter
        ) -> None:
            await reader.readuntil(b"\r\n\r\n")
            writer.write(
                b"HTTP/1.1 200 OK\r\ntransfer-encoding: chunked\r\n\r\n5\r\nfirst\r\n"
            )
            await writer.drain()
            await send_second.wait()
            try:
                writer.write(b"6\r\nsecond\r\n0\r\n\r\n")
                await writer.drain()
            except ConnectionError:
                pass
            finally:
                second_sent.set()
                writer.close()

        async with _raw_server(_two_chunks) as url:
            response = await client.get(url)
            response_stream = response.bytes_stream()
            assert await anext(response_stream) == b"first"

            pending = asyncio.ensure_future(anext(response_stream))
            await asyncio.sleep(0)
            assert not pending.done()
            pending.cancel()
            with pytest.raises(asyncio.CancelledError):
                await pending

            # cancelled read must not eat the next chunk
            send_second.set()
            assert await asyncio.wait_for(anext(response_stream), 10) == b"second"
            with pytest.raises(StopAsyncIteration):
                await anext(response_stream)
            await asyncio.wait_for(second_sent.wait(), 10)

    @pytest.mark.anyio
    async def test_stream_async_with(
        self, server: ReqtestServer, client: TClient
    ) -> None:
        response = await client.get(str(server.url) + "long")
        response_stream = response.bytes_stream()
        parts = b""
        async with response_stream as s:
            assert s is response_stream
            async for chunk in s:
                parts += chunk
        expected = "".join([f"howdy partner {i}\n" for i in range(100)]).encode()
        assert parts == expected
        with pytest.raises(StopAsyncIteration):
            await anext(response_stream)

    @pytest.mark.anyio
    async def test_stream_async_with_starts_read_ahead(
        self, server: ReqtestServer, client: TClient
    ) -> None:
        """reader start on __aenter__ ~ first chunk should be ready w/o fut"""
        response = await client.get(str(server.url) + "long")
        response_stream = response.bytes_stream()
        async with response_stream:
            await asyncio.sleep(0.05)
            first = anext(response_stream)
            assert not asyncio.isfuture(first)
            assert await first

    @pytest.mark.anyio
    async def test_stream_close_b4_read(
        self, server: ReqtestServer, client: TClient
    ) -> None:
        response = await client.get(str(server.url) + "long")
        response_stream = response.bytes_stream()
        await response_stream.close()
        with pytest.raises(StopAsyncIteration):
            await anext(response_stream)

    @pytest.mark.anyio
    async def test_stream_async_with_err_closes(
        self, server: ReqtestServer, client: TClient
    ) -> None:
        response = await client.get(str(server.url) + "long")
        response_stream = response.bytes_stream()

        async def _read_then_bail() -> None:
            async with response_stream:
                assert await anext(response_stream)
                msg = "bail"
                raise ValueError(msg)

        with pytest.raises(ValueError, match="bail"):
            await _read_then_bail()
        # closed on the way out even tho it blew up
        with pytest.raises(StopAsyncIteration):
            await anext(response_stream)

    @pytest.mark.anyio
    async def test_stream_close_wakes_pending_anext(self, client: TClient) -> None:
        stall = asyncio.Event()

        async def _headers_and_chill(
            reader: asyncio.StreamReader, writer: asyncio.StreamWriter
        ) -> None:
            await reader.readuntil(b"\r\n\r\n")
            writer.write(b"HTTP/1.1 200 OK\r\ncontent-length: 10\r\n\r\n")
            await writer.drain()
            await stall.wait()
            writer.close()

        async with _raw_server(_headers_and_chill) as url:
            response = await client.get(url)
            response_stream = response.bytes_stream()
            try:
                pending = asyncio.ensure_future(anext(response_stream))
                await asyncio.sleep(0.05)
                # close gotta wake the parked anext, not wait behind it
                await asyncio.wait_for(response_stream.close(), 5)
                with pytest.raises(StopAsyncIteration):
                    await asyncio.wait_for(pending, 5)
            finally:
                stall.set()

    @pytest.mark.anyio
    async def test_stream_min_read_size(
        self, server: ReqtestServer, client: TClient
    ) -> None:
        url = server.url

        response = await client.get(str(url) + "long")
        expected = "".join([f"howdy partner {i}\n" for i in range(100)]).encode()
        expected_length = len(expected)
        parts = b""
        async for thing in response.bytes_stream(expected_length):
            assert (
                len(thing) >= expected_length
                or len(parts) + len(thing) == expected_length
            )
            assert len(thing) != 0, "stream yielded empty chunk"
            parts += thing
        assert parts == expected

    @pytest.mark.anyio
    async def test_stream_min_read_size_one_chunk(
        self, server: ReqtestServer, client: TClient
    ) -> None:
        url = server.url

        response = await client.get(str(url) + "howdy")
        expected = b'{"howdy": "partner"}'
        expected_length = len(expected)
        parts = b""
        async for thing in response.bytes_stream(expected_length):
            assert (
                len(thing) >= expected_length
                or len(parts) + len(thing) == expected_length
            )
            assert len(thing) != 0, "stream yielded empty chunk"
            parts += thing
        assert parts == expected

    @pytest.mark.anyio
    async def test_get_stream(self, server: ReqtestServer, client: TClient) -> None:
        url = server.url
        response = await client.get(str(url) + "long")

        expected = "".join([f"howdy partner {i}\n" for i in range(100)]).encode()
        parts = b""
        async for thing in response.stream():
            parts += thing
        assert parts == expected

    @pytest.mark.anyio
    async def test_get_stream_take_huge_n(
        self, server: ReqtestServer, client: TClient
    ) -> None:
        response = await client.get(str(server.url) + "long")
        expected = "".join([f"howdy partner {i}\n" for i in range(100)]).encode()
        taken = await response.bytes_stream().take(2**62)
        assert b"".join(taken) == expected

    @pytest.mark.anyio
    async def test_get_stream_take_collect(
        self, server: ReqtestServer, client: TClient
    ) -> None:
        url = server.url
        response = await client.get(str(url) + "long")

        expected = "".join([f"howdy partner {i}\n" for i in range(100)]).encode()
        response_stream = response.bytes_stream()

        take1 = await response_stream.take(1)
        take2 = await response_stream.take(2)
        assert len(take1) == 1
        assert len(take2) == 2
        rest = await response_stream.collect()
        joined = b"".join(take1 + take2 + rest)
        assert joined == expected
        expected_len = len(expected) - (
            sum(len(t) for t in take1) + sum(len(t) for t in take2)
        )
        rest_total_inner_len = sum(len(t) for t in rest)
        assert rest_total_inner_len == expected_len

    @pytest.mark.anyio
    async def test_get_stream_readall(
        self, server: ReqtestServer, client: TClient
    ) -> None:
        url = server.url

        response = await client.get(str(url) + "long")
        expected = "".join([f"howdy partner {i}\n" for i in range(100)]).encode()
        collected = await response.bytes_stream().readall()
        assert isinstance(collected, ry.Bytes)
        assert collected == expected

    @pytest.mark.anyio
    @pytest.mark.parametrize("method", ["readall", "take", "collect"])
    async def test_consumers_are_coroutines(
        self, server: ReqtestServer, method: str
    ) -> None:
        response = await ry.Client().get(str(server.url) + "long")
        coro = getattr(response.bytes_stream(), method)()
        assert coro.__qualname__ == f"ResponseStream.{method}"
        assert asyncio.iscoroutine(coro)
        await coro

    @pytest.mark.anyio
    async def test_yield_from_await_adapter(self, server: ReqtestServer) -> None:
        """generator-based adapters do `yield from aw.__await__()` ~ needs `__iter__`"""

        @types.coroutine
        def _yield_from_awaitable(aw: t.Any) -> t.Generator[t.Any, t.Any, t.Any]:
            return (yield from aw.__await__())

        response = await _yield_from_awaitable(
            ry.Client().get(str(server.url) + "echo")
        )
        stream = response.bytes_stream()
        assert await _yield_from_awaitable(stream.__aenter__()) is stream  # noqa: PLC2801
        assert b'"method": "GET"' in bytes(await _yield_from_awaitable(anext(stream)))
        with pytest.raises(StopAsyncIteration):
            await _yield_from_awaitable(anext(stream))

    @pytest.mark.anyio
    async def test_anext_fast_path(self, server: ReqtestServer) -> None:
        # small known-length body ~ prefetched, every step is already done
        response = await ry.Client().get(str(server.url) + "echo")
        stream = response.bytes_stream()
        async with stream as entered:
            assert entered is stream
            chunk = anext(stream)
            assert chunk.__qualname__ == "ResponseStream.__anext__"  # type: ignore[attr-defined]
            assert b'"method": "GET"' in bytes(await chunk)
            with pytest.raises(StopAsyncIteration):
                await anext(stream)

    @pytest.mark.anyio
    async def test_consumed_raises_on_await_not_call(
        self, server: ReqtestServer
    ) -> None:
        response = await ry.Client().get(str(server.url) + "echo")
        await response.bytes()
        coro = response.bytes()  # no raise here...
        with pytest.raises(Exception, match="consumed"):  # noqa: PT011
            await coro  # ...but here raise!

    @pytest.mark.anyio
    async def test_readall_cancel_aborts(self) -> None:
        hung_up = asyncio.Event()

        async def _http_body_hang(
            reader: asyncio.StreamReader, writer: asyncio.StreamWriter
        ) -> None:
            await reader.readuntil(b"\r\n\r\n")
            writer.write(b"HTTP/1.1 200 OK\r\ntransfer-encoding: chunked\r\n\r\n")
            writer.write(b"5\r\nhowdy\r\n")
            await writer.drain()
            # never finish the body; wait for the client to go away
            await reader.read()
            hung_up.set()
            writer.close()

        async with _raw_server(_http_body_hang) as url:
            response = await ry.Client().get(url)
            stream = response.bytes_stream()
            with pytest.raises(TimeoutError):
                await asyncio.wait_for(stream.readall(), 0.2)
            await stream.close()
            del response, stream
            gc.collect()
            await asyncio.wait_for(hung_up.wait(), 5)

    @pytest.mark.anyio
    async def test_err_after_chunks_still_raises(self) -> None:
        """chunks that are ready get glued; a trailing err still comes out"""

        async def _ok_then_oshit(
            reader: asyncio.StreamReader, writer: asyncio.StreamWriter
        ) -> None:
            await reader.readuntil(b"\r\n\r\n")
            # chunked so the client cant prefetch it
            writer.write(b"HTTP/1.1 200 OK\r\ntransfer-encoding: chunked\r\n\r\n")
            for _ in range(8):
                writer.write(b"5\r\nhowdy\r\n")
            await writer.drain()
            writer.close()

        async with _raw_server(_ok_then_oshit) as url:
            response = await ry.Client().get(url)
            got: list[bytes] = []

            async def _drain() -> None:
                async for chunk in response.stream():
                    got.append(bytes(chunk))

            with pytest.raises(ry.ReqwestError):
                await _drain()
            assert b"".join(got) == b"howdy" * 8


class TestPrefetch:
    """small known-length bodies come back w/ the head"""

    @pytest.mark.anyio
    async def test_prefetch_head(self, server: ReqtestServer) -> None:
        client = ry.Client()
        url = str(server.url) + "echo"
        response = await client.get(url)
        assert (await response.json())["method"] == "GET"
        response = await client.get(url)
        assert '"method": "GET"' in await response.text()
        response = await client.get(url)
        assert b'"method": "GET"' in bytes(await response.bytes())

    @pytest.mark.anyio
    async def test_streamed(self, server: ReqtestServer) -> None:
        client = ry.Client()
        url = str(server.url) + "long"
        expected = "".join([f"howdy partner {i}\n" for i in range(100)])
        response = await client.get(url)
        assert await response.text() == expected
        response = await client.get(url)
        assert bytes(await response.bytes()) == expected.encode()

    @pytest.mark.anyio
    async def test_streamed_json(self) -> None:
        async def _chunked_json_body(
            reader: asyncio.StreamReader, writer: asyncio.StreamWriter
        ) -> None:
            await reader.readuntil(b"\r\n\r\n")
            writer.write(b"HTTP/1.1 200 OK\r\ntransfer-encoding: chunked\r\n\r\n")
            for part in (b'{"dingo":', b" true}"):
                writer.write(f"{len(part):x}\r\n".encode() + part + b"\r\n")
            writer.write(b"0\r\n\r\n")
            await writer.drain()
            writer.close()

        async with _raw_server(_chunked_json_body) as url:
            response = await ry.Client().get(url)
            assert await response.json() == {"dingo": True}

    @pytest.mark.anyio
    async def test_broken_json_raises_on_await(self, server: ReqtestServer) -> None:
        response = await ry.Client().get(str(server.url) + "broken-json")
        coro = response.json()
        with pytest.raises(ValueError):  # noqa: PT011
            await coro

    @pytest.mark.anyio
    async def test_small_body_methods(self, server: ReqtestServer) -> None:
        client = ry.Client()
        url = str(server.url) + "howdy"
        expected = bytes(await (await client.get(url)).bytes())
        assert await (await client.get(url)).text() == expected.decode()
        chunks = [bytes(c) async for c in (await client.get(url)).stream()]
        assert b"".join(chunks) == expected

    @pytest.mark.anyio
    async def test_body_used_after_read(self, server: ReqtestServer) -> None:
        response = await ry.Client().get(str(server.url) + "howdy")
        assert not response.body_used
        await response.bytes()
        assert response.body_used

    @pytest.mark.anyio
    async def test_empty_body(self) -> None:
        async def _empty_af_body(
            reader: asyncio.StreamReader, writer: asyncio.StreamWriter
        ) -> None:
            await reader.readuntil(b"\r\n\r\n")
            writer.write(b"HTTP/1.1 204 No Content\r\ncontent-length: 0\r\n\r\n")
            await writer.drain()
            writer.close()

        async with _raw_server(_empty_af_body) as url:
            response = await ry.Client().get(url)
            assert response.status == 204
            assert [c async for c in response.stream()] == []

    @pytest.mark.anyio
    async def test_slow_body_does_not_hold_the_request(self) -> None:
        """the head comes back right away even if the (small) body is slow"""
        send_body = asyncio.Event()

        async def _slow_af_body(
            reader: asyncio.StreamReader, writer: asyncio.StreamWriter
        ) -> None:
            await reader.readuntil(b"\r\n\r\n")
            writer.write(b"HTTP/1.1 200 OK\r\ncontent-length: 5\r\n\r\n")
            await writer.drain()
            await send_body.wait()
            writer.write(b"howdy")
            await writer.drain()
            writer.close()

        async with _raw_server(_slow_af_body) as url:
            response = await asyncio.wait_for(ry.Client().get(url), 2)
            send_body.set()
            assert bytes(await asyncio.wait_for(response.bytes(), 2)) == b"howdy"


async def test_client_headers_req(server: ReqtestServer, client: TClient) -> None:
    """Test that headers are sent with the request and work good"""
    url = server.url
    headers = {"User-Agent": "ry-test", "babydog": "dingo"}
    response = await client.get(str(url) + "echo", headers=headers)
    assert response.status_code == 200
    res_json = await response.json()
    assert res_json["headers"]["user-agent"] == "ry-test"
    assert res_json["headers"]["babydog"] == "dingo"


async def test_client_headers_obj_req(server: ReqtestServer, client: TClient) -> None:
    """Test that headers are sent with the request and work good"""
    url = server.url
    headers = {"User-Agent": "ry-test", "babydog": "dingo"}
    response = await client.get(str(url) + "echo", headers=ry.Headers(headers))
    assert response.status_code == 200
    res_json = await response.json()
    assert res_json["headers"]["user-agent"] == "ry-test"
    assert res_json["headers"]["babydog"] == "dingo"


async def test_client_default_headers_get(
    server: ReqtestServer, client_cls: type[TClient]
) -> None:
    """Test that default headers are sent with the request and work good"""
    url = server.url
    client = client_cls(headers={"User-Agent": "ry-test", "babydog": "dingo"})
    response = await client.get(str(url) + "echo")
    assert response.status_code == 200
    res_json = await response.json()
    assert res_json["headers"]["user-agent"] == "ry-test"
    assert res_json["headers"]["babydog"] == "dingo"


# ---------------------------------------------------------------------------
# POST TESTS ~ POST TESTS ~ POST TESTS ~ POST TESTS ~ POST TESTS ~ POST TESTS
# ---------------------------------------------------------------------------


def _sync_generator_o_bytes() -> t.Generator[bytes, None, None]:
    yield from (b"BABY", b"DOG")


async def _async_generator_o_bytes() -> t.AsyncGenerator[bytes, None]:
    for b in (b"BABY", b"DOG"):
        yield b
        await asyncio.sleep(0)


class _SyncIterator:
    def __init__(self) -> None:
        self._data = (b"BABY", b"DOG")
        self._index = 0

    def __iter__(self) -> _SyncIterator:
        return self

    def __next__(self) -> bytes:
        if self._index >= len(self._data):
            raise StopIteration
        value = self._data[self._index]
        self._index += 1
        return value


class _SyncIterable:
    def __init__(self) -> None:
        self._data = (b"BABY", b"DOG")
        self._index = 0

    def __iter__(self) -> _SyncIterator:
        return _SyncIterator()


class _AsyncIterator:
    def __init__(self) -> None:
        self._data = (b"BABY", b"DOG")
        self._index = 0

    def __aiter__(self) -> _AsyncIterator:
        return self

    async def __anext__(self) -> bytes:
        if self._index >= len(self._data):
            raise StopAsyncIteration
        value = self._data[self._index]
        self._index += 1
        await asyncio.sleep(0)
        return value


class _AsyncIterable:
    def __init__(self) -> None:
        self._data = (b"BABY", b"DOG")
        self._index = 0

    def __aiter__(self) -> _AsyncIterator:
        return _AsyncIterator()


@pytest.mark.parametrize(
    "body",
    [
        b"BABYDOG",
        ry.Bytes(b"BABYDOG"),
        _sync_generator_o_bytes,
        _async_generator_o_bytes,
        _SyncIterable,
        _SyncIterator,
        _AsyncIterable,
        _AsyncIterator,
    ],
)
async def test_client_post(
    server: ReqtestServer,
    body: bytes
    | ry.Bytes
    | t.AsyncGenerator[Buffer]
    | t.AsyncIterable[Buffer]
    | t.AsyncIterator[Buffer]
    | t.Generator[Buffer]
    | t.Iterable[Buffer]
    | t.Iterator[Buffer],
    client: TClient,
) -> None:
    url = server.url
    _body = body() if callable(body) else body
    response = await client.post(str(url) + "echo", body=_body)  # ty:ignore[invalid-argument-type]
    assert response.status_code == 200
    res_json = await response.json()
    assert res_json["body"] == "BABYDOG"


_BODY_VAR: contextvars.ContextVar[str] = contextvars.ContextVar("_BODY_VAR")


class TestClientStreamingBody:
    async def test_client_post_stream_body_invalid_anext(
        self, server: ReqtestServer, client: TClient
    ) -> None:
        class _BadAsyncIterator:
            def __aiter__(self) -> _BadAsyncIterator:
                return self

            def __anext__(self) -> object:
                return object()

        with pytest.raises(
            ry.ReqwestError,
            match=r"TypeError.*__anext__ returned non-awaitable of type 'object'",
        ):
            await client.post(str(server.url) + "echo", body=_BadAsyncIterator())  # type: ignore[arg-type]  # ty: ignore[invalid-argument-type]

    async def test_client_post_stream_body_anext_await_error(
        self, server: ReqtestServer, client: TClient
    ) -> None:
        class _BrokenAwaitable:
            def __await__(self) -> t.Iterator[object]:
                msg = "BABOOM"
                raise RuntimeError(msg)

        class _BadAsyncIterator:
            def __aiter__(self) -> _BadAsyncIterator:
                return self

            def __anext__(self) -> _BrokenAwaitable:
                return _BrokenAwaitable()

        with pytest.raises(ry.ReqwestError, match=r"RuntimeError.*BABOOM"):
            await client.post(
                str(server.url) + "echo",
                body=_BadAsyncIterator(),  # type: ignore[arg-type]  # ty:ignore[invalid-argument-type]
            )

    async def test_client_post_stream_body_await_shitty_iterator(
        self, server: ReqtestServer, client: TClient
    ) -> None:
        class _ListAwaitable:
            def __await__(self) -> list[int]:
                return [1, 2]

        class _ShittyAsyncIterator:
            def __aiter__(self) -> _ShittyAsyncIterator:
                return self

            def __anext__(self) -> _ListAwaitable:
                return _ListAwaitable()

        with pytest.raises(
            ry.ReqwestError,
            match=r"TypeError.*__await__\(\) returned non-iterator of type 'list'",
        ):
            await client.post(
                str(server.url) + "echo",
                body=_ShittyAsyncIterator(),  # type: ignore[arg-type]  # ty:ignore[invalid-argument-type]
            )

    async def test_client_post_stream_body_pump_cancels(
        self, server: ReqtestServer, client: TClient
    ) -> None:
        """cancelling the body pump task (eg loop dtor) fails req w/o nuking the body"""
        second = asyncio.Event()

        async def _body() -> t.AsyncGenerator[bytes, None]:
            yield b"first"
            second.set()
            await asyncio.Event().wait()
            yield b"never"

        async def _cancel_pump() -> None:
            await asyncio.wait_for(second.wait(), 10)
            for task in asyncio.all_tasks():
                if "RequestBodyPump" in repr(task):
                    task.cancel()

        canceller = asyncio.ensure_future(_cancel_pump())
        with pytest.raises(ry.ReqwestError):
            await asyncio.wait_for(
                client.post(str(server.url) + "echo", body=_body()), 10
            )
        await canceller

    async def test_client_post_stream_body_cancel(
        self, server: ReqtestServer, client: TClient
    ) -> None:
        """test cancelling a request cancels a `__anext__`"""
        started = asyncio.Event()
        closed = asyncio.Event()

        async def _stalled_body() -> t.AsyncGenerator[bytes, None]:
            try:
                yield b"BABY"
                started.set()
                await asyncio.Event().wait()
            finally:
                closed.set()

        req = asyncio.ensure_future(
            client.post(str(server.url) + "echo", body=_stalled_body())
        )
        await asyncio.wait_for(started.wait(), 10)
        req.cancel()
        await asyncio.wait_for(closed.wait(), 10)

    async def test_client_post_stream_body_early_response(
        self, client: TClient
    ) -> None:
        """test that the server bailing mid-upload cancels a pending `__anext__`"""
        closed = asyncio.Event()

        async def _stalled_body() -> t.AsyncGenerator[bytes, None]:
            try:
                yield b"BABY"
                await asyncio.Event().wait()
            finally:
                closed.set()

        async def _respond_n_hangup(
            reader: asyncio.StreamReader, writer: asyncio.StreamWriter
        ) -> None:
            # wait for 1st chunk before responding
            await reader.readuntil(b"BABY")
            writer.write(b"HTTP/1.1 200 OK\r\ncontent-length: 5\r\n\r\nhowdy")
            await writer.drain()
            writer.close()

        async with _raw_server(_respond_n_hangup) as url:
            response = await client.post(url, body=_stalled_body())
            assert response.status_code == 200
            await asyncio.wait_for(closed.wait(), 10)

    @pytest.mark.parametrize("n_chunks", [1, 8, 64, 256])
    async def test_client_post_stream_body_backpressure(
        self, server: ReqtestServer, client: TClient, n_chunks: int
    ) -> None:
        chunk = b"x" * (64 * 1024)

        async def _body() -> t.AsyncGenerator[bytes, None]:
            for _ in range(n_chunks):
                yield chunk

        response = await client.post(str(server.url) + "echo", body=_body())
        assert response.status_code == 200
        assert len((await response.json())["body"]) == n_chunks * len(chunk)

    @pytest.mark.parametrize("n_chunks", [1, 8, 64, 256])
    async def test_client_post_stream_body_raises(
        self, server: ReqtestServer, client: TClient, n_chunks: int
    ) -> None:
        chunk = b"x" * (64 * 1024)

        async def _body() -> t.AsyncGenerator[bytes, None]:
            for _ in range(n_chunks):
                yield chunk
            msg = "BABOOM"
            raise RuntimeError(msg)

        with pytest.raises(Exception, match="BABOOM"):
            await client.post(str(server.url) + "echo", body=_body())

    @staticmethod
    async def _echoed(client: ry.Client, url: str, body: t.Any) -> str:
        response = await client.post(url, body=body)
        data = await response.json()
        return t.cast("str", data["body"])

    @pytest.mark.anyio
    async def test_overflows_channel(self, server: ReqtestServer) -> None:
        # more chunks than the channel holds w/o ever awaiting
        async def _body() -> t.AsyncGenerator[bytes]:
            for i in range(500):
                yield f"{i},".encode()

        expected = "".join(f"{i}," for i in range(500))
        url = str(server.url) + "echo"
        assert await self._echoed(ry.Client(), url, _body()) == expected

    @pytest.mark.anyio
    async def test_awaits_mid_body(self, server: ReqtestServer) -> None:
        async def _body() -> t.AsyncGenerator[bytes]:
            yield b"a"
            await asyncio.sleep(0.01)
            yield b"b"
            await asyncio.sleep(0)
            yield b"c"

        url = str(server.url) + "echo"
        assert await self._echoed(ry.Client(), url, _body()) == "abc"

    @pytest.mark.anyio
    async def test_one_task_and_context_across_suspensions(
        self, server: ReqtestServer
    ) -> None:
        # token set before the first suspension + reset after it ~ needs the
        # same Context (and the same task) the whole way thru
        tasks: list[asyncio.Task[t.Any] | None] = []

        async def _body() -> t.AsyncGenerator[bytes]:
            token = _BODY_VAR.set("in-body")
            tasks.append(asyncio.current_task())
            yield b"a"
            await asyncio.sleep(0.01)
            tasks.append(asyncio.current_task())
            _BODY_VAR.reset(token)
            yield b"b"

        _BODY_VAR.set("caller")
        caller = asyncio.current_task()
        url = str(server.url) + "echo"
        assert await self._echoed(ry.Client(), url, _body()) == "ab"
        assert tasks[0] is tasks[1]
        assert tasks[0] is not caller
        # body's sets dont leak into the caller
        assert _BODY_VAR.get() == "caller"

    @staticmethod
    async def _gen_body() -> t.AsyncGenerator[bytes]:
        yield b"a"
        await asyncio.sleep(0.01)
        yield b"b"

    @pytest.mark.anyio
    async def test_made_without_a_loop(self, server: ReqtestServer) -> None:
        # eg built in a plain thread for `run_coroutine_threadsafe`
        client = ry.Client()
        url = str(server.url) + "echo"
        coro = await asyncio.to_thread(lambda: client.post(url, body=self._gen_body()))
        response = await coro
        assert (await response.json())["body"] == "ab"

    @pytest.mark.anyio
    async def test_made_on_other_loop(self, server: ReqtestServer) -> None:
        # made on loop A (now closed), awaited here ~ runs on the awaiting loop
        client = ry.Client()
        url = str(server.url) + "echo"

        async def _make() -> t.Any:
            return client.post(url, body=self._gen_body())

        # annotated bc ty cant solve `asyncio.run`'s `_T` thru `to_thread`
        coro: t.Any = await asyncio.to_thread(asyncio.run, _make())
        response = await coro
        assert (await response.json())["body"] == "ab"

    @pytest.mark.anyio
    async def test_sees_contextvars(self, server: ReqtestServer) -> None:
        async def _body() -> t.AsyncGenerator[bytes]:
            yield _BODY_VAR.get().encode()
            await asyncio.sleep(0)
            yield _BODY_VAR.get().encode()

        _BODY_VAR.set("x")
        url = str(server.url) + "echo"
        assert await self._echoed(ry.Client(), url, _body()) == "xx"

    @pytest.mark.anyio
    async def test_never_awaited_never_pulled(self, server: ReqtestServer) -> None:
        pulled = False

        async def _body() -> t.AsyncGenerator[bytes]:
            nonlocal pulled
            pulled = True
            yield b"nope"

        coro = ry.Client().post(str(server.url) + "echo", body=_body())
        coro.close()
        del coro
        gc.collect()
        await asyncio.sleep(0.01)
        assert not pulled

    @pytest.mark.anyio
    async def test_raises_mid_body(self, server: ReqtestServer) -> None:
        async def _body() -> t.AsyncGenerator[bytes]:
            yield b"a"
            msg = "body boom"
            raise ValueError(msg)

        with pytest.raises(ry.ReqwestError):
            await ry.Client().post(str(server.url) + "echo", body=_body())


@pytest.mark.parametrize(
    "body",
    [
        # Invalid type for body
        12345,
        complex(1, 2),
    ],
)
async def test_client_post_body_err(
    server: ReqtestServer, body: complex | list[str], client: TClient
) -> None:
    url = server.url
    with pytest.raises(
        TypeError,
        match="Expected bytes-like object or an async or sync iterable for request body",
    ):
        _ = await client.post(str(url) + "echo", body=body)  # type: ignore[arg-type]  # ty:ignore[invalid-argument-type]


async def test_client_post_json(server: ReqtestServer, client: TClient) -> None:
    url = server.url
    response = await client.post(str(url) + "echo", json={"body": "BABOOM"})
    assert response.status_code == 200
    res_json = await response.json()
    assert res_json["headers"]["content-type"] == "application/json"
    assert res_json["body"] == '{"body":"BABOOM"}'


@pytest.mark.parametrize(
    "kwargs",
    [
        {"body": b"test", "json": {"a": 1}},
        {"json": {"a": 1}, "body": b"test"},
        {"body": b"test", "form": {"a": 1}},
        {"json": {"a": 1}, "form": {"a": 1}},
        {"body": b"test", "multipart": {"a": 1}},
        {"json": {"a": 1}, "multipart": {"a": 1}},
        {"form": {"a": 1}, "multipart": {"a": 1}},
    ],
)
async def test_client_post_json_and_form_errors(
    server: ReqtestServer, client: TClient, kwargs: dict[str, t.Any]
) -> None:
    url = server.url / "echo"
    if "multipart" in kwargs:
        pytest.skip("multipart not implemented yet")
    with pytest.raises(
        ValueError, match="body, json, form, multipart are mutually exclusive"
    ):
        _response = await client.post(url, **kwargs)


class TestListBody:
    """list/tuple bodies"""

    @pytest.mark.anyio
    @pytest.mark.parametrize("n_chunks", [0, 1, 4, 64, 1024, 1025, 5000])
    @pytest.mark.parametrize("container", [list, tuple])
    async def test_list_body(
        self, server: ReqtestServer, n_chunks: int, container: type
    ) -> None:
        chunks = container(f"chunk{i}-".encode() for i in range(n_chunks))
        response = await ry.Client().post(str(server.url) + "echo", body=chunks)
        res = await response.json()
        assert res["body"] == b"".join(chunks).decode()
        if n_chunks:
            assert res["headers"]["transfer-encoding"] == "chunked"

    @pytest.mark.anyio
    async def test_mixed_buffers(self, server: ReqtestServer) -> None:
        body = [b"abc", bytearray(b"def"), ry.Bytes(b"ghi")]
        response = await ry.Client().post(str(server.url) + "echo", body=body)  # type: ignore[arg-type]
        assert (await response.json())["body"] == "abcdefghi"

    @pytest.mark.anyio
    async def test_bad_item(self, server: ReqtestServer) -> None:
        # raised by `post()` itself, not when the body is sent
        with pytest.raises(TypeError, match="ix 1"):
            _coro = ry.Client().post(str(server.url) + "echo", body=(b"ok", 12345))  # type: ignore[arg-type]  # ty:ignore[invalid-argument-type]


class TestKnownLengthStream:
    """known-length bodies bigger than the prefetch"""

    @pytest.mark.anyio
    @pytest.mark.parametrize("n", [100_000, 1_000_000])
    @pytest.mark.parametrize("encoding", ["identity", "gzip"])
    async def test_stream(self, server: ReqtestServer, n: int, encoding: str) -> None:
        url = str(server.url) + f"big/{n}" + ("/gzip" if encoding == "gzip" else "")
        client = ry.Client()
        response = await client.get(url)
        chunks = [bytes(c) async for c in response.stream()]
        assert b"".join(chunks) == big_body(n)
        assert bytes(await (await client.get(url)).bytes()) == big_body(n)

    @pytest.mark.anyio
    async def test_concurrent_streams(self, server: ReqtestServer) -> None:
        client = ry.Client()

        async def _one(i: int) -> bytes:
            path = f"big/{200_000 + i}" + ("/gzip" if i % 2 else "")
            response = await client.get(str(server.url) + path)
            return b"".join([bytes(c) async for c in response.stream()])

        got = await asyncio.gather(*(_one(i) for i in range(16)))
        assert got == [big_body(200_000 + i) for i in range(16)]


class TestTimeout:
    async def test_client_timeout(
        self, server: ReqtestServer, client_cls: type[TClient]
    ) -> None:
        url = server.url
        client = client_cls(timeout=ry.Duration.from_secs_f64(0.1))

        async def _do_get() -> str:
            res = await client.get(str(url) + "slow")
            assert res.status_code == 200
            return await res.text()

        with pytest.raises(ry.ReqwestError, match="TimedOut"):
            _text = await _do_get()

    async def test_client_timeout_get_both_time(
        self, server: ReqtestServer, client_cls: type[TClient]
    ) -> None:
        """Test that getting both text and bytes at the same time errors"""
        url = server.url
        client = client_cls()
        res = await client.get(str(url) + "slow")
        text_task = asyncio.ensure_future(res.text())
        await asyncio.sleep(0)
        with pytest.raises(ValueError, match="Response already consumed"):
            _bytes_future = await res.bytes()
        text = await text_task
        assert text == "".join([f"howdy partner {i}\n" for i in range(10)])


class TestCookies:
    async def test_client_cookie_jar_cookies_disabled(
        self, server: ReqtestServer, client: TClient
    ) -> None:
        """Test for cookies being set and sent back

        Should not be set in the echo response, as cookies are not enabled
        """
        url = server.url
        response = await client.get(str(url) + "cookies")
        assert response.status_code == 200, f"response: {response}"
        res_json = await response.json()

        header_set_cookie = response.headers["set-cookie"]
        assert header_set_cookie == "ryo3=ryo3; Path=/"

        # send to echo endpoint
        response = await client.get(str(url) + "echo")
        assert response.status_code == 200, f"response: {response}"
        res_json = await response.json()
        assert "cookie" not in res_json["headers"] or res_json["headers"]["cookie"] in (
            None,
            "",
        ), "cookie should not be set in the echo response"

    async def test_client_cookie_jar_cookies_enabled(
        self, server: ReqtestServer, client_cls: type[TClient]
    ) -> None:
        """Test for cookies being set and sent back

        Should be set in the echo response, as cookies are enabled
        """
        url = server.url
        client = client_cls(cookies=True)
        response = await client.get(str(url) + "cookies")
        assert response.status_code == 200, f"response: {response}"
        _res_json = await response.json()

        c = response.cookies
        assert isinstance(c, list)
        assert len(c) == 1

        assert isinstance(c[0], ry.Cookie)
        assert c[0].name == "ryo3"
        assert c[0].value == "ryo3"
        assert c[0].path == "/"

        header_set_cookie = response.headers["set-cookie"]
        assert header_set_cookie == "ryo3=ryo3; Path=/", (
            f"header_set_cookie: {header_set_cookie}"
        )

        # send to echo endpoint
        response = await client.get(str(url) + "echo")
        assert response.status_code == 200, f"response: {response}"
        res_json = await response.json()
        assert res_json["headers"]["cookie"] == "ryo3=ryo3", f"res_json: {res_json}"


class TestTodo:
    def test_response_new_errs(self) -> None:
        with pytest.raises(NotImplementedError):
            _res = ry.Response()  # type: ignore[var-annotated]

    @pytest.mark.anyio
    async def test_post_multipart_not_impl(
        self,
        client: TClient,
    ) -> None:
        with pytest.raises(NotImplementedError):
            _r = await client.post("http://example.com", multipart={"a": 1})

    @pytest.mark.anyio
    async def test_client_fetch_multipart_not_impl(
        self,
        client: TClient,
    ) -> None:
        with pytest.raises(NotImplementedError):
            _r = await client.fetch(
                "http://example.com", method="POST", multipart={"a": 1}
            )

    @pytest.mark.anyio
    async def test_fetch_multipart_not_impl(
        self,
    ) -> None:
        with pytest.raises(NotImplementedError):
            _r = await ry.fetch("http://example.com", method="POST", multipart={"a": 1})
