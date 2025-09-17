import typing as t
from collections.abc import Mapping

# fmt: off
HttpVersionLike: t.TypeAlias = t.Literal[
    "HTTP/0.9", "0.9", 0,
    "HTTP/1.0", "1.0", 1, 10,
    "HTTP/1.1", "1.1", 11,
    "HTTP/2.0", "2.0", 2, 20,
    "HTTP/3.0", "3.0", 3, 30,
]
# fmt: on

_StandardHeader: t.TypeAlias = t.Literal[
    "accept",
    "accept-charset",
    "accept-encoding",
    "accept-language",
    "accept-ranges",
    "access-control-allow-credentials",
    "access-control-allow-headers",
    "access-control-allow-methods",
    "access-control-allow-origin",
    "access-control-expose-headers",
    "access-control-max-age",
    "access-control-request-headers",
    "access-control-request-method",
    "age",
    "allow",
    "alt-svc",
    "authorization",
    "cache-control",
    "cache-status",
    "cdn-cache-control",
    "connection",
    "content-disposition",
    "content-encoding",
    "content-language",
    "content-length",
    "content-location",
    "content-range",
    "content-security-policy",
    "content-security-policy-report-only",
    "content-type",
    "cookie",
    "dnt",
    "date",
    "etag",
    "expect",
    "expires",
    "forwarded",
    "from",
    "host",
    "if-match",
    "if-modified-since",
    "if-none-match",
    "if-range",
    "if-unmodified-since",
    "last-modified",
    "link",
    "location",
    "max-forwards",
    "origin",
    "pragma",
    "proxy-authenticate",
    "proxy-authorization",
    "public-key-pins",
    "public-key-pins-report-only",
    "range",
    "referer",
    "referrer-policy",
    "refresh",
    "retry-after",
    "sec-websocket-accept",
    "sec-websocket-extensions",
    "sec-websocket-key",
    "sec-websocket-protocol",
    "sec-websocket-version",
    "server",
    "set-cookie",
    "strict-transport-security",
    "te",
    "trailer",
    "transfer-encoding",
    "user-agent",
    "upgrade",
    "upgrade-insecure-requests",
    "vary",
    "via",
    "warning",
    "www-authenticate",
    "x-content-type-options",
    "x-dns-prefetch-control",
    "x-frame-options",
    "x-xss-protection",
]

_HeaderName: t.TypeAlias = _StandardHeader | str
_VT = t.TypeVar("_VT", bound=str | t.Sequence[str])

@t.final
class Headers:
    """python-ryo3-http `http::HeadersMap` wrapper"""

    def __init__(
        self,
        headers: Mapping[_HeaderName, _VT] | t.Self | None = None,
        /,
        **kwargs: _VT,
    ) -> None: ...

    # =========================================================================
    # STRING
    # =========================================================================
    def __dbg__(self) -> str: ...

    # =========================================================================
    # MAGIC METHODS
    # =========================================================================
    def __len__(self) -> int: ...
    def __getitem__(self, key: _HeaderName) -> str: ...
    def __setitem__(self, key: _HeaderName, value: str) -> None: ...
    def __delitem__(self, key: _HeaderName) -> None: ...
    def __contains__(self, key: _HeaderName) -> bool: ...
    def __or__(self, other: t.Self | dict[str, str]) -> t.Self: ...
    def __ror__(self, other: t.Self | dict[str, str]) -> t.Self: ...
    def __iter__(self) -> t.Iterator[_HeaderName]: ...
    def __bool__(self) -> bool: ...

    # =========================================================================
    # INSTANCE METHODS
    # =========================================================================
    def to_py(self) -> dict[str, str | t.Sequence[str]]: ...
    def to_dict(self) -> dict[str, str | t.Sequence[str]]: ...
    def stringify(self, *, fmt: bool = False) -> str: ...
    def append(self, key: _HeaderName, value: str) -> None: ...
    def clear(self) -> None: ...
    def contains_key(self, key: _HeaderName) -> bool: ...
    def get(self, key: _HeaderName) -> str | None: ...
    def get_all(self, key: _HeaderName) -> list[str]: ...
    def insert(self, key: _HeaderName, value: str) -> None: ...
    def is_empty(self) -> bool: ...
    def keys(self) -> list[str]: ...
    def keys_len(self) -> int: ...
    def len(self) -> int: ...
    def pop(self, key: _HeaderName) -> str: ...
    def remove(self, key: _HeaderName) -> None: ...
    def update(self, headers: t.Self | dict[str, str]) -> None: ...
    def values(self) -> list[str]: ...
    @property
    def is_flat(self) -> bool: ...

