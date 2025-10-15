"""ryo3-glob types"""

import typing as t
from os import PathLike
from pathlib import Path

from ry.protocols import RyIterator
from ry.ryo3._fspath import FsPath

_T = t.TypeVar("_T", bound=str | Path | FsPath)

@t.final
class GlobPaths(RyIterator[_T]):
    """glob::Paths iterable wrapper"""

    def __next__(self) -> _T: ...
    def __iter__(self) -> GlobPaths[_T]: ...
    def collect(self) -> list[_T]: ...
    def take(self, n: int = 1) -> list[_T]: ...

@t.overload
def glob(
    pattern: str,
    *,
    case_sensitive: bool = True,
    require_literal_separator: bool = False,
    require_literal_leading_dot: bool = False,
    strict: bool = False,
) -> GlobPaths[Path]: ...
@t.overload
def glob(
    pattern: str,
    *,
    case_sensitive: bool = True,
    require_literal_separator: bool = False,
    require_literal_leading_dot: bool = False,
    strict: bool = False,
    dtype: type[_T],
) -> GlobPaths[_T]: ...

@t.final
class Pattern:
    def __init__(
        self,
        pattern: str,
        *,
        case_sensitive: bool = True,
        require_literal_separator: bool = False,
        require_literal_leading_dot: bool = False,
    ) -> None: ...
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
