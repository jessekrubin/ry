"""ryo3-walkdir types"""

import typing as t
from os import PathLike

from ry import FileType, FsPath, Glob, GlobSet, Globster

@t.final
class WalkDirEntry:
    def __fspath__(self) -> str: ...
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

_T_walkdir = t.TypeVar(
    "_T_walkdir",
    bound=WalkDirEntry | str,
)

@t.final
class WalkdirGen(t.Generic[_T_walkdir]):
    """walkdir::Walkdir iterable wrapper"""
    def __init__(
        self,
    ) -> t.NoReturn: ...
    def __next__(self) -> _T_walkdir: ...
    def collect(self) -> list[_T_walkdir]: ...
    def take(self, n: int = 1) -> list[_T_walkdir]: ...
    def __iter__(self) -> t.Iterator[_T_walkdir]: ...

@t.overload
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
    objects: t.Literal[True],
) -> WalkdirGen[WalkDirEntry]: ...
@t.overload
def walkdir(
    path: str | PathLike[str] | None = None,
    *,
    objects: t.Literal[False] = False,
    files: bool = True,
    dirs: bool = True,
    contents_first: bool = False,
    min_depth: int = 0,
    max_depth: int | None = None,
    follow_links: bool = False,
    same_file_system: bool = False,
    glob: Glob | GlobSet | Globster | t.Sequence[str] | str | None = None,
) -> WalkdirGen[str]: ...
