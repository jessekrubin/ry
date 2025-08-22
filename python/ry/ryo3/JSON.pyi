"""ry.ryo3.JSON"""

import typing as t

import typing_extensions as te

from ry._types import Buffer
from ry.ryo3._bytes import Bytes
from ry.ryo3._jiter import JsonParseKwargs, JsonValue

def minify(data: Buffer) -> Bytes:
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

def fmt(data: Buffer) -> Bytes:
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
    data: t.Any,
    *,
    default: t.Callable[[t.Any], t.Any] | None = None,
    fmt: bool = False,
    sort_keys: bool = False,
    append_newline: bool = False,
    pybytes: t.Literal[True],
) -> bytes: ...
@t.overload
def stringify(
    data: t.Any,
    *,
    default: t.Callable[[t.Any], t.Any] | None = None,
    fmt: bool = False,
    sort_keys: bool = False,
    append_newline: bool = False,
    pybytes: t.Literal[False] = False,
) -> Bytes: ...
@t.overload
def dumps(
    data: t.Any,
    *,
    default: t.Callable[[t.Any], t.Any] | None = None,
    fmt: bool = False,
    sort_keys: bool = False,
    append_newline: bool = False,
    pybytes: t.Literal[True],
) -> bytes: ...
@t.overload
def dumps(
    data: t.Any,
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
    **kwargs: te.Unpack[JsonParseKwargs],
) -> JsonValue: ...
def parse(
    data: Buffer | bytes | str,
    /,
    **kwargs: te.Unpack[JsonParseKwargs],
) -> JsonValue: ...
def cache_clear() -> None: ...
def cache_usage() -> int: ...

# under construction
def stringify_unsafe(data: t.Any) -> Bytes: ...
