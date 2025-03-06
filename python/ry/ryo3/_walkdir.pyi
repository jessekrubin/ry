"""ryo3-walkdir types"""

from __future__ import annotations

import typing as t
from os import PathLike

from ry import FileType, FsPath, Glob, GlobSet, Globster

class WalkDirEntry:
    def __fspath__(self) -> str: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...
    @property
    def path(self) -> FsPath: ...
    @property
    def file_name(self) -> str: ...
    @property
    def depth(self) -> int: ...
    @property
    def path_is_symlink(self) -> bool: ...
    @property
    def file_type(self) -> FileType: ...
    @property
    def is_dir(self) -> bool: ...
    @property
    def is_file(self) -> bool: ...
    @property
    def is_symlink(self) -> bool: ...
    @property
    def len(self) -> int: ...

class WalkdirGen:
    """walkdir::Walkdir iterable wrapper"""

    files: bool
    dirs: bool

    def __next__(self) -> str: ...
    def __iter__(self) -> t.Iterator[str]: ...
    def collect(self) -> list[str]: ...

def walkdir(
    path: str | PathLike[str] | None = None,
    *,
    files: bool = True,
    dirs: bool = True,
    contents_first: bool = False,
    min_depth: int = 0,
    max_depth: int | None = None,
    follow_links: bool = False,
    same_file_system: bool = False,
    glob: Glob | GlobSet | Globster | t.Sequence[str] | str | None = None,
) -> WalkdirGen: ...
