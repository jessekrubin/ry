"""ryo3-glob ~ types"""

import typing as t
from os import PathLike
from pathlib import Path

from ry.protocols import RyIterator
from ry.ryo3._fspath import FsPath

_TGlobPath = t.TypeVar("_TGlobPath", bound=str | Path | FsPath)

@t.final
class GlobPaths(RyIterator[_TGlobPath]):
    """glob::Paths iterable wrapper"""

    def __next__(self) -> _TGlobPath: ...
    def __iter__(self) -> GlobPaths[_TGlobPath]: ...
    def collect(self) -> list[_TGlobPath]: ...
    def take(self, n: int = 1) -> list[_TGlobPath]: ...

@t.overload
def glob(
    pattern: str,
    *,
    case_sensitive: bool = True,
    require_literal_separator: bool = False,
    require_literal_leading_dot: bool = False,
    strict: bool = False,
    dtype: None = None,
) -> GlobPaths[Path]: ...
@t.overload
def glob(
    pattern: str,
    *,
    case_sensitive: bool = True,
    require_literal_separator: bool = False,
    require_literal_leading_dot: bool = False,
    strict: bool = False,
    dtype: type[_TGlobPath],
) -> GlobPaths[_TGlobPath]: ...

@t.final
class GlobPattern:
    """Glob pattern matching ~ `::glob::Pattern`

    [DOCS](https://docs.rs/glob/latest/glob/struct.Pattern.html)

    Prefer the `::globset` wrappers (`Glob` | `Globset` | `Globster`).
    """
    def __new__(
        cls,
        pattern: str,
        *,
        case_sensitive: bool = True,
        require_literal_separator: bool = False,
        require_literal_leading_dot: bool = False,
    ) -> t.Self: ...
    def __call__(
        self,
        ob: str | PathLike[str],
        *,
        case_sensitive: bool | None = None,
        require_literal_separator: bool | None = None,
        require_literal_leading_dot: bool | None = None,
    ) -> bool: ...
    def matches(self, s: str) -> bool: ...
    def matches_path(self, path: PathLike[str]) -> bool: ...
    def matches_with(
        self,
        s: str,
        *,
        case_sensitive: bool | None = None,
        require_literal_separator: bool | None = None,
        require_literal_leading_dot: bool | None = None,
    ) -> bool: ...
    def matches_path_with(
        self,
        path: PathLike[str],
        *,
        case_sensitive: bool | None = None,
        require_literal_separator: bool | None = None,
        require_literal_leading_dot: bool | None = None,
    ) -> bool: ...
    @staticmethod
    def escape(pattern: str) -> str: ...
    @property
    def pattern(self) -> str: ...
    def __hash__(self) -> int: ...
