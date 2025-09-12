import typing as t

import ry
from ry._types import Buffer, Unpack
from ry.ryo3._http import Headers, HttpStatus, HttpVersionLike
from ry.ryo3._std import Duration, SocketAddr
from ry.ryo3._url import URL

class RequestKwargs(t.TypedDict, total=False):
    body: Buffer | None
    headers: Headers | dict[str, str] | None
    query: dict[str, t.Any] | t.Sequence[tuple[str, t.Any]] | None
    json: t.Any
    form: t.Any
    multipart: t.Any
    timeout: Duration | None
    version: HttpVersionLike | None

@t.final
class HttpClient:
    def __init__(
        self,
        *,
        headers: dict[str, str] | None = None,
        cookies: bool = False,
        user_agent: str | None = None,  # default ~ 'ry-reqwest/<VERSION> ...'
        timeout: Duration | None = None,
        connect_timeout: Duration | None = None,
        read_timeout: Duration | None = None,
        gzip: bool = True,
        brotli: bool = True,
        deflate: bool = True,
    ) -> None: ...
    async def get(
        self,
        url: str | URL,
        **kwargs: Unpack[RequestKwargs],
    ) -> Response: ...
    async def post(
        self,
        url: str | URL,
        **kwargs: Unpack[RequestKwargs],
    ) -> Response: ...
    async def put(
        self,
        url: str | URL,
        **kwargs: Unpack[RequestKwargs],
    ) -> Response: ...
    async def delete(
        self,
        url: str | URL,
        **kwargs: Unpack[RequestKwargs],
    ) -> Response: ...
    async def patch(
        self,
        url: str | URL,
        **kwargs: Unpack[RequestKwargs],
    ) -> Response: ...
    async def options(
        self,
        url: str | URL,
        **kwargs: Unpack[RequestKwargs],
    ) -> Response: ...
    async def head(
        self,
        url: str | URL,
        **kwargs: Unpack[RequestKwargs],
    ) -> Response: ...
    async def fetch(
        self,
        url: str | URL,
        *,
        method: str = "GET",
        **kwargs: Unpack[RequestKwargs],
    ) -> Response: ...
    async def __call__(
        self,
        url: str | URL,
        *,
        method: str = "GET",
        **kwargs: Unpack[RequestKwargs],
    ) -> Response: ...

@t.final
class ReqwestError(Exception):
    def __init__(self, *args: t.Any, **kwargs: t.Any) -> None: ...
    def __dbg__(self) -> str: ...
    def is_body(self) -> bool: ...
    def is_builder(self) -> bool: ...
    def is_connect(self) -> bool: ...
    def is_decode(self) -> bool: ...
    def is_redirect(self) -> bool: ...
    def is_request(self) -> bool: ...
    def is_status(self) -> bool: ...
    def is_timeout(self) -> bool: ...
    def status(self) -> HttpStatus | None: ...
    def url(self) -> URL | None: ...

@t.final
class Response:
    @property
    def headers(self) -> Headers: ...
    async def text(self) -> str: ...
    async def json(
        self,
        *,
        allow_inf_nan: bool = False,
        cache_mode: t.Literal[True, False, "all", "keys", "none"] = "all",
        partial_mode: t.Literal[True, False, "off", "on", "trailing-strings"] = False,
        catch_duplicate_keys: bool = False,
    ) -> t.Any: ...
    async def bytes(self) -> ry.Bytes: ...
    def bytes_stream(self) -> ResponseStream: ...
    def stream(self) -> ResponseStream: ...
    @property
    def url(self) -> URL: ...
    @property
    def version(
        self,
    ) -> t.Literal["HTTP/0.9", "HTTP/1.0", "HTTP/1.1", "HTTP/2.0", "HTTP/3.0"]: ...
    @property
    def http_version(
        self,
    ) -> t.Literal["HTTP/0.9", "HTTP/1.0", "HTTP/1.1", "HTTP/2.0", "HTTP/3.0"]: ...
    @property
    def redirected(self) -> bool: ...
    @property
    def content_length(self) -> int | None: ...
    @property
    def content_encoding(self) -> str | None: ...
    @property
    def body_used(self) -> bool:
        """True if the body has been consumed"""
    @property
    def ok(self) -> bool:
        """True if the status is a success (2xx)"""
    @property
    def remote_addr(self) -> SocketAddr | None: ...
    @property
    def status(self) -> int: ...
    @property
    def status_text(self) -> str: ...
    @property
    def status_code(self) -> HttpStatus: ...
    def __bool__(self) -> bool:
        """True if the status is a success (2xx)"""

@t.final
class ResponseStream:
    def __aiter__(self) -> ResponseStream: ...
    async def __anext__(self) -> ry.Bytes: ...
    async def take(self, n: int = 1) -> list[ry.Bytes]: ...
    @t.overload
    async def collect(self, join: t.Literal[False] = False) -> list[ry.Bytes]: ...
    @t.overload
    async def collect(self, join: t.Literal[True] = True) -> ry.Bytes: ...

async def fetch(
    url: str | URL,
    *,
    client: HttpClient | None = None,
    method: str = "GET",
    **kwargs: Unpack[RequestKwargs],
) -> Response: ...
