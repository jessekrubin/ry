from __future__ import annotations

import pytest

import ry

_REASONS_MAP = {
    100: "Continue",
    101: "Switching Protocols",
    102: "Processing",
    103: "Early Hints",
    200: "OK",
    201: "Created",
    202: "Accepted",
    203: "Non Authoritative Information",
    204: "No Content",
    205: "Reset Content",
    206: "Partial Content",
    207: "Multi-Status",
    208: "Already Reported",
    226: "IM Used",
    300: "Multiple Choices",
    301: "Moved Permanently",
    302: "Found",
    303: "See Other",
    304: "Not Modified",
    305: "Use Proxy",
    307: "Temporary Redirect",
    308: "Permanent Redirect",
    400: "Bad Request",
    401: "Unauthorized",
    402: "Payment Required",
    403: "Forbidden",
    404: "Not Found",
    405: "Method Not Allowed",
    406: "Not Acceptable",
    407: "Proxy Authentication Required",
    408: "Request Timeout",
    409: "Conflict",
    410: "Gone",
    411: "Length Required",
    412: "Precondition Failed",
    413: "Payload Too Large",
    414: "URI Too Long",
    415: "Unsupported Media Type",
    416: "Range Not Satisfiable",
    417: "Expectation Failed",
    418: "I'm a teapot",
    421: "Misdirected Request",
    422: "Unprocessable Entity",
    423: "Locked",
    424: "Failed Dependency",
    425: "Too Early",
    426: "Upgrade Required",
    428: "Precondition Required",
    429: "Too Many Requests",
    431: "Request Header Fields Too Large",
    451: "Unavailable For Legal Reasons",
    500: "Internal Server Error",
    501: "Not Implemented",
    502: "Bad Gateway",
    503: "Service Unavailable",
    504: "Gateway Timeout",
    505: "HTTP Version Not Supported",
    506: "Variant Also Negotiates",
    507: "Insufficient Storage",
    508: "Loop Detected",
    510: "Not Extended",
    511: "Network Authentication Required",
}


@pytest.mark.parametrize(
    ("code", "reason"),
    list(_REASONS_MAP.items()),
)
def test_http_status_code(
    code: int,
    reason: str,
) -> None:
    s = ry.HttpStatus(code)
    assert s == code
    assert s == ry.HttpStatus(code)
    assert s is ry.HttpStatus(code)
    assert s.canonical_reason == reason
    assert s.reason == reason
    assert isinstance(s.is_informational, bool)
    assert isinstance(s.is_success, bool)
    assert isinstance(s.is_redirect, bool)
    assert isinstance(s.is_redirection, bool)
    assert isinstance(s.is_client_error, bool)
    assert isinstance(s.is_server_error, bool)
    assert isinstance(s.is_error, bool)
    assert isinstance(s.ok, bool)
    assert isinstance(s.is_ok, bool)
    assert isinstance(s.to_py(), int)


@pytest.mark.parametrize(
    "code",
    [
        199,
        299,
        399,
        499,
        599,
        600,
        700,
        800,
        999,
    ],
)
def test_http_status_non_existing_reason(code: int) -> None:
    s = ry.HttpStatus(code)
    assert s == code
    assert s.canonical_reason is None
    assert s.reason is None
    assert isinstance(s.is_informational, bool)
    assert isinstance(s.is_success, bool)
    assert isinstance(s.is_redirect, bool)
    assert isinstance(s.is_redirection, bool)
    assert isinstance(s.is_client_error, bool)
    assert isinstance(s.is_server_error, bool)
    assert isinstance(s.is_error, bool)
    assert isinstance(s.ok, bool)
    assert isinstance(s.is_ok, bool)
    assert isinstance(s.to_py(), int)


def test_status_200() -> None:
    s = ry.HttpStatus(200)
    assert str(s) == "200"
    assert int(s) == 200
    assert repr(s) == "HttpStatus(200)"
    assert hash(s) == hash(200)
    assert s == ry.HttpStatus.OK
    assert s is ry.HttpStatus.OK


def get_all_status_code_class_attrs() -> set[int]:
    attrs = set()
    for attr_name in dir(ry.HttpStatus):
        attr_value = getattr(ry.HttpStatus, attr_name)
        if isinstance(attr_value, ry.HttpStatus):
            attrs.add(attr_value.to_py())
    return attrs


def test_reason_map_is_up_to_date() -> None:
    for code in range(100, 1000):
        s = ry.HttpStatus(code)
        expected_reason = _REASONS_MAP.get(code)
        assert s.canonical_reason == expected_reason, f"Mismatch for code {code}"
        assert s.reason == expected_reason, f"Mismatch for code {code}"


def test_reprs() -> None:
    for i in range(100, 1000):
        s = ry.HttpStatus(i)
        assert repr(s) == f"HttpStatus({i})"
        evaluated = eval(repr(s), {"HttpStatus": ry.HttpStatus})
        assert evaluated == s
        if i < 600:
            assert evaluated is s
