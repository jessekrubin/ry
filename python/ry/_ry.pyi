"""ry type annotations"""

from os import PathLike

__version__: str
__build_profile__: str
__build_timestamp__: str

# ==============================================================================
# TYPE ALIASES
# ==============================================================================
JsonPrimitive = None | bool | int | float | str
JsonValue = (
    JsonPrimitive
    | dict[str, JsonPrimitive | JsonValue]
    | list[JsonPrimitive | JsonValue]
)

class FsPath:
    def __init__(self, path: str | None) -> None: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...
    def __eq__(self, other: object) -> bool: ...
    def __ne__(self, other: object) -> bool: ...

FsPathLike = str | FsPath | PathLike

def pwd() -> str: ...
def cd(path: FsPathLike) -> None: ...
def ls(path: FsPathLike | None) -> list[FsPath]: ...

# ==============================================================================
# JSON
# ==============================================================================
def parse_json(input: str | bytes) -> JsonValue: ...

# ==============================================================================
# FORMATTING
# ==============================================================================
def fmt_nbytes(nbytes: int) -> str: ...
