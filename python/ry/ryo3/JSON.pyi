"""ry.ryo3.JSON"""

import typing as t

from ry._types import Buffer, Unpack
from ry.ryo3._bytes import Bytes
from ry.ryo3._jiter import JsonParseKwargs, JsonValue

def minify(buf: Buffer | str, /) -> Bytes:
    """Return minified json data (remove whitespace, newlines)

    Args:
        data: The JSON data to minify.

    Returns:
        Minified JSON data as a `Bytes` object.

    Examples:
        >>> import json as pyjson
        >>> from ry.ryo3 import JSON
        >>> data = {"key": "value", "number": 123, "bool": True}
        >>> json_str = pyjson.dumps(data, indent=2)
        >>> print(json_str)
        {
          "key": "value",
          "number": 123,
          "bool": true
        }
        >>> bytes(JSON.minify(json_str))
        b'{"key":"value","number":123,"bool":true}'

    """

def fmt(buf: Buffer | str, /) -> Bytes:
    """Return minified json data (remove whitespace, newlines)

    Args:
        data: The JSON data to minify.

    Returns:
        Minified JSON data as a `Bytes` object.

    Examples:
        >>> import json as pyjson
        >>> from ry.ryo3 import JSON
        >>> data = {"key": "value", "number": 123, "bool": True}
        >>> json_str = pyjson.dumps(data, indent=2)
        >>> print(json_str)
        {
          "key": "value",
          "number": 123,
          "bool": true
        }
        >>> bytes(JSON.fmt(json_str)).decode()
        '{\n  "key": "value",\n  "number": 123,\n  "bool": true\n}'
        >>> print(bytes(JSON.fmt(json_str)).decode())
        {
          "key": "value",
          "number": 123,
          "bool": true
        }

    """

@t.overload
def stringify(
    obj: t.Any,
    *,
    default: t.Callable[[t.Any], t.Any] | None = None,
    fmt: bool = False,
    sort_keys: bool = False,
    append_newline: bool = False,
    pybytes: t.Literal[True],
) -> bytes: ...
@t.overload
def stringify(
    obj: t.Any,
    *,
    default: t.Callable[[t.Any], t.Any] | None = None,
    fmt: bool = False,
    sort_keys: bool = False,
    append_newline: bool = False,
    pybytes: t.Literal[False] = False,
) -> Bytes: ...
@t.overload
def dumps(
    obj: t.Any,
    *,
    default: t.Callable[[t.Any], t.Any] | None = None,
    fmt: bool = False,
    sort_keys: bool = False,
    append_newline: bool = False,
    pybytes: t.Literal[True],
) -> bytes: ...
@t.overload
def dumps(
    obj: t.Any,
    *,
    default: t.Callable[[t.Any], t.Any] | None = None,
    fmt: bool = False,
    sort_keys: bool = False,
    append_newline: bool = False,
    pybytes: t.Literal[False] = False,
) -> Bytes: ...
def loads(
    data: Buffer | bytes | str,
    /,
    **kwargs: Unpack[JsonParseKwargs],
) -> JsonValue: ...
def parse(
    data: Buffer | bytes | str,
    /,
    **kwargs: Unpack[JsonParseKwargs],
) -> JsonValue: ...
def cache_clear() -> None: ...
def cache_usage() -> int: ...

# under construction
def stringify_unsafe(
    obj: t.Any,
    *,
    default: t.Callable[[t.Any], t.Any] | None = None,
    fmt: bool = False,
    sort_keys: bool = False,
    append_newline: bool = False,
    pybytes: bool = False,
) -> t.NoReturn: ...
