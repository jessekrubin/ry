import typing as t
from os import PathLike

from ry.ryo3._fspath import FsPath

def pwd() -> str: ...
def home() -> str: ...
def cd(path: str | PathLike[str]) -> None: ...
@t.overload
def ls(
    path: str | PathLike[str] | None = None,  # defaults to '.' if None
    *,
    absolute: bool = False,
    sort: bool = False,
    objects: t.Literal[False] = False,
) -> list[str]:
    """List directory contents - returns list of strings"""

@t.overload
def ls(
    path: str | PathLike[str] | None = None,  # defaults to '.' if None
    *,
    absolute: bool = False,
    sort: bool = False,
    objects: t.Literal[True],
) -> list[FsPath]:
    """List directory contents - returns list of FsPath objects"""

def mkdir(path: str | PathLike[str]) -> None: ...
