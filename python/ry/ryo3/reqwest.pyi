import typing as t

import ry

if t.TYPE_CHECKING:
    from ry import Duration, Headers

class HttpClient:
    def __init__(
        self,
        *,
        headers: dict[str, str] | None = None,
        user_agent: str | None = None,  # default ~ 'ry-reqwest/<VERSION> ...'
        timeout: Duration | None = None,
        connect_timeout: Duration | None = None,
        read_timeout: Duration | None = None,
        gzip: bool = True,
        brotli: bool = True,
        deflate: bool = True,
    ) -> None: ...
    async def get(
        self, url: str, *, headers: dict[str, str] | None = None
    ) -> Response: ...
    async def post(
        self,
        url: str,
        *,
        body: bytes | None = None,
        headers: dict[str, str] | None = None,
    ) -> Response: ...
    async def put(
        self,
        url: str,
        *,
        body: bytes | None = None,
        headers: dict[str, str] | None = None,
    ) -> Response: ...
    async def delete(
        self, url: str, *, headers: dict[str, str] | None = None
    ) -> Response: ...
    async def patch(
        self,
        url: str,
        *,
        body: bytes | None = None,
        headers: dict[str, str] | None = None,
    ) -> Response: ...
    async def head(
        self, url: str, *, headers: dict[str, str] | None = None
    ) -> Response: ...
    async def fetch(
        self,
        url: str,
        *,
        method: str = "GET",
        body: bytes | None = None,
        headers: dict[str, str] | None = None,
    ) -> Response: ...

class ReqwestError(Exception):
    def __init__(self, *args: t.Any, **kwargs: t.Any) -> None: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...
    def __dbg__(self) -> str: ...

class Response:
    status_code: int

    @property
    def headers(self) -> Headers: ...
    async def text(self) -> str: ...
    async def json(self) -> t.Any: ...
    async def bytes(self) -> ry.Bytes: ...
    def bytes_stream(self) -> ResponseStream: ...

class ResponseStream:
    def __aiter__(self) -> ResponseStream: ...
    async def __anext__(self) -> ry.Bytes: ...

async def fetch(
    url: str,
    *,
    client: HttpClient | None = None,
    method: str = "GET",
    body: bytes | None = None,
    headers: dict[str, str] | None = None,
) -> Response: ...
