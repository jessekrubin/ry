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

def parse_json(
    data: Buffer | bytes | str,
    *,
    allow_inf_nan: bool = False,
    cache_mode: t.Literal[True, False, "all", "keys", "none"] = "all",
    partial_mode: t.Literal[True, False, "off", "on", "trailing-strings"] = False,
    catch_duplicate_keys: bool = False,
) -> JsonValue: ...
def parse_jsonl(
    data: Buffer | bytes | str,
    *,
    allow_inf_nan: bool = False,
    cache_mode: t.Literal[True, False, "all", "keys", "none"] = "all",
    partial_mode: t.Literal[True, False, "off", "on", "trailing-strings"] = False,
    catch_duplicate_keys: bool = False,
) -> list[JsonValue]: ...
def read_json(
    p: str | PathLike[str],
    *,
    allow_inf_nan: bool = False,
    cache_mode: t.Literal[True, False, "all", "keys", "none"] = "all",
    partial_mode: t.Literal[True, False, "off", "on", "trailing-strings"] = False,
    catch_duplicate_keys: bool = False,
    lines: bool = False,
) -> JsonValue: ...
def json_cache_clear() -> None: ...
def json_cache_usage() -> int: ...
