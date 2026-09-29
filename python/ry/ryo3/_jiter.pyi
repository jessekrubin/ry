"""ryo3-jiter ~ types"""

import typing as t
from os import PathLike

from ry._types import Buffer
from ry.ryo3._bytes import Bytes, ReadableBuffer

# =============================================================================
# JSON
# =============================================================================
_JsonPrimitive: t.TypeAlias = bool | int | float | str | None
_JsonValue: t.TypeAlias = _JsonPrimitive | dict[str, _JsonValue] | list[_JsonValue]

def parse_json(
    data: Buffer | bytes | str,
    *,
    allow_inf_nan: bool = False,
    cache_mode: t.Literal[True, False, "all", "keys", "none"] = "all",
    partial_mode: t.Literal[True, False, "off", "on", "trailing-strings"] = False,
    catch_duplicate_keys: bool = False,
) -> _JsonValue: ...
def parse_jsonl(
    data: Buffer | bytes | str,
    *,
    allow_inf_nan: bool = False,
    cache_mode: t.Literal[True, False, "all", "keys", "none"] = "all",
    partial_mode: t.Literal[True, False, "off", "on", "trailing-strings"] = False,
    catch_duplicate_keys: bool = False,
) -> list[_JsonValue]: ...
def read_json(
    p: str | PathLike[str],
    *,
    allow_inf_nan: bool = False,
    cache_mode: t.Literal[True, False, "all", "keys", "none"] = "all",
    partial_mode: t.Literal[True, False, "off", "on", "trailing-strings"] = False,
    catch_duplicate_keys: bool = False,
    lines: bool = False,
) -> _JsonValue: ...
def json_cache_clear() -> None: ...
def json_cache_usage() -> int: ...

# =============================================================================
# JSON-DECODE-ERROR
# =============================================================================
_JsonErrorKind: t.TypeAlias = t.Literal[
    "float-expecting-int",
    "duplicate-key",
    "internal-error",
    "eof-while-parsing-list",
    "eof-while-parsing-object",
    "eof-while-parsing-string",
    "eof-while-parsing-value",
    "expected-colon",
    "expected-list-comma-or-end",
    "expected-object-comma-or-end",
    "expected-some-ident",
    "expected-some-value",
    "invalid-escape",
    "invalid-number",
    "number-out-of-range",
    "invalid-unicode-code-point",
    "control-character-while-parsing-string",
    "key-must-be-a-string",
    "lone-leading-surrogate-in-hex-escape",
    "trailing-comma",
    "trailing-characters",
    "unexpected-end-of-hex-escape",
    "recursion-limit-exceeded",
]
_JsonErrorKindArg: t.TypeAlias = (
    _JsonErrorKind | tuple[t.Literal["duplicate-key", "internal-error"], str]
)

@t.final
class JSONDecodeError(ValueError):
    r"""JSON decode error; semi compatible w/ `json.JSONDecodeError`

    `doc` is only set when rust owns the bytes (response bodies, `read_json`)
    so you can still get at a consumed response body.

    Note
    ----
    This aint a subclass of `json.JSONDecodeError`; catch `ry.JSONDecodeError`
    or `ValueError`.

    Attributes
    ----------
    msg : str
        The unformatted error message.
    doc : Bytes | None
        The JSON document being parsed (or `None`); response bodies attach the
        json doc
    pos : int
        The start (byte) index of `doc` where parsing failed.
    lineno : int
        The line corresponding to `pos` (1-based).
    colno : int
        The column corresponding to `pos` (1-based).
    kind : str
        The error kind (e.g. `"trailing-comma"`).

    Examples
    --------
    >>> import ry
    >>> try:
    ...     ry.parse_json('{"a": 1,\n "b": 2,}')
    ... except ry.JSONDecodeError as e:
    ...     print(e)
    ...     print((e.msg, e.pos, e.lineno, e.colno, e.kind))
    trailing comma: line 2 column 9 (char 17)
    ('trailing comma', 17, 2, 9, 'trailing-comma')

    """

    def __new__(
        cls,
        kind: _JsonErrorKindArg,
        doc: ReadableBuffer | None = None,
        pos: int = 0,
        lineno: int | None = None,
        colno: int | None = None,
    ) -> t.Self: ...
    @property
    def msg(self) -> str: ...
    @property
    def doc(self) -> Bytes | None: ...
    @property
    def pos(self) -> int: ...
    @property
    def lineno(self) -> int: ...
    @property
    def colno(self) -> int: ...
    @property
    def kind(self) -> _JsonErrorKind: ...
