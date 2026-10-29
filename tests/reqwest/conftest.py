"""tests/reqwest/conftest.py

Test-server is based on but not the same as the test-server in the httpx tests
"""

from __future__ import annotations

import asyncio
import contextlib
import email.parser
import email.policy
import gzip
import json
import threading
import time
import typing as t
from asyncio import sleep as aiosleep
from collections.abc import (
    AsyncGenerator,
    Awaitable,
    Callable,
    Coroutine,
    Generator,
    Iterator,
)

import pytest
from uvicorn import _types as uvt
from uvicorn.config import Config
from uvicorn.server import Server

import ry

TClient: t.TypeAlias = ry.Client | ry.BlockingClient
Receive: t.TypeAlias = t.Callable[[], Awaitable[dict[str, t.Any]]]
Send: t.TypeAlias = t.Callable[[uvt.ASGISendEvent], Coroutine[None, None, None]]
Scope: t.TypeAlias = dict[str, t.Any]


async def echo(
    scope: uvt.HTTPScope, receive: Receive, send: Send
) -> AsyncGenerator[uvt.ASGISendEvent]:
    body = b""
    more_body = True
    while more_body:
        message = await receive()
        body += message.get("body", b"")
        more_body = message.get("more_body", False)
    data_body_dict = {
        "method": scope["method"],
        "path": scope["path"],
        "query_string": scope["query_string"].decode(),
        "headers": {k.decode(): v.decode() for k, v in scope["headers"]},
        "body": body.decode(),
    }
    body = json.dumps(data_body_dict).encode()

    yield uvt.HTTPResponseStartEvent(
        type="http.response.start",
        status=200,
        headers=[
            (b"content-type", b"application/json"),
            (b"content-length", str(len(body)).encode()),
            (b"x-request-method", scope["method"].encode()),
        ],
    )
    yield uvt.HTTPResponseBodyEvent(
        type="http.response.body",
        body=body,
        more_body=False,
    )


async def cookie_monster(
    scope: uvt.HTTPScope, receive: Receive, send: Send
) -> AsyncGenerator[uvt.ASGISendEvent]:
    """Route for testing cookies

    This route will set a cookie and return the cookie in the response
    """
    assert scope["method"] == "GET"
    headers = {k.decode(): v.decode() for k, v in scope.get("headers", [])}
    cookie = headers.get("cookie", "")
    cookie_dict = {}
    if cookie:
        for c in cookie.split(";"):
            k, v = c.split("=")
            cookie_dict[k.strip()] = v.strip()

    yield uvt.HTTPResponseStartEvent(
        type="http.response.start",
        status=200,
        headers=[
            (b"content-type", b"application/json"),
            (b"set-cookie", b"ryo3=ryo3; Path=/"),
        ],
    )
    yield uvt.HTTPResponseBodyEvent(
        type="http.response.body",
        body=json.dumps({"message": "Cookie set", "cookie": cookie_dict}).encode(),
    )


async def four_oh_four(
    scope: uvt.HTTPScope, receive: Receive, send: Send
) -> AsyncGenerator[uvt.ASGISendEvent]:
    yield uvt.HTTPResponseStartEvent(
        type="http.response.start",
        status=404,
        headers=[(b"content-type", b"text/plain")],
    )
    yield uvt.HTTPResponseBodyEvent(
        type="http.response.body",
        body=b"Not Found",
        more_body=False,
    )


async def five_hundred(
    scope: uvt.HTTPScope, receive: Receive, send: Send
) -> AsyncGenerator[uvt.ASGISendEvent]:
    yield uvt.HTTPResponseStartEvent(
        type="http.response.start",
        status=500,
        headers=[(b"content-type", b"text/plain")],
    )
    yield uvt.HTTPResponseBodyEvent(
        type="http.response.body",
        body=b"Internal Server Error",
        more_body=False,
    )


async def howdy(
    scope: uvt.HTTPScope, receive: Receive, send: Send
) -> AsyncGenerator[uvt.ASGISendEvent]:
    body = b'{"howdy": "partner"}'
    yield uvt.HTTPResponseStartEvent(
        type="http.response.start",
        status=200,
        headers=[
            (
                b"content-type",
                b"application/json",
            ),
            (b"content-length", str(len(body)).encode()),
        ],
    )
    yield uvt.HTTPResponseBodyEvent(
        type="http.response.body",
        body=body,
    )


async def slow_response(
    scope: uvt.HTTPScope, receive: Receive, send: Send
) -> AsyncGenerator[uvt.ASGISendEvent]:
    yield uvt.HTTPResponseStartEvent(
        type="http.response.start",
        status=200,
        headers=[(b"content-type", b"text/plain")],
    )
    for i in range(10):
        body_chunk = f"howdy partner {i}\n".encode()
        yield uvt.HTTPResponseBodyEvent(
            type="http.response.body",
            body=body_chunk,
            more_body=True,
        )

        await aiosleep(0.2)
    yield uvt.HTTPResponseBodyEvent(
        type="http.response.body",
        body=b"",
        more_body=False,
    )


