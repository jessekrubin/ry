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
    basic_auth: tuple[str, str | None] | None
    bearer_auth: str | None
    version: HttpVersionLike | None

@t.final
class HttpClient:
    def __init__(
        self,
        *,
        headers: dict[str, str] | Headers | None = None,
        cookies: bool = False,
        user_agent: str | None = None,
        timeout: Duration | None = None,
        connect_timeout: Duration | None = None,
        read_timeout: Duration | None = None,
        redirect: int | None = 10,
        referer: bool = True,
        gzip: bool = True,
        brotli: bool = True,
        deflate: bool = True,
        zstd: bool = True,
        hickory_dns: bool = True,
        http1_only: bool = False,
        https_only: bool = False,
        http1_title_case_headers: bool = False,
        http1_allow_obsolete_multiline_headers_in_responses: bool = False,
        http1_allow_spaces_after_header_name_in_responses: bool = False,
        http1_ignore_invalid_headers_in_responses: bool = False,
        http2_prior_knowledge: bool = False,
        http2_initial_stream_window_size: int | None = None,
        http2_initial_connection_window_size: int | None = None,
        http2_adaptive_window: bool = False,
        http2_max_frame_size: int | None = None,
        http2_max_header_list_size: int | None = None,
        http2_keep_alive_interval: Duration | None = None,
        http2_keep_alive_timeout: Duration | None = None,
        http2_keep_alive_while_idle: bool = False,
        pool_idle_timeout: Duration | None = ...,  # 90 seconds
        pool_max_idle_per_host: int | None = ...,  # usize::MAX
        tcp_keepalive: Duration | None = ...,  # 15 seconds
        tcp_keepalive_interval: Duration | None = ...,  # 15 seconds
        tcp_keepalive_retries: int | None = 3,
        tcp_nodelay: bool = True,
        root_certificates: list[Certificate] | None = None,
        tls_min_version: t.Literal["1.0", "1.1", "1.2", "1.3"] | None = None,
        tls_max_version: t.Literal["1.0", "1.1", "1.2", "1.3"] | None = None,
        tls_info: bool = False,
        tls_sni: bool = True,
        danger_accept_invalid_certs: bool = False,
        danger_accept_invalid_hostnames: bool = False,
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
    def __init__(self) -> t.NoReturn: ...
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
    def cookies(self) -> list[Cookie] | None: ...
    @property
    def set_cookies(self) -> list[Cookie] | None: ...
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

@t.final
class Cookie:
    def __init__(
        self,
        name: str,
        value: str,
        *,
        domain: str | None = None,
        expires: int | None = None,
        http_only: bool | None = None,
        max_age: Duration | None = None,
        partitioned: bool | None = None,
        path: str | None = None,
        permanent: bool = False,
        removal: bool = False,
        same_site: t.Literal["Lax", "Strict", "None"] | None = None,
        secure: bool | None = None,
    ) -> None: ...
    @staticmethod
    def parse(s: str) -> Cookie: ...
    @staticmethod
    def parse_encoded(s: str) -> Cookie: ...

    # -------------------------------------------------------------------------
    # METHODS
    # -------------------------------------------------------------------------
    # -- STRING --
    def encoded(self) -> str: ...
    def stripped(self) -> str: ...
    def encoded_stripped(self) -> str: ...
    def stripped_encoded(self) -> str: ...

    # -------------------------------------------------------------------------
    # PROPERTIES
    # -------------------------------------------------------------------------
    @property
    def name(self) -> str: ...
    @property
    def value(self) -> str: ...
    @property
    def value_trimmed(self) -> str: ...
    @property
    def name_value(self) -> tuple[str, str]: ...
    @property
    def name_value_trimmed(self) -> tuple[str, str]: ...
    @property
    def domain(self) -> str | None: ...
    @property
    def expires(self) -> int | None: ...
    @property
    def http_only(self) -> bool | None: ...
    @property
    def max_age(self) -> Duration | None: ...
    @property
    def partitioned(self) -> bool | None: ...
    @property
    def path(self) -> str | None: ...
    @property
    def same_site(self) -> t.Literal["Lax", "Strict", "None"] | None: ...
    @property
    def secure(self) -> bool | None: ...

class Certificate:
    def __init__(self) -> t.NoReturn: ...
    def __hash__(self) -> int: ...
    def __eq__(self, other: object) -> bool: ...
    def __ne__(self, other: object) -> bool: ...
    @staticmethod
    def from_der(der: Buffer) -> Certificate: ...
    @staticmethod
    def from_pem(pem: Buffer) -> Certificate: ...
    @staticmethod
    def from_pem_bundle(pem_bundle: Buffer) -> list[Certificate]: ...
