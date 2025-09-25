"""tests/reqwest/conftest.py

Test-server is based on but not the same as the test-server in the httpx tests
"""

from __future__ import annotations

import asyncio
import contextlib
import json
import threading
import time
from asyncio import sleep as aiosleep
from collections.abc import AsyncGenerator, Awaitable, Callable, Coroutine, Iterator
from typing import Any, TypeAlias

import pytest
from uvicorn import _types as uvt
from uvicorn.config import Config
from uvicorn.server import Server

import ry

Receive: TypeAlias = Callable[[], Awaitable[dict[str, Any]]]
Send: TypeAlias = Callable[[uvt.ASGISendEvent], Coroutine[None, None, None]]
Scope: TypeAlias = dict[str, Any]


async def echo(
    scope: Scope, receive: Receive, send: Send
) -> AsyncGenerator[uvt.ASGISendEvent]:
    body = b""
    more_body = True
    while more_body:
        message = await receive()
        body += message.get("body", b"")
        more_body = message.get("more_body", False)

    yield uvt.HTTPResponseStartEvent(
        type="http.response.start",
        status=200,
        headers=[(b"content-type", b"application/json")],
    )
    data_body_dict = {
        "method": scope["method"],
        "path": scope["path"],
        "query_string": scope["query_string"].decode(),
        "headers": {
            name.decode(): value.decode() for name, value in scope.get("headers", [])
        },
        "body": body.decode(),
    }
    yield uvt.HTTPResponseBodyEvent(
        type="http.response.body",
        body=json.dumps(data_body_dict).encode(),
        more_body=False,
    )


async def cookie_monster(
    scope: Scope, receive: Receive, send: Send
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
    scope: Scope, receive: Receive, send: Send
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
    scope: Scope, receive: Receive, send: Send
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
    scope: Scope, receive: Receive, send: Send
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
    scope: Scope, receive: Receive, send: Send
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
    scope: Scope, receive: Receive, send: Send
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


async def upload_file(
    scope: Scope, receive: Receive, send: Send
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
    scope: Scope, receive: Receive, send: Send
) -> AsyncGenerator[uvt.ASGISendEvent]:
    yield uvt.HTTPResponseStartEvent(
        type="http.response.start",
        status=200,
        headers=[(b"content-type", b"application/json")],
    )
    broken_json = b'{"dog":"dingo","is-dingo":true,"bluey-fam-size":4,"fraction-red-heelers":0.5,"activities":["screwing up the garden","barking at strangers for exisiting","'
    yield uvt.HTTPResponseBodyEvent(
        type="http.response.body",
        body=broken_json,
        more_body=False,
    )


def router(
    scope: Scope, receive: Receive, send: Send
) -> Callable[[Scope, Receive, Send], AsyncGenerator[uvt.ASGISendEvent]]:
    if scope["path"].startswith("/echo"):
        return echo
    elif scope["path"].startswith("/howdy"):
        return howdy
    elif scope["path"].startswith("/500") or scope["path"].startswith("/five-hundred"):
        return five_hundred
    elif scope["path"].startswith("/long"):
        return loooooooong_response
    elif scope["path"].startswith("/slow"):
        return slow_response
    elif scope["path"].startswith("/upload"):
        return upload_file
    elif scope["path"].startswith("/cookie"):
        return cookie_monster
    elif scope["path"].startswith("/broken-json"):
        return broken_json
    else:
        return four_oh_four


async def reqtest_server(scope: Scope, receive: Receive, send: Send) -> None:
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

    async def serve(self, sockets: Any = None) -> None:
        self.restart_requested = asyncio.Event()

        loop = asyncio.get_event_loop()
        tasks = {
            loop.create_task(super().serve(sockets=sockets)),
            loop.create_task(self.watch_restarts()),
        }
        await asyncio.wait(tasks)

    async def restart(self) -> None:  # pragma: no cover
        # This coroutine may be called from a different thread than the one the
        # server is running on, and from an async environment that's not asyncio.
        # For this reason, we use an event to coordinate with the server
        # instead of calling shutdown()/startup() directly, and should not make
        # any asyncio-specific operations.
        self.started = False
        self.restart_requested.set()
        while not self.started:
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
def serve_in_thread(server: ReqtestServer) -> Iterator[ReqtestServer]:
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
        port=0,  # ← ask OS for a free port
        lifespan="off",
        loop="asyncio",
    )
    srv = ReqtestServer(config=cfg)
    with serve_in_thread(srv) as running:
        bound_port = running.servers[0].sockets[0].getsockname()[1]
        running.config.port = bound_port  # make .url work
        yield running
