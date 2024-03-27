"""ry type annotations"""

from os import PathLike
from typing import AnyStr

__version__: str
__authors__: str
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
    def __init__(self, path: str | None = None) -> None: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...
    def __eq__(self, other: object) -> bool: ...
    def __ne__(self, other: object) -> bool: ...

FsPathLike = str | FsPath | PathLike[str]

def pwd() -> str: ...
def cd(path: FsPathLike) -> None: ...
def ls(path: FsPathLike | None = None) -> list[FsPath]: ...

# ==============================================================================
# SLEEP
# ==============================================================================
def sleep(seconds: float) -> float: ...
async def sleep_async(seconds: float) -> float: ...

# ==============================================================================
# FILESYSTEM
# ==============================================================================
def read_text(path: FsPathLike) -> str: ...
def read_bytes(path: FsPathLike) -> bytes: ...

# ==============================================================================
# WHICH
# ==============================================================================
def which(cmd: str, path: None | str = None) -> str | None: ...
def which_all(cmd: str, path: None | str = None) -> list[str]: ...

# ==============================================================================
# SHLEX
# ==============================================================================
def shplit(s: str) -> list[str]: ...

# ==============================================================================
# JSON
# ==============================================================================
def parse_json(input: str | bytes) -> JsonValue: ...
def parse_json_str(input: str) -> JsonValue: ...
def parse_json_bytes(input: bytes) -> JsonValue: ...

# ==============================================================================
# FORMATTING
# ==============================================================================
def fmt_nbytes(nbytes: int) -> str: ...

# ==============================================================================
# FNV
# ==============================================================================
class FnvHasher:
    def __init__(self, input: bytes | None = None) -> None: ...
    def update(self, input: bytes) -> None: ...
    def digest(self) -> int: ...
    def hexdigest(self) -> str: ...
    def copy(self) -> FnvHasher: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...

def fnv1a(input: bytes) -> FnvHasher: ...

# ==============================================================================
# DEV
# ==============================================================================
def anystr_noop(s: AnyStr) -> AnyStr: ...

# ==============================================================================
# BROTLI
# ==============================================================================
def brotli_encode(input: bytes, quality: int = 11) -> bytes: ...
def brotli_decode(input: bytes) -> bytes: ...
