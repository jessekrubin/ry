"""ryo3-glob types"""

import typing as t
from os import PathLike
from pathlib import Path

import typing_extensions as te

_T = t.TypeVar("_T")

class _MatchOptions(t.TypedDict, total=False):
    case_sensitive: bool
    require_literal_separator: bool
    require_literal_leading_dot: bool

class GlobPaths(t.Generic[_T]):
    """glob::Paths iterable wrapper"""
    def __next__(self) -> _T: ...
    def __iter__(self) -> t.Iterator[_T]: ...
    def collect(self) -> list[_T]: ...
    def take(self, n: int) -> list[_T]: ...

def glob(
    pattern: str,
    **kwargs: te.Unpack[_MatchOptions],
) -> GlobPaths[Path]:
    """Return glob iterable for paths matching the pattern."""

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