@t.final
class HttpStatus:
    def __init__(self, code: int) -> None: ...
    def __int__(self) -> int: ...
    def __bool__(self) -> bool: ...
    def __hash__(self) -> int: ...
    def __eq__(self, other: object) -> bool: ...
    def __ne__(self, other: object) -> bool: ...
    def __lt__(self, other: t.Self | int) -> bool: ...
    def __le__(self, other: t.Self | int) -> bool: ...
    def __gt__(self, other: t.Self | int) -> bool: ...
    def __ge__(self, other: t.Self | int) -> bool: ...
    def to_py(self) -> int: ...
    @property
    def reason(self) -> str: ...
    @property
    def canonical_reason(self) -> str: ...
    @property
    def is_informational(self) -> bool: ...
    @property
    def is_success(self) -> bool: ...
    @property
    def is_redirect(self) -> bool: ...
    @property
    def is_redirection(self) -> bool: ...
    @property
    def is_client_error(self) -> bool: ...
    @property
    def is_server_error(self) -> bool: ...
    @property
    def is_error(self) -> bool: ...
    @property
    def is_ok(self) -> bool: ...
    @property
    def ok(self) -> bool: ...

    # =========================================================================
    # CONST STATUS CODES
    # =========================================================================
    # fmt: off
    # 1xx: Informational
    CONTINUE: t.ClassVar[HttpStatus]  # 100 ~ Continue
    SWITCHING_PROTOCOLS: t.ClassVar[HttpStatus]  # 101 ~ Switching Protocols
    PROCESSING: t.ClassVar[HttpStatus]  # 102 ~ Processing
    # 2xx: Success
    OK: t.ClassVar[HttpStatus]  # 200 ~ OK
    CREATED: t.ClassVar[HttpStatus]  # 201 ~ Created
    ACCEPTED: t.ClassVar[HttpStatus]  # 202 ~ Accepted
    NON_AUTHORITATIVE_INFORMATION: t.ClassVar[HttpStatus]  # 203 ~ Non Authoritative Information
    NO_CONTENT: t.ClassVar[HttpStatus]  # 204 ~ No Content
    RESET_CONTENT: t.ClassVar[HttpStatus]  # 205 ~ Reset Content
    PARTIAL_CONTENT: t.ClassVar[HttpStatus]  # 206 ~ Partial Content
    MULTI_STATUS: t.ClassVar[HttpStatus]  # 207 ~ Multi-Status
    ALREADY_REPORTED: t.ClassVar[HttpStatus]  # 208 ~ Already Reported
    IM_USED: t.ClassVar[HttpStatus]  # 226 ~ IM Used
    # 3xx: Redirection
    MULTIPLE_CHOICES: t.ClassVar[HttpStatus]  # 300 ~ Multiple Choices
    MOVED_PERMANENTLY: t.ClassVar[HttpStatus]  # 301 ~ Moved Permanently
    FOUND: t.ClassVar[HttpStatus]  # 302 ~ Found
    SEE_OTHER: t.ClassVar[HttpStatus]  # 303 ~ See Other
    NOT_MODIFIED: t.ClassVar[HttpStatus]  # 304 ~ Not Modified
    USE_PROXY: t.ClassVar[HttpStatus]  # 305 ~ Use Proxy
    TEMPORARY_REDIRECT: t.ClassVar[HttpStatus]  # 307 ~ Temporary Redirect
    PERMANENT_REDIRECT: t.ClassVar[HttpStatus]  # 308 ~ Permanent Redirect
    # 4xx: Client Error
    BAD_REQUEST: t.ClassVar[HttpStatus]  # 400 ~ Bad Request
    UNAUTHORIZED: t.ClassVar[HttpStatus]  # 401 ~ Unauthorized
    PAYMENT_REQUIRED: t.ClassVar[HttpStatus]  # 402 ~ Payment Required
    FORBIDDEN: t.ClassVar[HttpStatus]  # 403 ~ Forbidden
    NOT_FOUND: t.ClassVar[HttpStatus]  # 404 ~ Not Found
    METHOD_NOT_ALLOWED: t.ClassVar[HttpStatus]  # 405 ~ Method Not Allowed
    NOT_ACCEPTABLE: t.ClassVar[HttpStatus]  # 406 ~ Not Acceptable
    PROXY_AUTHENTICATION_REQUIRED: t.ClassVar[HttpStatus]  # 407 ~ Proxy Authentication Required
    REQUEST_TIMEOUT: t.ClassVar[HttpStatus]  # 408 ~ Request Timeout
    CONFLICT: t.ClassVar[HttpStatus]  # 409 ~ Conflict
    GONE: t.ClassVar[HttpStatus]  # 410 ~ Gone
    LENGTH_REQUIRED: t.ClassVar[HttpStatus]  # 411 ~ Length Required
    PRECONDITION_FAILED: t.ClassVar[HttpStatus]  # 412 ~ Precondition Failed
    PAYLOAD_TOO_LARGE: t.ClassVar[HttpStatus]  # 413 ~ Payload Too Large
    URI_TOO_LONG: t.ClassVar[HttpStatus]  # 414 ~ URI Too Long
    UNSUPPORTED_MEDIA_TYPE: t.ClassVar[HttpStatus]  # 415 ~ Unsupported Media Type
    RANGE_NOT_SATISFIABLE: t.ClassVar[HttpStatus]  # 416 ~ Range Not Satisfiable
    EXPECTATION_FAILED: t.ClassVar[HttpStatus]  # 417 ~ Expectation Failed
    IM_A_TEAPOT: t.ClassVar[HttpStatus]  # 418 ~ I'm a teapot
    MISDIRECTED_REQUEST: t.ClassVar[HttpStatus]  # 421 ~ Misdirected Request
    UNPROCESSABLE_ENTITY: t.ClassVar[HttpStatus]  # 422 ~ Unprocessable Entity
    LOCKED: t.ClassVar[HttpStatus]  # 423 ~ Locked
    FAILED_DEPENDENCY: t.ClassVar[HttpStatus]  # 424 ~ Failed Dependency
    TOO_EARLY: t.ClassVar[HttpStatus]  # 425 ~ Too Early
    UPGRADE_REQUIRED: t.ClassVar[HttpStatus]  # 426 ~ Upgrade Required
    PRECONDITION_REQUIRED: t.ClassVar[HttpStatus]  # 428 ~ Precondition Required
    TOO_MANY_REQUESTS: t.ClassVar[HttpStatus]  # 429 ~ Too Many Requests
    REQUEST_HEADER_FIELDS_TOO_LARGE: t.ClassVar[HttpStatus]  # 431 ~ Request Header Fields Too Large
    UNAVAILABLE_FOR_LEGAL_REASONS: t.ClassVar[HttpStatus]  # 451 ~ Unavailable For Legal Reasons
    # 5xx: Server Error
    INTERNAL_SERVER_ERROR: t.ClassVar[HttpStatus]  # 500 ~ Internal Server Error
    NOT_IMPLEMENTED: t.ClassVar[HttpStatus]  # 501 ~ Not Implemented
    BAD_GATEWAY: t.ClassVar[HttpStatus]  # 502 ~ Bad Gateway
    SERVICE_UNAVAILABLE: t.ClassVar[HttpStatus]  # 503 ~ Service Unavailable
    GATEWAY_TIMEOUT: t.ClassVar[HttpStatus]  # 504 ~ Gateway Timeout
    HTTP_VERSION_NOT_SUPPORTED: t.ClassVar[HttpStatus]  # 505 ~ HTTP Version Not Supported
    VARIANT_ALSO_NEGOTIATES: t.ClassVar[HttpStatus]  # 506 ~ Variant Also Negotiates
    INSUFFICIENT_STORAGE: t.ClassVar[HttpStatus]  # 507 ~ Insufficient Storage
    LOOP_DETECTED: t.ClassVar[HttpStatus]  # 508 ~ Loop Detected
    NOT_EXTENDED: t.ClassVar[HttpStatus]  # 510 ~ Not Extended
    NETWORK_AUTHENTICATION_REQUIRED: t.ClassVar[HttpStatus]  # 511 ~ Network Authentication Required
    # fmt: on
