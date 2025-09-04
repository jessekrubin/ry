import typing as t
from os import PathLike

from ry._types import Buffer, Unpack

# =============================================================================
# JSON
# =============================================================================
JsonPrimitive: t.TypeAlias = None | bool | int | float | str
JsonValue: t.TypeAlias = (
    JsonPrimitive
    | dict[str, JsonPrimitive | JsonValue]
    | list[JsonPrimitive | JsonValue]
)

class JsonParseKwargs(t.TypedDict, total=False):
    allow_inf_nan: bool
    """Allow parsing of `Infinity`, `-Infinity`, `NaN` ~ default: False"""
    cache_mode: t.Literal[True, False, "all", "keys", "none"]
    """Cache mode for JSON parsing ~ default: `all` """
    partial_mode: t.Literal[True, False, "off", "on", "trailing-strings"]
    """Partial mode for JSON parsing ~ default: False"""
    catch_duplicate_keys: bool
    """Catch duplicate keys in JSON objects ~ default: False"""

def parse_json(
    data: Buffer | bytes | str,
    /,
    **kwargs: Unpack[JsonParseKwargs],
) -> JsonValue: ...
def parse_jsonl(
    data: Buffer | bytes | str,
    /,
    **kwargs: Unpack[JsonParseKwargs],
) -> list[JsonValue]: ...
def read_json(
    p: str | PathLike[str],
    /,
    lines: bool = False,
    **kwargs: Unpack[JsonParseKwargs],
) -> JsonValue: ...
def json_cache_clear() -> None: ...
def json_cache_usage() -> int: ...
