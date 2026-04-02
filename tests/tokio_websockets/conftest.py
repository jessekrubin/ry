from __future__ import annotations

from dataclasses import dataclass, field
from typing import TYPE_CHECKING
from urllib.parse import urlsplit

import pytest
from websockets.asyncio.server import Server, ServerConnection, serve

if TYPE_CHECKING:
    from collections.abc import AsyncIterator

    from websockets.http11 import Request, Response


@dataclass(slots=True)
class WsRequest:
    path: str
    query: str
    headers: dict[str, str]


@dataclass(slots=True)
class WsTestServer:
    _server: Server
    requests: list[WsRequest] = field(default_factory=list)

    @property
    def port(self) -> int:
        socket = self._server.sockets[0]
        return int(socket.getsockname()[1])

    def url(self, path: str = "/echo") -> str:
        return f"ws://127.0.0.1:{self.port}{path}"

    async def close(self) -> None:
        self._server.close()
        await self._server.wait_closed()


async def _serve_echo(websocket: ServerConnection) -> None:
    async for message in websocket:
        await websocket.send(message)


async def _serve_push_close(websocket: ServerConnection) -> None:
    await websocket.send("alpha")
    await websocket.send(b"\x00\x01\x02")
    await websocket.close(code=1000, reason="done")


async def _handle_connection(websocket: ServerConnection) -> None:
    if websocket.request is None:
        await websocket.close(code=1008, reason="missing request information")
        return
    path = urlsplit(websocket.request.path).path
    if path == "/echo":
        await _serve_echo(websocket)
    elif path == "/push-close":
        await _serve_push_close(websocket)
    else:
        await websocket.close(code=1008, reason="unknown test route")


@pytest.fixture
async def websocket_server() -> AsyncIterator[WsTestServer]:
    server_ref: WsTestServer | None = None

    def _process_response(
        connection: ServerConnection, request: Request, response: Response
    ) -> Response:
        del connection
        assert server_ref is not None

        split = urlsplit(request.path)
        headers = {key.lower(): value for key, value in request.headers.items()}
        server_ref.requests.append(
            WsRequest(path=split.path, query=split.query, headers=headers)
        )

        response.headers["X-Ws-Path"] = split.path
        response.headers["X-Ws-Query"] = split.query
        response.headers["X-Seen-Client-Header"] = headers.get("x-client-header", "")
        return response

    server = await serve(
        _handle_connection,
        host="127.0.0.1",
        port=0,
        process_response=_process_response,
    )
    server_ref = WsTestServer(server)

    try:
        yield server_ref
    finally:
        await server_ref.close()