async def loooooooong_response(
    scope: uvt.HTTPScope, receive: Receive, send: Send
) -> AsyncGenerator[uvt.ASGISendEvent]:
    yield uvt.HTTPResponseStartEvent(
        type="http.response.start",
        status=200,
        headers=[(b"content-type", b"text/plain")],
    )
    for i in range(100):
        body_chunk = f"howdy partner {i}\n".encode()
        yield uvt.HTTPResponseBodyEvent(
            type="http.response.body",
            body=body_chunk,
            more_body=True,
        )
    yield uvt.HTTPResponseBodyEvent(
        type="http.response.body",
        body=b"",
        more_body=False,
    )


def parse_multipart(content_type: str, body: bytes) -> list[dict[str, t.Any]]:
    """parse a multipart/form-data with the stdlib

    I forgot that this was builtin!
    """
    msg = email.parser.BytesParser(policy=email.policy.HTTP).parsebytes(
        b"Content-Type: " + content_type.encode() + b"\r\n\r\n" + body
    )
    return [
        {
            "name": part.get_param("name", header="content-disposition"),
            "filename": part.get_filename(),
            "content_type": part.get("content-type"),
            "headers": {k.lower(): str(v) for k, v in part.items()},
            "value": t.cast("bytes", part.get_payload(decode=True) or b"").decode(
                "latin-1"
            ),
        }
        for part in msg.iter_parts()
    ]


async def upload_file(
    scope: uvt.HTTPScope, receive: Receive, send: Send
) -> AsyncGenerator[uvt.ASGISendEvent]:
    assert scope["method"] == "POST"
    headers = {k.decode(): v.decode() for k, v in scope.get("headers", [])}
    content_type = headers.get("content-type", "")

    if not content_type.startswith("multipart/form-data"):
        yield uvt.HTTPResponseStartEvent(
            type="http.response.start",
            status=400,
            headers=[(b"content-type", b"text/plain")],
        )
        yield uvt.HTTPResponseBodyEvent(
            type="http.response.body",
            body=b"",
            more_body=False,
        )
        return

    body = b""
    more_body = True
    while more_body:
        message = await receive()
        body += message.get("body", b"")
        more_body = message.get("more_body", False)
    response_json = json.dumps({
        "received_bytes": len(body),
        "content_type": content_type,
        "content_length": headers.get("content-length"),
        "parts": parse_multipart(content_type, body),
    }).encode()
    yield uvt.HTTPResponseStartEvent(
        type="http.response.start",
        status=200,
        headers=[(b"content-type", b"application/json")],
    )
    yield uvt.HTTPResponseBodyEvent(
        type="http.response.body",
        body=response_json,
        more_body=False,
    )


async def broken_json(
    scope: uvt.HTTPScope, receive: Receive, send: Send
) -> AsyncGenerator[uvt.ASGISendEvent]:
    yield uvt.HTTPResponseStartEvent(
        type="http.response.start",
        status=200,
        headers=[(b"content-type", b"application/json")],
    )
    broken_json = b'{"dog":"dingo","is-dingo":true,"bluey-fam-size":4,"fraction-red-heelers":0.5,"activities":["screwing up the garden","barking at strangers for existing","'
    yield uvt.HTTPResponseBodyEvent(
        type="http.response.body",
        body=broken_json,
        more_body=False,
    )


