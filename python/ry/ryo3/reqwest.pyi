import typing as t

class AsyncClient:
    def __init__(
        self,
        *,
        headers: dict[str, str] | None = None,
        timeout: float | None = None,
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

class Response:
    status_code: int
    headers: dict[str, str]

    async def text(self) -> str: ...
    async def json(self) -> t.Any: ...
    async def bytes(self) -> bytes: ...
    def bytes_stream(self) -> ResponseStream: ...

class ResponseStream:
    async def __aiter__(self) -> t.AsyncIterator[bytes]: ...
    async def __anext__(self) -> bytes: ...
