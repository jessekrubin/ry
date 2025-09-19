from __future__ import annotations

import dataclasses

import pytest

import ry


@dataclasses.dataclass
class HttpStatusMetadata:
    code: int
    reason: str
    const_name: str


def test_status_200() -> None:
    s = ry.HttpStatus(200)
    assert str(s) == "200"
    assert int(s) == 200


def _test_dev_status_code(num: int) -> None:
    try:
        s = ry.HttpStatus(num)
    except ValueError:
        return
    assert str(s) == str(num)
    assert int(s) == num
    assert s == num
    assert s == s
    assert s == ry.HttpStatus(num)
    try:
        assert s != ry.HttpStatus(num + 1)
    except ValueError:
        ...
    assert s != num + 1
    assert s is not None
    assert s != 0


def test_dev_status_code() -> None:
    for i in range(1000):
        _test_dev_status_code(i)


REASONS_MAP = {
    100: "Continue",
    101: "Switching Protocols",
    102: "Processing",
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

CONST_CHARS = "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789_"


def _reason2const_name(reason: str) -> str:
    return "".join(
        c
        for c in reason.upper().replace(" ", "_").replace("-", "_")
        if c in CONST_CHARS
    )


def _class_attr_names() -> list[HttpStatusMetadata]:
    def _int2metadata(i: int) -> HttpStatusMetadata | None:
        try:
            status = ry.HttpStatus(i)
            reason = status.reason
            if reason is None:
                return None
            return HttpStatusMetadata(
                code=i,
                reason=reason,
                const_name=_reason2const_name(reason),
            )
        except ValueError:
            return None

    reasons = [
        el for el in (_int2metadata(i) for i in range(100, 600)) if el is not None
    ]
    return sorted(reasons, key=lambda x: x.code)


@pytest.mark.parametrize("status_meta", _class_attr_names())
def test_class_attr_const(status_meta: HttpStatusMetadata) -> None:
    const_status_code = getattr(ry.HttpStatus, status_meta.const_name)
    assert const_status_code == status_meta.code


class CODEGEN:
    @staticmethod
    def gen_status_code_rust_code() -> str:
        class_attrs = _class_attr_names()

        assert len(class_attrs) == len({m.const_name for m in class_attrs})
        parts = []
        for status_meta in class_attrs:
            name = status_meta.const_name
            string = "\n".join((
                "    #[allow(non_snake_case)]",
                "    #[classattr]",
                f"    fn {name}() -> PyHttpStatus {{",
                f"        PyHttpStatus(ryo3-http::StatusCode::{name})",
                "    }\n",
            ))
            parts.append(string)
        return "\n".join(parts)

    @staticmethod
    def gen_status_code_py_type_annotations() -> str:
        parts = [
            f"    {status_meta.const_name}: HttpStatus  # {status_meta.code} ~ {status_meta.reason}"
            for status_meta in _class_attr_names()
        ]
        return "\n".join(parts)


if __name__ == "__main__":
    import sys

    s = CODEGEN.gen_status_code_rust_code()
    sys.stdout.write(s)
