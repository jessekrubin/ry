"""ry.ryo3.JSON"""

import typing as t

from ry._types import Buffer
from ry.ryo3._bytes import Bytes, ReadableBuffer
from ry.ryo3._jiter import _JsonPrimitive as _JsonPrimitive
from ry.ryo3._jiter import _JsonValue as _JsonValue

@t.overload
def minify(buf: str, /, *, append_newline: bool = False) -> str: ...
@t.overload
def minify(buf: bytes, /, *, append_newline: bool = False) -> bytes: ...
@t.overload
def minify(
    buf: Bytes | bytearray | memoryview, /, *, append_newline: bool = False
) -> Bytes:
    """Return minified json data (remove whitespace, newlines)

    Parameters
    ----------
    buf : _TRawJSON
        JSON buffer/string to minify
    append_newline : bool, default False
        append trailing newline; useful for ndjson/jsonl writing

    Returns
    -------
    _TRawJSON
        Minified JSON data as a `Bytes` object.

    Examples
    --------
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
    >>> JSON.minify(json_str)
    '{"key":"value","number":123,"bool":true}'
    >>> JSON.minify(json_str, append_newline=True)
    '{"key":"value","number":123,"bool":true}\n'

    """

@t.overload
def fmt(buf: str, /, *, append_newline: bool = False) -> str: ...
@t.overload
def fmt(buf: bytes, /, *, append_newline: bool = False) -> bytes: ...
@t.overload
def fmt(
    buf: Bytes | bytearray | memoryview, /, *, append_newline: bool = False
) -> Bytes:
    """Return formatted json data (add indentation, newlines)

    Parameters
    ----------
    buf : _TRawJSON
        JSON buffer/string to format
    append_newline : bool, default False
        append trailing newline; useful for ndjson/jsonl writing

    Returns
    -------
    _TRawJSON
        Formatted JSON data as a `Bytes` object.

    Examples
    --------
    >>> import json as pyjson
    >>> import ry
    >>> from ry import JSON
    >>> data = {"key": "value", "number": 123, "bool": True}
    >>> json_str = pyjson.dumps(data, indent=2)
    >>> print(json_str)
    {
      "key": "value",
      "number": 123,
      "bool": true
    }
    >>> formatted = JSON.fmt(json_str)
    >>> print(formatted)
    {
      "key": "value",
      "number": 123,
      "bool": true
    }
    >>> assert isinstance(formatted, str)
    >>> assert isinstance(JSON.fmt(json_str).encode(), bytes)
    >>> assert isinstance(ry.Bytes(JSON.fmt(json_str).encode()), ry.Bytes)
    >>> formatted
    '{\n  "key": "value",\n  "number": 123,\n  "bool": true\n}'

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
    *,
    allow_inf_nan: bool = False,
    cache_mode: t.Literal[True, False, "all", "keys", "none"] = "all",
    partial_mode: t.Literal[True, False, "off", "on", "trailing-strings"] = False,
    catch_duplicate_keys: bool = False,
) -> _JsonValue: ...
def parse(
    data: Buffer | bytes | str,
    *,
    allow_inf_nan: bool = False,
    cache_mode: t.Literal[True, False, "all", "keys", "none"] = "all",
    partial_mode: t.Literal[True, False, "off", "on", "trailing-strings"] = False,
    catch_duplicate_keys: bool = False,
) -> _JsonValue: ...
def cache_clear() -> None: ...
def cache_usage() -> int: ...
