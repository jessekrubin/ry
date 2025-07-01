"""ry-types"""

from __future__ import annotations

import datetime as dt
import sys
from os import PathLike
from typing import Protocol, TypedDict, TypeVar

if sys.version_info >= (3, 12):
    from collections.abc import Buffer
else:
    from typing_extensions import Buffer

__all__ = (
    "Buffer",
    "DateTimeTypedDict",
    "DateTypedDict",
    "FsPathLike",
    "TimeSpanTypedDict",
    "TimeTypedDict",
)

FsPathLike = str | PathLike[str]

T_co = TypeVar("T_co", covariant=True)


class ToPy(Protocol[T_co]):
    def to_py(self) -> T_co: ...


# protocol for function defining __json__() -> bytes / buffer:
class Stringify(Protocol):
    def __json__(self) -> Buffer | bytes | str: ...


# =============================================================================
# STD
# =============================================================================
class FileTypeDict(TypedDict):
    is_dir: bool
    is_file: bool
    is_symlink: bool


class MetadataDict(TypedDict):
    is_dir: bool
    is_file: bool
    is_symlink: bool
    len: int
    readonly: bool
    file_type: FileTypeDict | None
    accessed: dt.datetime
    created: dt.datetime
    modified: dt.datetime


# =============================================================================
# JIFF
# =============================================================================
class DateTypedDict(TypedDict):
    year: int
    month: int
    day: int


class TimeTypedDict(TypedDict):
    hour: int
    minute: int
    second: int
    millisecond: int
    microsecond: int
    nanosecond: int
    subsec_nanosecond: int


class DateTimeTypedDict(DateTypedDict, TimeTypedDict): ...


class TimeSpanTypedDict(TypedDict):
    years: int
    months: int
    weeks: int
    days: int
    hours: int
    minutes: int
    seconds: int
    milliseconds: int
    microseconds: int
    nanoseconds: int