def big_body(n: int) -> bytes:
    return bytes(range(256)) * (n // 256) + bytes(range(n % 256))


async def big_response(
    scope: uvt.HTTPScope, receive: Receive, send: Send
) -> AsyncGenerator[uvt.ASGISendEvent]:
    """`/big/{n}` and `/big/{n}/gzip`

    TODO: use ry.gzip?
    """
    parts = scope["path"].strip("/").split("/")
    body = big_body(int(parts[1]))
    headers = [(b"content-type", b"application/octet-stream")]
    if parts[-1] == "gzip":
        body = gzip.compress(body)
        headers.append((b"content-encoding", b"gzip"))
    headers.append((b"content-length", str(len(body)).encode()))
    yield uvt.HTTPResponseStartEvent(
        type="http.response.start",
        status=200,
        headers=headers,
    )
    step = 64 * 1024
    for i in range(0, max(len(body), 1), step):
        yield uvt.HTTPResponseBodyEvent(
            type="http.response.body",
            body=body[i : i + step],
            more_body=i + step < len(body),
        )


_SSE_CHUNKS = (
    b": keepalive\n\n",
    b"event: greeting\nid: 1\ndata: howdy\n\n",
    b'data: {"a": 1}\r\n\r',
    b"\nid: 2\ndata: mul",
    b"ti\ndata: line\n\n",
    b"retry: 1500\n\n",
    # never dispatched (no trailing blank line)
    b"data: never-dispatched\n",
)


async def sse_response(
    scope: uvt.HTTPScope, receive: Receive, send: Send
) -> AsyncGenerator[uvt.ASGISendEvent]:
    """`/sse` ~ w/ a `Last-Event-ID` header it sends a single `resumed` event"""
    headers = {k.decode().lower(): v.decode() for k, v in scope["headers"]}
    yield uvt.HTTPResponseStartEvent(
        type="http.response.start",
        status=200,
        headers=[
            (b"content-type", b"text/event-stream"),
            (b"cache-control", b"no-cache"),
        ],
    )
    last_event_id = headers.get("last-event-id")
    chunks = (
        (f"event: resumed\ndata: {last_event_id}\n\n".encode(),)
        if last_event_id is not None
        else _SSE_CHUNKS
    )
    for chunk in chunks:
        yield uvt.HTTPResponseBodyEvent(
            type="http.response.body", body=chunk, more_body=True
        )
    yield uvt.HTTPResponseBodyEvent(
        type="http.response.body", body=b"", more_body=False
    )


_Handler: t.TypeAlias = Callable[
    [uvt.HTTPScope, Receive, Send], AsyncGenerator[uvt.ASGISendEvent]
]
_ROUTES: tuple[tuple[str, _Handler], ...] = (
    ("/echo", echo),
    ("/howdy", howdy),
    ("/500", five_hundred),
    ("/five-hundred", five_hundred),
    ("/long", loooooooong_response),
    ("/slow", slow_response),
    ("/upload", upload_file),
    ("/cookie", cookie_monster),
    ("/broken-json", broken_json),
    ("/big/", big_response),
    ("/sse", sse_response),
)


def router(scope: uvt.HTTPScope, receive: Receive, send: Send) -> _Handler:
    return next(
        (h for prefix, h in _ROUTES if scope["path"].startswith(prefix)),
        four_oh_four,
    )


async def reqtest_server(scope: uvt.HTTPScope, receive: Receive, send: Send) -> None:
    assert scope["type"] == "http"
    handler = router(scope, receive, send)
    async for message in handler(scope, receive, send):
        await send(message)


class ReqtestServer(Server):
    @property
    def url(self) -> ry.URL:
        protocol = "https" if self.config.is_ssl else "http"
        return ry.URL(f"{protocol}://{self.config.host}:{self.config.port}/")

    def install_signal_handlers(self) -> None:
        # Disable the default installation of handlers for signals such as SIGTERM,
        # because it can only be done in the main thread.
        ...  # pragma: nocover

    async def serve(self, sockets: t.Any = None) -> None:
        self.restart_requested = asyncio.Event()

        loop = asyncio.get_event_loop()
        tasks = {
            loop.create_task(super().serve(sockets=sockets)),
            loop.create_task(self.watch_restarts()),
        }
        await asyncio.wait(tasks)

    async def restart(self) -> None:  # pragma: no cover
        # I am not sure where I found this fix, but I believe it was the httpx repo?
        # ------
        # This coroutine may be called from a different thread than the one the
        # server is running on, and from an async environment that's not asyncio.
        # For this reason, an event is used to coordinate with the server
        # instead of calling shutdown()/startup(), and should not make
        # any asyncio-specific operations.
        self.started = False
        self.restart_requested.set()
        while not self.started:  # noqa: ASYNC110
            await aiosleep(0.2)

    async def watch_restarts(self) -> None:  # pragma: no cover
        while True:
            if self.should_exit:
                return

            try:
                await asyncio.wait_for(self.restart_requested.wait(), timeout=0.1)
            except TimeoutError:
                continue

            self.restart_requested.clear()
            await self.shutdown()
            await self.startup()


@contextlib.contextmanager
def serve_in_thread(server: ReqtestServer) -> Generator[ReqtestServer, None, None]:
    thread = threading.Thread(target=server.run)
    thread.start()
    try:
        while not server.started:
            time.sleep(1e-3)
        yield server
    finally:
        server.should_exit = True
        thread.join()


@pytest.fixture(scope="session")
def server() -> Iterator[ReqtestServer]:
    cfg = Config(
        app=reqtest_server,
        host="127.0.0.1",
        port=0,
        lifespan="off",
        loop="asyncio",
        ws="none",
    )
    srv = ReqtestServer(config=cfg)
    with serve_in_thread(srv) as running:
        bound_port = running.servers[0].sockets[0].getsockname()[1]
        running.config.port = bound_port  # make .url work
        yield running


@pytest.fixture(params=[ry.Client, ry.BlockingClient])
def client_cls(request: pytest.FixtureRequest) -> type[TClient]:
    return t.cast("type[TClient]", request.param)


def _main() -> None:
    import sys

    cfg = Config(
        app=reqtest_server,
        host="127.0.0.1",
        port=8000,
        lifespan="off",
        loop="asyncio",
        ws="none",
    )
    srv = ReqtestServer(config=cfg)
    sys.stdout.write(f"Server running at {srv.url}")
    srv.run()


if __name__ == "__main__":
    _main()
