"""ryo3-glob types"""

import typing as t
from os import PathLike
from pathlib import Path

import typing_extensions as te

from ry.ryo3._fspath import FsPath

_T = t.TypeVar("_T", bound=str | Path | FsPath)

class _MatchOptions(t.TypedDict, total=False):
    case_sensitive: bool
    require_literal_separator: bool
    require_literal_leading_dot: bool

@t.final
class GlobPaths(t.Generic[_T]):
    """glob::Paths iterable wrapper"""

    def __next__(self) -> _T: ...
    def __iter__(self) -> GlobPaths[_T]: ...
    def collect(self) -> list[_T]: ...
    def take(self, n: int = 1) -> list[_T]: ...

@t.overload
def glob(
    pattern: str,
    *,
    case_sensitive: bool = False,
    require_literal_separator: bool = False,
    require_literal_leading_dot: bool = False,
) -> GlobPaths[Path]: ...
@t.overload
def glob(
    pattern: str,
    *,
    case_sensitive: bool = False,
    require_literal_separator: bool = False,
    require_literal_leading_dot: bool = False,
    dtype: type[_T],
) -> GlobPaths[_T]: ...
@t.final
class Pattern:
    def __init__(self, pattern: str) -> None: ...
    def __call__(
        self,
        ob: str | PathLike[str],
        **kwargs: te.Unpack[_MatchOptions],
    ) -> bool: ...
    def matches(self, s: str) -> bool: ...
    def matches_path(self, path: PathLike[str]) -> bool: ...
    def matches_with(
        self,
        s: str,
        **kwargs: te.Unpack[_MatchOptions],
    ) -> bool: ...
    def matches_path_with(
        self,
        path: PathLike[str],
        **kwargs: te.Unpack[_MatchOptions],
    ) -> bool: ...
    @staticmethod
    def escape(pattern: str) -> str: ...
    @property
    def pattern(self) -> str: ...
