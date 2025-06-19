"""ry.ryo3.JSON"""

from typing import Any, Callable, Literal, overload

import typing_extensions

from ry.ryo3._bytes import Bytes

JsonPrimitive: typing_extensions.TypeAlias = None | bool | int | float | str
JsonValue: typing_extensions.TypeAlias = (
    JsonPrimitive
    | dict[str, JsonPrimitive | JsonValue]
    | list[JsonPrimitive | JsonValue]
)

@overload
def stringify(
    data: Any,
    *,
    default: Callable[[Any], Any] | None = None,
    fmt: bool = False,
    sort_keys: bool = False,
    pybytes: Literal[True] = True,
) -> bytes: ...
@overload
def stringify(
    data: Any,
    *,
    default: Callable[[Any], Any] | None = None,
    fmt: bool = False,
    sort_keys: bool = False,
    pybytes: bool = ...,
) -> Bytes: ...
def parse_json(
    data: bytes | str,
    /,
    *,
    allow_inf_nan: bool = True,
    cache_mode: Literal[True, False, "all", "keys", "none"] = "all",
    partial_mode: Literal[True, False, "off", "on", "trailing-strings"] = False,
    catch_duplicate_keys: bool = False,
    float_mode: Literal["float", "decimal", "lossless-float"] | bool = False,
) -> JsonValue: ...
def parse_json_bytes(
    data: bytes,
    /,
    *,
    allow_inf_nan: bool = True,
    cache_mode: Literal[True, False, "all", "keys", "none"] = "all",
    partial_mode: Literal[True, False, "off", "on", "trailing-strings"] = False,
    catch_duplicate_keys: bool = False,
    float_mode: Literal["float", "decimal", "lossless-float"] | bool = False,
) -> JsonValue: ...
def json_cache_clear() -> None: ...
def json_cache_usage() -> int: ...
