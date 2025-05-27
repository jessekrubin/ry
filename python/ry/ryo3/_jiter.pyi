import typing as t
from os import PathLike

import typing_extensions as te

from ry._types import Buffer

# =============================================================================
# JSON
# =============================================================================
JsonPrimitive: te.TypeAlias = None | bool | int | float | str
JsonValue: te.TypeAlias = (
    JsonPrimitive
    | dict[str, JsonPrimitive | JsonValue]
    | list[JsonPrimitive | JsonValue]
)

class JsonParseKwargs(t.TypedDict, total=False):
    allow_inf_nan: bool
    """Allow parsing of `Infinity`, `-Infinity`, `NaN` ~ default: True"""
    cache_mode: t.Literal[True, False, "all", "keys", "none"]
    """Cache mode for JSON parsing ~ default: `all` """
    partial_mode: t.Literal[True, False, "off", "on", "trailing-strings"]
    """Partial mode for JSON parsing ~ default: False"""
    catch_duplicate_keys: bool
    """Catch duplicate keys in JSON objects ~ default: False"""
    float_mode: t.Literal["float", "decimal", "lossless-float"] | bool
    """Mode for parsing JSON floats ~ default: False"""

def parse_json(
    data: Buffer | bytes | str,
    /,
    **kwargs: te.Unpack[JsonParseKwargs],
) -> JsonValue: ...
def parse_jsonl(
    data: Buffer | bytes | str,
    /,
    **kwargs: te.Unpack[JsonParseKwargs],
) -> list[JsonValue]: ...
def parse_json_bytes(
    data: bytes,
    /,
    **kwargs: te.Unpack[JsonParseKwargs],
) -> JsonValue: ...
def read_json(
    p: str | PathLike[str],
    /,
    lines: bool = False,
    **kwargs: te.Unpack[JsonParseKwargs],
) -> JsonValue: ...
def json_cache_clear() -> None: ...
def json_cache_usage() -> int: ...
