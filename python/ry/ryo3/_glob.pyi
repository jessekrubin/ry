"""ryo3-glob types"""

from __future__ import annotations

import typing as t
from os import PathLike
from pathlib import Path

import typing_extensions as te

T = t.TypeVar("T")

class _MatchOptions(t.TypedDict, total=False):
    case_sensitive: bool
    require_literal_separator: bool
    require_literal_leading_dot: bool

class GlobPaths(t.Generic[T]):
    """glob::Paths iterable wrapper"""
    def __next__(self) -> T: ...
    def __iter__(self) -> t.Iterator[T]: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...
    def collect(self) -> list[T]: ...
    def take(self, n: int) -> list[T]: ...

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
