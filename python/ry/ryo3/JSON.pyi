"""ry.ryo3.JSON"""

from typing import Any, Callable, Literal, overload

import typing_extensions
import typing_extensions as te

from ry._types import Buffer
from ry.ryo3._bytes import Bytes
from ry.ryo3._jiter import JsonParseKwargs

JsonPrimitive: typing_extensions.TypeAlias = None | bool | int | float | str
JsonValue: typing_extensions.TypeAlias = (
    JsonPrimitive
    | dict[str, JsonPrimitive | JsonValue]
    | list[JsonPrimitive | JsonValue]
)

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

@overload
def stringify(
    data: Any,
    *,
    default: Callable[[Any], Any] | None = None,
    fmt: bool = False,
    sort_keys: bool = False,
    pybytes: Literal[True],
) -> bytes: ...
@overload
def stringify(
    data: Any,
    *,
    default: Callable[[Any], Any] | None = None,
    fmt: bool = False,
    sort_keys: bool = False,
    pybytes: Literal[False] = False,
) -> Bytes: ...
def parse(
    data: Buffer | bytes | str,
    /,
    **kwargs: te.Unpack[JsonParseKwargs],
) -> JsonValue: ...
def cache_clear() -> None: ...
def cache_usage() -> int: ...

loads = parse
dumps = stringify
