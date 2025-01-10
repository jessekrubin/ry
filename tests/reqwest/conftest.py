"""tests/reqwest/conftest.py

Test-server is based on but not the same as the test-server in the httpx tests
"""

from __future__ import annotations

import asyncio
import json
import threading
import time
from asyncio import sleep as aiosleep
from collections.abc import AsyncGenerator, Awaitable, Coroutine, Iterator
from typing import Any, Callable

import pytest
from uvicorn import _types as uvt
from uvicorn.config import Config
from uvicorn.server import Server

import ry

ENVIRONMENT_VARIABLES = {
    "SSL_CERT_FILE",
    "SSL_CERT_DIR",
    "HTTP_PROXY",
    "HTTPS_PROXY",
    "ALL_PROXY",
    "NO_PROXY",
    "SSLKEYLOGFILE",
}

Message = dict[str, Any]
Receive = Callable[[], Awaitable[Message]]
Send = Callable[[uvt.ASGISendEvent], Coroutine[None, None, None]]
Scope = dict[str, Any]


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
    yield {"type": "http.response.body", "body": json.dumps(data_body_dict).encode()}


async def four_oh_four(
    scope: Scope, receive: Receive, send: Send
) -> AsyncGenerator[uvt.ASGISendEvent]:
    yield uvt.HTTPResponseStartEvent(
        type="http.response.start",
        status=404,
        headers=[(b"content-type", b"text/plain")],
    )
    yield {"type": "http.response.body", "body": b"Not Found"}


async def five_hundred(
    scope: Scope, receive: Receive, send: Send
) -> AsyncGenerator[uvt.ASGISendEvent]:
    yield uvt.HTTPResponseStartEvent(
        type="http.response.start",
        status=500,
        headers=[(b"content-type", b"text/plain")],
    )
    yield {"type": "http.response.body", "body": b"Internal Server Error"}


async def howdy(
    scope: Scope, receive: Receive, send: Send
) -> AsyncGenerator[uvt.ASGISendEvent]:
    yield uvt.HTTPResponseStartEvent(
        type="http.response.start",
        status=200,
        headers=[(b"content-type", b"application/json")],
    )
    yield {"type": "http.response.body", "body": b'{"howdy": "partner"}'}


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
        yield {"type": "http.response.body", "body": body_chunk, "more_body": True}
        await aiosleep(0.2)
    yield {"type": "http.response.body", "body": b"", "more_body": False}


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
        yield {"type": "http.response.body", "body": body_chunk, "more_body": True}
    yield {"type": "http.response.body", "body": b"", "more_body": False}


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
        pass  # pragma: nocover

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
            except asyncio.TimeoutError:
                continue

            self.restart_requested.clear()
            await self.shutdown()
            await self.startup()


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
    config = Config(app=reqtest_server, lifespan="off", loop="asyncio")
    server = ReqtestServer(config=config)
    yield from serve_in_thread(server)
